# Layer 2 — Reasoning: Cognitive Model

Three-layer reasoning framework. **Do not answer immediately. Trace through layers first.**

---

## Cognitive Model

```
Layer 3 — Domain Constraints    WHY   (business rules, invariants, requirements)
Layer 2 — Design Choices        WHAT  (patterns, abstractions, architecture)
Layer 1 — Language Mechanics    HOW   (Rust syntax, borrow checker, types)
```

A compiler error (HOW) is a symptom. Trace upward: the design choice (WHAT) may be wrong
because the domain constraint (WHY) was misunderstood. Fix the root cause, not the symptom.

### Example trace

```
Problem: E0382 in a financial system

Layer 1 (HOW):  E0382 = use of moved value; value used after move
Layer 2 (WHAT): Trade records are being transferred when they should be shared
Layer 3 (WHY):  Financial audit records are immutable and must be accessible by
                multiple components simultaneously

Fix: Arc<TradeRecord>, not clone(); ownership model was wrong, not the syntax
```

---

## Three-Layer Reasoning Process

### Step 1 — Domain (WHY)

Ask before touching any code:

- What is the real-world invariant this code must preserve?
- Who owns this data and for how long?
- What happens if this operation fails?
- What are the concurrency requirements?
- What are the performance constraints and why do they exist?

### Step 2 — Design (WHAT)

Select the right abstraction:

- Use `enum` for mutually exclusive states; exhaustive match catches bugs at compile time
- Use `trait` for shared behavior across types that may vary independently
- Use `struct` with private fields for data with invariants to enforce
- Use `type` aliases to name complex signatures and improve readability
- Use `mod` boundaries to enforce encapsulation and define API surfaces
- Use typestate pattern when state transitions must be enforced at compile time

### Step 3 — Mechanics (HOW)

Write idiomatic Rust:

- Verify borrow lifetimes; restructure scope before adding `clone()`
- Handle all `Result` and `Option` variants explicitly
- Ensure `Send + Sync` bounds are correct for concurrent types
- Choose the right smart pointer: `Box` (unique heap), `Rc` (shared single-thread), `Arc` (shared multi-thread)
- Apply `#[inline]`, LTO, and other optimizations only after profiling confirms the need

---

## Question Routing

Route every question to the correct rule prefix before answering:

| Question | Layer | Rule prefix |
|---|---|---|
| Should this be owned or borrowed? | Domain → Design | `own-` |
| Should I use `Arc` or `Rc`? | Domain | `own-` |
| Should I use a trait or an enum? | Design | `type-`, `api-` |
| How do I handle this error? | Design → Mechanics | `err-` |
| Why won't this compile? | Mechanics | `own-`, `type-` |
| How do I structure this module? | Design | `proj-` |
| Is this the right data structure? | Domain → Design | `perf-`, `mem-` |
| How do I make this faster? | Domain → Mechanics | `opt-`, `perf-` |
| How do I write async code here? | Design → Mechanics | `async-` |
| How should I name this? | Mechanics | `name-` |
| Is this code correct? | All | `anti-`, `lint-` |

---

## Compiler Error Quick Reference

### E0106 — Missing lifetime specifier

```rust
// Error: fn longest(x: &str, y: &str) -> &str
// Cause: compiler cannot infer which input lifetime flows to output
// Fix:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

### E0277 — Trait bound not satisfied

```rust
// Error: T doesn't implement SomeTrait
// Cause: generic type lacks required bound
// Fix:
fn foo<T: SomeTrait>(x: T) { ... }
// or add where clause:
fn foo<T>(x: T) where T: SomeTrait { ... }
```

### E0308 — Type mismatch

```rust
// Error: expected X, found Y
// Cause: type inference resolved to wrong type, or logic error
// Fix: use .into(), explicit cast, or correct the logic
```

### E0382 — Use of moved value

```rust
// Error: value used here after move
// Cause: ownership transferred; original binding invalid
// Trace to domain: should this be shared (Arc), borrowed (&T), or cloned?
// Fix options:
let shared = Arc::new(value);     // shared ownership
let borrowed = &value;            // temporary borrow
let copy = value.clone();         // explicit copy (last resort)
```

### E0499 — Cannot borrow as mutable more than once

```rust
// Error: cannot borrow `x` as mutable more than once at a time
// Cause: two &mut references to same data in overlapping scopes
// Fix: reduce borrow scope, use split_at_mut(), or restructure
let (left, right) = slice.split_at_mut(mid); // safe split borrows
```

### E0502 — Cannot borrow as immutable because mutable borrow exists

```rust
// Error: cannot borrow `x` as immutable because it is also borrowed as mutable
// Cause: &mut borrow still live when & borrow attempted
// Fix: drop the &mut before creating &, or restructure scope
{
    let r = &mut x;
    do_something(r);
} // r dropped here
let s = &x; // now safe
```

### E0505 — Cannot move out of value because it is borrowed

```rust
// Error: cannot move out of `x` because it is borrowed
// Cause: attempting to transfer ownership while a borrow is live
// Fix: ensure all borrows are dropped before moving
```

### E0507 — Cannot move out of a shared reference

```rust
// Error: cannot move out of `*x` which is behind a shared reference
// Cause: dereferencing &T and trying to take ownership
// Fix: clone the value, or restructure to pass &T
let owned = (*x).clone();
```

### E0515 — Cannot return value referencing local variable

```rust
// Error: returns a value referencing data owned by the current function
// Cause: returning a reference to a local that will be dropped
// Fix: return owned data, or take a reference as parameter
fn foo(s: &str) -> &str { s } // borrow from input, not local
```

### E0597 — Borrowed value does not live long enough

```rust
// Error: `x` does not live long enough
// Cause: reference outlives the value it points to
// Fix: extend the lifetime of the value, or restructure ownership
```

### E0716 — Temporary value dropped while borrowed

```rust
// Error: temporary value dropped while borrowed
// Cause: taking a reference to a temporary that is immediately dropped
// Fix: bind the temporary to a let binding first
let temp = some_expression();
let reference = &temp; // temp lives for the block
```

### E0502 / async — Future cannot be sent between threads

```rust
// Error: future cannot be sent between threads safely
// Cause: async block captures non-Send data (e.g., Rc, raw pointer)
// Fix: use Arc instead of Rc; ensure all captured types are Send + 'static
tokio::spawn(async move {
    // all captured values must be Send + 'static
});
```

### E0433 — Failed to resolve: use of undeclared crate or module

```rust
// Error: use of undeclared crate or module `foo`
// Fix: add to Cargo.toml [dependencies], or use crate:: / super:: prefix
```

### E0599 — No method named X found

```rust
// Error: no method named `foo` found for type T
// Cause: trait not in scope, method doesn't exist, or wrong type
// Fix: bring trait into scope with `use`, check spelling, check type
use std::io::Write; // trait must be in scope to call its methods
```

---

## Unsafe Contract

Every `unsafe` block **must** have a `// SAFETY:` comment:

```rust
// SAFETY: <invariant that makes this safe>
// - <condition 1 that holds>
// - <condition 2 that holds>
unsafe { ... }
```

Example:

```rust
// SAFETY: index was checked against slice.len() above; guaranteed in bounds.
// - No other code holds a mutable reference to this slice.
unsafe { *slice.get_unchecked(index) }
```

No exceptions. `#![warn(clippy::undocumented_unsafe_blocks)]` enforces this in CI.
