# cargo-skill

A Cargo subcommand for deploying and activating layered AI agent skills in Rust projects.

```
cargo install cargo-skill
```

---

## What it does

`cargo-skill` has two jobs:

**Install-time** — detects your repo structure and agent tooling, then deploys the right
skill files to the right paths.

**Execution-time** — writes a scoped context file (`.skill/context.md`) that controls
which skill layer the agent uses for the current session.

---

## Commands

```
cargo skill init                  Detect repo + agents, deploy skill files
cargo skill init --dry-run        Preview what would be deployed
cargo skill init --force          Overwrite existing skill files

cargo skill lookup [prefix]       Activate Layer 1 only (rule index, optional prefix filter)
cargo skill think                 Activate Layer 1 + 2 (lookup + reasoning)
cargo skill write                 Activate all layers (lookup + reasoning + execution)
cargo skill clear                 Remove .skill/context.md

cargo skill status                Show repo, agents, context, and gitignore status

cargo skill review                Review-focused context (err + test + lint + reasoning)
cargo skill refactor              Refactor-focused context (type + api + name + reasoning)
cargo skill debug                 Debug-focused context (err + mem + compiler quick-ref)
```

### Quick prefix shorthand

```
cargo skill own            # Same as: cargo skill lookup own
cargo skill async          # Same as: cargo skill lookup async
cargo skill err            # Same as: cargo skill lookup err
```

All 14 prefixes work: `own`, `err`, `mem`, `api`, `async`, `opt`, `type`, `perf`, `test`, `doc`, `name`, `proj`, `lint`, `anti`

See [Layer 1 reference](#skill-content-v027) for all available prefixes.

---

## How it works

### Install-time (`init`)

1. Detects repo type — single crate or workspace
2. Detects which agents are present (`.claude/`, `.cursor/`, `.windsurf/`, `AGENTS.md`)
3. Deploys bundled skill layers (`layer1.md`, `layer2.md`, `layer3.md`) and agent personas
   to the correct paths for each detected agent
4. Generates Claude Code slash commands (`.claude/commands/*.md`)
5. Adds `.skill/` and context files to `.gitignore`

### Execution-time (`lookup` / `think` / `write`)

Writes `.skill/context.md` with the appropriate layer subset:

| Command              | Layer 1 (Lookup) | Layer 2 (Reasoning) | Layer 3 (Execution) |
|----------------------|:----------------:|:-------------------:|:-------------------:|
| `lookup [prefix]`    | ✓ (filtered)     |                     |                     |
| `think`              | ✓                | ✓                   |                     |
| `write`              | ✓                | ✓                   | ✓                   |
| `review`             | ✓ (`err`,`test`,`lint`) | ✓            |                     |
| `refactor`           | ✓ (`type`,`api`,`name`) | ✓            |                     |
| `debug`              | ✓ (`err`,`mem`)  | ✓ (quick-ref only)  |                     |

The agent reads `.skill/context.md` as session context. This file is ephemeral —
gitignored, overwritten on each invocation, deleted by `clear`.

---

## Agent support

| Agent       | Install path                             | Content                        |
|-------------|------------------------------------------|--------------------------------|
| Claude Code | `.claude/skills/rust.md`                 | Rust skill layers              |
|             | `.claude/commands/*.md` (slash)          | Skill workflow slash commands  |
|             | `.claude/skills/agents/rust-*.md`        | Agent personas                 |
| Cursor      | `.cursor/rules/rust.md`                  | Rust skill layers              |
| Windsurf    | `.windsurf/rules/rust.md`                | Rust skill layers              |
| AGENTS.md   | `AGENTS.md` (appended)                   | Fallback for other agents      |

---

## Skill content (v0.2.7)

### Rust (stable)

Three-layer skill with 14-category rule index:

- **Layer 1 — Lookup**: Rule categories with priority levels (`own-`, `err-`,
  `mem-`, `api-`, `async-`, `opt-`, `type-`, `perf-`, `test-`, `doc-`, `name-`,
  `proj-`, `lint-`, `anti-`)
- **Layer 2 — Reasoning**: 3-layer cognitive model (Domain → Design → Mechanics),
  question routing, compiler error quick-ref (E0001–E0716)
- **Layer 3 — Execution**: RPI loop (Research → Plan → Implement), verification
  checklist, task-to-rule mapping

Sources: `leonardomso/rust-skills` (MIT), `actionbook/rust-skills` (MIT),
`udapy/rust-agentic-skills` (MIT).

### Agent Personas (v0.2.7)

Deployed to `.claude/skills/agents/` on `cargo skill init`:

- **rust-architect.md** — Systems architect persona for API and module design decisions,
  maps to `api-`, `proj-`, `type-` prefix rules
- **rust-reviewer.md** — Senior code reviewer persona with five-axis review
  (correctness, safety, perf, API, docs), maps to `anti-` + `lint-` prefix rules

### Python (in development)

Layer assets exist (`assets/python/`) but stack detection (pyproject.toml, uv.lock)
is not yet implemented. Multi-language context routing planned for v0.3.x.

---

## Latest Updates

See [CHANGELOG.md](CHANGELOG.md) for the full release history. Recent highlights:

- **v0.2.7** — Workflow aliases (`review`, `refactor`, `debug`), Claude Code slash commands, and agent personas (rust-architect, rust-reviewer)
- **v0.2.6** — Provenance sidecar (`--quiet` flag, deployment metadata)
- **v0.2.5** — Colored terminal output (green ✓, yellow ⚠, red ✗)
- **v0.2.4** — Agent-specific context files for Cursor and Windsurf
- **v0.2.3** — Claude Code context injection footer
- **v0.2.2** — `status` command and comprehensive project overview
- **v0.2.1** — `--dry-run` and `--force` flags for `init`
- **v0.3.0** (in progress) — Python skill layers authored; stack detection and multi-language routing planned

---

## Installation

```bash
cargo install cargo-skill
```

### Post-installation setup

After installation, ensure `~/.cargo/bin` is in your PATH (Cargo usually does this automatically). Then optionally add a shell alias for convenience:

**Bash/Zsh:**
```bash
echo 'alias skill="cargo skill"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or ~/.zshrc
```

**Fish:**
```bash
alias skill "cargo skill"
funcsave skill
```

Now you can use `skill` as a shorthand:
```bash
skill status
skill review
skill own
```

---

## Requirements

- Rust stable (latest)
- Cargo

---

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
