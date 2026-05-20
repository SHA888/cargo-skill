# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-05-20

### Changed

- **Python skill content brought to expert-level parity with Rust**:
  - **Layer 1** rewritten with the same 14-prefix rule index format as Rust
    (`own-`, `err-`, `mem-`, `api-`, `async-`, `opt-`, `type-`, `perf-`,
    `test-`, `doc-`, `name-`, `proj-`, `lint-`, `anti-`)
  - 192 specific rules across all prefixes, each with an identifier slug
    and actionable guidance (e.g., `err-07 errorgroup-311`, `async-06 task-group-311`)
  - Modern Python: PEP 604 union syntax (`int | None`), lowercase generics
    (`list[T]`), Python 3.11+ features (TaskGroup, ExceptionGroup, Self,
    StrEnum), PEP 702 deprecated decorator, `type X = ...` statement (3.12+)
  - Real library recommendations: pydantic v2, attrs, anyio, httpx,
    hypothesis, ruff, mypy, uv, freezegun, pytest-mock, testcontainers,
    tenacity
  - **Layer 2** adds cognitive model (Domain → Design → Mechanics), question
    routing table, anti-rationalization table, 14-entry mypy error quick
    reference with code examples, GIL & concurrency model guide, pydantic
    vs attrs vs dataclass selection matrix, "parse don't validate" guidance
  - **Layer 3** adds RPI loop, verification command sequence with ordering
    rationale, full verification checklist (correctness/code quality/type
    safety/async/performance), task-to-rule mapping, common pitfalls table,
    pre-commit configuration

### Fixed

- `cargo skill lookup py:<prefix>` now actually filters Python content by
  category (previously dumped the whole file because Python Layer 1 lacked
  the prefix structure)

## [0.3.0] - 2026-05-20

### Added

- **Python/uv stack detection and skill routing**:
  - Detect Python projects via `pyproject.toml` presence
  - Detect uv package manager via `uv.lock` or `[tool.uv]` in `pyproject.toml`
  - `cargo skill init` deploys Python skill layers when Python stack detected
  - `cargo skill lookup <prefix>` routes to correct language asset (Rust or Python)
  - Python Layer 1 covers: typing, error handling, async, packaging, testing, performance, documentation, naming, project layout, linting, and anti-patterns
  - Python Layer 2 includes cognitive model adapted for duck typing and GIL implications with mypy error quick-reference
  - Python Layer 3 provides RPI loop for Python development with uv, ruff, and mypy verification checklist

- **Mixed repository support** (both Cargo.toml + pyproject.toml):
  - `cargo skill status` shows both Rust and Python stacks when present
  - Bare `cargo skill lookup <prefix>` errors with helpful hint when both stacks detected
  - Explicit language selectors: `cargo skill lookup rust:<prefix>` and `cargo skill lookup py:<prefix>`
  - Shorthand shorthands work: `cargo skill rust:err`, `cargo skill py:test`, etc.
  - Both skill files deployable to same agent directory for polyglot projects

### Changed

- `cargo skill status` now shows language stack detection (✓ for detected, ✗ for not present)
- Multi-language skill content deployed to agents when multiple stacks detected

## [0.2.7] - 2026-04-22

### Added

- **Workflow aliases** — Three new commands for common development workflows:
  - `cargo skill review` — combines `err` + `test` + `lint` prefixes with Layer 2
  - `cargo skill refactor` — combines `type` + `api` + `name` prefixes with Layer 2
  - `cargo skill debug` — combines `err` + `mem` prefixes with Layer 2 quick-reference
  - All workflows write combined context to `.skill/context.md`
  - `cargo skill status` detects and displays active workflow context
- **Claude Code slash commands** — Generated on `cargo skill init` when Claude Code is detected:
  - `.claude/commands/skill-lookup.md` → `/skill-lookup` slash command
  - `.claude/commands/skill-think.md` → `/skill-think` slash command
  - `.claude/commands/skill-write.md` → `/skill-write` slash command
  - `.claude/commands/skill-clear.md` → `/skill-clear` slash command
  - Each command contains the prompt for Claude Code to execute
  - `.claude/commands/` added to gitignore management

### Fixed

- `cmd_debug` now fails explicitly if Layer 2 "Compiler Error Quick Reference" section is missing
- Removed compiler warnings: unused `Layer` import, unused `temp` variables in tests

## [0.2.6] - 2026-04-14

### Added

- **Provenance sidecar** — `cargo skill init` now writes `.skill/provenance.md` with deployment
  metadata including:
  - cargo-skill version
  - SHA256 content hash of deployed layers
  - List of detected agents
  - Deployed skill file paths
  - RFC 3339 timestamp with Unix epoch fallback
