//! Session 2026-09-30 — "Why does `*r = v` compile when `let x = *r` is refused?"
//!
//! THE ANSWER (user's words): `*r` is a noun, not an action. It names a place —
//! "the value `r` points at". Nothing happens just from writing it. What rustc
//! judges is the VERB you apply to that place:
//!
//!   assign to it     `*r = v`            ok   — old occupant is dropped, new one lands
//!   copy out of it   `*r` where `T: Copy` ok   — a copy leaves no hole
//!   borrow it        `r.push()`, `&*r`   ok   — you look, you don't take
//!   move out of it   `let x = *r`        NO   — leaves a hole in a place you don't own
//!
//! "The refused lines move the content out from behind the reference. The accepted
//! ones don't." Same rule as 2026-09-28's partial move: you don't own the place,
//! so you may not leave it empty — not even for one statement (case 3).
//!
//! Two things that looked like exceptions and aren't:
//!   - `*r` on an `i32` is a REAL dereference (case 4). Copy doesn't skip the deref,
//!     it just makes the read-out not a move.
//!   - `r.push('!')` is not a replacement (case 5). Nothing is dropped; it borrows.
//!
//! Verified against rustc and against `Drop`, not asserted. Run:
//!   cargo test deref -- --nocapture
#![allow(dead_code, unused_variables)]

// ---------------------------------------------------------------------------
// case 1 — the whole question. One reference, three verbs, two verdicts.
// ---------------------------------------------------------------------------

fn f(r: &mut String) {
    *r = String::from("new"); // assign  -> ok
    r.push('!'); // borrow  -> ok

    // does not compile: cannot move out of `*r` which is behind a mutable reference
    // let x = *r;            // move    -> refused
}

// ---------------------------------------------------------------------------
// case 2 — "does anything happen to the thing you dereferenced?"
// Yes: assigning to a place DROPS whatever was living there. `Drop` is the only
// instrument that makes a value's disappearance visible, so use it to look.
// Printed order proves the old value dies BEFORE the new one lands:
//   about to assign / dropped: first / assigned / ... / dropped: second
// ---------------------------------------------------------------------------

struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("  dropped: {}", self.0);
    }
}

fn overwrite(r: &mut Loud) {
    println!("  about to assign");
    *r = Loud("second");
    println!("  assigned");
}

// ---------------------------------------------------------------------------
// case 3 — the move and the refill must be ONE operation.
// This is why `mem::replace` exists: it is not a convenience, it is the only
// way to express "take it out and put something back" without an empty instant.
// ---------------------------------------------------------------------------

fn steal_with_replace(r: &mut String) -> String {
    std::mem::replace(r, String::new()) // ok — atomic swap
}

// does not compile: cannot move out of `*r` which is behind a mutable reference
// The refill on the next line does not save it. The hole would exist for one
// statement, and one statement is one too many.
// fn steal_in_two_steps(r: &mut String) -> String {
//     let old = *r;
//     *r = String::new();
//     old
// }

// ---------------------------------------------------------------------------
// case 4 — the `Copy` case, and the trap in it.
// PREDICTED WRONG: guessed `*r` on an i32 fails, then explained the compile by
// "we're not actually dereferencing". Both halves wrong. `p1` proves the deref
// is real and required — Copy changes the verb (copy, not move), not the `*`.
// ---------------------------------------------------------------------------

fn p2(r: &mut i32) -> i32 {
    *r // ok — a copy is read out of the place; no hole
}

// does not compile: mismatched types, expected `i32`, found `&mut i32`
// No auto-deref rescues this. The `*` in `p2` is doing real work.
// fn p1(r: &mut i32) -> i32 {
//     r
// }

// ---------------------------------------------------------------------------
// case 5 — a `&mut self` method is a BORROW, not a replacement.
// `bump` grows the value the way `push` does. If it worked by replacing the
// whole value, `Drop` would fire at the call. It does not.
// ---------------------------------------------------------------------------

struct Counter {
    name: &'static str,
    hits: u32,
}

impl Drop for Counter {
    fn drop(&mut self) {
        println!("  dropped: {}", self.name);
    }
}

impl Counter {
    fn bump(&mut self) {
        self.hits += 1;
    }
}

// The same verb with no mutation at all, which is why "mutates" is the wrong
// name for it and "borrows" is the right one.
fn how_long(r: &String) -> usize {
    r.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1_assign_and_borrow_are_fine() {
        let mut s = String::from("old");
        f(&mut s);
        assert_eq!(s, "new!");
    }

    #[test]
    fn case2_assigning_drops_the_old_occupant() {
        println!("--- enter");
        let mut l = Loud("first");
        overwrite(&mut l);
        println!("--- leaving scope");
        // expected:
        //   --- enter
        //     about to assign
        //     dropped: first      <- the old value dies here, mid-statement
        //     assigned
        //   --- leaving scope
        //     dropped: second
    }

    #[test]
    fn case3_replace_gets_the_value_out_legally() {
        let mut s = String::from("old");
        assert_eq!(steal_with_replace(&mut s), "old");
        assert_eq!(s, ""); // the place is not empty; it holds the new String
    }

    #[test]
    fn case4_copy_reads_out_without_a_hole() {
        let mut n = 7;
        assert_eq!(p2(&mut n), 7);
        assert_eq!(n, 7); // still there — copied, not taken
    }

    #[test]
    fn case5_a_mut_method_drops_nothing() {
        println!("--- enter");
        let mut c = Counter { name: "first", hits: 0 };
        c.bump();
        println!("--- leaving scope"); // `dropped: first` prints only AFTER this
        assert_eq!(c.hits, 1);
        assert_eq!(how_long(&String::from("abc")), 3);
    }
}
