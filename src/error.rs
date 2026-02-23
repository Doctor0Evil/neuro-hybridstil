use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeoError {
    #[error("Lexicon not found: {0}")]
    LexiconNotFound(String),

    #[error("Term validation failed: {0}")]
    TermValidationFailed(String),

    #[error("Word math violation: {0}")]
    WordMathViolation(String),

    #[error("Hybridlast threshold exceeded: {score} > {threshold}")]
    HybridlastThresholdExceeded { score: f64, threshold: f64 },

    #[error("Interop validation failed: {component} – {reason}")]
    InteropValidationFailed { component: String, reason: String },

    #[error("Neologism generation failed: {0}")]
    NeologismGenFailed(String),

    #[error("Sprachknoten sync failed: {source_lang} → {target_lang}")]
    SprachknotenSyncFailed { source_lang: String, target_lang: String },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid cluster type: {0}")]
    InvalidClusterType(String),

    #[error("Missing node: {node_id}")]
    MissingNode { node_id: String },

    #[error("Compression strategy not applicable: {0}")]
    CompressionNotApplicable(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::error::Error),
}

pub type Result<T> = std::result::Result<T, NeoError>;
