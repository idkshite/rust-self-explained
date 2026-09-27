# Parked

Tangents named during learning sessions. Next session starts here instead of a blank page.

## 2026-09-26 — session: "Which of my types COULD derive Copy?"

Pre-written at session start:

- **(b) What does `Copy` actually cost me?** ← the real elevator question. Provenance:
  derived `Copy + Clone` on `Level` because the methods took ownership and every call
  moved it. Still unresolved whether that was a cheat.
- **(c) Types where `Copy` is legal either way — what decides?**
- "My methods take ownership" — should they? Would borrowing remove the pressure to derive `Copy`?
- `Rc` / `Arc`
- Why cloning an `Arc` is cheap (looks like the Copy question, isn't)
- `Cow`
- `to_string()` vs `to_owned()`
- clone-in-a-closure / `move`

Raised mid-session:

- **"You can clone anything."** Claimed in passing, never tested. Is `Clone` universally
  derivable the way it felt? Find a type that can't be `Clone`.
- **Interior mutability: `Cell` vs `RefCell` vs `UnsafeCell`.** Reached the point that
  `UnsafeCell` refuses `Copy` because a bitwise duplicate through `&` could read bytes
  mid-write. "Interior mutability" itself still hand-wavy — "whatever that truly means".

### Added at close of session (2026-09-26)

- **Question (a) is unfinished — 14 of 23 types unpredicted.** Resume at batch 3:
  `GuestCount` / `Madness`, `MapIt<Iter, Func, Item>`, `LimitTracker<'a, T>`,
  `Car<T>` with `PhantomData`, `Cake` / `CakeRecipe`, plus `traits.rs` and the
  remaining `smart_pointers.rs` types. `LimitTracker` and `Car<T>` are the two
  predicted to trip you up.
- **Next session's card, already sharp:** "`&mut T` is a value that moves. Which
  of my functions actually need ownership, and which were just moving a `&mut`
  around?" — this is the elevator `Level` question in its real form.
- `PhantomData<T>` — does it affect whether the outer type can be Copy?
