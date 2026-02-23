use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptAnchor {
    pub concept_id: String,
    pub multilingual_surface_forms: HashMap<String, String>,
    pub definition: String,
    pub external_ontology_refs: Vec<String>,
    pub is_validated: bool,
}

impl ConceptAnchor {
    pub fn new(concept_id: String, definition: String) -> Self {
        Self {
            concept_id,
            multilingual_surface_forms: HashMap::new(),
            definition,
            external_ontology_refs: Vec::new(),
            is_validated: false,
        }
    }

    pub fn add_surface_form(&mut self, lang: String, form: String) -> &mut Self {
        self.multilingual_surface_forms.insert(lang, form);
        self
    }

    pub fn add_ontology_ref(&mut self, ontology_ref: String) -> &mut Self {
        self.external_ontology_refs.push(ontology_ref);
        self
    }

    pub fn mark_validated(&mut self) -> Result<()> {
        self.is_validated = !self.multilingual_surface_forms.is_empty();
        Ok(())
    }
}
