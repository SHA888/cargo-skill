# Python Layer 1 – Asset Authoring

## Overview
This document defines the foundational guidelines for authoring Python assets in the **cargo‑skill** repository. It focuses on coding style, type safety, async patterns, packaging, testing, performance, documentation, naming, project layout, linting, and anti‑patterns.

## 1. Coding Style (PEP 8)
- Use 4‑space indentation.
- Limit lines to **79 characters** (soft limit 99 for comments/docstrings).
- Prefer `import … as …` only when it improves readability.
- Blank lines: two before top‑level definitions, one inside functions/classes.

## 2. Type Annotations (PEP 484)
- Annotate **all public** functions, methods, and class attributes.
- Use `typing` primitives (`List`, `Dict`, `Tuple`, `Optional`, `Union`, `Literal`).
- Prefer **`Protocol`** for structural subtyping over concrete base classes.
- Use **`StrEnum`** or `Literal` for string‑based enums.

### 2.1 Validation at Boundaries: Pydantic & attrs
- Use **Pydantic** (`BaseModel` with `model_validate()`) for HTTP payloads, JSON, and untrusted input. Set `ConfigDict(frozen=True)` for immutability.
- Use **attrs** (`@define` with `frozen=True`) for internal domain types that do not require serialization.
- Both enforce **parse‑don't‑validate**: parsing produces a valid type or raises `ValidationError` / `ValueError`; no strings smuggled into domain logic.
- Derive `__eq__`, `__hash__`, `__repr__` automatically; do not hand‑write them.

## 3. Async Guidelines
- Use `async def` for I/O‑bound operations (network, file, DB).
- Propagate cancellation with `await` and avoid blocking calls inside async code.
- Prefer `anyio`/`trio`‑compatible abstractions; fallback to `asyncio` when needed.

## 4. Packaging (PEP 517/518)
- Declare build‑system in `pyproject.toml` (`[build-system]` with `requires = ["setuptools", "wheel"]`).
- Keep runtime dependencies minimal; pin exact versions in `uv.lock`.
- Use **editable installs** (`uv pip install -e .`) for local development.

## 5. Testing (pytest)
- Write **unit tests** for pure functions; **integration tests** for async I/O.
- Place tests under `tests/` mirroring the source layout.
- Use `pytest.mark.asyncio` for async test functions.
- Enforce **100 % coverage** for new modules (`uv run coverage run -m pytest`).

## 6. Performance (Perf‑First)
- Profile with `uv run python -m cProfile -m pytest`.
- Optimize only after a measurable bottleneck is identified.
- Prefer built‑in data structures (`list`, `dict`) and comprehensions over manual loops.

## 7. Documentation
- Write docstrings in **Google style**; include `Args`, `Returns`, `Raises`.
- Generate API docs with `uv run pdoc -o docs src/`.
- Keep module‑level docs concise and up‑to‑date.

## 8. Naming Conventions
- **Modules**: `snake_case.py`
- **Classes**: `PascalCase`
- **Functions/variables**: `snake_case`
- **Constants**: `UPPER_SNAKE_CASE`
- Avoid ambiguous names (`data`, `result`) unless context is crystal clear.

## 9. Project Layout
```
src/
├── __init__.py
├── core/
│   └── …          # Core domain logic
└── utils/
    └── …          # Helper utilities
tests/
└── …              # Test suite mirroring src/
```

## 10. Linting (ruff)
- Run `uv run ruff format --check` and `uv run ruff check`.
- Enforce **no unused imports**, **no `# noqa` without comment**, and **no `type: ignore`** unless justified with an issue link.

## 11. Error Handling
- Define custom exception classes inheriting from `RuntimeError` or a domain base: `class ValidationError(RuntimeError): pass`.
- Do not silence exceptions; if caught, log with context and re‑raise or convert to a domain exception.
- For functions where errors are expected and part of control flow, use **Result‑like** patterns: `Union[Success, Failure]` with dataclasses or typed tuples.
- Never catch broad `Exception` or `BaseException`; catch specific types.

## 12. Anti‑Patterns
| Pattern | Why it's bad | Remedy |
|---------|--------------|--------|
| `# TODO` without issue link | Untracked work | Create an issue and reference it |
| `type: ignore` without comment | Silences real errors | Add justification or fix the type |
| Mixing sync and async I/O in same function | Leads to deadlocks | Separate sync/async paths |
| Over‑use of `Any` | Loses type safety | Use concrete types or `Protocol` |
| Deeply nested `if`/`else` | Reduces readability | Refactor into early returns or helpers |
| Silent exception catch (`except: pass`) | Untraced failures | Log with context; convert to domain error |
| Returning booleans for success/failure | Lost error context | Return `Union[Success, Failure]` or raise exception |

## 13. Verification Checklist
- `uv run ruff format --check`
- `uv run ruff check`
- `uv run mypy . --strict`
- `uv run pytest --doctest-modules`
- Run performance profile if new code touches hot paths.

---
*This file is part of the asset authoring series. Subsequent layers will expand on advanced topics such as structural typing, GIL considerations, and detailed error‑code references.*
