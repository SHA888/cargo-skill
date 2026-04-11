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
