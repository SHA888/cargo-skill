# Layer 1 — Lookup: Rust Rule Index

Quick-reference rule categories for Rust development. Use `cargo skill lookup <prefix>` to filter.

---

## **own-** — Ownership & Borrowing

Priority: Critical

- **own-01** Prefer immutable borrows (`&T`) over mutable (`&mut T`) when possible
- **own-02** Use `clone()` sparingly; prefer references or `Arc` for shared ownership
- **own-03** Leverage `Cow<'_, T>` for clone-on-write scenarios
- **own-04** Drop borrows before mutating: restructure to reduce borrow scope
- **own-05** Use `std::mem::drop()` explicitly when early drop semantics matter

## **err-** — Error Handling

Priority: Critical

- **err-01** Use `Result<T, E>` for recoverable errors; `panic!` for unrecoverable bugs
- **err-02** Define custom error types implementing `std::error::Error`
- **err-03** Prefer `thiserror` for library errors, `anyhow` for application errors
- **err-04** Use `?` operator to propagate errors; avoid nested `match` noise
- **err-05** Include context with errors: `#[from]` + `#[error("...")]` attributes

## **mem-** — Memory Management

Priority: High

- **mem-01** Use `Box<T>` for heap allocation when size matters
- **mem-02** Use `Vec<T>` with `with_capacity()` when size is known
- **mem-03** Prefer `&str` over `String`, `&[T]` over `Vec<T>` for borrowing
- **mem-04** Use `std::mem::take()` for efficient value replacement
- **mem-05** Consider `parking_lot` for synchronization primitives

## **api-** — API Design

Priority: High

- **api-01** Follow RAII: constructors return `Self`, fallible ones return `Result`
- **api-02** Implement `Default` for types with sensible defaults
- **api-03** Use builder pattern for complex configuration
- **api-04** Return iterators instead of collections when possible
- **api-05** Mark `unsafe` APIs with safety documentation comments

## **async-** — Async/Await

Priority: High

- **async-01** Use `tokio` as default runtime; specify `rt-multi-thread` for apps
- **async-02** Prefer `spawn` for concurrent work; `join!` for waiting on multiple
- **async-03** Use `select!` for race conditions and cancellation
- **async-04** Avoid blocking operations in async context; use `spawn_blocking`
- **async-05** Pin futures only when necessary; prefer `Box::pin` for simplicity

## **opt-** — Option & Result

Priority: Medium

- **opt-01** Use `is_some()`/`is_none()` for checks; `map`/`and_then` for chaining
- **opt-02** Prefer `unwrap_or()` / `unwrap_or_default()` over `unwrap()`
- **opt-03** Use `ok_or()` / `ok_or_else()` to convert `Option` to `Result`
- **opt-04** Leverage `?` for early returns in `Option`-returning functions
- **opt-05** Use `filter()` + `map()` chains over nested `if let`

## **type-** — Types & Traits

Priority: Medium

- **type-01** Derive common traits: `Debug`, `Clone`, `PartialEq` where applicable
- **type-02** Use `AsRef<T>` / `Into<T>` for flexible function parameters
- **type-03** Implement `From<T>` to get `Into<T>` for free
- **type-04** Use associated types for single-implementation traits
- **type-05** Mark `PhantomData` fields for generic lifetime/type parameters

## **perf-** — Performance

Priority: Medium

- **perf-01** Profile before optimizing; use `cargo flamegraph`
- **perf-02** Use `iter()` / `into_iter()` appropriately; avoid `collect()` if possible
- **perf-03** Enable LTO and strip symbols in release builds
- **perf-04** Use `SmallVec` or `ArrayVec` for stack-allocated small collections
- **perf-05** Consider `memmap2` for large file I/O instead of buffered reads

## **test-** — Testing

Priority: Medium

- **test-01** Name tests `snake_case` with descriptive intent: `fn panics_on_invalid_input()`
- **test-02** Use `rstest` for parameterized tests
- **test-03** Mock with `mockall` or trait-based DI
- **test-04** Test error cases and edge conditions, not just happy path
- **test-05** Keep unit tests in `src/` files; integration tests in `tests/`

## **doc-** — Documentation

Priority: Medium

- **doc-01** Document all public APIs with rustdoc examples
- **doc-02** Use `/// # Errors` section to document error conditions
- **doc-03** Use `/// # Panics` section to document panic conditions
- **doc-04** Include `//!` crate-level documentation in `lib.rs` or `main.rs`
- **doc-05** Run `cargo doc` and fix all warnings before committing

## **name-** — Naming Conventions

Priority: Low

- **name-01** Use `SCREAMING_SNAKE_CASE` for constants and statics
- **name-02** Use `UpperCamelCase` for types, `snake_case` for functions/variables
- **name-03** Use `T`, `U` for generic types; `E` for error types; `I` for iterators
- **name-04** Avoid single-letter names except for loop indices and math
- **name-05** Prefix `_unused` for intentionally unused bindings

## **proj-** — Project Structure

Priority: Low

- **proj-01** One crate = one concern; use workspaces for multi-crate projects
- **proj-02** Put integration tests in `tests/` directory, not inline
- **proj-03** Use `benches/` for Criterion benchmarks
- **proj-04** Use `examples/` for demonstrating public API usage
- **proj-05** Keep `Cargo.toml` metadata up-to-date for crates.io

## **lint-** — Linting & Formatting

Priority: Low

- **lint-01** Run `cargo clippy -- -D warnings` in CI
- **lint-02** Run `cargo fmt --check` in CI
- **lint-03** Use `cargo deny` to check for banned/duplicate dependencies
- **lint-04** Enable `#![warn(missing_docs)]` in library crates
- **lint-05** Fix all `clippy::pedantic` warnings for public libraries

## **anti-** — Anti-patterns to Avoid

Priority: Critical

- **anti-01** Don't use `unwrap()` in production code
- **anti-02** Don't use `unsafe` without documented safety invariants
- **anti-03** Don't ignore `Result` values with `let _ = ...`
- **anti-04** Don't use `std::mem::forget` without explicit memory management need
- **anti-05** Don't implement `Drop` without considering `mem::needs_drop::<T>()`
