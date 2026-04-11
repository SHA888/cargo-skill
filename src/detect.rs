use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Type of Rust repository detected
#[derive(Debug, Clone, PartialEq)]
pub enum RepoKind {
    /// Single crate with a single Cargo.toml
    SingleCrate,
    /// Workspace with [workspace] section in root Cargo.toml
    Workspace,
}

/// Information about the detected repository
#[derive(Debug, Clone)]
pub struct Repo {
    pub kind: RepoKind,
    pub root: PathBuf,
}

/// Supported AI agents
#[derive(Debug, Clone, PartialEq)]
pub enum Agent {
    ClaudeCode,
    Cursor,
    Windsurf,
    AgentsMd,
}

impl Agent {
    /// Returns the install path for the skill file relative to repo root
    pub fn skill_path(&self) -> PathBuf {
        match self {
            Agent::ClaudeCode => PathBuf::from(".claude/skills/rust.md"),
            Agent::Cursor => PathBuf::from(".cursor/rules/rust.md"),
            Agent::Windsurf => PathBuf::from(".windsurf/rules/rust.md"),
            Agent::AgentsMd => PathBuf::from("AGENTS.md"),
        }
    }
}

/// Walk up from current directory to find Cargo.toml
pub fn repo() -> Result<Repo> {
    let cwd = env::current_dir().context("Failed to get current directory")?;
    let mut path = cwd.as_path();

    loop {
        let cargo_toml = path.join("Cargo.toml");
        if cargo_toml.exists() {
            let kind = determine_repo_kind(&cargo_toml)?;
            return Ok(Repo {
                kind,
                root: path.to_path_buf(),
            });
        }

        match path.parent() {
            Some(parent) => path = parent,
            None => break,
        }
    }

    anyhow::bail!("No Cargo.toml found in current directory or any parent. Not a Rust project?")
}

/// Determine if the Cargo.toml represents a workspace or single crate
#[allow(dead_code)]
fn determine_repo_kind(cargo_toml: &Path) -> Result<RepoKind> {
    let contents = fs::read_to_string(cargo_toml)
        .with_context(|| format!("Failed to read {}", cargo_toml.display()))?;

    // Check for [workspace] section at start of line
    // This avoids false positives from [workspace] appearing in comments/strings
    if contents.lines().any(|line| line.trim() == "[workspace]") {
        Ok(RepoKind::Workspace)
    } else {
        Ok(RepoKind::SingleCrate)
    }
}

/// Detect which AI agents are present in the repository
pub fn agents(repo_root: &Path) -> Vec<Agent> {
    let mut detected = Vec::new();

    if repo_root.join(".claude").is_dir() {
        detected.push(Agent::ClaudeCode);
    }

    if repo_root.join(".cursor").is_dir() {
        detected.push(Agent::Cursor);
    }

    if repo_root.join(".windsurf").is_dir() {
        detected.push(Agent::Windsurf);
    }

    if repo_root.join("AGENTS.md").is_file() {
        detected.push(Agent::AgentsMd);
    }

    detected
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_determine_repo_kind_single_crate() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"\n").unwrap();

        let kind = determine_repo_kind(&cargo_toml).unwrap();
        assert_eq!(kind, RepoKind::SingleCrate);
    }

    #[test]
    fn test_determine_repo_kind_workspace() {
        let temp = TempDir::new().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[workspace]\nmembers = [\"crate1\"]\n").unwrap();

        let kind = determine_repo_kind(&cargo_toml).unwrap();
        assert_eq!(kind, RepoKind::Workspace);
    }

    #[test]
    fn test_agents_detects_claude() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".claude")).unwrap();

        let agents = agents(temp.path());
        assert!(agents.contains(&Agent::ClaudeCode));
        assert_eq!(agents.len(), 1);
    }

    #[test]
    fn test_agents_detects_cursor() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".cursor")).unwrap();

        let agents = agents(temp.path());
        assert!(agents.contains(&Agent::Cursor));
        assert_eq!(agents.len(), 1);
    }

    #[test]
    fn test_agents_detects_windsurf() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".windsurf")).unwrap();

        let agents = agents(temp.path());
        assert!(agents.contains(&Agent::Windsurf));
        assert_eq!(agents.len(), 1);
    }

    #[test]
    fn test_agents_detects_agents_md() {
        let temp = TempDir::new().unwrap();
        fs::File::create(temp.path().join("AGENTS.md")).unwrap();

        let agents = agents(temp.path());
        assert!(agents.contains(&Agent::AgentsMd));
        assert_eq!(agents.len(), 1);
    }

    #[test]
    fn test_agents_detects_multiple() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".claude")).unwrap();
        fs::create_dir(temp.path().join(".windsurf")).unwrap();
        fs::File::create(temp.path().join("AGENTS.md")).unwrap();

        let agents = agents(temp.path());
        assert!(agents.contains(&Agent::ClaudeCode));
        assert!(agents.contains(&Agent::Windsurf));
        assert!(agents.contains(&Agent::AgentsMd));
        assert!(!agents.contains(&Agent::Cursor));
        assert_eq!(agents.len(), 3);
    }

    #[test]
    fn test_agents_detects_none() {
        let temp = TempDir::new().unwrap();
        let agents = agents(temp.path());
        assert!(agents.is_empty());
    }

    #[test]
    fn test_agent_skill_paths() {
        assert_eq!(
            Agent::ClaudeCode.skill_path(),
            PathBuf::from(".claude/skills/rust.md")
        );
        assert_eq!(
            Agent::Cursor.skill_path(),
            PathBuf::from(".cursor/rules/rust.md")
        );
        assert_eq!(
            Agent::Windsurf.skill_path(),
            PathBuf::from(".windsurf/rules/rust.md")
        );
        assert_eq!(Agent::AgentsMd.skill_path(), PathBuf::from("AGENTS.md"));
    }
}
