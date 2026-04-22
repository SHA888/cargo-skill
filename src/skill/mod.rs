pub mod layer;
pub mod prefix;

use anyhow::Result;
use layer::LayerSet;

/// Load skill content for the given layer set
///
/// Assets are embedded at compile time using `include_str!()`
pub fn load(layers: &LayerSet) -> Result<String> {
    let mut content = String::new();

    for layer in layers.iter() {
        let layer_content = match layer {
            layer::Layer::Lookup => include_str!("../../assets/rust/layer1.md"),
            layer::Layer::Reasoning => include_str!("../../assets/rust/layer2.md"),
            layer::Layer::Execution => include_str!("../../assets/rust/layer3.md"),
        };

        if !content.is_empty() {
            content.push_str("\n\n---\n\n");
        }
        content.push_str(layer_content);
    }

    Ok(content)
}

/// Load Layer 1 content and filter by prefix (if specified)
pub fn load_lookup_filtered(prefix: Option<&str>) -> Result<String> {
    let content = include_str!("../../assets/rust/layer1.md");

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
        let content = load(&set).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(!content.contains("Layer 2 — Reasoning"));
    }

    #[test]
    fn test_load_think() {
        let set = LayerSet::think();
        let content = load(&set).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(content.contains("Layer 2 — Reasoning"));
        assert!(!content.contains("Layer 3 — Execution"));
    }

    #[test]
    fn test_load_write() {
        let set = LayerSet::write();
        let content = load(&set).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(content.contains("Layer 2 — Reasoning"));
        assert!(content.contains("Layer 3 — Execution"));
    }

    #[test]
    fn test_load_lookup_filtered_no_prefix() {
        let content = load_lookup_filtered(None).unwrap();
        assert!(content.contains("Layer 1 — Lookup"));
        assert!(content.contains("## **own-**"));
    }

    #[test]
    fn test_load_lookup_filtered_with_prefix() {
        let content = load_lookup_filtered(Some("own")).unwrap();
        assert!(content.contains("Filtered for prefix: **own-**"));
        assert!(content.contains("## **own-**"));
        assert!(!content.contains("## **err-**"));
    }

    #[test]
    fn test_load_lookup_filtered_invalid_prefix() {
        let result = load_lookup_filtered(Some("unknown"));
        assert!(result.is_err());
    }
}
