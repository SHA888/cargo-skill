# Layer 1 — Lookup: Python Rule Index

Quick-reference rule index for modern Python development. Use `cargo skill lookup py:<prefix>` to filter.

Targets Python 3.11+ with modern tooling: `uv`, `ruff`, `mypy --strict`, `pytest`, `pydantic` v2, `attrs`, `anyio`, `httpx`, `hypothesis`.

Priority: CRITICAL > HIGH > MEDIUM > LOW > REF

---

## **own-** — Mutability & References (CRITICAL)

- **own-01** `immutable-default` — Default to immutable: `tuple` over `list`, `frozenset` over `set` for shared data
- **own-02** `no-mutable-defaults` — Never use mutable default args (`def f(x=[])`); use `None` and create inside
- **own-03** `no-mutable-class-attr` — Never use mutable class attributes; they're shared across instances
- **own-04** `frozen-dataclass` — Use `@dataclass(frozen=True)` for value objects; gives free `__hash__`
- **own-05** `frozen-attrs` — Use `attrs.frozen()` for immutable internal types; cheaper than `dataclass`
- **own-06** `pydantic-frozen` — Set `ConfigDict(frozen=True)` on pydantic models for immutable I/O types
- **own-07** `deepcopy-explicit` — Use `copy.deepcopy()` explicitly; never rely on slicing for nested copy
- **own-08** `dict-copy-explicit` — Use `dict(d)` or `{**d}` for shallow copy; `d1 = d2` is a reference, not a copy
- **own-09** `list-copy-explicit` — Use `list(l)` or `l.copy()`; never `l1 = l2` when you want a copy
- **own-10** `mappingproxy-readonly` — Use `types.MappingProxyType(d)` for read-only views into a dict
- **own-11** `tuple-record` — Use `tuple` or `NamedTuple` for fixed-shape records that won't mutate
- **own-12** `frozen-set-keys` — Use `frozenset` as dict keys when the key is a set of values

---

## **err-** — Error Handling (CRITICAL)

- **err-01** `specific-except` — Catch specific exception types; never `except:` or bare `except Exception`
- **err-02** `domain-exceptions` — Define a per-domain exception base: `class OrderError(Exception): pass`
- **err-03** `raise-from-chain` — Use `raise NewError(...) from original` to preserve the cause chain
- **err-04** `no-silent-pass` — Never `except X: pass` without logging; use `contextlib.suppress(X)` if truly intentional
- **err-05** `logger-exception` — Use `logger.exception(msg)` inside `except` to capture traceback automatically
- **err-06** `fail-fast` — Raise immediately on invariant violations; never return `None` or sentinels as errors
- **err-07** `errorgroup-311` — Use `ExceptionGroup` / `except*` (Python 3.11+) to aggregate concurrent failures
- **err-08** `result-pattern` — Return `Result[T, E]` (tagged dataclasses) when failure is part of normal control flow
- **err-09** `no-assert-validation` — Never use `assert` for input validation; `python -O` strips it. Raise explicitly
- **err-10** `pydantic-boundary` — Use `pydantic.ValidationError` at I/O boundaries; convert to domain errors internally
- **err-11** `errors-section` — Document every raised exception in the docstring `Raises:` section
- **err-12** `narrow-suppress` — Wrap `contextlib.suppress(SpecificError)` around the smallest possible block
- **err-13** `retry-with-backoff` — Use `tenacity` or `stamina` for retry logic; never hand-roll retry loops
- **err-14** `cleanup-finally` — Use `finally` or `with` for cleanup that must run regardless of error path

---

## **mem-** — Memory Optimization (CRITICAL)

