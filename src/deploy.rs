use crate::detect::Agent;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

const SKILL_HEADER: &str = "# Rust Skill Reference\n\nThis section provides a quick lookup index for Rust development rules.\n";

/// Deploy skill files to all detected agents
#[allow(dead_code)]
pub fn deploy(agents: &[Agent], repo_root: &Path) -> Result<()> {
    let skill_content = include_str!("../assets/rust/layer1.md");

    for agent in agents {
        match agent {
            Agent::AgentsMd => deploy_to_agents_md(repo_root, skill_content)?,
            _ => deploy_to_file(agent, repo_root, skill_content)?,
        }
        println!("✓ deployed to {}", agent.skill_path().display());
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

    // Write the skill file
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

/// Deploy skill content to AGENTS.md (append or create)
#[allow(dead_code)]
fn deploy_to_agents_md(repo_root: &Path, content: &str) -> Result<()> {
    let path = repo_root.join("AGENTS.md");

    if path.exists() {
        // Append to existing file
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
        fs::write(&path, format!("{}\n{}", SKILL_HEADER, content))
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }

    Ok(())
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
}
