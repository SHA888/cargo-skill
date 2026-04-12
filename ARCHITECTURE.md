# Architecture

## Overview

`cargo-skill` is a single-binary Cargo subcommand. No runtime dependencies beyond
the standard library and a minimal set of crates. Designed to be fast, offline-capable,
and constrained-machine friendly.

---

## Repository layout

```
cargo-skill/
├── src/
│   ├── main.rs           Entry point, CLI parsing, subcommand dispatch
│   ├── detect.rs         Repo structure + agent detection
│   ├── deploy.rs         Install-time: copy skill files to agent paths
│   ├── context.rs        Execution-time: write/clear .skill/context.md
│   ├── skill/
│   │   ├── mod.rs        Skill content loader (bundled assets)
│   │   ├── layer.rs      Layer enum + filter logic
│   │   └── prefix.rs     Prefix filter parsing + validation
│   └── gitignore.rs      .gitignore mutation
├── assets/
│   └── rust/
│       ├── layer1.md     Lookup layer (rule index)
│       ├── layer2.md     Reasoning layer (cognitive model)
│       └── layer3.md     Execution layer (RPI loop + verification)
├── tests/
│   ├── detect.rs         Integration tests for repo/agent detection
│   ├── deploy.rs         Integration tests for file deployment
│   └── context.rs        Integration tests for context file writes
├── ARCHITECTURE.md       This file
├── README.md
├── TODO.md
├── Cargo.toml
├── LICENSE-MIT
└── LICENSE-APACHE
```

---

## Core data flow

### Install-time (`init`)

```
main()
  → parse_args()           clap: subcommand = Init
  → detect::repo()         find Cargo.toml, determine workspace vs single
  → detect::agents()       scan for .claude/, .cursor/, .windsurf/, AGENTS.md
  → deploy::skill_files()  for each detected agent, write assets/rust/* to agent path
  → gitignore::ensure()    append .skill/ to .gitignore if not present
  → provenance::write()    write .skill/provenance.md (version, hashes, agents, timestamp)
```

### Execution-time (`lookup` / `think` / `write`)

```
main()
  → parse_args()           clap: subcommand = Lookup(prefix) | Think | Write
  → skill::load(layers)    read bundled assets for requested layers
  → skill::filter(prefix)  if lookup with prefix, filter rule index to matching section
  → context::write()       write filtered content to .skill/context.md
```

### Clear

```
main()
  → parse_args()           clap: subcommand = Clear
  → context::clear()       delete .skill/context.md if present
```

---

## Key design decisions

### Bundled assets, no network

Skill content is embedded at compile time via `include_str!()`. No HTTP client,
no registry fetch, no network dependency. Works fully offline.

Tradeoff: skill content updates require a new release. Acceptable for v0.1.0.

### `.skill/context.md` is ephemeral

Execution-time context is gitignored and overwritten on every invocation.
Session state is never persisted across invocations unless the user explicitly
does not call `clear`.

Install-time files (in `.claude/skills/`, `.cursor/rules/`, etc.) are committed
and persistent.

### Layer splitting in assets

Skill content is split into three separate asset files (`layer1.md`, `layer2.md`,
`layer3.md`) rather than one monolithic file. This allows:
- Precise layer composition per command
- Prefix filtering scoped to Layer 1 only
- Independent updates to each layer without touching others

### Prefix filter

`cargo skill lookup <prefix>` extracts only the section of `layer1.md` matching
the given prefix (e.g., `own`, `async`, `err`). Implemented as a simple line-range
extraction — no parser, no AST, sections are delimited by `**<prefix>-**` markers.

### Workflow aliases

`cargo skill review`, `refactor`, and `debug` are intention-based aliases that compose
specific prefix filters and layer combinations without exposing the layer model to the user.
Each alias is a pure mapping — no new skill content, no new files. The mapping is defined
statically in `src/workflow.rs`:

| Alias      | Prefixes active           | Layers  |
|------------|---------------------------|---------|
| `review`   | `err`, `test`, `lint`     | 1 + 2   |
| `refactor` | `type`, `api`, `name`     | 1 + 2   |
| `debug`    | `err`, `mem`              | 1 + 2*  |

*Layer 2 scoped to compiler quick-ref section only for `debug`.

### Agent detection

Detection is filesystem-based, not config-based:

| Signal                  | Agent assumed  |
|-------------------------|----------------|
| `.claude/` directory    | Claude Code    |
| `.cursor/` directory    | Cursor         |
| `.windsurf/` directory  | Windsurf       |
| `AGENTS.md` present     | AGENTS.md std  |

Multiple agents can be detected simultaneously. `init` deploys to all of them.

---

## Dependencies (planned)

| Crate    | Purpose                        | Justification              |
|----------|--------------------------------|----------------------------|
| `clap`   | CLI argument parsing           | Standard, derive macros    |
| `anyhow` | Error handling                 | Application-level errors   |

No async runtime. No HTTP client. No serde. Intentionally minimal.

---

## Out of scope (v0.1.0)

- Remote skill fetching
- Config file (`skill.toml`)
- Python / TypeScript skill content
- `--dry-run` flag
- Skill content versioning / update checks
- Windows path handling beyond basic `PathBuf` usage
- `skill.lock` (planned v0.7.0)
- Workflow alias commands (planned v0.2.x)
- Provenance sidecar beyond v0.2.x (full hash verification planned v0.7.0)
