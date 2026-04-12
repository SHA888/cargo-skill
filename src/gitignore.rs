use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
const SKILL_GITIGNORE_ENTRY: &str = ".skill/";
#[allow(dead_code)]
const CURSOR_CONTEXT_GITIGNORE_ENTRY: &str = ".cursor/rules/skill-context.md";
#[allow(dead_code)]
const WINDSURF_CONTEXT_GITIGNORE_ENTRY: &str = ".windsurf/rules/skill-context.md";

/// Ensure skill-related paths are in `.gitignore`
///
/// Ensures the following entries are present:
/// - `.skill/` — main context directory
/// - `.cursor/rules/skill-context.md` — Cursor agent context
/// - `.windsurf/rules/skill-context.md` — Windsurf agent context
///
/// - Reads `.gitignore` if present
/// - Checks if entries already present
/// - Appends missing entries
/// - Creates `.gitignore` if absent
pub fn ensure(repo_root: &Path) -> Result<()> {
    let gitignore_path = repo_root.join(".gitignore");
    let entries = [
        SKILL_GITIGNORE_ENTRY,
        CURSOR_CONTEXT_GITIGNORE_ENTRY,
        WINDSURF_CONTEXT_GITIGNORE_ENTRY,
    ];

    if gitignore_path.exists() {
        // Read existing .gitignore
        let content = fs::read_to_string(&gitignore_path)
            .with_context(|| format!("Failed to read {}", gitignore_path.display()))?;

        // Find which entries are missing
        let missing_entries: Vec<&str> = entries
            .iter()
            .filter(|entry| !content.lines().any(|line| line.trim() == **entry))
            .copied()
            .collect();

        if missing_entries.is_empty() {
            // All entries present, nothing to do
            return Ok(());
        }

        // Append missing entries to existing .gitignore
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&gitignore_path)
            .with_context(|| format!("Failed to open {} for append", gitignore_path.display()))?;

        // Add newline if file doesn't end with one
        if !content.ends_with('\n') {
            writeln!(file)
                .with_context(|| format!("Failed to write to {}", gitignore_path.display()))?;
        }

        for entry in missing_entries {
            writeln!(file, "{}", entry)
                .with_context(|| format!("Failed to append to {}", gitignore_path.display()))?;
        }
    } else {
        // Create new .gitignore with all entries
        let gitignore_content = entries.join("\n") + "\n";
        fs::write(&gitignore_path, gitignore_content)
            .with_context(|| format!("Failed to create {}", gitignore_path.display()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_gitignore_absent_created_with_all_entries() {
        let temp = TempDir::new().unwrap();

        ensure(temp.path()).unwrap();

        let gitignore_path = temp.path().join(".gitignore");
        assert!(gitignore_path.exists());

        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(content.contains(".skill/"));
        assert!(content.contains(".cursor/rules/skill-context.md"));
        assert!(content.contains(".windsurf/rules/skill-context.md"));
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
        assert!(content.contains(".cursor/rules/skill-context.md"));
        assert!(content.contains(".windsurf/rules/skill-context.md"));
    }

    #[test]
    fn test_gitignore_present_entry_already_present_no_op() {
        let temp = TempDir::new().unwrap();
        let gitignore_path = temp.path().join(".gitignore");

        fs::write(
            &gitignore_path,
            "target/\n.skill/\n.cursor/rules/skill-context.md\n.windsurf/rules/skill-context.md\n",
        )
        .unwrap();

        let original_content = fs::read_to_string(&gitignore_path).unwrap();

        ensure(temp.path()).unwrap();

        let final_content = fs::read_to_string(&gitignore_path).unwrap();
        assert_eq!(original_content, final_content);
        // Ensure entries only appear once
        assert_eq!(final_content.matches(".skill/").count(), 1);
        assert_eq!(
            final_content
                .matches(".cursor/rules/skill-context.md")
                .count(),
            1
        );
        assert_eq!(
            final_content
                .matches(".windsurf/rules/skill-context.md")
                .count(),
            1
        );
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
