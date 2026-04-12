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

See [skill layer reference](docs/layers.md) for all available prefixes.

---

## How it works

### Install-time (`init`)

1. Detects repo type — single crate or workspace
2. Detects which agents are present (`.claude/`, `.cursor/`, `.windsurf/`, `AGENTS.md`)
3. Deploys bundled `SKILL.md` to the correct path for each detected agent
4. Adds `.skill/` to `.gitignore`

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

| Agent       | Install path                  |
|-------------|-------------------------------|
| Claude Code | `.claude/skills/rust.md`      |
| Cursor      | `.cursor/rules/rust.md`       |
| Windsurf    | `.windsurf/rules/rust.md`     |
| AGENTS.md   | `AGENTS.md` (appended)        |

---

## Skill content (v0.2.2)

Bundled skill covers Rust only. Three layers:

- **Layer 1 — Lookup**: 14-category rule index with priority levels (`own-`, `err-`,
  `mem-`, `api-`, `async-`, `opt-`, `type-`, `perf-`, `test-`, `doc-`, `name-`,
  `proj-`, `lint-`, `anti-`)
- **Layer 2 — Reasoning**: 3-layer cognitive model (Domain → Design → Mechanics),
  question routing, compiler error quick-ref
- **Layer 3 — Execution**: RPI loop (Research → Plan → Implement), verification
  checklist, task-to-rule mapping

Sources: `leonardomso/rust-skills` (MIT), `actionbook/rust-skills` (MIT),
`udapy/rust-agentic-skills` (MIT).

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
