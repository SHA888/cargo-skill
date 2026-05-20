pub mod layer;
pub mod prefix;

use anyhow::Result;
use layer::LayerSet;

/// Programming language for skill selection
#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
    Python,
}

/// Load skill content for the given layer set and language
///
/// Assets are embedded at compile time using `include_str!()`
pub fn load(layers: &LayerSet, lang: Language) -> Result<String> {
    let mut content = String::new();

    for layer in layers.iter() {
        let layer_content = match (lang, layer) {
            (Language::Rust, layer::Layer::Lookup) => include_str!("../../assets/rust/layer1.md"),
            (Language::Rust, layer::Layer::Reasoning) => {
                include_str!("../../assets/rust/layer2.md")
            }
            (Language::Rust, layer::Layer::Execution) => {
                include_str!("../../assets/rust/layer3.md")
            }
            (Language::Python, layer::Layer::Lookup) => {
                include_str!("../../assets/python/layer1.md")
            }
            (Language::Python, layer::Layer::Reasoning) => {
                include_str!("../../assets/python/layer2.md")
            }
            (Language::Python, layer::Layer::Execution) => {
                include_str!("../../assets/python/layer3.md")
            }
        };

        if !content.is_empty() {
            content.push_str("\n\n---\n\n");
        }
        content.push_str(layer_content);
    }

    Ok(content)
}

/// Load Layer 1 content and filter by prefix (if specified)
pub fn load_lookup_filtered(prefix: Option<&str>, lang: Language) -> Result<String> {
    let content = match lang {
        Language::Rust => include_str!("../../assets/rust/layer1.md"),
        Language::Python => include_str!("../../assets/python/layer1.md"),
    };

    match prefix {
        None => Ok(content.to_string()),
        Some(p) => {
            prefix::validate(p)?;
            Ok(prefix::filter(content, p))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use layer::LayerSet;

    #[test]
    fn test_load_lookup() {
        let set = LayerSet::lookup();
        let content = load(&set, Language::Rust).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(!content.contains("Layer 2 — Reasoning"));
    }

    #[test]
    fn test_load_think() {
        let set = LayerSet::think();
        let content = load(&set, Language::Rust).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(content.contains("Layer 2 — Reasoning"));
        assert!(!content.contains("Layer 3 — Execution"));
    }

    #[test]
    fn test_load_write() {
        let set = LayerSet::write();
        let content = load(&set, Language::Rust).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(content.contains("Layer 2 — Reasoning"));
        assert!(content.contains("Layer 3 — Execution"));
    }

    #[test]
    fn test_load_lookup_filtered_no_prefix() {
        let content = load_lookup_filtered(None, Language::Rust).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(content.contains("## **own-**"));
    }

    #[test]
    fn test_load_lookup_filtered_with_prefix() {
        let content = load_lookup_filtered(Some("own"), Language::Rust).unwrap();
        assert!(content.contains("Filtered for prefix: **own-**"));
        assert!(content.contains("## **own-**"));
        assert!(!content.contains("## **err-**"));
    }

    #[test]
    fn test_load_lookup_filtered_invalid_prefix() {
        let result = load_lookup_filtered(Some("unknown"), Language::Rust);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_python_lookup() {
        let set = LayerSet::lookup();
        let content = load(&set, Language::Python).unwrap();
        assert!(content.contains("Layer 1"));
        assert!(!content.contains("Layer 2 — Reasoning"));
    }

    #[test]
    fn test_load_python_lookup_filtered_no_prefix() {
        let content = load_lookup_filtered(None, Language::Python).unwrap();
        // Python layer1 should load successfully
        assert!(content.len() > 0);
        assert!(content.contains("Python"));
    }
}
