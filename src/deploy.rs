use crate::detect::Agent;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const SKILL_HEADER: &str = "# Rust Skill Reference\n\nThis section provides a quick lookup index for Rust development rules.\n";

const CLAUDE_CONTEXT_FOOTER: &str = r#"
## Active Session Context
If `.skill/context.md` exists, load it now.
It contains the active skill layer for this session.
Apply it on top of this index.
@.skill/context.md
"#;

// Claude Code slash command prompts
const CMD_LOOKUP_PROMPT: &str = "Load Layer 1 (lookup) skill context. Run: cargo skill lookup";
const CMD_THINK_PROMPT: &str =
    "Load Layers 1+2 (lookup + reasoning) skill context. Run: cargo skill think";
const CMD_WRITE_PROMPT: &str =
    "Load all layers (lookup + reasoning + execution) skill context. Run: cargo skill write";
const CMD_CLEAR_PROMPT: &str = "Clear the active skill context. Run: cargo skill clear";

/// Deploy skill files to all detected agents
pub fn deploy(agents: &[Agent], repo_root: &Path) -> Result<()> {
    let skill_content = include_str!("../assets/rust/layer1.md");

    for agent in agents {
        match agent {
            Agent::AgentsMd => deploy_to_agents_md(repo_root, skill_content)?,
            _ => deploy_to_file(agent, repo_root, skill_content)?,
        }
    }

    Ok(())
}

