# Layer 3 — Execution: RPI Loop

Research → Plan → Implement workflow for task execution.

---

## RPI Loop

### 1. Research

Gather context before coding:

- [ ] Read relevant code (callers, callees, tests)
- [ ] Understand invariants and preconditions
- [ ] Check existing patterns in the codebase
- [ ] Identify which rules from Layer 1 apply
- [ ] Note any `unsafe` or `async` boundaries

### 2. Plan

Design the solution:

- [ ] Write a one-sentence goal statement
- [ ] List the minimal changes needed
- [ ] Identify which modules/types need modification
- [ ] Plan error handling strategy
- [ ] Consider test coverage requirements

### 3. Implement

Execute the plan:

- [ ] Make the smallest change that works
- [ ] Follow the prefix rules from Layer 1
- [ ] Run `cargo check` after each meaningful edit
- [ ] Add or update tests as you go
- [ ] Refactor only after tests pass

---

## Verification Checklist

Before considering a task complete:

### Correctness
- [ ] `cargo test` passes (all existing + new tests)
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] Edge cases handled (empty input, errors, panics)

### Code Quality
- [ ] No `unwrap()` or `expect()` without justification
- [ ] All `unsafe` blocks have safety comments
- [ ] Public APIs documented with examples
- [ ] Error messages are actionable
- [ ] No compiler warnings

### Performance
- [ ] No unnecessary allocations in hot paths
- [ ] Appropriate data structures for the workload
- [ ] Async code doesn't block the runtime

---

## Task-to-Rule Mapping

Common tasks and the rules that apply:

| Task | Primary Rules | Secondary Rules |
|------|---------------|-----------------|
| Add new function | `api-01` to `api-05`, `name-02` | `doc-01` to `doc-05` |
| Add new type | `type-01` to `type-05`, `name-02` | `test-01` to `test-05` |
| Fix borrow error | `own-01` to `own-05` | `mem-01` to `mem-05` |
| Add error handling | `err-01` to `err-05` | `opt-01` to `opt-05` |
| Optimize code | `perf-01` to `perf-05` | `test-04` |
| Add async code | `async-01` to `async-05` | `err-03`, `type-04` |
| Refactor module | `proj-01` to `proj-05` | `lint-01` to `lint-05` |

---

## Safety Comments Template

For `unsafe` blocks, use this format:

```rust
// SAFETY: <invariant that makes this safe>
// - <condition 1>
// - <condition 2>
unsafe { ... }
```

Example:

```rust
// SAFETY: We hold a mutable reference, no other references exist
// - `ptr` was obtained from `Box::into_raw`
// - No other code accesses `ptr` while we hold this reference
unsafe { &mut *ptr }
```
