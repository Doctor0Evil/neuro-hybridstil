use crate::error::{NeoError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteropValidationResult {
    pub term_code_label: String,
    pub platform: String,
    pub is_compatible: bool,
    pub schema_compliant: bool,
    pub config_compatible: bool,
    pub api_compatible: bool,
    pub test_artifacts: Vec<String>,
    pub issues: Vec<String>,
}

pub struct InteropPath {
    supported_platforms: Vec<String>,
    schema_validation_enabled: bool,
}

impl InteropPath {
    pub fn new(supported_platforms: Vec<String>, schema_validation_enabled: bool) -> Self {
        Self {
            supported_platforms,
            schema_validation_enabled,
        }
    }

    pub fn validate_term_interop(&self, code_label: &str, platform: &str) -> Result<InteropValidationResult> {
        if !self.supported_platforms.contains(&platform.to_string()) {
            return Err(NeoError::InteropValidationFailed {
                component: platform.to_string(),
                reason: "Unsupported platform".to_string(),
            });
        }

        let is_compatible = self.is_identifier_valid(code_label);
        let schema_compliant = if self.schema_validation_enabled {
            self.validate_schema_compliance(code_label, platform)?
        } else {
            true
        };

        Ok(InteropValidationResult {
            term_code_label: code_label.to_string(),
            platform: platform.to_string(),
            is_compatible,
            schema_compliant,
            config_compatible: true,
            api_compatible: is_compatible,
            test_artifacts: vec![],
            issues: vec![],
        })
    }

    fn is_identifier_valid(&self, label: &str) -> bool {
        label.chars().all(|c| c.is_alphanumeric()) && label.chars().next().map_or(false, |c| c.is_alphabetic())
    }

    fn validate_schema_compliance(&self, _label: &str, _platform: &str) -> Result<bool> {
        Ok(true)
    }

    pub fn build_corpus_entry(&self, term: &str, identifier: &str, behavior_desc: &str) -> String {
        format!(
            r#"{{"term": "{}", "identifier": "{}", "behavior": "{}"}}"#,
            term, identifier, behavior_desc
        )
    }
}

impl Default for InteropPath {
    fn default() -> Self {
        Self::new(
            vec![
                "ONNX".to_string(),
                "NeuroML".to_string(),
                "Loihi".to_string(),
                "SpiNNaker".to_string(),
                "TrueNorth".to_string(),
            ],
            true,
        )
    }
}
