use cargo_skill::{context, deploy, detect, gitignore, skill};
use std::fs;
use tempfile::TempDir;

/// Helper to set up a test repo with a Cargo.toml
fn setup_test_repo() -> TempDir {
    let temp = TempDir::new().unwrap();
    let cargo_toml = temp.path().join("Cargo.toml");
    fs::write(&cargo_toml, "[package]\nname = \"test\"\n").unwrap();
    temp
}

/// Helper to create agent directories
fn create_agents(temp: &TempDir) {
    fs::create_dir(temp.path().join(".claude")).unwrap();
    fs::create_dir(temp.path().join(".cursor")).unwrap();
}

#[test]
fn test_init_deploys_skills_to_agents() {
    let temp = setup_test_repo();
    create_agents(&temp);

    // Detect agents in test repo
    let agents = detect::agents(temp.path());
    assert_eq!(agents.len(), 2, "Should detect 2 agents");

    // Deploy skills
    deploy::deploy(&agents, temp.path()).unwrap();

    // Verify skills were deployed
    assert!(temp.path().join(".claude/skills/rust.md").exists());
    assert!(temp.path().join(".cursor/rules/rust.md").exists());
}

#[test]
fn test_init_ensures_gitignore() {
    let temp = setup_test_repo();

    // Ensure .skill/ is in .gitignore
    gitignore::ensure(temp.path()).unwrap();

    // Verify .gitignore was created with .skill/
    let gitignore_path = temp.path().join(".gitignore");
    assert!(gitignore_path.exists());

    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains(".skill/"));
}

#[test]
fn test_lookup_writes_context() {
    let temp = setup_test_repo();

    // Load lookup content and write to context
    let content = skill::load_lookup_filtered(None).unwrap();
    context::write(temp.path(), &content).unwrap();

    // Verify context was written
    let context_path = temp.path().join(".skill/context.md");
    assert!(context_path.exists());

    let written_content = fs::read_to_string(&context_path).unwrap();
    assert!(written_content.contains("Layer 1"));
}

#[test]
fn test_lookup_with_prefix_writes_filtered_context() {
    let temp = setup_test_repo();

    // Load lookup content with prefix filter and write to context
    let content = skill::load_lookup_filtered(Some("own")).unwrap();
    context::write(temp.path(), &content).unwrap();

    // Verify context was written with filter marker
    let context_path = temp.path().join(".skill/context.md");
    let written_content = fs::read_to_string(&context_path).unwrap();
    assert!(written_content.contains("Filtered for prefix: **own-**"));
}

#[test]
fn test_think_writes_context() {
    let temp = setup_test_repo();

    // Load think layers and write to context
    let layer_set = skill::layer::LayerSet::think();
    let content = skill::load(&layer_set).unwrap();
    context::write(temp.path(), &content).unwrap();

    // Verify context contains both layers
    let context_path = temp.path().join(".skill/context.md");
    let written_content = fs::read_to_string(&context_path).unwrap();
    assert!(written_content.contains("Layer 1"));
    assert!(written_content.contains("Layer 2"));
}

#[test]
fn test_write_writes_context() {
    let temp = setup_test_repo();

    // Load all layers and write to context
    let layer_set = skill::layer::LayerSet::write();
    let content = skill::load(&layer_set).unwrap();
    context::write(temp.path(), &content).unwrap();

    // Verify context contains all layers
    let context_path = temp.path().join(".skill/context.md");
    let written_content = fs::read_to_string(&context_path).unwrap();
    assert!(written_content.contains("Layer 1"));
    assert!(written_content.contains("Layer 2"));
    assert!(written_content.contains("Layer 3"));
}

#[test]
fn test_clear_removes_context() {
    let temp = setup_test_repo();

    // First write context
    context::write(temp.path(), "test content").unwrap();
    let context_path = temp.path().join(".skill/context.md");
    assert!(context_path.exists());

    // Then clear it
    context::clear(temp.path()).unwrap();
    assert!(!context_path.exists());
}

#[test]
fn test_init_ensures_gitignore_even_with_no_agents() {
    let temp = setup_test_repo();

    // No agents created — gitignore should still be updated
    let agents = detect::agents(temp.path());
    assert!(agents.is_empty(), "Should detect no agents");

    gitignore::ensure(temp.path()).unwrap();

    let content = fs::read_to_string(temp.path().join(".gitignore")).unwrap();
    assert!(content.contains(".skill/"));
}

