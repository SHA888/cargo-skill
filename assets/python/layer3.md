# Python Layer 3 – RPI Loop & Verification Checklist

## 1. RPI (Run‑Plan‑Inspect) Loop for Python Development
The RPI loop defines the iterative workflow that ensures code quality at every step.

1. **Run** – Execute the command suite for the current change.
2. **Plan** – Review the output, identify failures or style violations, and decide on edits.
3. **Inspect** – Re‑run the suite to confirm the issue is resolved before committing.

### Command Sequence (in order)
Run these commands in the following order from the repository root:

1. **Format:** `uv run ruff format --check`  
   Ensures code adheres to style rules. Run this first to avoid cascading false positives in lint and type‑check.

2. **Lint:** `uv run ruff check`  
   Identifies unused imports, potential bugs, and style violations. Comes after format to avoid noise.

3. **Type Check:** `uv run mypy . --strict`  
   Enforces static type safety. Comes after lint since some lint fixes (e.g., removing unused imports) can affect type errors.

4. **Test:** `uv run pytest --doctest-modules`  
   Runs unit, integration, and doctest suites. Comes last to validate that all prior checks pass and behavior is correct.

These commands are fast enough to be run after each small edit, keeping the feedback loop tight. The sequence ensures that earlier checks do not produce false positives for subsequent checks.

## 2. Verification Checklist
Before marking a change as ready for PR, ensure the following checks all pass:
- **Formatting** – `uv run ruff format --check` reports no changes.
- **Linting** – `uv run ruff check` reports zero errors/warnings.
- **Static Typing** – `uv run mypy . --strict` reports no type errors.
- **Testing** – `uv run pytest --doctest-modules` runs all unit, integration, and doctest suites with a passing result.
- **Coverage** – Optional: `uv run coverage run -m pytest && uv run coverage report --fail-under=90` to enforce ≥90 % coverage for new code.
- **Performance** – If the change touches a hot path, run `uv run python -m cProfile -m pytest` and verify no regressions.

## 3. CI Integration
The same command set should be used in CI to guarantee parity between local and remote builds. Add a script `scripts/ci_check.sh` that runs the above commands in sequence and exits with a non‑zero status on failure.

## 4. Common Pitfalls
| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| `ruff format` reports changes | Code not formatted according to `ruff` rules | Run `uv run ruff format` locally and re‑stage changes |
| `mypy` errors on `Any` | Missing type annotation or implicit `Any` | Add explicit type or `# type: ignore` with issue link |
| Tests failing only on CI | Environment difference (e.g., missing dev dependencies) | Ensure `uv sync --frozen` is run in CI and local env |
| Coverage drop | New code lacks tests | Add unit tests covering new branches |

---
*Layer 3 completes the asset authoring series by codifying the development feedback loop and providing a concrete checklist for verification.*
