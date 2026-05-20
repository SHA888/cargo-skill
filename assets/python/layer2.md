# Layer 2 — Reasoning: Cognitive Model

Three-layer reasoning framework for Python. **Do not answer immediately. Trace through layers first.**

---

## Cognitive Model

```
Layer 3 — Domain Constraints    WHY   (business rules, invariants, requirements)
Layer 2 — Design Choices        WHAT  (patterns, abstractions, architecture)
Layer 1 — Language Mechanics    HOW   (Python syntax, types, runtime model)
```

A mypy error or `AttributeError` (HOW) is a symptom. Trace upward: the design choice (WHAT) may be wrong
because the domain constraint (WHY) was misunderstood. Fix the root cause, not the symptom.

### Example trace

```
Problem: "Cannot determine type of 'user'" mypy error in an auth handler

Layer 1 (HOW):  mypy can't narrow `user: User | None` after a runtime check that mypy
                doesn't recognize as a type guard
Layer 2 (WHAT): The handler is mixing presence-check and authorization concerns in
                an ad-hoc `if` chain
Layer 3 (WHY):  Authentication has a single invariant — once authenticated, `user` is
                non-None for the rest of the handler — that the type system should encode

Fix: Extract a `require_authenticated(...) -> User` boundary function with a
     `TypeGuard[User]`, not `assert user is not None` scattered throughout.
     The mypy error pointed at the symptom; the design needed a real boundary.
```

---

## Three-Layer Reasoning Process

### Step 1 — Domain (WHY)

Ask before touching any code:

- What is the real-world invariant this code must preserve?
- Where does untrusted data enter the system, and where does it become trusted?
- What happens if this operation fails — partial state, retry, user-visible error?
- Is this synchronous (request/response) or asynchronous (event/stream)?
- What are the performance constraints, and why do they exist?

### Step 2 — Design (WHAT)

Select the right abstraction:

- Use `Protocol` for behavior an API consumer plugs into; inheritance is rarely the answer
- Use `pydantic.BaseModel` at trust boundaries; `attrs.define` or `dataclass` for internal types
- Use `Enum` / `Literal` / `StrEnum` for finite sets of states; never strings or magic numbers
- Use `NewType` for domain identifiers that must not be interchangeable
- Use `TypedDict` for short-lived structured dicts; `dataclass` if it lives more than a function
- Use module boundaries to define API surfaces; not class boundaries

### Step 3 — Mechanics (HOW)

Write idiomatic, typed Python:

- Annotate signatures with PEP 604 unions (`X | None`) and lowercase generics (`list[T]`)
- Handle every exception explicitly; choose between propagation, conversion, and `contextlib.suppress`
- In async code, never block the event loop — offload via `asyncio.to_thread`
- Profile with `cProfile` + `snakeviz` before optimizing; do not guess
- Run `ruff format → ruff check → mypy --strict → pytest` after every meaningful edit

---

## Question Routing

Route every question to the correct rule prefix before answering:

| Question | Layer | Rule prefix |
|---|---|---|
| Should this be mutable or frozen? | Domain → Design | `own-` |
| Should I use `Protocol` or `ABC`? | Design | `api-`, `type-` |
| How should I validate this input? | Domain → Design | `api-` (pydantic), `err-` |
| How do I handle this error? | Design → Mechanics | `err-` |
| Why does mypy reject this? | Mechanics | `type-` |
| How do I structure this package? | Design | `proj-` |
| Is this the right data structure? | Domain → Design | `perf-`, `mem-` |
| How do I make this faster? | Domain → Mechanics | `opt-`, `perf-` |
| How do I write this async code? | Design → Mechanics | `async-` |
| How should I name this? | Mechanics | `name-` |
| Is this idiomatic? | All | `anti-`, `lint-` |
| Where do I put this test? | Design → Mechanics | `test-` |

---

## Anti-rationalization Table

Common shortcuts that lead to fragile Python. Each is a signal to route to the proper rule prefix.

