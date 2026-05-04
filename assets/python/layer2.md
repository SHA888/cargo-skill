# Python Layer 2 – Cognitive Model & Type System

## 1. Structural vs. Nominal Typing
- **Structural (duck) typing** is expressed via `typing.Protocol`. A type is satisfied if it implements the required attributes/methods, regardless of explicit inheritance.
- **Nominal typing** uses concrete base classes or `typing.NewType` for branding. Use when the identity of the type matters (e.g., `UserId`, `OrderId`).
- Prefer **Protocol** for public APIs that accept any object with the right shape; reserve nominal types for domain identifiers.

## 2. Common Type Errors & Quick‑Reference
| Mypy Error Code | Typical Cause | Fix |
|-----------------|---------------|-----|
| `error: Incompatible return value type` | Function annotated to return `int` but returns `str` | Align return expression or adjust annotation |
| `error: Argument 1 to "foo" has incompatible type` | Passing wrong type to a function | Cast or convert, or widen the parameter type |
| `error: Missing type parameters for generic type` | Using `list` instead of `list[int]` | Provide concrete type arguments |
| `error: "Any" type used` | Implicit `Any` from untyped code | Add explicit annotations or `# type: ignore` with issue link |
| `error: Incompatible overload` | Overload signatures conflict | Ensure each overload is a strict superset/subset |

## 3. GIL Implications & Concurrency Model
- **CPU‑bound work** should use **process‑based parallelism** (`multiprocessing`, `concurrent.futures.ProcessPoolExecutor`). The Global Interpreter Lock (GIL) prevents true parallel threads for CPU work.
- **I/O‑bound work** benefits from **asyncio** or libraries built on it (`httpx`, `anyio`). No GIL contention as tasks yield on await.
- When mixing, keep **async** code pure (no blocking calls). Offload blocking calls to a thread pool (`run_in_executor`).

## 4. Error Handling Patterns
- Use **exception classes** for domain errors (`class ValidationError(RuntimeError): …`).
- Prefer **`Result`‑like** patterns with `typing.Union[Success, Failure]` for functions where errors are expected and part of the control flow.
- Never swallow exceptions silently; log with context and re‑raise or convert.

## 5. Packaging & Distribution Checklist
- Ensure `pyproject.toml` includes `[tool.uv]` with `requires = ["<deps>"]`.
- Use `uv build` to create wheels; test install in a clean venv.
- Declare **extras** for optional features (e.g., `dev = ["ruff", "pytest"]`).

## 6. Testing Strategies
- **Unit tests**: pure functions, fast, no external resources.
- **Integration tests**: spin up temporary resources (databases, HTTP servers) using fixtures (`pytest-asyncio`, `pytest-docker`).
- **Property‑based tests**: use `hypothesis` to generate diverse inputs, catching edge cases.

## 7. Performance Tips Specific to Python
- Use **list comprehensions** and **generator expressions** instead of manual loops where possible.
- Leverage **built‑in functions** (`sum`, `any`, `all`) which are implemented in C.
- For heavy numeric work, consider **NumPy** or **Cython** extensions.
- Profile with `uv run python -m cProfile -m pytest` and visualize with `snakeviz`.

## 8. Documentation Practices
- Adopt **Google style docstrings** with sections `Args`, `Returns`, `Raises`.
- Include **type hints** directly in signatures; docstring types are redundant.
- Generate API reference with `pdoc` and host under `docs/`.

## 9. Naming Conventions Recap
- **Modules**: `snake_case.py`
- **Classes**: `PascalCase`
- **Functions/variables**: `snake_case`
- **Constants**: `UPPER_SNAKE_CASE`
- **Type aliases**: `CamelCase` (e.g., `UserId = NewType('UserId', int)`).

## 10. Linting Rules (ruff)
- Enforce `E501` (line length) with soft limit 99.
- Disallow `F401` (unused imports) and `F403` (import *).
- Require `# noqa` comments to include a justification URL or issue link.
- Prohibit `type: ignore` without a comment linking to an issue.

## 11. Anti‑Patterns Revisited
- **Over‑using `Any`** – defeats static analysis; replace with `Protocol` or concrete types.
- **Deep inheritance hierarchies** – favor composition.
- **Mixing sync I/O in async functions** – leads to blocking the event loop.
- **Hard‑coded paths** – use `pathlib.Path` and configuration.

---
*Layer 2 builds on Layer 1 by introducing the cognitive model for typing, concurrency, and error handling. Subsequent layers will cover CI pipelines, advanced async patterns, and performance benchmarking.*