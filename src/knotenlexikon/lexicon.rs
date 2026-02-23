use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermEntry {
    pub node_id: NodeId,
    pub german_form: String,
    pub code_label: String,
    pub multilingual_labels: HashMap<String, String>,
    pub definition: String,
    pub examples: Vec<String>,
    pub category: TermCategory,
    pub word_math_compliance: WordMathScore,
    pub phonotactic_score: f64,
    pub creation_timestamp: u64,
    pub validation_status: ValidationStatus,
    pub interop_metadata: InteropMetadata,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TermCategory {
    NodeType,
    ChannelType,
    ClusterType,
    PathType,
    InterfaceType,
    MetaphorsystemType,
    SystemInfrastructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordMathScore {
    pub head_final_compliance: f64,
    pub modifier_left_compliance: f64,
    pub structural_well_formedness: f64,
    pub cognitive_efficiency: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationStatus {
    Anchored,
    ConceptValidated,
    DualpfadPassed,
    Lexicalized,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteropMetadata {
    pub onnx_compatible: bool,
    pub neuroml_compatible: bool,
    pub loihi_compatible: bool,
    pub spinnaker_compatible: bool,
    pub truenorth_compatible: bool,
    pub schema_compliant: bool,
    pub platforms_tested: Vec<String>,
}

impl TermEntry {
    pub fn new(
        german_form: String,
        code_label: String,
        definition: String,
        category: TermCategory,
    ) -> Self {
        Self {
            node_id: NodeId(Uuid::new_v4().to_string()),
            german_form,
            code_label,
            multilingual_labels: HashMap::new(),
            definition,
            examples: Vec::new(),
            category,
            word_math_compliance: WordMathScore {
                head_final_compliance: 1.0,
                modifier_left_compliance: 1.0,
                structural_well_formedness: 1.0,
                cognitive_efficiency: 1.0,
                overall_score: 1.0,
            },
            phonotactic_score: 0.8,
            creation_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            validation_status: ValidationStatus::Anchored,
            interop_metadata: InteropMetadata {
                onnx_compatible: false,
                neuroml_compatible: false,
                loihi_compatible: false,
                spinnaker_compatible: false,
                truenorth_compatible: false,
                schema_compliant: false,
                platforms_tested: Vec::new(),
            },
        }
    }

    pub fn with_multilingual(mut self, lang: String, label: String) -> Self {
        self.multilingual_labels.insert(lang, label);
        self
    }

    pub fn with_examples(mut self, examples: Vec<String>) -> Self {
        self.examples = examples;
        self
    }

    pub fn mark_interop_compliant(&mut self, platforms: Vec<String>) {
        for platform in &platforms {
            match platform.as_str() {
                "ONNX" => self.interop_metadata.onnx_compatible = true,
                "NeuroML" => self.interop_metadata.neuroml_compatible = true,
                "Loihi" => self.interop_metadata.loihi_compatible = true,
                "SpiNNaker" => self.interop_metadata.spinnaker_compatible = true,
                "TrueNorth" => self.interop_metadata.truenorth_compatible = true,
                _ => {}
            }
        }
        self.interop_metadata.platforms_tested.extend(platforms);
    }
}
