use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[allow(dead_code)]
const SKILL_DIR: &str = ".skill";
#[allow(dead_code)]
const CONTEXT_FILE: &str = "context.md";

/// Write content to `.skill/context.md`
///
/// - Creates `.skill/` directory if missing
/// - Writes `content` to `.skill/context.md`
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

    Ok(())
}

/// Delete `.skill/context.md` if present
///
/// - No-op if file is absent (no error)
pub fn clear(repo_root: &Path) -> Result<()> {
    let context_path = repo_root.join(SKILL_DIR).join(CONTEXT_FILE);

    // Try to remove file, ignore NotFound error
    match fs::remove_file(&context_path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e).with_context(|| format!("Failed to remove {}", context_path.display())),
    }
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
}
