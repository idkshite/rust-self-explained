//! Session 2026-09-27 — "When does `Copy` let a bug compile that a move error
//! would have caught?"
//!
//! THE ANSWER: only where the original gets used after it was handed over.
//! `Copy` does not cause errors, it removes one — and the error it removes is
//! never "you lost a write", it is always "you read a value that was moved out".
//! So sort any snippet into three buckets:
//!
//!   1. rustc rejects it without `Copy`; with `Copy` it compiles and silently
//!      hands back a stale value          -> case 1, case 3.  Copy cost the error.
//!   2. compiles either way, same wrong value, because nothing reads the
//!      original afterwards               -> case 2.  The error was never there.
//!   3. `Copy` is irrelevant: it is a borrow, not a hand-over
//!                                        -> case 4.
//!
//! Verified against the compiler, not asserted. Run: `cargo test copy_cost`.
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// case 1 — a function that means to raise a Level.
// Your elevator's Level, verbatim.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
struct Level(i32);

fn ascend(mut level: Level) {
    level.0 += 1;
}

// case 1, step 3 — the same type and the same function, minus Copy.
// Only the derive changed.
#[derive(Clone, Debug, PartialEq)]
struct MoveLevel(i32);

fn ascend_moved(mut level: MoveLevel) -> MoveLevel {
    level.0 += 1;
    level
}

// ---------------------------------------------------------------------------
// case 2 — the same lost write, but nobody reads the original afterwards.
// ---------------------------------------------------------------------------

fn ascend_moved_discard(mut level: MoveLevel) {
    level.0 += 1;
}

// ---------------------------------------------------------------------------
// case 3 — no function call at all. A field copied out of a struct.
// This is the shape that appears in the elevator: read cart.level, work on it,
// forget to write it back.
// ---------------------------------------------------------------------------

struct Cart {
    level: Level,
}

struct MoveCart {
    level: MoveLevel,
}

// ---------------------------------------------------------------------------
// case 4 — the same intent, expressed as a borrow instead of a hand-over.
// ---------------------------------------------------------------------------

fn ascend_ref(level: &mut Level) {
    level.0 += 1;
}

fn ascend_moved_ref(level: &mut MoveLevel) {
    level.0 += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1_copy_version() {
        let l = Level(0);
        ascend(l);
        // the write landed on the copy; l never saw it
        assert_eq!(l, Level(0));
    }

    // case 1, step 3 — without Copy this refused to compile until the value
    // was handed back. The move checker forced the fix.
    #[test]
    fn case1_move_version() {
        let mut l = MoveLevel(0);
        l = ascend_moved(l);
        assert_eq!(l, MoveLevel(1));
    }

    #[test]
    fn case2_copy_version() {
        let l = Level(0);
        ascend(l);
        // nothing reads l after this line
    }

    // case 2, move version — PREDICT before uncommenting.
    #[test]
    fn case2_move_version() {
        let l = MoveLevel(0);
        ascend_moved_discard(l);
        // nothing reads l after this line
    }

    #[test]
    fn case3_copy_version() {
        let cart = Cart { level: Level(0) };
        let mut l = cart.level;
        l.0 += 1;
        // PREDICT: fill in the number, then run.
        assert_eq!(cart.level, Level(0));
    }

    // case 3, move version — does not compile. E0382 fires on the READ, not on
    // the move out of the struct; the move line is only listed as context.
    // #[test]
    // fn case3_move_version() {
    //     let cart = MoveCart { level: MoveLevel(0) };
    //     let mut l = cart.level;
    //     l.0 += 1;
    //     assert_eq!(cart.level, MoveLevel(0));   // <-- error[E0382] here
    // }

    #[test]
    fn case4_copy_version() {
        let mut l = Level(0);
        ascend_ref(&mut l);
        // PREDICT: fill in the number, then run.
        assert_eq!(l, Level(1));
    }

    // case 4, move version — identical result. Copy is irrelevant to a borrow.
    #[test]
    fn case4_move_version() {
        let mut l = MoveLevel(0);
        ascend_moved_ref(&mut l);
        assert_eq!(l, MoveLevel(1));
    }
}
