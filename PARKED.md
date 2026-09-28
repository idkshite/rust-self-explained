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

## 2026-09-27 — session: "When does `Copy` let a bug compile that a move error would have caught?"

Pre-written at session start (the two costs split off the compound question):

- **What does deriving `Copy` lock me out of later?** No `Drop`, no non-`Copy` field ever,
  and removing it breaks callers. Fully mechanical — `cargo check` decides.
- **At what size does passing a `Copy` type by value start to cost?** `Level(i32)` vs a fat
  `Copy` struct. Judge is a benchmark or the generated asm.

Raised mid-session:

- ⭐ **When is a partial move refused?** Moving a field out of an owned struct is allowed —
  `cart` just becomes partially moved. What makes it illegal? (`Drop` impl suspected,
  borrowed `cart` suspected. Untested.)
- ⭐ **When do I need to dereference to write through a reference, and when does `level.0 += 1`
  on a `&mut Level` just work?** Raised at case 4. Auto-deref suspected; untested.

## 2026-09-28 — session: "Why is a partial move out of `&mut self` refused?"

Split off the compound "when is a partial move refused?" — these are the siblings
NOT taken this session. Each is mechanical; `cargo check` decides each one.

- **(b) Does `impl Drop` make an otherwise-legal partial move illegal?** Own the struct,
  no reference involved, but it has a destructor. Suspected yes since 2026-09-27, untested.
- **(d) Field out of a struct behind a smart pointer.** `Box<T>` vs `Rc<T>` vs `RefCell<T>`
  vs gdext's ⭐ `Gd<T>` — which permit moving a field out, which refuse, and why they differ.
  Closest sibling to the real gdext problem.
- **(e) Life after a partial move.** Once one field has left, what can you still read,
  borrow, or pass? About the state afterwards, not the refusal.

Raised mid-session (2026-09-28):

- **How do I actually get the field out in a gdext `&mut self` method?** The refusal is
  now understood; the workaround is not. Candidates named by rustc itself: `clone()`.
  Others suspected: `std::mem::take`, `std::mem::replace`, `Option::take`. Untested.
- **Passing one field instead of the whole struct.** Calling an owning function moves the
  WHOLE struct, not the field — so `take_name_owned(p)` costs you `p.hp` too. Would a
  function that takes only `name: String` be the right shape? When is that the fix?
- **`*` on a reference.** Confirmed `*player` cannot move a non-`Copy` value out. Still no
  mental model of what deref is *for* — reading through, writing through, and moving out
  are apparently three different things. Overlaps the starred auto-deref question from 2026-09-27.
