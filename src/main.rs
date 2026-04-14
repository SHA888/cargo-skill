use anstream::println;
use anyhow::{Context, Result};
use cargo_skill::{context, deploy, detect, gitignore, skill};
use clap::{Parser, Subcommand};
use std::sync::atomic::{AtomicBool, Ordering};

// Global quiet flag — when set, suppress all non-error output
static QUIET: AtomicBool = AtomicBool::new(false);

/// Set the global quiet flag
fn set_quiet(quiet: bool) {
    QUIET.store(quiet, Ordering::SeqCst);
}

/// Print info message only if not in quiet mode
fn info(msg: &str) {
    if !QUIET.load(Ordering::SeqCst) {
        println!("{}", msg);
    }
}

/// Print info with format args (convenience wrapper)
#[allow(unused_macros)]
macro_rules! infof {
    ($($arg:tt)*) => {
        if !$crate::QUIET.load(::std::sync::atomic::Ordering::SeqCst) {
            ::anstream::println!($($arg)*);
        }
    };
}

// Color styles using ANSI escape codes
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn success(msg: &str) -> String {
    format!("{}✓{}{}", GREEN, RESET, msg)
}

fn warning(msg: &str) -> String {
    format!("{}⚠{}{}", YELLOW, RESET, msg)
}

fn error_style(msg: &str) -> String {
    format!("{}✗{}{}", RED, RESET, msg)
}

#[derive(Parser)]
#[command(name = "cargo-skill")]
#[command(version, about = "Deploy and activate layered AI agent skills")]
struct Cli {
    /// Suppress all output except errors
    #[arg(short = 'q', long, global = true)]
    quiet: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect repo + agents, deploy skill files
    Init {
        /// Print what would be deployed without writing any files
        #[arg(long)]
        dry_run: bool,
        /// Overwrite existing skill files even if unchanged
        #[arg(long)]
        force: bool,
    },
    /// Activate Layer 1 only (rule index, optional prefix filter)
    Lookup {
        /// Optional prefix filter (e.g., "own", "err", "async")
        prefix: Option<String>,
    },
    /// Activate Layer 1 + 2 (lookup + reasoning)
    Think,
    /// Activate all layers (lookup + reasoning + execution)
    Write,
    /// Remove .skill/context.md
    Clear,
    /// Show repo status, agents, and active context
    Status,
    /// Catch unrecognized subcommands for prefix shorthand
    #[command(external_subcommand)]
    External(Vec<String>),
}

fn main() -> Result<()> {
    // When invoked as `cargo skill <cmd>`, cargo inserts "skill" as argv[1].
    // Strip it so clap receives the correct arguments.
    let mut args: Vec<_> = std::env::args_os().collect();
    if args.get(1).and_then(|s| s.to_str()) == Some("skill") {
        args.remove(1);
    }
    let cli = Cli::parse_from(args);
    set_quiet(cli.quiet);

    match cli.command {
        Commands::Init { dry_run, force } => cmd_init(dry_run, force),
        Commands::Lookup { prefix } => cmd_lookup(prefix),
        Commands::Think => cmd_think(),
        Commands::Write => cmd_write(),
        Commands::Clear => cmd_clear(),
        Commands::Status => cmd_status(),
        Commands::External(args) => {
            // Shorthand: cargo skill <prefix> → cargo skill lookup <prefix>
            if let Some(first) = args.first() {
                let cmd = first.as_str();
                if skill::prefix::VALID_PREFIXES.contains(&cmd) {
                    cmd_lookup(Some(cmd.to_string()))
                } else {
                    anyhow::bail!(
                        "Unknown command: '{}'.\nValid commands: init, lookup, think, write, clear\nValid prefixes for shorthand: {}",
                        cmd,
                        skill::prefix::VALID_PREFIXES.join(", ")
                    )
                }
            } else {
                anyhow::bail!("No command provided")
            }
        }
    }
}

fn cmd_init(dry_run: bool, force: bool) -> Result<()> {
    if dry_run {
        info("[DRY RUN] Would initialize cargo-skill...");
    } else {
        info("Initializing cargo-skill...");
    }

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;
    info(&success(&format!(
        " detected {} repo at {}",
        format_repo_kind(&repo.kind),
        repo.root.display()
    )));

    // Handle .gitignore
    let gitignore_path = repo.root.join(".gitignore");
    let skill_entry = ".skill/";
    let needs_gitignore = !gitignore_path.exists()
        || !std::fs::read_to_string(&gitignore_path)
            .map(|s| s.contains(skill_entry))
            .unwrap_or(false);

    if dry_run {
        if needs_gitignore {
            info("[DRY RUN] Would add .skill/ to .gitignore");
        } else {
            info("[DRY RUN] .skill/ already in .gitignore (no change)");
        }
    } else {
        gitignore::ensure(&repo.root).context("Failed to update .gitignore")?;
        info(&success(" ensured .skill/ is in .gitignore"));
    }

    // Detect agents
    let agents = detect::agents(&repo.root);
    if agents.is_empty() {
        info(&warning(
            " no agents detected (create .claude/, .cursor/, .windsurf/, or AGENTS.md)",
        ));
        return Ok(());
    }
    for agent in &agents {
        info(&success(&format!(" detected agent: {:?}", agent)));
    }

    // Check existing skill files for force flag
    if !force && !dry_run {
        let existing: Vec<_> = agents
            .iter()
            .filter_map(|a| {
                let path = repo.root.join(a.skill_path());
                if path.exists() {
                    Some(path.display().to_string())
                } else {
                    None
                }
            })
            .collect();
        if !existing.is_empty() {
            info(&warning(" Skipping deploy — skill files already exist:"));
            for f in &existing {
                info(&format!("  - {}", f));
            }
            info(&warning(" Use --force to overwrite"));
            info(&success(" Initialization complete! (skipped deploy)"));
            return Ok(());
        }
    }

    // Deploy skill files
    if dry_run {
        for agent in &agents {
            let path = repo.root.join(agent.skill_path());
            if force || !path.exists() {
                info(&format!("[DRY RUN] Would deploy to {}", path.display()));
            } else {
                info(&format!(
                    "[DRY RUN] Skill file already exists at {} (use --force to overwrite)",
                    path.display()
                ));
            }
        }
        info(&format!(
            "\n[DRY RUN] {}",
            success(" Initialization would complete")
        ));
    } else {
        deploy::deploy(&agents, &repo.root).context("Failed to deploy skill files")?;
        info(&format!("\n{}", success(" Initialization complete!")));
    }
    Ok(())
}

