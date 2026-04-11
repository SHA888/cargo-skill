mod context;
mod deploy;
mod detect;
mod gitignore;
mod skill;

use anyhow::Result;
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
    let cli = Cli::parse();

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
    // TODO: Implement repo detection, agent detection, deploy, gitignore
    Ok(())
}

fn cmd_lookup(prefix: Option<String>) -> Result<()> {
    if let Some(p) = prefix {
        println!("Activating Layer 1 with prefix: {}", p);
    } else {
        println!("Activating Layer 1 (full lookup)");
    }
    // TODO: Load layer1, filter by prefix, write context
    Ok(())
}

fn cmd_think() -> Result<()> {
    println!("Activating Layer 1 + 2 (lookup + reasoning)");
    // TODO: Load layers 1+2, write context
    Ok(())
}

fn cmd_write() -> Result<()> {
    println!("Activating all layers (lookup + reasoning + execution)");
    // TODO: Load all layers, write context
    Ok(())
}

fn cmd_clear() -> Result<()> {
    println!("Clearing skill context...");
    // TODO: Remove .skill/context.md
    Ok(())
}
