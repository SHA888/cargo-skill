# Layer 3 — Execution: RPI Loop

Research → Plan → Implement workflow for Python tasks.

---

## RPI Loop

### 1. Research

Gather context before coding:

- [ ] Read relevant code (callers, callees, tests, fixtures)
- [ ] Understand the trust boundary — where does this code sit relative to validated types?
- [ ] Check existing patterns in the codebase (validation, error handling, async style)
- [ ] Identify which prefix rules from Layer 1 apply (`own-`, `err-`, `type-`, ...)
- [ ] Note any sync/async boundaries or blocking I/O
- [ ] Verify Python version support; check `pyproject.toml` `requires-python`

### 2. Plan

Design the solution:

- [ ] Write a one-sentence goal statement
- [ ] List the minimal changes needed (files, functions, types)
- [ ] Pick the right abstraction: `pydantic` at boundary, `attrs`/`dataclass` inside
- [ ] Plan error handling: propagate, convert to domain error, or suppress?
- [ ] Identify required test coverage (unit, integration, property)
- [ ] If async: confirm no blocking I/O in the new path

### 3. Implement

Execute the plan:

- [ ] Make the smallest change that works
- [ ] Type-annotate every new signature (PEP 604 `|`, lowercase generics)
- [ ] Follow the prefix rules from Layer 1 — refer to specific rule IDs in commits
- [ ] Run the verification command sequence after each meaningful edit
- [ ] Add or update tests alongside the implementation
- [ ] Refactor only after tests pass

---

## Verification Command Sequence

Run **in this order** from the repository root after every meaningful edit:

```bash
uv run ruff format --check    # 1. format
uv run ruff check             # 2. lint
uv run mypy . --strict        # 3. type-check
uv run pytest                 # 4. tests
```

Why this order:

1. **Format first** — avoids cascading false positives in lint (a trailing comma may be invalid in old format but expected in new).
2. **Lint second** — removes unused imports / dead code before type-checking, so mypy doesn't waste effort on garbage.
3. **Type-check third** — runs over clean, formatted, linted code; errors are now signal not noise.
4. **Tests last** — validates behaviour with confidence that the previous gates passed.

The sequence is fast enough to run after every small edit. Use `pre-commit` to enforce locally.

---

## Verification Checklist

Before considering a task complete:

### Correctness
- [ ] `uv run pytest` passes (all existing + new tests)
- [ ] `uv run pytest --doctest-modules` passes (docstring examples run)
- [ ] `uv run mypy --strict` reports no errors
- [ ] Edge cases handled (empty input, validation failure, network error, cancellation)
- [ ] No unhandled `KeyError` / `AttributeError` paths

### Code Quality
- [ ] `uv run ruff format --check` reports no diff
- [ ] `uv run ruff check` reports zero warnings
- [ ] No `# type: ignore` without `[error-code]` and tracking issue
- [ ] No bare `except:` or `except Exception:` without re-raise
- [ ] Public APIs documented with Google-style docstring (`Args:`, `Returns:`, `Raises:`)
- [ ] No mutable default arguments

### Type Safety
- [ ] All public signatures annotated
- [ ] Domain identifiers use `NewType`, not raw `int` / `str`
- [ ] Untrusted input parsed with `pydantic` at the boundary
- [ ] No `Any` in public APIs (private inference OK)
- [ ] Optional values use `T | None`, not sentinel values

### Async (if touching async code)
- [ ] No blocking I/O inside `async def` (file open, requests, time.sleep)
- [ ] Blocking calls offloaded via `asyncio.to_thread` or `run_in_executor`
- [ ] `TaskGroup` or explicit cancellation for concurrent tasks
- [ ] Timeouts via `asyncio.timeout()` on external calls

### Performance (if change touches hot paths)
- [ ] Profile with `cProfile` + `snakeviz` before claiming a win
- [ ] No quadratic patterns (linear scan inside a loop, repeated `in list`)
- [ ] Generators over materializing lists for streaming data
- [ ] Built-ins (`sum`, `any`, `all`) over hand-rolled loops

---

## Task-to-Rule Mapping

Common Python tasks and the rule prefixes that apply:

| Task                              | Primary Rules                       | Secondary Rules            |
|-----------------------------------|-------------------------------------|----------------------------|
| Add a new function                | `api-`, `type-`, `name-`            | `doc-`, `test-`            |
| Add a new data type               | `api-` (pydantic/attrs), `own-`, `type-` | `name-`, `test-`      |
| Add input validation              | `api-04` (pydantic), `err-10`       | `type-`                    |
| Add error handling                | `err-`                              | `doc-05` (Raises section)  |
| Optimize a hot loop               | `opt-01` (profile first), `perf-`, `mem-` | `test-` (benchmark)  |
| Add async code                    | `async-`                            | `err-`, `type-`            |
| Refactor module structure         | `proj-`, `name-`                    | `lint-`                    |
| Tighten types                     | `type-`, `lint-` (mypy strict)      | `anti-`                    |
| Add tests for existing code       | `test-`                             | `doc-04` (doctest)         |
| Modernize legacy code             | `lint-03` (UP rules), `type-01` (PEP 604) | `anti-`              |
| Cache an expensive function       | `mem-10` (lru_cache), `mem-05` (weakref) | `perf-`               |
| Add a CLI                         | `proj-06` (entrypoints)             | `api-`, `err-`             |

---

## Common Pitfalls

| Symptom                                          | Root Cause                                       | Fix |
|--------------------------------------------------|--------------------------------------------------|-----|
| `mypy: Incompatible return value type`           | Annotation drifted from implementation           | Update one or the other; re-run mypy |
| `ruff format` reports changes locally not in CI  | Different `ruff` versions                        | Pin `ruff` in `dependency-groups.dev`; commit `uv.lock` |
| Tests pass locally, fail in CI                   | Hidden dependency on env / file system / time    | Use `monkeypatch`, `tmp_path`, `freezegun` |
| Coverage drops after refactor                    | New branches lack tests                          | Add parametrized cases; check `coverage --branch` |
| Async test hangs                                 | Forgot `@pytest.mark.asyncio` or `asyncio_mode = "auto"` | Set `asyncio_mode = "auto"` in `pyproject.toml` |
| `RuntimeError: There is no current event loop`   | Mixing `asyncio.get_event_loop()` and `asyncio.run` | Use `asyncio.get_running_loop()` inside async code only |
| Memory grows linearly under load                 | Cache without bound, or held references          | `lru_cache(maxsize=N)`; `weakref`; profile with `tracemalloc` |
| `pickle.UnpicklingError` from external source    | Pickle on untrusted data                         | Replace with JSON or `pydantic.model_validate_json` |

---

## Commit Hygiene

Reference rules in commit messages when relevant:

```
fix: validate user input at boundary

Move email validation from scattered checks into a pydantic.EmailStr
field on UserCreateRequest (api-04, api-15). Internal callers now
receive a validated email and can drop their own checks.
```

This creates a searchable audit trail: future contributors can grep `api-04` to find every place that boundary validation was applied.

---

## Pre-commit Configuration

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.9.0
    hooks:
      - id: ruff-format
      - id: ruff
        args: [--fix]

  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.11.0
    hooks:
      - id: mypy
        args: [--strict]
        additional_dependencies: [pydantic, types-requests]
```

Run `uv run pre-commit install` once per clone. Hooks fire on every `git commit` and block bad code before it lands.
