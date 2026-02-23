use crate::error::Result;
use crate::knotenlexikon::lexicon::{NodeId, TermEntry, ValidationStatus};
use dashmap::DashMap;
use std::sync::Arc;

pub struct KnotenLexikon {
    entries: Arc<DashMap<NodeId, TermEntry>>,
    code_label_index: Arc<DashMap<String, NodeId>>,
    german_form_index: Arc<DashMap<String, NodeId>>,
    category_index: Arc<DashMap<String, Vec<NodeId>>>,
}

impl KnotenLexikon {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
            code_label_index: Arc::new(DashMap::new()),
            german_form_index: Arc::new(DashMap::new()),
            category_index: Arc::new(DashMap::new()),
        }
    }

    pub fn register(&self, mut entry: TermEntry) -> Result<NodeId> {
        entry.validation_status = ValidationStatus::Anchored;
        let node_id = entry.node_id.clone();

        self.entries.insert(node_id.clone(), entry.clone());
        self.code_label_index.insert(entry.code_label.clone(), node_id.clone());
        self.german_form_index.insert(entry.german_form.clone(), node_id.clone());

        let category_key = format!("{:?}", entry.category);
        self.category_index
            .entry(category_key)
            .or_insert_with(Vec::new)
            .push(node_id.clone());

        Ok(node_id)
    }

    pub fn lookup_by_id(&self, node_id: &NodeId) -> Result<TermEntry> {
        self.entries
            .get(node_id)
            .map(|e| e.clone())
            .ok_or_else(|| crate::error::NeoError::LexiconNotFound(node_id.0.clone()))
    }

    pub fn lookup_by_code_label(&self, label: &str) -> Result<TermEntry> {
        let node_id = self
            .code_label_index
            .get(label)
            .ok_or_else(|| crate::error::NeoError::LexiconNotFound(label.to_string()))?
            .clone();
        self.lookup_by_id(&node_id)
    }

    pub fn lookup_by_german_form(&self, form: &str) -> Result<TermEntry> {
        let node_id = self
            .german_form_index
            .get(form)
            .ok_or_else(|| crate::error::NeoError::LexiconNotFound(form.to_string()))?
            .clone();
        self.lookup_by_id(&node_id)
    }

    pub fn list_by_category(&self, category: &str) -> Vec<TermEntry> {
        self.category_index
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.entries.get(id).map(|e| e.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn update_validation_status(&self, node_id: &NodeId, status: ValidationStatus) -> Result<()> {
        self.entries
            .alter(node_id, |_, mut entry| {
                entry.validation_status = status;
                entry
            });
        Ok(())
    }

    pub fn export_json(&self) -> Result<String> {
        let entries: Vec<TermEntry> = self.entries.iter().map(|ref_multi| ref_multi.clone()).collect();
        Ok(serde_json::to_string_pretty(&entries)?)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for KnotenLexikon {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for KnotenLexikon {
    fn clone(&self) -> Self {
        Self {
            entries: Arc::clone(&self.entries),
            code_label_index: Arc::clone(&self.code_label_index),
            german_form_index: Arc::clone(&self.german_form_index),
            category_index: Arc::clone(&self.category_index),
        }
    }
}
