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
    Init,
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
        Commands::Init => cmd_init(),
        Commands::Lookup { prefix } => cmd_lookup(prefix),
        Commands::Think => cmd_think(),
        Commands::Write => cmd_write(),
        Commands::Clear => cmd_clear(),
    }
}

fn cmd_init() -> Result<()> {
    println!("Initializing cargo-skill...");

    // Detect repository
    let repo = detect::repo().context("Failed to detect repository")?;
    println!(
        "✓ detected {} repo at {}",
        format_repo_kind(&repo.kind),
        repo.root.display()
    );

    // Ensure .skill/ is in .gitignore (always, regardless of agents)
    gitignore::ensure(&repo.root).context("Failed to update .gitignore")?;
    println!("✓ ensured .skill/ is in .gitignore");

    // Detect agents
    let agents = detect::agents(&repo.root);
    if agents.is_empty() {
        println!("⚠ no agents detected (create .claude/, .cursor/, .windsurf/, or AGENTS.md)");
        return Ok(());
    }
    for agent in &agents {
        println!("✓ detected agent: {:?}", agent);
    }

    // Deploy skill files
    deploy::deploy(&agents, &repo.root).context("Failed to deploy skill files")?;

    println!("\nInitialization complete!");
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
