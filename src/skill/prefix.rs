use anyhow::{Result, bail};

/// Valid prefixes for filtering Layer 1 content
pub const VALID_PREFIXES: &[&str] = &[
    "own",   // Ownership & Borrowing
    "err",   // Error Handling
    "mem",   // Memory Management
    "api",   // API Design
    "async", // Async/Await
    "opt",   // Option & Result
    "type",  // Types & Traits
    "perf",  // Performance
    "test",  // Testing
    "doc",   // Documentation
    "name",  // Naming Conventions
    "proj",  // Project Structure
    "lint",  // Linting & Formatting
    "anti",  // Anti-patterns
];

/// Validate that a prefix is known
pub fn validate(prefix: &str) -> Result<()> {
    if prefix.is_empty() {
        return Ok(());
    }

    if VALID_PREFIXES.contains(&prefix) {
        Ok(())
    } else {
        bail!(
            "Unknown prefix: '{}'. Valid prefixes: {}",
            prefix,
            VALID_PREFIXES.join(", ")
        )
    }
}

/// Filter Layer 1 content to extract only sections matching the given prefix
///
/// The format expects sections to start with `## **{prefix}-**`
///
/// If prefix is empty, returns the full content.
pub fn filter(content: &str, prefix: &str) -> String {
    if prefix.is_empty() {
        return content.to_string();
    }

    let marker = format!("## **{}-**", prefix);
    let mut result = String::new();
    let mut in_target_section = false;
    let mut section_content = String::new();

    for line in content.lines() {
        if line.starts_with("## **") {
            // Starting a new section
            if in_target_section {
                // We were in the target section, now leaving it
                break;
            }
            if line.starts_with(&marker) {
                in_target_section = true;
                section_content.push_str(line);
                section_content.push('\n');
            }
        } else if in_target_section {
            section_content.push_str(line);
            section_content.push('\n');
        }
    }

    // Build result with header and filtered section
    let header = content
        .lines()
        .next()
        .unwrap_or("# Layer 1 — Lookup: Rust Rule Index");
    result.push_str(header);
    result.push_str("\n\nFiltered for prefix: **");
    result.push_str(prefix);
    result.push_str("-**\n\n---\n\n");
    result.push_str(&section_content);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_known_prefix() {
        assert!(validate("own").is_ok());
        assert!(validate("err").is_ok());
        assert!(validate("async").is_ok());
        assert!(validate("anti").is_ok());
    }

    #[test]
    fn test_validate_unknown_prefix() {
        let result = validate("unknown");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unknown prefix"));
        assert!(err.contains("unknown"));
    }

    #[test]
    fn test_validate_empty() {
        assert!(validate("").is_ok());
    }

    #[test]
    fn test_filter_empty_prefix_returns_full() {
        let content = "# Header\n\nSome content\n\n## Section 1\nText\n\n## Section 2\nMore text";
        let result = filter(content, "");
        assert_eq!(result, content);
    }

    #[test]
    fn test_filter_known_prefix() {
        let content = r#"# Layer 1

## **own-** — Ownership
- rule 1
- rule 2

## **err-** — Errors
- error rule

## **type-** — Types
- type rule
"#;

        let result = filter(content, "own");
        assert!(result.contains("Filtered for prefix: **own-**"));
        assert!(result.contains("## **own-**"));
        assert!(result.contains("- rule 1"));
        assert!(!result.contains("## **err-**"));
        assert!(!result.contains("## **type-**"));
    }

    #[test]
    fn test_filter_unknown_prefix_returns_empty_section() {
        let content = "# Layer 1\n\n## **own-** — Ownership\n- rule";
        let result = filter(content, "xyz");
        assert!(result.contains("Filtered for prefix: **xyz-**"));
        assert!(!result.contains("## **own-**"));
    }

    #[test]
    fn test_filter_with_blank_lines_between_sections() {
        let content = r#"# Layer 1

## **own-** — Ownership
- rule 1

(blank line above)

## **err-** — Errors
- error rule
"#;

        let result = filter(content, "own");
        assert!(result.contains("Filtered for prefix: **own-**"));
        assert!(result.contains("## **own-**"));
        assert!(result.contains("- rule 1"));
        assert!(result.contains("(blank line above)"));
        assert!(!result.contains("## **err-**"));
        assert!(!result.contains("error rule"));
    }
}
