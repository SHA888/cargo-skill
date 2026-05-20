use crate::skill;
use anyhow::Result;
use std::path::Path;

/// Parse a prefix string into optional language and bare prefix.
/// Recognizes `rust:err` → `(Some(Rust), "err")`, `py:err` → `(Some(Python), "err")`, `err` → `(None, "err")`.
pub fn parse_qualified_prefix(s: &str) -> (Option<skill::Language>, &str) {
    if let Some(rest) = s.strip_prefix("rust:") {
        (Some(skill::Language::Rust), rest)
    } else if let Some(rest) = s.strip_prefix("py:") {
        (Some(skill::Language::Python), rest)
    } else {
        (None, s)
    }
}

/// Resolve the language to use, considering both stacks and explicit selection.
/// If both Cargo.toml and pyproject.toml exist and no explicit language, returns error.
pub fn resolve_language(
    repo_root: &Path,
    explicit: Option<skill::Language>,
) -> Result<skill::Language> {
    if let Some(lang) = explicit {
        return Ok(lang);
    }

    let has_rust = repo_root.join("Cargo.toml").exists();
    let has_python = crate::detect::python_stack(repo_root).is_some();

    match (has_rust, has_python) {
        (true, true) => anyhow::bail!(
            "Multiple language stacks detected.\n\
             Use an explicit selector:\n\
             \x20  cargo skill lookup rust:<prefix>\n\
             \x20  cargo skill lookup py:<prefix>"
        ),
        (false, true) => Ok(skill::Language::Python),
        _ => Ok(skill::Language::Rust),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qualified_prefix_rust() {
        let (lang, prefix) = parse_qualified_prefix("rust:err");
        assert_eq!(lang, Some(skill::Language::Rust));
        assert_eq!(prefix, "err");
    }

    #[test]
    fn test_parse_qualified_prefix_py() {
        let (lang, prefix) = parse_qualified_prefix("py:mem");
        assert_eq!(lang, Some(skill::Language::Python));
        assert_eq!(prefix, "mem");
    }

    #[test]
    fn test_parse_qualified_prefix_bare() {
        let (lang, prefix) = parse_qualified_prefix("err");
        assert_eq!(lang, None);
        assert_eq!(prefix, "err");
    }

    #[test]
    fn test_parse_qualified_prefix_empty() {
        let (lang, prefix) = parse_qualified_prefix("");
        assert_eq!(lang, None);
        assert_eq!(prefix, "");
    }
}