- **mem-01** `slots-dataclass` — Use `@dataclass(slots=True)` (3.10+) for memory-tight types; saves ~40% per instance
- **mem-02** `slots-class` — Define `__slots__ = ("a", "b")` on hot classes to skip `__dict__` overhead
- **mem-03** `generator-stream` — Use generators (`yield`) over returning a list; constant memory for streams
- **mem-04** `itertools-pipeline` — Chain `itertools.islice`, `chain`, `groupby` for streaming transformations
- **mem-05** `weakref-cache` — Use `weakref.WeakValueDictionary` for caches that shouldn't pin objects in memory
- **mem-06** `array-numeric` — Use `array.array('i', ...)` for homogeneous numeric data; 4x smaller than `list`
- **mem-07** `deque-fifo` — Use `collections.deque(maxlen=N)` for bounded FIFOs with O(1) pop from both ends
- **mem-08** `numpy-bulk` — Use NumPy arrays for large numeric datasets; ~10x smaller than equivalent `list[float]`
- **mem-09** `frozenset-large` — Use `frozenset` for large immutable sets; lighter and shareable across instances
- **mem-10** `lru-cache-bounded` — Use `functools.lru_cache(maxsize=N)` with a bound, never `maxsize=None` on long-lived funcs
- **mem-11** `gc-collect-batch` — Disable `gc` (`gc.disable()`) for batch jobs that allocate many short-lived objects
- **mem-12** `sys-getsizeof` — Profile with `sys.getsizeof()`, `pympler.asizeof`, or `tracemalloc` before optimizing
- **mem-13** `del-large-locals` — `del large_obj` inside hot functions to release memory before returning

---

## **api-** — API Design (HIGH)

- **api-01** `protocol-structural` — Use `typing.Protocol` for structural subtyping; no inheritance required
- **api-02** `runtime-checkable-rare` — Add `@runtime_checkable` only when you actually need `isinstance()` checks
- **api-03** `newtype-ids` — Wrap primitive IDs: `UserId = NewType("UserId", int)`; prevents mixing distinct IDs
- **api-04** `pydantic-io-boundary` — Use `pydantic.BaseModel.model_validate()` at every I/O boundary (HTTP, JSON, env)
- **api-05** `attrs-internal` — Use `@attrs.define` for internal domain types not requiring serialization
- **api-06** `dataclass-plain` — Use `@dataclass` for plain data with no validation; standard library, zero deps
- **api-07** `typeddict-shape` — Use `TypedDict` for dicts with a known shape; cheaper than dataclass for short-lived
- **api-08** `keyword-only-star` — Add `*` in signatures to force keyword-only args: `def f(x, *, opt=None)`
- **api-09** `positional-only-slash` — Use `/` for positional-only when API stability matters (3.8+)
- **api-10** `final-class` — Use `@final` on classes not designed for inheritance; documents intent + checked by mypy
- **api-11** `abc-true-abstract` — Use `ABC` + `@abstractmethod` only for true polymorphism; prefer `Protocol` otherwise
- **api-12** `overload-multi-sig` — Use `@overload` for functions with multiple type signatures (e.g., overloaded return)
- **api-13** `self-fluent` — Use `typing.Self` (3.11+) for fluent / builder return types
- **api-14** `paramspec-decorator` — Use `ParamSpec` so decorators preserve the wrapped function's signature
- **api-15** `parse-dont-validate` — Parse untrusted input into a validated type at the boundary; downstream trusts the type
- **api-16** `factory-classmethod` — Use `@classmethod` factories (`from_dict`, `from_env`) instead of overloaded `__init__`

---

## **async-** — Async/Await (HIGH)

