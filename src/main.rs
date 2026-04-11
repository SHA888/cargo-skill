use anyhow::{Context, Result};
use cargo_skill::{context, deploy, detect, gitignore, skill};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo-skill")]
#[command(version, about = "Deploy and activate layered AI agent skills")]
struct Cli {
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
        println!("[DRY RUN] Would initialize cargo-skill...");
    } else {
        println!("Initializing cargo-skill...");
    }

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;
    println!(
        "✓ detected {} repo at {}",
        format_repo_kind(&repo.kind),
        repo.root.display()
    );

    // Handle .gitignore
    let gitignore_path = repo.root.join(".gitignore");
    let skill_entry = ".skill/";
    let needs_gitignore = !gitignore_path.exists()
        || !std::fs::read_to_string(&gitignore_path)
            .map(|s| s.contains(skill_entry))
            .unwrap_or(false);

    if dry_run {
        if needs_gitignore {
            println!("[DRY RUN] Would add .skill/ to .gitignore");
        } else {
            println!("[DRY RUN] .skill/ already in .gitignore (no change)");
        }
    } else {
        gitignore::ensure(&repo.root).context("Failed to update .gitignore")?;
        println!("✓ ensured .skill/ is in .gitignore");
    }

    // Detect agents
    let agents = detect::agents(&repo.root);
    if agents.is_empty() {
        println!("⚠ no agents detected (create .claude/, .cursor/, .windsurf/, or AGENTS.md)");
        return Ok(());
    }
    for agent in &agents {
        println!("✓ detected agent: {:?}", agent);
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
            println!("\n⚠ Skipping deploy — skill files already exist:");
            for f in &existing {
                println!("  - {}", f);
            }
            println!("  Use --force to overwrite");
            println!("\nInitialization complete! (skipped deploy)");
            return Ok(());
        }
    }

    // Deploy skill files
    if dry_run {
        for agent in &agents {
            let path = repo.root.join(agent.skill_path());
            if force || !path.exists() {
                println!("[DRY RUN] Would deploy to {}", path.display());
            } else {
                println!(
                    "[DRY RUN] Skill file already exists at {} (use --force to overwrite)",
                    path.display()
                );
            }
        }
        println!("\n[DRY RUN] Initialization would complete");
    } else {
        deploy::deploy(&agents, &repo.root).context("Failed to deploy skill files")?;
        println!("\nInitialization complete!");
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
        println!("Activating Layer 1 with prefix: {}", p);
    } else {
        println!("Activating Layer 1 (full lookup)");
    }

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Load Layer 1 with optional prefix filter
    let content =
        skill::load_lookup_filtered(prefix.as_deref()).context("Failed to load skill content")?;

    // Write to context
    context::write(&repo.root, &content).context("Failed to write context")?;

    println!("✓ wrote context to .skill/context.md");
    Ok(())
}

fn cmd_think() -> Result<()> {
    println!("Activating Layer 1 + 2 (lookup + reasoning)");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Load Layers 1 + 2
    let layer_set = skill::layer::LayerSet::think();
    let content = skill::load(&layer_set).context("Failed to load skill content")?;

    // Write to context
    context::write(&repo.root, &content).context("Failed to write context")?;

    println!("✓ wrote context to .skill/context.md");
    Ok(())
}

fn cmd_write() -> Result<()> {
    println!("Activating all layers (lookup + reasoning + execution)");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Load all layers
    let layer_set = skill::layer::LayerSet::write();
    let content = skill::load(&layer_set).context("Failed to load skill content")?;

    // Write to context
    context::write(&repo.root, &content).context("Failed to write context")?;

    println!("✓ wrote context to .skill/context.md");
    Ok(())
}

fn cmd_clear() -> Result<()> {
    println!("Clearing skill context...");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;

    // Clear context
    context::clear(&repo.root).context("Failed to clear context")?;

    println!("✓ cleared .skill/context.md");
    Ok(())
}

fn cmd_status() -> Result<()> {
    println!("cargo-skill status\n");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;
    println!("Repository:");
    println!("  Kind: {}", format_repo_kind(&repo.kind));
    println!("  Root: {}", repo.root.display());

    // Detect agents
    let agents = detect::agents(&repo.root);
    println!("\nAgents detected: {}", agents.len());
    for agent in &agents {
        let path = repo.root.join(agent.skill_path());
        let deployed = if path.exists() { "✓" } else { "✗" };
        println!("  {} {:?} -> {}", deployed, agent, path.display());
    }
    if agents.is_empty() {
        println!("  (none — create .claude/, .cursor/, .windsurf/, or AGENTS.md)");
    }

    // Check context.md
    let context_path = repo.root.join(".skill/context.md");
    println!("\nContext:");
    if context_path.exists() {
        let content =
            std::fs::read_to_string(&context_path).context("Failed to read context.md")?;
        let lines = content.lines().count();
        let (mode, prefix) = detect_context_mode(&content);
        println!("  Status: active ({}, {} lines)", mode, lines);
        if let Some(p) = prefix {
            println!("  Prefix: {}", p);
        }
    } else {
        println!("  Status: none (run `cargo skill lookup|think|write` to activate)");
    }

    // Check .gitignore
    let gitignore_path = repo.root.join(".gitignore");
    let gitignore_ok = gitignore_path.exists()
        && std::fs::read_to_string(&gitignore_path)
            .map(|s| s.contains(".skill/"))
            .unwrap_or(false);
    println!("\nGitignore:");
    if gitignore_ok {
        println!("  ✓ .skill/ is ignored");
    } else {
        println!("  ✗ .skill/ is NOT in .gitignore (run `cargo skill init`)");
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