/// Deploy skill content to a file-based agent (creates parent directories)
#[allow(dead_code)]
fn deploy_to_file(agent: &Agent, repo_root: &Path, content: &str) -> Result<()> {
    let path = repo_root.join(agent.skill_path());

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    // Build content: skill + context footer (Claude Code only)
    let full_content = if matches!(agent, Agent::ClaudeCode) {
        format!("{}{}", content, CLAUDE_CONTEXT_FOOTER)
    } else {
        content.to_string()
    };

    // Write the skill file
    fs::write(&path, full_content)
        .with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

/// Deploy skill content to AGENTS.md (append or create)
///
/// This function is idempotent - if the skill content already exists in AGENTS.md,
/// it will not be duplicated.
#[allow(dead_code)]
fn deploy_to_agents_md(repo_root: &Path, content: &str) -> Result<()> {
    let path = repo_root.join(Agent::AgentsMd.skill_path());
    let skill_marker = "# Layer 1";

    if path.exists() {
        // Check if skill content already exists (idempotency check)
        let existing_content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        if existing_content.contains(skill_marker) {
            // Skill content already exists, skip to maintain idempotency
            return Ok(());
        }

        // Append to existing file (skill content not found)
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .with_context(|| format!("Failed to open {} for append", path.display()))?;

        writeln!(file)?;
        writeln!(file, "---")?;
        writeln!(file)?;
        writeln!(file, "{}", SKILL_HEADER)?;
        writeln!(file)?;
        write!(file, "{}", content)?;
    } else {
        // Create new file with skill content
        fs::write(&path, format!("{}\n{}\n", SKILL_HEADER, content))
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }

    Ok(())
}

/// Deploy agent personas to `.claude/skills/agents/`
///
/// Agent personas are Claude Code system prompt suggestions for specialized review/architecture tasks.
/// Deploys rust-reviewer.md and rust-architect.md as reference guides.
pub fn deploy_agent_personas(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let agents_dir = repo_root.join(".claude/skills/agents");
    fs::create_dir_all(&agents_dir)
        .with_context(|| format!("Failed to create {}", agents_dir.display()))?;

    let personas = [
        (
            "rust-reviewer.md",
            include_str!("../assets/agents/rust-reviewer.md"),
        ),
        (
            "rust-architect.md",
            include_str!("../assets/agents/rust-architect.md"),
        ),
    ];

    let mut deployed = Vec::new();

    for (filename, content) in personas {
        let path = agents_dir.join(filename);
        fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;
        deployed.push(path);
    }

    Ok(deployed)
}

/// Deploy Claude Code slash commands to `.claude/commands/`
///
/// Creates command files for `/skill-lookup`, `/skill-think`, `/skill-write`, `/skill-clear`
pub fn deploy_claude_commands(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let commands_dir = repo_root.join(".claude/commands");
    fs::create_dir_all(&commands_dir)
        .with_context(|| format!("Failed to create {}", commands_dir.display()))?;

    let commands = [
        ("skill-lookup.md", CMD_LOOKUP_PROMPT),
        ("skill-think.md", CMD_THINK_PROMPT),
        ("skill-write.md", CMD_WRITE_PROMPT),
        ("skill-clear.md", CMD_CLEAR_PROMPT),
    ];

    let mut deployed = Vec::new();

    for (filename, prompt) in commands {
        let path = commands_dir.join(filename);
        fs::write(&path, prompt).with_context(|| format!("Failed to write {}", path.display()))?;
        deployed.push(path);
    }

    Ok(deployed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_deploy_creates_claude_directory() {
        let temp = TempDir::new().unwrap();
        deploy(&[Agent::ClaudeCode], temp.path()).unwrap();

        let skill_path = temp.path().join(".claude/skills/rust.md");
        assert!(skill_path.exists());
        let content = fs::read_to_string(&skill_path).unwrap();
        assert!(content.contains("Layer 1"));
    }

    #[test]
    fn test_deploy_claude_commands_creates_files() {
        let temp = TempDir::new().unwrap();

        let deployed = deploy_claude_commands(temp.path()).unwrap();
        assert_eq!(deployed.len(), 4);

        // Verify all command files exist
        assert!(
            temp.path()
                .join(".claude/commands/skill-lookup.md")
                .exists()
        );
        assert!(temp.path().join(".claude/commands/skill-think.md").exists());
        assert!(temp.path().join(".claude/commands/skill-write.md").exists());
        assert!(temp.path().join(".claude/commands/skill-clear.md").exists());
    }

    #[test]
    fn test_deploy_claude_commands_content() {
        let temp = TempDir::new().unwrap();

        deploy_claude_commands(temp.path()).unwrap();

        // Verify lookup command content
        let lookup_content =
            fs::read_to_string(temp.path().join(".claude/commands/skill-lookup.md")).unwrap();
        assert!(lookup_content.contains("cargo skill lookup"));

        // Verify think command content
        let think_content =
            fs::read_to_string(temp.path().join(".claude/commands/skill-think.md")).unwrap();
        assert!(think_content.contains("cargo skill think"));

        // Verify write command content
        let write_content =
            fs::read_to_string(temp.path().join(".claude/commands/skill-write.md")).unwrap();
        assert!(write_content.contains("cargo skill write"));

        // Verify clear command content
        let clear_content =
            fs::read_to_string(temp.path().join(".claude/commands/skill-clear.md")).unwrap();
        assert!(clear_content.contains("cargo skill clear"));
    }

    #[test]
    fn test_deploy_claude_commands_is_idempotent() {
        let temp = TempDir::new().unwrap();

        // Deploy twice
        deploy_claude_commands(temp.path()).unwrap();
        deploy_claude_commands(temp.path()).unwrap();

        // Should still have 4 files
        let commands_dir = temp.path().join(".claude/commands");
        let entries: Vec<_> = fs::read_dir(&commands_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
            .collect();
        assert_eq!(entries.len(), 4);
    }

    #[test]
    fn test_deploy_creates_cursor_directory() {
        let temp = TempDir::new().unwrap();
        deploy(&[Agent::Cursor], temp.path()).unwrap();

        let skill_path = temp.path().join(".cursor/rules/rust.md");
        assert!(skill_path.exists());
    }

    #[test]
    fn test_deploy_creates_windsurf_directory() {
        let temp = TempDir::new().unwrap();
        deploy(&[Agent::Windsurf], temp.path()).unwrap();

        let skill_path = temp.path().join(".windsurf/rules/rust.md");
        assert!(skill_path.exists());
    }

    #[test]
    fn test_deploy_creates_agents_md() {
        let temp = TempDir::new().unwrap();
        deploy(&[Agent::AgentsMd], temp.path()).unwrap();

        let agents_md = temp.path().join("AGENTS.md");
        assert!(agents_md.exists());
        let content = fs::read_to_string(&agents_md).unwrap();
        assert!(content.contains("Rust Skill Reference"));
        assert!(content.contains("Layer 1"));
    }

    #[test]
    fn test_deploy_appends_to_existing_agents_md() {
        let temp = TempDir::new().unwrap();
        let agents_md = temp.path().join("AGENTS.md");
        fs::write(&agents_md, "# Existing AGENTS.md\n\nSome content.").unwrap();

        deploy(&[Agent::AgentsMd], temp.path()).unwrap();

        let content = fs::read_to_string(&agents_md).unwrap();
        assert!(content.contains("# Existing AGENTS.md"));
        assert!(content.contains("---"));
        assert!(content.contains("Rust Skill Reference"));
        assert!(content.contains("Layer 1"));
    }

    #[test]
    fn test_deploy_is_idempotent() {
        let temp = TempDir::new().unwrap();

        // Deploy twice
        deploy(&[Agent::ClaudeCode], temp.path()).unwrap();
        let first_content = fs::read_to_string(temp.path().join(".claude/skills/rust.md")).unwrap();

        deploy(&[Agent::ClaudeCode], temp.path()).unwrap();
        let second_content =
            fs::read_to_string(temp.path().join(".claude/skills/rust.md")).unwrap();

        assert_eq!(first_content, second_content);
    }

    #[test]
    fn test_deploy_agents_md_is_idempotent() {
        let temp = TempDir::new().unwrap();

        // Deploy twice to AGENTS.md
        deploy(&[Agent::AgentsMd], temp.path()).unwrap();
        let first_content = fs::read_to_string(temp.path().join("AGENTS.md")).unwrap();

        deploy(&[Agent::AgentsMd], temp.path()).unwrap();
        let second_content = fs::read_to_string(temp.path().join("AGENTS.md")).unwrap();

        assert_eq!(first_content, second_content);
        // Should only contain one skill reference header
        assert_eq!(first_content.matches("Rust Skill Reference").count(), 1);
    }

    #[test]
    fn test_deploy_multiple_agents() {
        let temp = TempDir::new().unwrap();
        deploy(
            &[Agent::ClaudeCode, Agent::Cursor, Agent::AgentsMd],
            temp.path(),
        )
        .unwrap();

        assert!(temp.path().join(".claude/skills/rust.md").exists());
        assert!(temp.path().join(".cursor/rules/rust.md").exists());
        assert!(temp.path().join("AGENTS.md").exists());
    }

    #[test]
    fn test_deploy_claude_gets_context_footer() {
        let temp = TempDir::new().unwrap();
        deploy(&[Agent::ClaudeCode], temp.path()).unwrap();

        let skill_path = temp.path().join(".claude/skills/rust.md");
        let content = fs::read_to_string(&skill_path).unwrap();

        // Claude Code should have the context footer
        assert!(content.contains("## Active Session Context"));
        assert!(content.contains("@.skill/context.md"));
    }

    #[test]
    fn test_deploy_cursor_no_context_footer() {
        let temp = TempDir::new().unwrap();
        deploy(&[Agent::Cursor], temp.path()).unwrap();

        let skill_path = temp.path().join(".cursor/rules/rust.md");
        let content = fs::read_to_string(&skill_path).unwrap();

        // Cursor should NOT have the context footer
        assert!(!content.contains("## Active Session Context"));
        assert!(!content.contains("@.skill/context.md"));
    }

    #[test]
    fn test_deploy_agent_personas_creates_directory() {
        let temp = TempDir::new().unwrap();

        deploy_agent_personas(temp.path()).unwrap();

        let agents_dir = temp.path().join(".claude/skills/agents");
        assert!(agents_dir.exists());
    }

    #[test]
    fn test_deploy_agent_personas_deploys_files() {
        let temp = TempDir::new().unwrap();

        let deployed = deploy_agent_personas(temp.path()).unwrap();
        assert_eq!(deployed.len(), 2);

        // Verify both persona files exist
        assert!(
            temp.path()
                .join(".claude/skills/agents/rust-reviewer.md")
                .exists()
        );
        assert!(
            temp.path()
                .join(".claude/skills/agents/rust-architect.md")
                .exists()
        );
    }

    #[test]
    fn test_deploy_agent_personas_content() {
        let temp = TempDir::new().unwrap();

        deploy_agent_personas(temp.path()).unwrap();

        // Verify reviewer persona content
        let reviewer_content =
            fs::read_to_string(temp.path().join(".claude/skills/agents/rust-reviewer.md")).unwrap();
        assert!(reviewer_content.contains("Rust Reviewer Agent"));
        assert!(reviewer_content.contains("Correctness"));
        assert!(reviewer_content.contains("Safety"));
        assert!(reviewer_content.contains("anti-"));
        assert!(reviewer_content.contains("lint-"));

        // Verify architect persona content
        let architect_content =
            fs::read_to_string(temp.path().join(".claude/skills/agents/rust-architect.md"))
                .unwrap();
        assert!(architect_content.contains("Rust Architect Agent"));
        assert!(architect_content.contains("API Design"));
        assert!(architect_content.contains("api-"));
        assert!(architect_content.contains("proj-"));
        assert!(architect_content.contains("type-"));
    }

    #[test]
    fn test_deploy_agent_personas_is_idempotent() {
        let temp = TempDir::new().unwrap();

        // Deploy twice
        deploy_agent_personas(temp.path()).unwrap();
        let first_content =
            fs::read_to_string(temp.path().join(".claude/skills/agents/rust-reviewer.md")).unwrap();

        deploy_agent_personas(temp.path()).unwrap();
        let second_content =
            fs::read_to_string(temp.path().join(".claude/skills/agents/rust-reviewer.md")).unwrap();

        assert_eq!(first_content, second_content);
    }
}
