//! Session 2026-09-28 — "Why is a partial move out of `&mut self` refused?"
//!
//! THE ANSWER (user's words): a reference is a contract that lets me look at the
//! value, and if it's `&mut`, change it — but it must stay intact. It does not let
//! me destroy it. Moving a field out would leave a hole, and the owner lives
//! somewhere else and still expects a whole value.
//!
//! So the rule keys on ONE thing: do you own the place you are reading out of?
//!   - own it        -> the move is yours to make
//!   - hold `&` / `&mut` -> refused, and `&` vs `&mut` makes no difference
//!
//! Two things that look like exceptions and aren't:
//!   - `Copy` fields read fine through a reference. That was never a move.
//!   - `*player` on a `&mut Player` fails for the identical reason. The rule is
//!     about places behind references, not about fields specifically.
//!
//! Verified against rustc, not asserted. Run: `cargo test partial_move`.
#![allow(dead_code, unused_variables)]

struct Player {
    name: String, // not Copy — reading it out is a move
    hp: i32,      // Copy — reading it out is not
}

// ---------------------------------------------------------------------------
// case 1 — the whole question in two functions. Only the parameter differs.
// ---------------------------------------------------------------------------

fn take_name_owned(player: Player) -> String {
    player.name // ALLOWED. We own `player`; nobody else is waiting for it.
}

// does not compile: cannot move out of `player.name` which is behind a mutable reference
// fn take_name_borrowed(player: &mut Player) -> String {
//     player.name
// }

// does not compile either — `&` refuses identically, so mutability is NOT the cause.
// fn take_name_shared(player: &Player) -> String {
//     player.name
// }

// does not compile — and note the return is irrelevant. Binding it moves it too.
// The move is the read-out, not the handing-back.
// fn take_name_and_drop_it(player: &mut Player) -> String {
//     let n = player.name;
//     String::new()
// }

// ---------------------------------------------------------------------------
// case 2 — same borrow, Copy field. The apparent exception.
// ---------------------------------------------------------------------------

fn take_hp_borrowed(player: &mut Player) -> i32 {
    player.hp // ALLOWED. A copy leaves no hole, so the rule never applies.
}

// ---------------------------------------------------------------------------
// case 3 — the gdext shape. `self` is not special; it is just a parameter.
// ---------------------------------------------------------------------------

impl Player {
    fn into_name(self) -> String {
        self.name // ALLOWED. `self` by value = we own it.
    }

    // does not compile: cannot move out of `self.name` which is behind a mutable reference
    // This is the error every gdext `#[func]` method produces, because those methods
    // take `&mut self`.
    // fn borrow_name(&mut self) -> String {
    //     self.name
    // }
}

// The stress test: not a field at all, same refusal. `*player` tries to move the
// whole struct out from behind the reference.
// does not compile: cannot move out of `*player` which is behind a mutable reference
// fn steal_whole(player: &mut Player) -> Player {
//     *player
// }

// ---------------------------------------------------------------------------
// The other side of the coin, found while poking at case 3: calling an
// owning function does not move one field, it moves the WHOLE struct.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_move_is_fine() {
        let p = Player { name: "Ada".into(), hp: 10 };
        assert_eq!(take_name_owned(p), "Ada");
        // does not compile: borrow of moved value `p`
        //   p.hp
        // Not just `name` is gone — `p` entirely is. `take_name_owned` took the
        // struct, not the field.
    }

    #[test]
    fn copy_field_through_a_borrow_is_fine() {
        let mut p = Player { name: "Ada".into(), hp: 10 };
        assert_eq!(take_hp_borrowed(&mut p), 10);
        assert_eq!(p.hp, 10); // still there — it was copied, not taken
        assert_eq!(p.name, "Ada");
    }

    #[test]
    fn consuming_method_is_fine() {
        let p = Player { name: "Ada".into(), hp: 10 };
        assert_eq!(p.into_name(), "Ada");
    }
}
