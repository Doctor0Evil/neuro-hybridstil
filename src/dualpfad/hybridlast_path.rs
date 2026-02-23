use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridlastTestResult {
    pub term_german: String,
    pub term_english: String,
    pub term_hybrid: String,
    pub accuracy: f64,
    pub reaction_time_ms: u32,
    pub error_rate: f64,
    pub confidence: f64,
    pub cognitive_load_estimate: f64,
    pub passes_threshold: bool,
}

pub struct HybridlastPath {
    min_accuracy: f64,
    max_reaction_time: u32,
    max_error_rate: f64,
    min_confidence: f64,
    max_cognitive_load: f64,
}

impl HybridlastPath {
    pub fn new(
        min_accuracy: f64,
        max_reaction_time: u32,
        max_error_rate: f64,
        min_confidence: f64,
        max_cognitive_load: f64,
    ) -> Self {
        Self {
            min_accuracy,
            max_reaction_time,
            max_error_rate,
            min_confidence,
            max_cognitive_load,
        }
    }

    pub fn evaluate_comprehension(&self, result: &mut HybridlastTestResult) -> Result<bool> {
        result.passes_threshold = result.accuracy >= self.min_accuracy
            && result.reaction_time_ms <= self.max_reaction_time
            && result.error_rate <= self.max_error_rate
            && result.confidence >= self.min_confidence
            && result.cognitive_load_estimate <= self.max_cognitive_load;

        Ok(result.passes_threshold)
    }

    pub fn compute_cognitive_load(&self, reaction_time_ms: u32, error_rate: f64) -> f64 {
        let rt_component = (reaction_time_ms as f64 / 1000.0).min(1.0);
        let er_component = error_rate.min(1.0);
        (rt_component * 0.6 + er_component * 0.4).min(1.0)
    }
}

impl Default for HybridlastPath {
    fn default() -> Self {
        Self::new(0.90, 1200, 0.05, 0.85, 0.65)
    }
}
