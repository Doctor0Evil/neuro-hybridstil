use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoConfig {
    pub hybridlast_threshold: f64,
    pub compression_strategy: CompressionStrategy,
    pub supported_languages: Vec<String>,
    pub interop_platforms: Vec<String>,
    pub phonotactic_min_pronounceability: f64,
    pub lexicon_persistence: LexiconPersistence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionStrategy {
    Minimal,
    Balanced,
    Descriptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexiconPersistence {
    pub storage_backend: StorageBackend,
    pub enable_versioning: bool,
    pub audit_trail: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageBackend {
    InMemory,
    FileSystem,
    Database,
}

impl Default for NeoConfig {
    fn default() -> Self {
        Self {
            hybridlast_threshold: 0.65,
            compression_strategy: CompressionStrategy::Balanced,
            supported_languages: vec![
                "de".to_string(),
                "en".to_string(),
                "la".to_string(),
                "el".to_string(),
            ],
            interop_platforms: vec![
                "ONNX".to_string(),
                "NeuroML".to_string(),
                "Loihi".to_string(),
                "SpiNNaker".to_string(),
                "TrueNorth".to_string(),
            ],
            phonotactic_min_pronounceability: 0.75,
            lexicon_persistence: LexiconPersistence {
                storage_backend: StorageBackend::InMemory,
                enable_versioning: true,
                audit_trail: true,
            },
        }
    }
}