- **async-01** `anyio-portable` — Prefer `anyio` for library code; works on both `asyncio` and `trio`
- **async-02** `asyncio-app` — Use `asyncio` directly when the application is CPython-only
- **async-03** `no-blocking-io` — Never call blocking I/O (`requests`, `open`, `time.sleep`) inside `async def`
- **async-04** `to-thread` — Use `asyncio.to_thread(fn, *args)` (3.9+) to offload blocking calls
- **async-05** `run-in-executor` — Use `loop.run_in_executor(None, fn)` when you need a specific executor
- **async-06** `task-group-311` — Use `asyncio.TaskGroup` (3.11+) for structured concurrency; replaces `gather`
- **async-07** `gather-results` — Use `asyncio.gather(*tasks, return_exceptions=True)` only outside `TaskGroup`
- **async-08** `wait-for-timeout` — Wrap awaits with `asyncio.wait_for(coro, timeout=N)` for hard deadlines
- **async-09** `timeout-context` — Use `asyncio.timeout(N)` context manager (3.11+) over `wait_for` for cleaner code
- **async-10** `httpx-async-client` — Use `httpx.AsyncClient(...)` as `async with` for connection pooling
- **async-11** `aiofiles-fs` — Use `aiofiles` for async filesystem I/O; never `open()` in async
- **async-12** `async-context` — Use `async with` for resources with async setup/teardown
- **async-13** `async-iterator` — Use `async for` over `async generators` (`async def f(): yield ...`) for streams
- **async-14** `cancel-shielded` — Wrap critical cleanup in `asyncio.shield(coro)` so cancellation doesn't abort it
- **async-15** `no-loop-create` — Never call `asyncio.get_event_loop()` in async code; use `get_running_loop()`
- **async-16** `tg-cancellation` — Inside `TaskGroup`, raising in one task cancels siblings; structure code accordingly

---

## **opt-** — Compiler / Runtime Optimization (HIGH)

- **opt-01** `cprofile-first` — Profile with `python -m cProfile -o out.prof` + `snakeviz out.prof` before any optimization
- **opt-02** `builtin-c-impl` — Prefer built-ins (`sum`, `any`, `all`, `min`, `max`, `sorted`); they're C-implemented
- **opt-03** `comprehension-over-loop` — Comprehensions are faster than `append` loops in CPython
- **opt-04** `numpy-vectorize` — Replace numeric `for` loops with NumPy vectorized ops; 10-100x speedup
- **opt-05** `numba-jit` — Apply `@numba.njit` to numeric pure-Python hot functions; near-C speed
- **opt-06** `cython-pyx` — Move hot loops with type hints into `.pyx`; 10-100x speedup
- **opt-07** `mypyc-compile` — Compile a typed module with `mypyc` for ~4x speedup with no code change
- **opt-08** `pypy-runtime` — Use PyPy for long-running CPU-bound services; JIT compensates for warm-up
- **opt-09** `local-attr-bind` — Bind frequently-accessed attributes to locals in hot loops: `_append = lst.append`
- **opt-10** `string-join` — Build strings with `"".join(iterable)`; never `+=` in a loop
- **opt-11** `fstring-format` — Use f-strings; faster than `%`, `.format()`, or string concatenation
- **opt-12** `functools-partial` — Use `functools.partial` over `lambda` for object identity and speed
- **opt-13** `release-gil-numeric` — NumPy / multiprocessing release the GIL for true CPU parallelism
- **opt-14** `process-pool-cpu` — Use `concurrent.futures.ProcessPoolExecutor` for CPU-bound parallel work

---

## **type-** — Type Annotations (HIGH)

- **type-01** `pep604-unions` — Use PEP 604 syntax: `int | None`, `str | bytes` (3.10+); not `Union[int, None]`
- **type-02** `lowercase-generics` — Use `list[T]`, `dict[K, V]`, `tuple[T, ...]` (3.9+); not `List`, `Dict`, `Tuple`
- **type-03** `strict-mypy` — Run `mypy --strict`; treat `Any` as a failure, not a fallback
- **type-04** `no-implicit-any` — Annotate all public function signatures; private may use inference
- **type-05** `final-constants` — Annotate module-level constants with `Final`: `MAX_RETRY: Final = 3`
- **type-06** `literal-narrow` — Use `Literal["read", "write"]` for stringy enums of known values
- **type-07** `strenum-311` — Use `StrEnum` (3.11+) when you need both enum semantics and string compatibility
- **type-08** `newtype-domain` — Use `NewType` for domain identifiers: `OrderId = NewType("OrderId", int)`
- **type-09** `generic-class` — Use `class Box(Generic[T])` for parameterized container types
- **type-10** `type-stmt-312` — Use `type X = ...` (3.12+) over `X: TypeAlias = ...` for type aliases
- **type-11** `typeguard-narrow` — Use `TypeGuard[T]` for user-defined type narrowing functions
- **type-12** `cast-escape-hatch` — Use `typing.cast()` when type narrowing requires an escape hatch
- **type-13** `self-return` — Use `typing.Self` (3.11+) for methods returning the instance type
- **type-14** `paramspec-decorator` — Use `ParamSpec` to type decorators that preserve signatures
- **type-15** `protocol-callable` — Define `Protocol` with `__call__` instead of `Callable[...]` when you need named args
- **type-16** `never-bottom` — Use `typing.Never` (3.11+) for unreachable code / exhaustiveness checks