fn format_repo_kind(kind: &detect::RepoKind) -> &'static str {
    match kind {
        detect::RepoKind::SingleCrate => "single crate",
        detect::RepoKind::Workspace => "workspace",
    }
}

fn cmd_lookup(prefix: Option<String>) -> Result<()> {
    if let Some(ref p) = prefix {
        info(&format!("Activating Layer 1 with prefix: {}", p));
    } else {
        info("Activating Layer 1 (full lookup)");
    }

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Load Layer 1 with optional prefix filter
    let content =
        skill::load_lookup_filtered(prefix.as_deref()).context("Failed to load skill content")?;

    // Write to context
    context::write(&repo.root, &content).context("Failed to write context")?;

    info(&success(" wrote context to .skill/context.md"));
    Ok(())
}

fn cmd_think() -> Result<()> {
    info("Activating Layer 1 + 2 (lookup + reasoning)");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Load Layers 1 + 2
    let layer_set = skill::layer::LayerSet::think();
    let content = skill::load(&layer_set).context("Failed to load skill content")?;

    // Write to context
    context::write(&repo.root, &content).context("Failed to write context")?;

    info(&success(" wrote context to .skill/context.md"));
    Ok(())
}

fn cmd_write() -> Result<()> {
    info("Activating all layers (lookup + reasoning + execution)");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Load all layers
    let layer_set = skill::layer::LayerSet::write();
    let content = skill::load(&layer_set).context("Failed to load skill content")?;

    // Write to context
    context::write(&repo.root, &content).context("Failed to write context")?;

    info(&success(" wrote context to .skill/context.md"));
    Ok(())
}

fn cmd_clear() -> Result<()> {
    info("Clearing skill context...");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Clear context
    context::clear(&repo.root).context("Failed to clear context")?;

    info(&success(" cleared .skill/context.md"));
    Ok(())
}

fn cmd_status() -> Result<()> {
    info("cargo-skill status\n");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;
    info("Repository:");
    info(&format!("  Kind: {}", format_repo_kind(&repo.kind)));
    info(&format!("  Root: {}", repo.root.display()));

    // Detect agents
    let agents = detect::agents(&repo.root);
    info(&format!("\nAgents detected: {}", agents.len()));
    for agent in &agents {
        let path = repo.root.join(agent.skill_path());
        let deployed = if path.exists() {
            success("")
        } else {
            error_style("")
        };
        info(&format!("  {} {:?} -> {}", deployed, agent, path.display()));
    }
    if agents.is_empty() {
        info("  (none — create .claude/, .cursor/, .windsurf/, or AGENTS.md)");
    }

    // Check context.md
    let context_path = repo.root.join(".skill/context.md");
    info("\nContext:");
    if context_path.exists() {
        let content =
            std::fs::read_to_string(&context_path).context("Failed to read context.md")?;
        let lines = content.lines().count();
        let (mode, prefix) = detect_context_mode(&content);
        info(&format!("  Status: active ({}, {} lines)", mode, lines));
        if let Some(p) = prefix {
            info(&format!("  Prefix: {}", p));
        }
    } else {
        info("  Status: none (run `cargo skill lookup|think|write` to activate)");
    }

    // Check .gitignore
    let gitignore_path = repo.root.join(".gitignore");
    let gitignore_ok = gitignore_path.exists()
        && std::fs::read_to_string(&gitignore_path)
            .map(|s| s.contains(".skill/"))
            .unwrap_or(false);
    info("\nGitignore:");
    if gitignore_ok {
        info(&format!("  {}", success(" .skill/ is ignored")));
    } else {
        info(&format!(
            "  {}",
            error_style(" .skill/ is NOT in .gitignore (run `cargo skill init`)")
        ));
    }

    Ok(())
}

/// Detect the context mode by analyzing context.md content
fn detect_context_mode(content: &str) -> (&'static str, Option<&str>) {
    let first_line = content.lines().next().unwrap_or("");

    // Check for prefix filter
    let prefix = content
        .lines()
        .find(|l| l.contains("Filtered for prefix:"))
        .and_then(|l| l.split("**").nth(1))
        .map(|s| s.trim_end_matches('-'));

    if first_line.contains("Layer 3") || content.contains("Layer 3") {
        ("write", prefix)
    } else if first_line.contains("Layer 2") || content.contains("Layer 2") {
        ("think", prefix)
    } else if first_line.contains("Layer 1") || content.contains("Layer 1") {
        ("lookup", prefix)
    } else {
        ("unknown", None)
    }
}
