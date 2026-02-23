use crate::error::Result;
use crate::knotenlexikon::lexicon::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprachknoten {
    pub id: NodeId,
    pub concept_labels: HashMap<String, String>, // language -> label
    pub definitions: HashMap<String, String>,     // language -> definition
    pub is_synchronized: bool,
}

impl Sprachknoten {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            concept_labels: HashMap::new(),
            definitions: HashMap::new(),
            is_synchronized: false,
        }
    }

    pub fn set_label(&mut self, lang: String, label: String) -> &mut Self {
        self.concept_labels.insert(lang, label);
        self
    }

    pub fn set_definition(&mut self, lang: String, definition: String) -> &mut Self {
        self.definitions.insert(lang, definition);
        self
    }

    pub fn get_label(&self, lang: &str) -> Option<String> {
        self.concept_labels.get(lang).cloned()
    }

    pub fn sync(&mut self) -> Result<()> {
        self.is_synchronized = !self.concept_labels.is_empty();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroBrücke {
    pub id: NodeId,
    pub source_sprachknoten: NodeId,
    pub target_sprachknoten: NodeId,
    pub source_lang: String,
    pub target_lang: String,
    pub synchronization_strength: f64,
    pub is_active: bool,
}

impl NeuroBrücke {
    pub fn new(
        id: NodeId,
        source: NodeId,
        target: NodeId,
        source_lang: String,
        target_lang: String,
    ) -> Self {
        Self {
            id,
            source_sprachknoten: source,
            target_sprachknoten: target,
            source_lang,
            target_lang,
            synchronization_strength: 1.0,
            is_active: true,
        }
    }

    pub fn modulate_strength(&mut self, factor: f64) -> Result<()> {
        self.synchronization_strength = (self.synchronization_strength * factor).clamp(0.0, 1.0);
        Ok(())
    }
}
