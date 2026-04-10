# Layer 2 — Reasoning: Cognitive Model

Three-layer reasoning framework for approaching Rust problems systematically.

---

## Cognitive Model: Domain → Design → Mechanics

### 1. Domain Layer (What & Why)

Before writing code, clarify the problem domain:

- What is the input/output contract?
- What are the ownership semantics? (who owns what, for how long)
- What errors can occur and how should they be handled?
- What are the performance constraints?

### 2. Design Layer (How)

Select patterns and abstractions:

- Use `enum` for state machines and variants
- Use `trait` for shared behavior and polymorphism
- Use `struct` for data with invariants
- Use `type` aliases for complex signatures
- Use `mod` for encapsulation and API boundaries

### 3. Mechanics Layer (Implementation)

Write the code with correct Rust mechanics:

- Verify lifetimes with borrow checker
- Handle all `Result` and `Option` variants
- Ensure `Send` / `Sync` for concurrent code
- Use appropriate smart pointers (`Box`, `Rc`, `Arc`)
- Apply optimizations only after profiling

---

## Question Routing

Route questions through the appropriate layer:

| Question Type | Route To | Example |
|--------------|----------|---------|
| "Should this be owned or borrowed?" | Domain → Design | `String` vs `&str` |
| "Should I use a trait or enum?" | Design | polymorphism vs variants |
| "Why won't this compile?" | Mechanics | borrow checker errors |
| "Is this the right algorithm?" | Domain | Big-O, data structures |
| "How do I structure this module?" | Design | cohesion, coupling |

---

## Compiler Error Quick Reference

Common error patterns and resolutions:

### E0106 — Missing lifetime specifier
```rust
// Error: fn foo(x: &str, y: &str) -> &str
// Fix:   fn foo<'a>(x: &'a str, y: &'a str) -> &'a str
```

### E0499 — Cannot borrow as mutable more than once
```rust
// Error: &mut x then &mut x again
// Fix:   Restructure to reduce scope, or use split borrows
```

### E0502 — Cannot borrow as immutable because mutable borrow exists
```rust
// Error: let r = &mut x; let s = &x;
// Fix:   Drop mutable borrow first, or restructure
```

### E0308 — Type mismatch
```rust
// Error: expected X, found Y
// Fix:   Use `.into()`, explicit types, or fix logic
```

### E0277 — Trait bound not satisfied
```rust
// Error: T doesn't implement Trait
// Fix:   Add `where T: Trait` or derive/implement
```

### E0599 — No method found
```rust
// Error: x.method() doesn't exist
// Fix:   Check imports, trait in scope, or method name
```

### E0433 — Use of undeclared crate/module
```rust
// Error: use foo::bar;
// Fix:   Add to Cargo.toml, or use crate:: prefix
```

### E0382 — Use of moved value
```rust
// Error: x was moved, used again
// Fix:   Implement Copy, use Clone, or pass by reference
```

### async errors
```rust
// Error: future cannot be sent between threads safely
// Fix:   Bound with Send: T: Send + 'static
```