---

## **perf-** — Performance Patterns (MEDIUM)

- **perf-01** `generator-pipeline` — Build pipelines with generators; don't materialize intermediates
- **perf-02** `dict-lookup` — Use `dict` for O(1) lookups; never linear-scan a list when keyed access exists
- **perf-03** `set-membership` — Use `set` / `frozenset` for O(1) membership tests; never `in list` on hot path
- **perf-04** `setdefault-vs-check` — Use `dict.setdefault(k, default)` over `if k not in d: d[k] = default`
- **perf-05** `defaultdict-accum` — Use `collections.defaultdict(list)` for accumulator patterns
- **perf-06** `counter-frequency` — Use `collections.Counter` for histogram / frequency counts
- **perf-07** `sort-key-not-cmp` — Use `sorted(items, key=...)`; never `cmp` (gone since Python 3)
- **perf-08** `bisect-sorted` — Use `bisect.insort` / `bisect_left` for sorted-list operations (O(log n))
- **perf-09** `str-startswith` — Use `str.startswith()` / `endswith()` over slicing comparisons
- **perf-10** `iter-zip-strict` — Use `zip(a, b, strict=True)` (3.10+) to catch length-mismatched iterations
- **perf-11** `early-return` — Return early on common cases; avoid building unused intermediate state
- **perf-12** `walrus-once` — Use the walrus operator (`x := expr`) to capture a value used twice without recomputing

---

## **test-** — Testing (MEDIUM)

- **test-01** `pytest-default` — Use `pytest`, not `unittest`; richer fixtures, plugins, parametrization
- **test-02** `pytest-asyncio` — Mark async tests with `@pytest.mark.asyncio`; configure `asyncio_mode = "auto"`
- **test-03** `fixture-scope` — Set fixture scope explicitly (`function` / `class` / `module` / `session`)
- **test-04** `parametrize-table` — Use `@pytest.mark.parametrize("x,expected", [...])` for table-driven tests
- **test-05** `hypothesis-property` — Use `hypothesis` for property-based testing on pure functions
- **test-06** `freezegun-time` — Use `freezegun` to test time-dependent code deterministically
- **test-07** `pytest-mock` — Use `mocker` fixture (`pytest-mock`) over manual `unittest.mock` boilerplate
- **test-08** `monkeypatch-env` — Use `monkeypatch.setenv` for environment / module patches scoped to the test
- **test-09** `tmp-path` — Use `tmp_path` (or `tmp_path_factory`) for per-test temp directories; never `/tmp`
- **test-10** `conftest-shared` — Put cross-file fixtures in `conftest.py` at appropriate scope; not module top
- **test-11** `arrange-act-assert` — Structure every test as Arrange → Act → Assert; separate with blank lines
- **test-12** `descriptive-names` — Name tests as sentences: `def test_returns_none_on_empty_input()`
- **test-13** `doctest-examples` — Run docstring examples with `pytest --doctest-modules`
- **test-14** `testcontainers-real` — Use `testcontainers` for integration tests against real databases
- **test-15** `coverage-branch` — Use `coverage --branch` to catch missed conditional branches
- **test-16** `respx-httpx` — Use `respx` to mock `httpx` calls; same for `aioresponses` with `aiohttp`

