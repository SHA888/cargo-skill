# TODO

Status legend: `[ ]` pending · `[x]` done · `[-]` skipped/deferred

---

## v0.1.x — Patch stabilization

- [x] 0.1.0 — Initial release
- [x] 0.1.1 — Fix `cargo skill <cmd>` subcommand dispatch (strip extra `skill` arg)
- [x] 0.1.2 — Replace corrected `layer1.md` and `layer2.md` assets
  - `opt-` prefix restored to compiler optimization (12 rules)
  - Full error table in layer2 (E0106 → E0716 + async Send)
  - Rule specificity aligned with `leonardomso/rust-skills` source naming
- [x] 0.1.3 — Add anti-rationalization table to `layer2.md` (pattern from `addyosmani/agent-skills`)
  - Table of common agent shortcuts with documented rebuttals
  - Entries: "I'll add tests later", "This unwrap is fine here", "clone() is cleaner",
    "unsafe is faster", "I'll handle errors later", "This is just a prototype"
  - Each entry maps to the violated rule prefix for cross-reference

---

## v0.2.0 — UX & Developer Experience

### Shorthand prefix commands
- [x] 2.1 — `cargo skill <prefix>` as implicit `lookup <prefix>`
  - Catch unrecognized subcommands that match a valid prefix
  - `cargo skill own` → equivalent to `cargo skill lookup own`
  - `cargo skill async` → equivalent to `cargo skill lookup async`
  - Error clearly on unrecognized non-prefix args

### Init improvements
- [x] 2.2 — `--dry-run` flag for `init`
  - Print what would be deployed without writing any files
  - Print what `.gitignore` entry would be added
- [x] 2.3 — `--force` flag for `init`
  - Overwrite existing skill files even if unchanged

### Status command
- [x] 2.4 — `cargo skill status`
  - Show detected repo kind (single / workspace)
  - Show detected agents + deployed skill file paths
  - Show current active context mode (lookup/think/write/none)
  - Show which prefix is active if in lookup mode
  - Show `.skill/context.md` line count if present

### Context injection per agent
- [x] 2.5 — Update `deploy.rs`: append `@.skill/context.md` footer to deployed `rust.md` for Claude Code only
  ```markdown
  ## Active Session Context
  If `.skill/context.md` exists, load it now.
  It contains the active skill layer for this session.
  Apply it on top of this index.
  @.skill/context.md
  ```
- [x] 2.6 — Update `context.rs`: for Cursor and Windsurf, write session context to agent rules dir in addition to `.skill/context.md`
  - `cargo skill lookup/think/write` → also writes `.cursor/rules/skill-context.md`
  - `cargo skill lookup/think/write` → also writes `.windsurf/rules/skill-context.md`
  - `cargo skill clear` → removes all three (`.skill/context.md`, `.cursor/rules/skill-context.md`, `.windsurf/rules/skill-context.md`)
- [x] 2.7 — Add agent-context paths to `.gitignore` management
  - `.cursor/rules/skill-context.md` → gitignored
  - `.windsurf/rules/skill-context.md` → gitignored

### Output polish
- [x] 2.8 — Colored terminal output via `anstream` (already a transitive dep)
  - `✓` lines in green
  - Warnings in yellow
  - Errors in red
- [ ] 2.9 — `--quiet` / `-q` flag to suppress all output except errors

### Provenance sidecar
- [ ] 2.15 — `cargo skill init` writes `.skill/provenance.md`
  - Records: `cargo-skill` version, content hashes of deployed layers, detected agents,
    deployed paths, timestamp
  - Gitignored alongside `.skill/context.md`
  - `cargo skill status` reads and displays provenance if present

### Workflow aliases
- [ ] 2.16 — `cargo skill review` — activate review-focused context
  - Equivalent to: `lookup err` + `lookup test` + `lookup lint` + Layer 2
- [ ] 2.17 — `cargo skill refactor` — activate refactor-focused context
  - Equivalent to: `lookup type` + `lookup api` + `lookup name` + Layer 2
- [ ] 2.18 — `cargo skill debug` — activate debug-focused context
  - Equivalent to: `lookup err` + `lookup mem` + Layer 2 (compiler quick-ref section only)

