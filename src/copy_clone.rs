//! Session 2026-09-26 — "Which of my types COULD derive Copy?"
//! Everything here was verified against the compiler, not asserted.

// ---------------------------------------------------------------------------
// Rule 1 — a type is not Copy if any field is not Copy.
// ---------------------------------------------------------------------------

// compiles fine because everything is Copy
#[derive(Copy, Clone)]
struct Avocado { _ripeness: Ripeness, _price: f32 }

#[derive(Copy, Clone)]
enum Ripeness { _Green, _Perfection }          // unit variants carry nothing

// does not compile: String is not Copy, so the whole enum is out
// #[derive(Copy, Clone)] enum NumberOrString { S(String), N(i32) }

// ---------------------------------------------------------------------------
// Rule 2 — Drop blocks Copy outright, regardless of fields.
// This is the rarer rule. Rule 1 fires far more often.
// ---------------------------------------------------------------------------

struct DropNotifier(String);
impl Drop for DropNotifier { fn drop(&mut self) {} }
// does not compile: #[derive(Copy, Clone)] on a type with a Drop impl

// Rc is the std example: it has a Drop impl that decrements the refcount.
// If Rc were Copy the count would stay put while two Rcs existed — the second
// drop would free memory the first already freed.

// ---------------------------------------------------------------------------
// Rule 3 — derive writes a CONDITIONAL impl, not an unconditional one.
// The wrong prediction of the session: "derive puts Copy on MyBox and then
// ignores it for String". It doesn't. MyBox<String> is simply not Copy.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
struct MyBox<T>(T);
// derive expands to roughly: impl<T: Copy> Copy for MyBox<T> {}
// does not compile by hand: impl<T> Copy for MyBox<T> {}   "not restricted enough"

fn _needs_copy<T: Copy>(_x: T) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mybox_of_copy_type_is_copy() {
        let a = MyBox(5i32);
        _needs_copy(a);
        _needs_copy(a);                        // still usable: it was copied
    }

    // does not compile: MyBox<String> does not implement Copy at all.
    // The discriminator — _needs_copy only asks "is this Copy", it does not
    // copy anything. "Derived but ignored" would have compiled here.
    // #[test] fn mybox_of_string() { _needs_copy(MyBox(String::from("hi"))); }

    // -----------------------------------------------------------------------
    // Rule 4 — &T is always Copy. &mut T never is, even when T is Copy.
    // -----------------------------------------------------------------------

    #[derive(Copy, Clone)]
    struct SharedHolder<'a> { _r: &'a i32 }    // fine

    // does not compile: &mut i32 is not Copy.
    // #[derive(Copy, Clone)] struct MutHolder<'a> { r: &'a mut i32 }
    //
    // Why: Copy would let `let b = a;` leave `a` usable — two live &mut to the
    // same value, which is the one thing &mut exists to prevent.

    #[test]
    fn a_mut_reference_is_itself_a_value_that_moves() {
        let mut n = 5;
        let a = &mut n;
        let b = a;                             // the REFERENCE moved, not n
        *b += 1;
        assert_eq!(n, 6);                      // n was never moved, just mutated
        // `a` is unusable here — moved out of, like any non-Copy value
    }
}