| Shortcut | Rebuttal | Violated rule prefix |
|---|---|---|
| "I'll add type hints later" | Hints document intent; retrofitting after refactors is slower and misses errors mypy would have caught the first time. | `type-`, `lint-` |
| "Just use `Any` to silence mypy" | `Any` is a contagion. It propagates and disables checking through every call site. Fix the type. | `type-`, `anti-` |
| "`# type: ignore` to unblock" | Untyped unblock-and-forget hides bugs. If you must ignore, link to a tracking issue. | `lint-`, `anti-` |
| "It's just a script" | Scripts become services. Apply the small-but-correct pattern (typed signatures, real error handling) now. | `anti-` |
| "I'll catch everything" | `except Exception` swallows bugs (KeyError on a misspelled key) the same way as real errors. Catch what you handle. | `err-`, `anti-` |
| "I'll add tests later" | Property-based + parametrized tests are faster to write up-front than to add after a refactor. | `test-` |
| "pickle is fine for this" | `pickle.load` on untrusted input is remote code execution. Use JSON or pydantic. | `anti-` |
| "I'll use `asyncio.run_until_complete`" | Mixing event loop creation across libraries causes subtle deadlocks. Stay inside `asyncio.run(main())`. | `async-` |

---

## Mypy Error Quick Reference

### `Incompatible return value type`

```python
# Error: Incompatible return value type (got "str", expected "int")
def parse_count(s: str) -> int:
    return s  # ← returning str, declared int

# Fix: align return with annotation, or change the annotation if the function truly returns a str
def parse_count(s: str) -> int:
    return int(s)
```

### `Argument has incompatible type`

```python
# Error: Argument 1 to "process" has incompatible type "list[str]"; expected "tuple[str, ...]"
def process(items: tuple[str, ...]) -> None: ...
process(["a", "b"])  # ← passing list, expected tuple

# Fix: convert, or widen the parameter type
process(tuple(["a", "b"]))            # convert at call site
# or change signature:
def process(items: Sequence[str]) -> None: ...  # accept both list and tuple
```

### `Missing type parameters for generic type`

```python
# Error: Missing type parameters for generic type "list"
def first(xs: list) -> object: ...   # ← bare list

# Fix: parameterize
def first(xs: list[T]) -> T: ...
```

### `Item "None" of "X | None" has no attribute "name"`

```python
# Error: Item "None" of "User | None" has no attribute "name"
def greet(user: User | None) -> str:
    return user.name              # ← user could be None

# Fix: narrow with a guard, or assert at the boundary
def greet(user: User | None) -> str:
    if user is None:
        raise ValueError("user required")
    return user.name              # narrowed to User
```

### `Need type annotation for variable`

```python
# Error: Need type annotation for "items"
items = []                        # ← empty collection, mypy can't infer

# Fix: annotate
items: list[str] = []
```

### `Returning Any from function declared to return X`

```python
# Error: Returning Any from function declared to return "int"
def get_count(data: dict) -> int:  # data is `dict[Any, Any]`
    return data["count"]           # ← returns Any

# Fix: narrow `data`, or cast deliberately
def get_count(data: dict[str, int]) -> int:
    return data["count"]
```

### `Cannot determine type of` (after `assert`)

```python
# Error: Cannot determine type of "result"
def find() -> Result | None: ...
result = find()
assert result is not None
# mypy 1.x narrows after assert, older versions did not
process(result)
```

### `Argument has incompatible type "Literal['x']"; expected "Literal['a', 'b']"`

```python
# Error: passing a Literal not in the allowed set
def render(mode: Literal["a", "b"]) -> None: ...
mode = "x"
render(mode)                       # ← "x" not allowed

# Fix: constrain the variable type, or assert/narrow before use
mode: Literal["a", "b"] = "a"
render(mode)
```

### `Incompatible types in assignment`

