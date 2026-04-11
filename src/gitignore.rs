use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
const SKILL_GITIGNORE_ENTRY: &str = ".skill/";

/// Ensure `.skill/` is in `.gitignore`
///
/// - Reads `.gitignore` if present
/// - Checks if `.skill/` already present
/// - Appends `.skill/` if missing
/// - Creates `.gitignore` if absent
pub fn ensure(repo_root: &Path) -> Result<()> {
    let gitignore_path = repo_root.join(".gitignore");

    if gitignore_path.exists() {
        // Read existing .gitignore
        let content = fs::read_to_string(&gitignore_path)
            .with_context(|| format!("Failed to read {}", gitignore_path.display()))?;

        // Check if .skill/ is already present
        if content
            .lines()
            .any(|line| line.trim() == SKILL_GITIGNORE_ENTRY)
        {
            // Already present, nothing to do
            return Ok(());
        }

        // Append .skill/ to existing .gitignore
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&gitignore_path)
            .with_context(|| format!("Failed to open {} for append", gitignore_path.display()))?;

        // Add newline if file doesn't end with one
        if !content.ends_with('\n') {
            writeln!(file)
                .with_context(|| format!("Failed to write to {}", gitignore_path.display()))?;
        }

        writeln!(file, "{}", SKILL_GITIGNORE_ENTRY)
            .with_context(|| format!("Failed to append to {}", gitignore_path.display()))?;
    } else {
        // Create new .gitignore with .skill/
        fs::write(&gitignore_path, format!("{}\n", SKILL_GITIGNORE_ENTRY))
            .with_context(|| format!("Failed to create {}", gitignore_path.display()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_gitignore_absent_created_with_skill() {
        let temp = TempDir::new().unwrap();

        ensure(temp.path()).unwrap();

        let gitignore_path = temp.path().join(".gitignore");
        assert!(gitignore_path.exists());

        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(content.contains(".skill/"));
    }

    #[test]
    fn test_gitignore_present_entry_absent_appended() {
        let temp = TempDir::new().unwrap();
        let gitignore_path = temp.path().join(".gitignore");

        fs::write(&gitignore_path, "target/\n*.log\n").unwrap();

        ensure(temp.path()).unwrap();

        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(content.contains("target/"));
        assert!(content.contains(".skill/"));
    }

    #[test]
    fn test_gitignore_present_entry_already_present_no_op() {
        let temp = TempDir::new().unwrap();
        let gitignore_path = temp.path().join(".gitignore");

        fs::write(&gitignore_path, "target/\n.skill/\n").unwrap();

        let original_content = fs::read_to_string(&gitignore_path).unwrap();

        ensure(temp.path()).unwrap();

        let final_content = fs::read_to_string(&gitignore_path).unwrap();
        assert_eq!(original_content, final_content);
        // Ensure .skill/ only appears once
        assert_eq!(final_content.matches(".skill/").count(), 1);
    }

    #[test]
    fn test_gitignore_present_entry_absent_no_trailing_newline() {
        let temp = TempDir::new().unwrap();
        let gitignore_path = temp.path().join(".gitignore");

        // File without trailing newline
        fs::write(&gitignore_path, "target/").unwrap();

        ensure(temp.path()).unwrap();

        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(content.contains("target/"));
        assert!(content.contains(".skill/"));
    }
}
