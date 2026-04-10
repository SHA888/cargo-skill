/// Skill layer types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// Layer 1 — Lookup: Rule index with prefix filtering
    Lookup,
    /// Layer 2 — Reasoning: Cognitive model, error reference
    Reasoning,
    /// Layer 3 — Execution: RPI loop, verification checklist
    Execution,
}

impl Layer {
    /// Returns the asset filename for this layer
    #[allow(dead_code)]
    pub fn asset_filename(&self) -> &'static str {
        match self {
            Layer::Lookup => "layer1.md",
            Layer::Reasoning => "layer2.md",
            Layer::Execution => "layer3.md",
        }
    }
}

/// A set of layers to load
#[derive(Debug, Clone)]
pub struct LayerSet {
    layers: Vec<Layer>,
}

impl LayerSet {
    /// Create a LayerSet for the lookup command (Layer 1 only)
    #[allow(dead_code)]
    pub fn lookup() -> Self {
        Self {
            layers: vec![Layer::Lookup],
        }
    }

    /// Create a LayerSet for the think command (Layers 1 + 2)
    #[allow(dead_code)]
    pub fn think() -> Self {
        Self {
            layers: vec![Layer::Lookup, Layer::Reasoning],
        }
    }

    /// Create a LayerSet for the write command (All layers)
    #[allow(dead_code)]
    pub fn write() -> Self {
        Self {
            layers: vec![Layer::Lookup, Layer::Reasoning, Layer::Execution],
        }
    }

    /// Iterate over the layers in order
    pub fn iter(&self) -> impl Iterator<Item = &Layer> {
        self.layers.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_asset_filenames() {
        assert_eq!(Layer::Lookup.asset_filename(), "layer1.md");
        assert_eq!(Layer::Reasoning.asset_filename(), "layer2.md");
        assert_eq!(Layer::Execution.asset_filename(), "layer3.md");
    }

    #[test]
    fn test_layer_set_lookup() {
        let set = LayerSet::lookup();
        assert_eq!(set.layers, vec![Layer::Lookup]);
    }

    #[test]
    fn test_layer_set_think() {
        let set = LayerSet::think();
        assert_eq!(set.layers, vec![Layer::Lookup, Layer::Reasoning]);
    }

    #[test]
    fn test_layer_set_write() {
        let set = LayerSet::write();
        assert_eq!(
            set.layers,
            vec![Layer::Lookup, Layer::Reasoning, Layer::Execution]
        );
    }
}
