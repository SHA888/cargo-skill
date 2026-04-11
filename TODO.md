# TODO

SemVer target: **v0.1.0**
Status legend: `[ ]` pending · `[x]` done · `[-]` skipped/deferred

---

## 0. Project bootstrap

- [x] 0.1 — Register `cargo-skill` on crates.io (reserve the name) v0.0.1
- [x] 0.2 — Create GitHub repository `cargo-skill`
- [x] 0.3 — Add `LICENSE-MIT` and `LICENSE-APACHE` files
- [x] 0.4 — Add `.gitignore` (standard Rust: `target/`, `.skill/`)
- [x] 0.5 — Initialize `Cargo.toml` with correct metadata
  - `name = "cargo-skill"`
  - `edition = "2024"`
  - `rust-version = "1.85"`
  - `license = "MIT OR Apache-2.0"`
  - `description`, `repository`, `keywords`, `categories`
- [x] 0.6 — Add `README.md` (already drafted)
- [x] 0.7 — Add `ARCHITECTURE.md` (already drafted)
- [x] 0.8 — Set up GitHub Actions CI workflow
  - `cargo check`
  - `cargo clippy -- -D warnings`
  - `cargo fmt --check`
  - `cargo test`
  - Trigger: push + PR to `main`

---

## 1. Asset preparation

- [x] 1.1 — Split unified `SKILL.md` into three asset files
  - `assets/rust/layer1.md` — Lookup (rule index, prefix sections)
  - `assets/rust/layer2.md` — Reasoning (cognitive model, routing, error ref)
  - `assets/rust/layer3.md` — Execution (RPI loop, verification checklist)
- [x] 1.2 — Ensure Layer 1 section headers use consistent prefix markers
  - Format: `**<prefix>-** …` per section for line-range extraction
- [x] 1.3 — Verify all three layers render correctly as standalone markdown

---

## 2. CLI skeleton (`src/main.rs`)

- [x] 2.1 — Add `clap` dependency (latest stable, derive feature)
- [x] 2.2 — Add `anyhow` dependency (latest stable)
- [x] 2.3 — Define `Cli` struct with `Commands` enum via clap derive
  - `Init`
  - `Lookup { prefix: Option<String> }`
  - `Think`
  - `Write`
  - `Clear`
- [x] 2.4 — Implement main dispatch to subcommand handlers (stubs)
- [x] 2.5 — Verify `cargo skill --help` output is correct

---

## 3. Repo + agent detection (`src/detect.rs`)

- [x] 3.1 — Implement `detect::repo()`
  - Walk up from `cwd` to find `Cargo.toml`
  - Determine workspace (contains `[workspace]`) vs single crate
  - Return `RepoKind` enum + root path
- [x] 3.2 — Implement `detect::agents()`
  - Check for `.claude/` directory → `Agent::ClaudeCode`
  - Check for `.cursor/` directory → `Agent::Cursor`
  - Check for `.windsurf/` directory → `Agent::Windsurf`
  - Check for `AGENTS.md` file → `Agent::AgentsMd`
  - Return `Vec<Agent>` (may be multiple)
- [x] 3.3 — Unit tests for `detect::repo()`
  - Single crate fixture
  - Workspace fixture
  - No `Cargo.toml` found (error case)
- [x] 3.4 — Unit tests for `detect::agents()`
  - No agents detected
  - Single agent
  - Multiple agents simultaneously

---

## 4. Skill content loader (`src/skill/`)

- [x] 4.1 — Implement `layer.rs`
  - `Layer` enum: `Lookup`, `Reasoning`, `Execution`
  - `LayerSet` struct: bitfield or `Vec<Layer>`
  - Mapping: `lookup` → `[Lookup]`, `think` → `[Lookup, Reasoning]`,
    `write` → `[Lookup, Reasoning, Execution]`
- [x] 4.2 — Implement `mod.rs`
  - Embed assets via `include_str!()` at compile time
  - `load(layers: &LayerSet) -> String` — concatenate requested layers
