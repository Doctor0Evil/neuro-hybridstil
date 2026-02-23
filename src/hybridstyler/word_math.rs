use crate::error::Result;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GERMAN_COMPOUND_PATTERN: Regex = Regex::new(r"^[A-Z][a-züöä]+[A-Z][a-züöä]+$").unwrap();
    static ref HEAD_FINAL_PATTERN: Regex = Regex::new(r"[A-Z][a-züöä]+$").unwrap();
    static ref MODIFIER_PATTERN: Regex = Regex::new(r"^[A-Z][a-züöä]+").unwrap();
}

pub struct WordMathEngine {
    phonotactic_inventory: Vec<String>,
}

pub struct WordMathAnalysis {
    pub is_head_final: bool,
    pub is_modifier_left: bool,
    pub structural_well_formedness: f64,
    pub phonotactic_score: f64,
    pub compressibility_potential: f64,
}

impl WordMathEngine {
    pub fn new() -> Self {
        Self {
            phonotactic_inventory: vec![
                "Kn", "Kh", "Sh", "Sp", "Str", "Schl", "Kr", "Kl", "Br", "Bl", "Dr", "Dw", "Fr", "Fl",
                "Gr", "Gl", "Pr", "Tr", "Tw", "Wr", "Sch", "Tsch", "Pf", "Kw",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }

    pub fn analyze_german_form(&self, form: &str) -> Result<WordMathAnalysis> {
        let is_head_final = HEAD_FINAL_PATTERN.is_match(form);
        let is_modifier_left = MODIFIER_PATTERN.is_match(form);
        let is_compound = GERMAN_COMPOUND_PATTERN.is_match(form);

        let structural_well_formedness = if is_compound && is_head_final && is_modifier_left {
            1.0
        } else if is_compound {
            0.85
        } else {
            0.6
        };

        let phonotactic_score = self.compute_phonotactic_score(form);
        let compressibility_potential = self.estimate_compressibility(form);

        Ok(WordMathAnalysis {
            is_head_final,
            is_modifier_left,
            structural_well_formedness,
            phonotactic_score,
            compressibility_potential,
        })
    }

    fn compute_phonotactic_score(&self, form: &str) -> f64 {
        let mut score = 0.8;
        for cluster in &self.phonotactic_inventory {
            if form.contains(cluster) {
                score += 0.05;
            }
        }
        score.min(1.0)
    }

    fn estimate_compressibility(&self, form: &str) -> f64 {
        let len = form.len();
        if len > 20 {
            0.95
        } else if len > 15 {
            0.80
        } else if len > 10 {
            0.60
        } else {
            0.40
        }
    }

    pub fn validate_code_label(&self, label: &str) -> Result<bool> {
        let is_camel_case = label.chars().next().map_or(false, |c| c.is_uppercase())
            && label.chars().all(|c| c.is_alphanumeric());
        let is_ascii = label.chars().all(|c| c.is_ascii());

        Ok(is_camel_case && is_ascii)
    }
}

impl Default for WordMathEngine {
    fn default() -> Self {
        Self::new()
    }
}