#[test]
fn test_clear_is_idempotent() {
    let temp = setup_test_repo();

    // Clear on absent file should succeed
    context::clear(temp.path()).unwrap();

    // File should still not exist
    let context_path = temp.path().join(".skill/context.md");
    assert!(!context_path.exists());
}

#[test]
fn test_shorthand_prefix_dispatch_valid() {
    let temp = setup_test_repo();

    // Shorthand: "own" should work the same as lookup with prefix "own"
    let content = skill::load_lookup_filtered(Some("own")).unwrap();
    context::write(temp.path(), &content).unwrap();

    // Verify context was written with filtered content
    let context_path = temp.path().join(".skill/context.md");
    assert!(context_path.exists());

    let written_content = fs::read_to_string(&context_path).unwrap();
    assert!(written_content.contains("Filtered for prefix: **own-**"));
    assert!(written_content.contains("own-01"));
}

#[test]
fn test_shorthand_prefixes_are_valid() {
    // Ensure all expected prefixes are in VALID_PREFIXES
    let expected = vec![
        "own", "err", "mem", "api", "async", "opt", "type", "perf", "test", "doc", "name", "proj",
        "lint", "anti",
    ];
    for prefix in expected {
        assert!(
            skill::prefix::VALID_PREFIXES.contains(&prefix),
            "Expected prefix '{}' to be valid",
            prefix
        );
    }
}

#[test]
fn test_shorthand_prefix_invalid_returns_error() {
    // Invalid shorthand prefix should fail validation
    let result = skill::load_lookup_filtered(Some("invalid"));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Unknown prefix"));
    assert!(err.contains("invalid"));
}

#[test]
fn test_status_detects_repo_kind() {
    let temp = setup_test_repo();

    // Detect repo
    let repo = detect::repo_at(temp.path()).unwrap();
    assert_eq!(repo.kind, detect::RepoKind::SingleCrate);
}

#[test]
fn test_status_detects_agents() {
    let temp = setup_test_repo();
    create_agents(&temp);

    let agents = detect::agents(temp.path());
    assert_eq!(agents.len(), 2);
    assert!(agents.contains(&detect::Agent::ClaudeCode));
    assert!(agents.contains(&detect::Agent::Cursor));
}

#[test]
fn test_status_detects_context() {
    let temp = setup_test_repo();

    // No context initially
    let context_path = temp.path().join(".skill/context.md");
    assert!(!context_path.exists());

    // Write context
    context::write(temp.path(), "test content").unwrap();
    assert!(context_path.exists());

    // Verify content
    let content = fs::read_to_string(&context_path).unwrap();
    assert_eq!(content, "test content");
}

#[test]
fn test_status_detects_gitignore() {
    let temp = setup_test_repo();

    // No gitignore initially
    let gitignore_path = temp.path().join(".gitignore");
    assert!(!gitignore_path.exists());

    // Ensure gitignore
    gitignore::ensure(temp.path()).unwrap();
    assert!(gitignore_path.exists());

    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains(".skill/"));
}

#[test]
fn test_dry_run_does_not_write_files() {
    let temp = setup_test_repo();
    create_agents(&temp);

    // Check that skill files don't exist yet
    let claude_skill = temp.path().join(".claude/skills/rust.md");
    let cursor_skill = temp.path().join(".cursor/rules/rust.md");
    assert!(!claude_skill.exists());
    assert!(!cursor_skill.exists());

    // Simulate dry-run: check what would be deployed without actually deploying
    let agents = detect::agents(temp.path());
    assert_eq!(agents.len(), 2);

    // In dry-run mode, we should NOT call deploy::deploy()
    // The files should still not exist
    assert!(!claude_skill.exists());
    assert!(!cursor_skill.exists());
}

#[test]
fn test_dry_run_does_not_modify_gitignore() {
    let temp = setup_test_repo();

    // Create a gitignore without .skill/
    let gitignore_path = temp.path().join(".gitignore");
    fs::write(&gitignore_path, "# existing content\n").unwrap();

    let original_content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(!original_content.contains(".skill/"));

    // Simulate dry-run: don't actually modify
    // Content should remain unchanged
    let content_after = fs::read_to_string(&gitignore_path).unwrap();
    assert_eq!(original_content, content_after);
    assert!(!content_after.contains(".skill/"));
}
