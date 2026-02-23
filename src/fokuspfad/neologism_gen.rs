use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetaphorFamily {
    Topological,    // Netz, Pfad, Knoten
    Biological,     // Haut, Klaue, Organismus
    Functional,     // Routing, Schnittstelle
    Organizational, // Cluster, Schicht
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeologismCandidate {
    pub german_form: String,
    pub code_label: String,
    pub metaphor_family: MetaphorFamily,
    pub definition: String,
    pub motivation: String,
    pub generation_method: String,
}

pub struct NeologismGenerator {
    base_heads: Vec<String>,
    base_modifiers: Vec<String>,
}

impl NeologismGenerator {
    pub fn new() -> Self {
        Self {
            base_heads: vec![
                "Knoten".to_string(),
                "Kanal".to_string(),
                "Pfad".to_string(),
                "Schicht".to_string(),
                "Netz".to_string(),
                "Cluster".to_string(),
                "Schnittstelle".to_string(),
                "Brücke".to_string(),
            ],
            base_modifiers: vec![
                "Neuro".to_string(),
                "Sprach".to_string(),
                "Netz".to_string(),
                "Routing".to_string(),
                "Bedeutungs".to_string(),
                "Ereignis".to_string(),
                "Adaptions".to_string(),
                "Kern".to_string(),
            ],
        }
    }

    pub fn generate_candidate(
        &self,
        modifier: &str,
        head: &str,
        metaphor_family: MetaphorFamily,
        definition: String,
    ) -> Result<NeologismCandidate> {
        let german_form = format!("{}{}", modifier, head);
        let code_label = self.derive_code_label(&german_form);

        Ok(NeologismCandidate {
            german_form,
            code_label,
            metaphor_family,
            definition,
            motivation: "Network topology analysis".to_string(),
            generation_method: "Head-final compounding".to_string(),
        })
    }

    fn derive_code_label(&self, german_form: &str) -> String {
        german_form
            .chars()
            .filter(|c| c.is_alphabetic())
            .fold(String::new(), |mut acc, c| {
                if c.is_uppercase() && !acc.is_empty() {
                    acc.push(c);
                } else if c.is_lowercase() {
                    acc.push(c);
                }
                acc
            })
    }
}

impl Default for NeologismGenerator {
    fn default() -> Self {
        Self::new()
    }
}