```python
# Error: Incompatible types in assignment (expression has type "str", variable has type "int")
x: int = 1
x = "two"                          # ← reassigning to different type

# Fix: declare as union, or rename variables
x: int | str = 1
x = "two"
```

### `Function is missing a return type annotation`

```python
# Error: Function is missing a return type annotation
def compute(x: int):               # ← no return annotation
    return x * 2

# Fix: annotate; use `None` for void functions
def compute(x: int) -> int:
    return x * 2
```

### `Untyped function calls disallowed` (`--strict`)

```python
# Error: Call to untyped function "legacy" in typed context
def legacy(x):                     # ← no annotations
    return x

# Fix: annotate the function, or wrap with cast
def legacy(x: int) -> int:
    return x
```

### `Class cannot subclass "X" (has type "Any")`

```python
# Error: Class cannot subclass "DynamicBase" (has type "Any")
# Cause: third-party lib has no type stubs

# Fix: install stubs (`pip install types-X`) or add `# type: ignore[misc]` with a tracking issue
```

### `Overload variants overlap`

```python
# Error: Overload variants are overlapping
@overload
def read(x: int) -> str: ...
@overload
def read(x: bool) -> str: ...      # ← bool is a subtype of int; signatures overlap

# Fix: reorder so the more specific overload comes first, or remove the redundant overload
```

### `"X" is not callable` after `assert`

```python
# Error: "X | None" is not callable
fn: Callable[[], int] | None = ...
result = fn()                       # ← could be None

# Fix: narrow first
if fn is not None:
    result = fn()                   # narrowed to Callable
```

---

## GIL & Concurrency Model

Choose the runtime model based on the work:

| Workload                       | Mechanism                          | Reason |
|--------------------------------|------------------------------------|--------|
| I/O-bound (network, files, DB) | `asyncio` / `anyio` / `TaskGroup`  | Tasks yield on `await`; no GIL contention |
| CPU-bound (compute, parsing)   | `ProcessPoolExecutor` or NumPy     | True parallelism; GIL released by C extensions |
| Mixed (CPU inside an async fn) | `asyncio.to_thread(fn, ...)`       | Offloads blocking work; event loop stays responsive |
| Hot numeric loop               | NumPy vectorization or `numba.njit`| Vectorization releases the GIL; JIT generates native code |

Anti-pattern: spinning threads (`threading.Thread`) for CPU-bound work. The GIL serializes them. Use processes or NumPy instead.

---

## Pydantic v2 vs attrs vs dataclass — When to Pick Which

| Use case                             | Pick           | Why |
|--------------------------------------|----------------|-----|
| HTTP request / response, JSON input  | `pydantic`     | Validation + serialization + JSON schema |
| Internal value object                | `attrs.frozen` | Cheap, immutable, no validation overhead |
| Pure data carrier (DTO between fns)  | `@dataclass`   | Standard library, zero deps |
| Settings / config                    | `pydantic-settings` | Env loading + validation |
| Short-lived structured dict          | `TypedDict`    | Lighter than class; type-checked by mypy |
| Stable on-the-wire schema            | `pydantic`     | Decouples wire format from internal types |

Rule of thumb: **pydantic at the boundary, attrs/dataclass inside.**

---

## Parse, Don't Validate

Code outside the trust boundary deals in *unvalidated* types: `dict`, `str`, `Any`. Inside the trust boundary, only *validated* types exist. The boundary is a one-way door.

```python
# Wrong: scattered validation; downstream still sees raw dict
def handle(payload: dict) -> None:
    if "email" not in payload or "@" not in payload["email"]:
        raise ValueError(...)
    send_email(payload["email"])

# Right: parse once at the boundary; downstream sees a validated type
class EmailRequest(BaseModel):
    email: EmailStr

def handle(payload: dict) -> None:
    request = EmailRequest.model_validate(payload)
    send_email(request.email)        # type system guarantees validity
```

Inside `send_email`, `request.email` is `EmailStr` — already validated, no recheck needed.
