# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/SHA888/cargo-skill/releases/tag/v0.1.0