- [x] 4.3 — Implement `prefix.rs`
  - `VALID_PREFIXES` constant list
  - `validate(prefix: &str) -> Result<()>` — error on unknown prefix
  - `filter(content: &str, prefix: &str) -> String` — extract matching section
    from Layer 1 content by prefix marker
- [x] 4.4 — Unit tests for `prefix::filter()`
  - Known prefix returns correct section
  - Unknown prefix returns error
  - Empty prefix returns full Layer 1

---

## 5. Deploy (`src/deploy.rs`)

- [x] 5.1 — Define agent install paths
  - `ClaudeCode` → `.claude/skills/rust.md`
  - `Cursor` → `.cursor/rules/rust.md`
  - `Windsurf` → `.windsurf/rules/rust.md`
  - `AgentsMd` → append section to `AGENTS.md`
- [x] 5.2 — Implement `deploy::skill_files(agents, repo_root)`
  - Create parent directories if missing
  - Write bundled skill index (`layer1.md` content) to each agent path
  - Print confirmation per agent: `✓ deployed to .claude/skills/rust.md`
- [x] 5.3 — Handle `AgentsMd` case
  - If `AGENTS.md` exists, append skill section with header
  - If not, create it with skill section
- [x] 5.4 — Integration tests for deploy
  - Verify files written to correct paths
  - Verify parent dirs created
  - Verify idempotent (re-running overwrites cleanly)

---

## 6. Gitignore (`src/gitignore.rs`)

- [x] 6.1 — Implement `gitignore::ensure(repo_root)`
  - Read `.gitignore` if present
  - Check if `.skill/` already present
  - Append `.skill/` if missing
  - Create `.gitignore` if absent
- [x] 6.2 — Unit tests
  - `.gitignore` absent → created with `.skill/`
  - `.gitignore` present, entry absent → appended
  - `.gitignore` present, entry already present → no-op

---

## 7. Context writer (`src/context.rs`)

- [x] 7.1 — Implement `context::write(repo_root, content)`
  - Create `.skill/` directory if missing
  - Write `content` to `.skill/context.md`
  - Overwrite if exists
- [x] 7.2 — Implement `context::clear(repo_root)`
  - Delete `.skill/context.md` if present
  - No-op if absent (no error)
- [x] 7.3 — Unit tests
  - Write creates file + dir
  - Write overwrites existing
  - Clear removes file
  - Clear on absent file is no-op

---

## 8. Subcommand integration

- [x] 8.1 — Wire `Init` → `detect::repo()` + `detect::agents()` + `deploy::skill_files()` + `gitignore::ensure()`
- [x] 8.2 — Wire `Lookup(prefix)` → `skill::load([Lookup])` + `prefix::filter()` + `context::write()`
- [x] 8.3 — Wire `Think` → `skill::load([Lookup, Reasoning])` + `context::write()`
- [x] 8.4 — Wire `Write` → `skill::load([Lookup, Reasoning, Execution])` + `context::write()`
- [x] 8.5 — Wire `Clear` → `context::clear()`
- [x] 8.6 — End-to-end integration test per subcommand

---

## 9. Polish + pre-publish

- [x] 9.1 — Confirm all `cargo clippy -- -D warnings` passes clean
- [x] 9.2 — Confirm `cargo fmt --check` passes clean
- [x] 9.3 — Confirm `cargo test` passes (unit + integration)
- [x] 9.4 — Confirm `cargo doc --no-deps` compiles without warnings
- [x] 9.5 — Run `cargo publish --dry-run` and resolve any issues
- [x] 9.6 — Tag `v0.1.0` on `main`
- [x] 9.7 — Publish to crates.io

---

## Deferred (v0.2.0+)

- `--dry-run` flag for `init`
- Config file (`skill.toml`) for custom agent paths
- Python/uv skill content (`assets/python/`)
- TypeScript/pnpm skill content (`assets/typescript/`)
- Remote skill fetch (`--remote` flag)
- Skill content update check (`cargo skill update`)
- `cargo skill status` — show what is deployed + current context mode
- 