### Claude Code slash commands (pattern from `addyosmani/agent-skills`)
- [ ] 2.21 — Generate `.claude/commands/` entries on `cargo skill init` for Claude Code
  - `skill-lookup.md` → `/skill-lookup` slash command
  - `skill-think.md` → `/skill-think` slash command
  - `skill-write.md` → `/skill-write` slash command
  - `skill-clear.md` → `/skill-clear` slash command
  - Each command file contains the prompt Claude Code should run when invoked
- [ ] 2.22 — Tests for `.claude/commands/` file generation

### Tests
- [ ] 2.10 — Tests for shorthand prefix dispatch
- [ ] 2.11 — Tests for `status` output correctness
- [ ] 2.12 — Tests for `--dry-run` (no files written)
- [ ] 2.13 — Tests for agent-specific context file writes (Cursor, Windsurf)
- [ ] 2.14 — Tests for `clear` removes all agent context files
- [ ] 2.19 — Tests for provenance file write (fields present, correct hash)
- [ ] 2.20 — Tests for workflow alias layer composition

---

## v0.3.0 — Python/uv Skill Content

### Asset authoring
- [ ] 3.1 — Author `assets/python/layer1.md`
  - Categories: typing, error, async, packaging, testing, perf, doc, name, proj, lint, anti
  - Rules aligned with: PEP 8, PEP 484, mypy docs, uv docs, ruff docs, attrs/pydantic patterns
- [ ] 3.2 — Author `assets/python/layer2.md`
  - Cognitive model adapted for Python: duck typing vs structural subtyping, GIL implications
  - Common type error quick-ref (mypy error codes)
- [ ] 3.3 — Author `assets/python/layer3.md`
  - RPI loop for Python: uv run, ruff check, mypy, pytest
  - Verification checklist: `uv run ruff check`, `uv run mypy`, `uv run pytest`

### Agent personas (pattern from `addyosmani/agent-skills`)
- [ ] 3.11 — Author `assets/agents/rust-reviewer.md`
  - Senior Rust code reviewer persona with five-axis review (correctness, safety, perf, API, docs)
  - Maps to `anti-` + `lint-` prefix rules
- [ ] 3.12 — Author `assets/agents/rust-architect.md`
  - Systems architect persona for API and module design decisions
  - Maps to `api-` + `proj-` + `type-` prefix rules
- [ ] 3.13 — `cargo skill init` deploys agent personas to `.claude/skills/agents/`
- [ ] 3.14 — Tests for agent persona deployment

### Stack detection
- [ ] 3.4 — Detect Python projects via `pyproject.toml` presence
- [ ] 3.5 — Detect uv via `uv.lock` or `[tool.uv]` in `pyproject.toml`
- [ ] 3.6 — `cargo skill init` deploys Python layers when Python stack detected
- [ ] 3.7 — `cargo skill lookup <prefix>` routes to correct language asset

### Multi-language context
- [ ] 3.8 — Mixed repo support (Rust + Python in same workspace)
  - Detect both stacks
  - Deploy both skill files to each agent
  - `cargo skill lookup err` → prompt user which language if ambiguous
  - `cargo skill lookup rust:err` and `cargo skill lookup py:err` as explicit selectors

### Tests
- [ ] 3.9 — Detection tests for Python/uv stack
- [ ] 3.10 — Asset content tests (prefix filter works on Python layer1)

---

## v0.4.0 — TypeScript/pnpm Skill Content

### Asset authoring
- [ ] 4.1 — Author `assets/typescript/layer1.md`
  - Categories: types, error, async, module, testing, perf, doc, name, proj, lint, anti
  - Rules aligned with: TypeScript handbook, pnpm docs, ESLint, Vitest, tsx patterns
- [ ] 4.2 — Author `assets/typescript/layer2.md`
  - Cognitive model: structural typing, type narrowing, tsc error codes
  - Common tsc error quick-ref (TS2345, TS2322, TS7006, etc.)
- [ ] 4.3 — Author `assets/typescript/layer3.md`
  - RPI loop: `pnpm check`, `pnpm lint`, `pnpm test`
  - Verification checklist aligned with pnpm scripts