---

## **doc-** — Documentation (MEDIUM)

- **doc-01** `google-docstring` — Use Google-style docstrings (`Args:`, `Returns:`, `Raises:`, `Example:`)
- **doc-02** `module-docstring` — First line: one-sentence summary. Blank line. Then expanded description
- **doc-03** `no-types-in-doc` — Never repeat types in docstrings; type hints in signatures are authoritative
- **doc-04** `examples-runnable` — Include runnable `>>> ` examples; verify with `pytest --doctest-modules`
- **doc-05** `raises-section` — Document every raised exception under `Raises:`; mirror to typed exception annotations
- **doc-06** `deprecated-decorator` — Mark deprecated APIs with `@deprecated` (PEP 702 / `typing_extensions`)
- **doc-07** `mkdocs-material` — Use `mkdocs-material` for user-facing docs; `pdoc` for API reference
- **doc-08** `readme-quickstart` — README opens with install + 30-second quickstart; not history
- **doc-09** `changelog-keepachangelog` — Maintain `CHANGELOG.md` in Keep a Changelog format
- **doc-10** `link-intersphinx` — Cross-link to stdlib docs via Sphinx intersphinx when relevant

---

## **name-** — Naming Conventions (MEDIUM)

- **name-01** `modules-snake` — `snake_case` for modules and packages (no hyphens in module names)
- **name-02** `classes-pascal` — `PascalCase` for classes, exceptions, type aliases
- **name-03** `funcs-snake` — `snake_case` for functions, methods, local variables
- **name-04** `constants-upper` — `UPPER_SNAKE_CASE` for module-level constants
- **name-05** `private-leading` — Single leading underscore (`_helper`) for internal/private names
- **name-06** `name-mangle-rare` — Double leading underscore only when name-mangling is desired (rare)
- **name-07** `dunder-reserved` — Never invent `__dunder__` names; that namespace is reserved for Python
- **name-08** `protocol-able` — Suffix Protocols with `-able` or descriptive form: `Comparable`, `Drawable`
- **name-09** `exception-error` — Suffix custom exceptions with `Error`: `ValidationError`, not `Invalid`
- **name-10** `abstract-prefix` — Prefix abstract base classes with `Abstract`, `Base`, or use `_Base`
- **name-11** `plural-collections` — Plural names for collections: `users`, `pending_jobs`; singular for items
- **name-12** `verb-functions` — Functions start with verbs: `get_user`, `save_record`, `compute_total`
- **name-13** `is-has-bool` — Predicates start with `is_`, `has_`, `can_`, `should_`
- **name-14** `factory-from` — Class factory methods named `from_*`: `from_dict`, `from_env`, `from_string`

---

## **proj-** — Project Structure (LOW)

- **proj-01** `src-layout` — Use `src/<package>/__init__.py` layout; protects against import shadowing in tests
- **proj-02** `pyproject-only` — Configure everything in `pyproject.toml`; no `setup.py`, `setup.cfg`, or `tox.ini`
- **proj-03** `uv-lock-commit` — Commit `uv.lock` to git for reproducible installs in CI and onboarding
- **proj-04** `tests-mirror-src` — `tests/` mirrors the `src/` layout; one test module per source module
- **proj-05** `dev-extra` — Declare dev deps in `[project.optional-dependencies] dev = [...]` or `[dependency-groups]`
- **proj-06** `entrypoints` — Use `[project.scripts]` for CLI entry points; no manual `bin/` scripts
- **proj-07** `namespace-pep420` — Use PEP 420 implicit namespace packages for plugin systems
- **proj-08** `conftest-root` — Place a root `conftest.py` to make `tests/` collectible without install
- **proj-09** `imports-absolute` — Use absolute imports (`from mypkg.x import y`) in published code
- **proj-10** `imports-grouped` — Group imports: stdlib, third-party, first-party; `ruff` enforces with `I` rule
- **proj-11** `init-thin` — Keep `__init__.py` thin: only re-exports and version; no logic
- **proj-12** `version-single-source` — Single-source version: `[project.version]` + `importlib.metadata.version()`