- `cargo skill status` now reads and displays provenance information when present
  - Shows version, content hash (first 16 chars), and deployment timestamp
- Added `--quiet` / `-q` global flag to suppress all non-error output
- Gitignore now covers `.skill/provenance.md` via the existing `.skill/` entry

### Fixed

- `println!` in library code (`deploy.rs`) now properly routed through `info()` wrapper
  respecting the `--quiet` flag

## [0.2.5] - 2026-04-13

### Added

- Colored terminal output using `anstream` — success messages (✓) in green, warnings (⚠) in
  yellow, and errors (✗) in red. ANSI codes are automatically stripped when output is not a TTY.

## [0.2.4] - 2026-04-13

### Added

- Agent-specific context files — `cargo skill lookup/think/write` now also writes session
  context to `.cursor/rules/skill-context.md` and `.windsurf/rules/skill-context.md` when
  those agent directories are present. This allows Cursor and Windsurf to detect and load
  session-specific context directly.
- `cargo skill clear` now removes all three context files: `.skill/context.md`,
  `.cursor/rules/skill-context.md`, and `.windsurf/rules/skill-context.md`.
- Gitignore management now includes agent context paths: `.cursor/rules/skill-context.md`
  and `.windsurf/rules/skill-context.md` are automatically gitignored on `init`.

## [0.2.3] - 2026-04-12

### Added

- Claude Code context injection — `cargo skill init` now appends `@.skill/context.md` footer
  to deployed `rust.md` for Claude Code only. This allows Claude Code to detect and load
  session-specific context when present.

## [0.2.2] - 2026-04-11

### Added

- `cargo skill status` command — comprehensive project status overview
  - Shows repo kind (single crate / workspace)
  - Shows detected agents and deployment status (✓/✗)
  - Shows active context mode (lookup/think/write/none) with line count
  - Shows active prefix when in lookup mode
  - Shows .gitignore status for .skill/

## [0.2.1] - 2026-04-11

### Added

- `--dry-run` flag for `init` — print what would be deployed without making changes
- `--force` flag for `init` — overwrite existing skill files even if unchanged
- Skip deploy if skill files already exist (suggest `--force` to overwrite)

## [0.2.0] - 2026-04-11

### Added

- Shorthand prefix commands — `cargo skill <prefix>` is now equivalent to `cargo skill lookup <prefix>`
  - Example: `cargo skill own` → loads ownership & borrowing rules
  - Example: `cargo skill async` → loads async/await rules
  - All 14 valid prefixes supported: own, err, mem, api, async, opt, type, perf, test, doc, name, proj, lint, anti
  - Clear error message on unrecognized commands listing valid options

## [0.1.1] - 2026-04-11

### Fixed

- `cargo skill <cmd>` now works correctly — strip the extra `skill` argument inserted by cargo when dispatching external subcommands

## [0.1.0] - 2026-04-11

### Added

- Initial release of `cargo-skill`
- `cargo skill init` command — Detect repository, detect AI agents, deploy skill files, ensure `.skill/` in `.gitignore`
- `cargo skill lookup [prefix]` command — Load Layer 1 (rule index) with optional prefix filter, write to `.skill/context.md`
- `cargo skill think` command — Load Layers 1+2 (lookup + reasoning), write to `.skill/context.md`
- `cargo skill write` command — Load all layers (lookup + reasoning + execution), write to `.skill/context.md`
- `cargo skill clear` command — Remove `.skill/context.md`
- Support for multiple AI agents: Claude Code (`.claude/`), Cursor (`.cursor/`), Windsurf (`.windsurf/`), AGENTS.md
- Layered skill system: Layer 1 (lookup), Layer 2 (reasoning), Layer 3 (execution)
- Prefix filtering for Layer 1 content (own, err, mem, api, async, opt, type, perf, test, doc, name, proj, lint, anti)
- Idempotent skill file deployment — won't duplicate content in AGENTS.md
- Gitignore management — automatically adds `.skill/` to `.gitignore`
- Comprehensive test suite: 51 tests (42 unit + 9 integration)

[0.2.6]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.6
[0.2.5]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.5
[0.2.4]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.4
[0.2.3]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.3
[0.2.2]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.2
[0.2.1]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.1
[0.2.0]: https://github.com/SHA888/cargo-skill/releases/tag/v0.2.0
[0.1.1]: https://github.com/SHA888/cargo-skill/releases/tag/v0.1.1
[0.1.0]: https://github.com/SHA888/cargo-skill/releases/tag/v0.1.0