### Stack detection
- [ ] 4.4 — Detect TypeScript via `tsconfig.json` or `package.json` with `typescript` dep
- [ ] 4.5 — Detect pnpm via `pnpm-lock.yaml`
- [ ] 4.6 — Deploy TypeScript layers on detection
- [ ] 4.7 — `ts:` prefix namespace for explicit TypeScript lookups

### Tests
- [ ] 4.8 — Detection tests for TypeScript/pnpm stack
- [ ] 4.9 — Prefix routing tests for mixed Rust+TS repos

---

## v0.5.0 — Configuration

### Config file (`skill.toml`)
- [ ] 5.1 — Define `skill.toml` schema
  ```toml
  [agents]
  claude = ".claude/skills"       # override default deploy path
  cursor = ".cursor/rules"
  windsurf = ".windsurf/rules"

  [skill]
  languages = ["rust", "python"]  # explicit language list

  [context]
  default_mode = "think"          # default mode when no command given
  ```
- [ ] 5.2 — Implement config file discovery (walk up from cwd, like Cargo.toml)
- [ ] 5.3 — Merge config with defaults (config wins over auto-detection)
- [ ] 5.4 — `cargo skill config init` — scaffold a `skill.toml` with commented defaults
- [ ] 5.5 — `cargo skill config show` — print resolved config (file + defaults merged)

### SDLC-phase skill overlays (pattern from `addyosmani/agent-skills`)
- [ ] 5.9 — Author `assets/sdlc/` phase overlays as optional context supplements
  - `spec.md` — spec-driven development workflow (Define phase)
  - `plan.md` — task decomposition and acceptance criteria (Plan phase)
  - `review.md` — multi-axis code review gates (Review phase)
  - `ship.md` — pre-launch checklist and rollback procedures (Ship phase)
- [ ] 5.10 — `cargo skill sdlc <phase>` — activate SDLC overlay on top of active language context
  - `cargo skill sdlc spec` → write spec overlay to `.skill/context.md`
  - `cargo skill sdlc review` → write review overlay to `.skill/context.md`
- [ ] 5.11 — Session hooks: `cargo skill init` generates `.claude/hooks/` stubs for Claude Code
  - Pre-session hook: load active skill context
  - Post-session hook: clear ephemeral context
- [ ] 5.12 — Tests for SDLC overlay composition and hook file generation

### Tests
- [ ] 5.6 — Config file parsing tests
- [ ] 5.7 — Config override tests (custom agent paths)
- [ ] 5.8 — Config discovery walk-up tests

---

## v0.6.0 — Remote Skill Fetch

### Remote source support
- [ ] 6.1 — Add `reqwest` (or `ureq` for lighter weight) behind `remote` feature flag
- [ ] 6.2 — Define remote skill source format (GitHub shorthand: `owner/repo`)
- [ ] 6.3 — `cargo skill install <source>` — fetch, verify, and cache remote skill assets
  - `cargo skill install leonardomso/rust-skills`
  - `cargo skill install actionbook/rust-skills`
  - Store in `~/.cargo/skill-cache/<owner>/<repo>/`
- [ ] 6.4 — Cache invalidation: `--refresh` flag re-fetches from remote
- [ ] 6.5 — Offline fallback: use cache if available, error clearly if not

### `skill.toml` remote sources
- [ ] 6.6 — Add `[[sources]]` table to `skill.toml`
  ```toml
  [[sources]]
  type = "git"
  repo = "leonardomso/rust-skills"
  layer = 1
  ```
- [ ] 6.7 — `cargo skill init` fetches and merges remote sources if configured

### Tests
- [ ] 6.8 — Cache write/read tests (mocked HTTP)
- [ ] 6.9 — Offline fallback tests
- [ ] 6.10 — Source merge tests (remote + bundled)

---

## v0.7.0 — Update & Maintenance

### Update check
- [ ] 7.1 — `cargo skill update` — check for newer versions of deployed skill files
  - Compare local asset hash against remote
  - Print diff summary (categories changed, rules added/removed)
  - `--apply` flag to actually update