---

## **lint-** — Linting & Formatting (LOW)

- **lint-01** `ruff-format` — Use `ruff format` (Black-compatible) as the only formatter
- **lint-02** `ruff-check-strict` — Enable selected `ruff` rule sets: `E`, `F`, `I`, `B`, `N`, `UP`, `S`, `SIM`
- **lint-03** `ruff-pyupgrade` — Enable `UP` rules so code stays on modern Python syntax automatically
- **lint-04** `ruff-bugbear` — Enable `B` (flake8-bugbear) for common bug-prone patterns
- **lint-05** `ruff-bandit` — Enable `S` (bandit) for security smells in production code
- **lint-06** `mypy-strict` — `mypy --strict` in CI; every `# type: ignore[code]` requires a comment linking to an issue
- **lint-07** `noqa-justified` — Every `# noqa: CODE` must include reason after the code
- **lint-08** `line-length-100` — Set `line-length = 100` in `[tool.ruff]`; the 79-char rule is from 1980s terminals
- **lint-09** `pre-commit` — Configure `pre-commit` with `ruff` + `mypy` hooks; prevent bad commits locally
- **lint-10** `ruff-target-version` — Set `target-version = "py311"` (or current min) for accurate `UP` lints
- **lint-11** `bandit-deps` — Use `pip-audit` for dependency vulnerability scanning in CI

---

## **anti-** — Anti-patterns (REF)

- **anti-01** `bare-except` — Never `except:` with no type; always specify what you're catching
- **anti-02** `mutable-default` — Never `def f(x=[])`; the list is shared across all calls
- **anti-03** `type-ignore-blanket` — Never `# type: ignore` without `[error-code]` and reason
- **anti-04** `import-star` — Never `from module import *` in production code; pollutes namespace
- **anti-05** `global-mutable` — Never mutate module-level globals from functions; use a class or pass state
- **anti-06** `string-format-sql` — Never build SQL with `%` or `f-string`; use parameterized queries
- **anti-07** `eval-untrusted` — Never `eval()` or `exec()` on untrusted input; it's RCE
- **anti-08** `pickle-untrusted` — Never `pickle.load()` untrusted data; use JSON / pydantic
- **anti-09** `except-pass` — Never `except: pass`; if intentional, log and use `contextlib.suppress`
- **anti-10** `any-everywhere` — Never sprinkle `Any` to silence mypy; fix the underlying type
- **anti-11** `datetime-naive` — Never `datetime.utcnow()`; use `datetime.now(timezone.utc)` for timezone-aware
- **anti-12** `string-concat-loop` — Never `s += x` in a loop; build a list and `"".join(...)`
- **anti-13** `sync-in-async` — Never call blocking I/O in `async def`; use `to_thread` or async client
- **anti-14** `heavy-init` — Never put I/O or heavy computation in `__init__`; use `from_*` classmethod
- **anti-15** `print-debug` — Never `print()` for production logging; use `logging.getLogger(__name__)`
- **anti-16** `requirements-txt-only` — Never use only `requirements.txt`; lock with `uv.lock` for reproducibility

---

## Recommended `pyproject.toml` configuration

```toml
[project]
name = "mypkg"
version = "0.1.0"
requires-python = ">=3.11"

[dependency-groups]
dev = ["ruff", "mypy", "pytest", "pytest-asyncio", "hypothesis"]

[tool.ruff]
line-length = 100
target-version = "py311"

[tool.ruff.lint]
select = ["E", "F", "I", "B", "N", "UP", "S", "SIM"]
ignore = ["S101"]  # assert is fine in tests

[tool.mypy]
strict = true
python_version = "3.11"

[tool.pytest.ini_options]
asyncio_mode = "auto"
addopts = "--doctest-modules --strict-markers"
```
