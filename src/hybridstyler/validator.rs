use crate::error::{NeoError, Result};
use crate::knotenlexikon::lexicon::TermEntry;

pub struct HybridValidator {
    max_hybridlast: f64,
    min_word_math_score: f64,
}

pub struct ValidationReport {
    pub is_valid: bool,
    pub hybridlast_score: f64,
    pub word_math_score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

impl HybridValidator {
    pub fn new(max_hybridlast: f64, min_word_math_score: f64) -> Self {
        Self {
            max_hybridlast,
            min_word_math_score,
        }
    }

    pub fn validate_term(&self, entry: &TermEntry) -> Result<ValidationReport> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        let wm_score = entry.word_math_compliance.overall_score;
        if wm_score < self.min_word_math_score {
            issues.push(format!(
                "Word-math compliance below threshold: {:.2}",
                wm_score
            ));
            recommendations.push("Restructure compound to improve head-final compliance".to_string());
        }

        let phonotactic_score = entry.phonotactic_score;
        if phonotactic_score < 0.65 {
            issues.push(format!("Phonotactic score too low: {:.2}", phonotactic_score));
            recommendations.push("Simplify phonetic clusters; aim for natural pronunciation".to_string());
        }

        let hybridlast_score = (1.0 - wm_score) + (1.0 - phonotactic_score) / 2.0;
        if hybridlast_score > self.max_hybridlast {
            issues.push(format!(
                "Hybridlast exceeds threshold: {:.2} > {:.2}",
                hybridlast_score, self.max_hybridlast
            ));
            recommendations.push("Reduce cognitive load by shortening or simplifying".to_string());
        }

        let is_valid = issues.is_empty();

        Ok(ValidationReport {
            is_valid,
            hybridlast_score,
            word_math_score: wm_score,
            issues,
            recommendations,
        })
    }
}

impl Default for HybridValidator {
    fn default() -> Self {
        Self::new(0.65, 0.75)
    }
}