### Version pinning
- [ ] 7.2 — Pin remote skill source versions in `skill.lock`
  - SHA-based locking for reproducible deployments
  - `cargo skill lock` — regenerate `skill.lock`
  - Schema includes: source, version, content_hash (sha256), deployed_at, agent paths
    ```toml
    [deployed]
    version = "0.2.2"
    content_hash = "sha256:abc123..."
    deployed_at = "2026-04-12T07:20:00Z"

    [agents]
    claude_code = ".claude/skills/rust.md"
    cursor = ".cursor/rules/rust.md"
    ```
  - `cargo skill status` validates deployed files against `skill.lock` hashes

### Self-update awareness
- [ ] 7.3 — On `init` or `status`, check if newer `cargo-skill` version exists on crates.io
  - Print one-line notice: `cargo-skill v0.7.1 available — cargo install cargo-skill`
  - Suppress with `--no-update-check`

### Tests
- [ ] 7.4 — Hash comparison tests
- [ ] 7.5 — Lock file generation + validation tests

---

## v0.8.0 — Workspace & Multi-Crate Intelligence

### Workspace-aware deployment
- [ ] 8.1 — Detect workspace root vs member crate
- [ ] 8.2 — Deploy shared skill to workspace root
- [ ] 8.3 — Support per-crate override `skill.toml`
  - Member crate `skill.toml` overrides workspace root config
  - `cargo skill init --member` deploys only to current crate

### Crate-specific skill narrowing
- [ ] 8.4 — Parse member crate `Cargo.toml` dependencies to narrow active rules
  - Crate uses `tokio` → `async-` rules active
  - Crate uses `serde` → include serde-specific api- rules
  - Crate has `no_std` in lib.rs → suppress `mem-arena`, `async-*`, std-specific rules
- [ ] 8.5 — `cargo skill init --narrow` — deploy narrowed skill file based on deps

### Tests
- [ ] 8.6 — Workspace detection + root vs member deploy tests
- [ ] 8.7 — Dependency-based narrowing tests

---

## v0.9.0 — Pre-1.0 Polish

### Stability & correctness
- [ ] 9.1 — Audit all error messages for clarity and actionability
- [ ] 9.2 — Ensure all `anyhow` errors have `.context()` at every boundary
- [ ] 9.3 — Windows path handling audit (`PathBuf` throughout, no `/` hardcoding)
- [ ] 9.4 — CI matrix: Linux + macOS + Windows
- [ ] 9.12 — Verify workflow alias layer compositions are stable across versions

### Documentation
- [ ] 9.5 — Full rustdoc on all public items
- [ ] 9.6 — `docs/` directory with:
  - `layers.md` — full prefix reference for all languages
  - `agents.md` — per-agent integration guide
  - `config.md` — `skill.toml` schema reference
- [ ] 9.7 — Update README to reflect all commands through v0.9.0

### Performance
- [ ] 9.8 — Benchmark `init` on large workspaces (100+ crates)
- [ ] 9.9 — Benchmark `lookup` prefix filter (should be <5ms)

### Test coverage
- [ ] 9.10 — Coverage report via `cargo-tarpaulin`; target ≥ 80%
- [ ] 9.11 — Fuzz `prefix::filter()` with `cargo-fuzz`

---

## v1.0.0 — Stable Release

- [ ] 1.0.1 — Final API review: no breaking changes planned post-1.0
- [ ] 1.0.2 — Deprecation policy documented in `CONTRIBUTING.md`
- [ ] 1.0.3 — MSRV policy documented: track latest stable - 2
- [ ] 1.0.4 — Security policy (`SECURITY.md`) added
- [ ] 1.0.5 — `cargo publish` v1.0.0 with complete changelog
- [ ] 1.0.6 — GitHub Release with binary artifacts (via `cargo-dist`)
- [ ] 1.0.7 — Announce on r/rust, This Week in Rust

---

## Ongoing (all versions)

- Keep `CHANGELOG.md` updated per release
- SemVer: breaking CLI changes → major bump, new commands → minor, fixes → patch
- All new commands must have `--help` text before merge
- CI must pass on all three platforms before any publish
