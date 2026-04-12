use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[allow(dead_code)]
const SKILL_DIR: &str = ".skill";
#[allow(dead_code)]
const CONTEXT_FILE: &str = "context.md";

// Agent-specific context paths
const CURSOR_CONTEXT_PATH: &str = ".cursor/rules/skill-context.md";
const WINDSURF_CONTEXT_PATH: &str = ".windsurf/rules/skill-context.md";

/// Write content to `.skill/context.md` and agent-specific paths
///
/// - Creates `.skill/` directory if missing
/// - Writes `content` to `.skill/context.md`
/// - Also writes to `.cursor/rules/skill-context.md` if `.cursor/rules/` exists
/// - Also writes to `.windsurf/rules/skill-context.md` if `.windsurf/rules/` exists
/// - Overwrites if exists
pub fn write(repo_root: &Path, content: &str) -> Result<()> {
    let skill_dir = repo_root.join(SKILL_DIR);
    let context_path = skill_dir.join(CONTEXT_FILE);

    // Create .skill/ directory if it doesn't exist
    fs::create_dir_all(&skill_dir)
        .with_context(|| format!("Failed to create directory {}", skill_dir.display()))?;

    // Write content to context.md (overwrites if exists)
    fs::write(&context_path, content)
        .with_context(|| format!("Failed to write {}", context_path.display()))?;

    // Write to agent-specific paths if their directories exist
    let cursor_path = repo_root.join(CURSOR_CONTEXT_PATH);
    let cursor_dir = cursor_path.parent().unwrap();
    if cursor_dir.exists() {
        fs::write(&cursor_path, content)
            .with_context(|| format!("Failed to write {}", cursor_path.display()))?;
    }

    let windsurf_path = repo_root.join(WINDSURF_CONTEXT_PATH);
    let windsurf_dir = windsurf_path.parent().unwrap();
    if windsurf_dir.exists() {
        fs::write(&windsurf_path, content)
            .with_context(|| format!("Failed to write {}", windsurf_path.display()))?;
    }

    Ok(())
}

/// Delete context files if present
///
/// Removes `.skill/context.md`, `.cursor/rules/skill-context.md`,
/// and `.windsurf/rules/skill-context.md` if they exist.
/// - No-op if files are absent (no error)
pub fn clear(repo_root: &Path) -> Result<()> {
    let paths = [
        repo_root.join(SKILL_DIR).join(CONTEXT_FILE),
        repo_root.join(CURSOR_CONTEXT_PATH),
        repo_root.join(WINDSURF_CONTEXT_PATH),
    ];

    for path in &paths {
        // Try to remove file, ignore NotFound error
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                return Err(e).with_context(|| format!("Failed to remove {}", path.display()));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_creates_file_and_dir() {
        let temp = TempDir::new().unwrap();

        write(temp.path(), "test content").unwrap();

        let context_path = temp.path().join(".skill/context.md");
        assert!(context_path.exists());

        let content = fs::read_to_string(&context_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_write_overwrites_existing() {
        let temp = TempDir::new().unwrap();
        let context_path = temp.path().join(".skill/context.md");

        // Create existing file with old content
        fs::create_dir(temp.path().join(".skill")).unwrap();
        fs::write(&context_path, "old content").unwrap();

        // Overwrite with new content
        write(temp.path(), "new content").unwrap();

        let content = fs::read_to_string(&context_path).unwrap();
        assert_eq!(content, "new content");
    }

    #[test]
    fn test_clear_removes_file() {
        let temp = TempDir::new().unwrap();
        let context_path = temp.path().join(".skill/context.md");

        // Create file first
        fs::create_dir(temp.path().join(".skill")).unwrap();
        fs::write(&context_path, "test content").unwrap();
        assert!(context_path.exists());

        // Clear should remove it
        clear(temp.path()).unwrap();
        assert!(!context_path.exists());
    }

    #[test]
    fn test_clear_noop_on_absent_file() {
        let temp = TempDir::new().unwrap();

        // Ensure file doesn't exist
        let context_path = temp.path().join(".skill/context.md");
        assert!(!context_path.exists());

        // Clear should succeed without error
        clear(temp.path()).unwrap();
        assert!(!context_path.exists());
    }

    #[test]
    fn test_write_creates_agent_context_files() {
        let temp = TempDir::new().unwrap();

        // Create agent directories
        fs::create_dir_all(temp.path().join(".cursor/rules")).unwrap();
        fs::create_dir_all(temp.path().join(".windsurf/rules")).unwrap();

        write(temp.path(), "agent context").unwrap();

        // All three files should exist
        assert!(temp.path().join(".skill/context.md").exists());
        assert!(temp.path().join(".cursor/rules/skill-context.md").exists());
        assert!(
            temp.path()
                .join(".windsurf/rules/skill-context.md")
                .exists()
        );

        // All should have same content
        let main_content = fs::read_to_string(temp.path().join(".skill/context.md")).unwrap();
        let cursor_content =
            fs::read_to_string(temp.path().join(".cursor/rules/skill-context.md")).unwrap();
        let windsurf_content =
            fs::read_to_string(temp.path().join(".windsurf/rules/skill-context.md")).unwrap();

        assert_eq!(main_content, "agent context");
        assert_eq!(cursor_content, "agent context");
        assert_eq!(windsurf_content, "agent context");
    }

    #[test]
    fn test_write_skips_agent_context_if_dirs_absent() {
        let temp = TempDir::new().unwrap();

        // Don't create agent directories
        write(temp.path(), "only main context").unwrap();

        // Only main context should exist
        assert!(temp.path().join(".skill/context.md").exists());
        assert!(!temp.path().join(".cursor/rules/skill-context.md").exists());
        assert!(
            !temp
                .path()
                .join(".windsurf/rules/skill-context.md")
                .exists()
        );
    }

    #[test]
    fn test_clear_removes_all_context_files() {
        let temp = TempDir::new().unwrap();

        // Create all directories and files
        fs::create_dir_all(temp.path().join(".skill")).unwrap();
        fs::create_dir_all(temp.path().join(".cursor/rules")).unwrap();
        fs::create_dir_all(temp.path().join(".windsurf/rules")).unwrap();

        fs::write(temp.path().join(".skill/context.md"), "main").unwrap();
        fs::write(temp.path().join(".cursor/rules/skill-context.md"), "cursor").unwrap();
        fs::write(
            temp.path().join(".windsurf/rules/skill-context.md"),
            "windsurf",
        )
        .unwrap();

        // Clear should remove all three
        clear(temp.path()).unwrap();

        assert!(!temp.path().join(".skill/context.md").exists());
        assert!(!temp.path().join(".cursor/rules/skill-context.md").exists());
        assert!(
            !temp
                .path()
                .join(".windsurf/rules/skill-context.md")
                .exists()
        );
    }
}
