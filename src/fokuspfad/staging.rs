use crate::error::Result;
use crate::fokuspfad::concept_anchor::ConceptAnchor;
use crate::fokuspfad::neologism_gen::NeologismCandidate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StageLevel {
    Phase1ConceptAnchoring,
    Phase2NeuralMetaphor,
    Phase3FullValidation,
    Phase4Lexicalization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedFokuspfad {
    pub stage: StageLevel,
    pub concept_anchors: Vec<ConceptAnchor>,
    pub neologism_candidates: Vec<NeologismCandidate>,
    pub lexicalized_terms: Vec<String>,
}

impl StagedFokuspfad {
    pub fn new() -> Self {
        Self {
            stage: StageLevel::Phase1ConceptAnchoring,
            concept_anchors: Vec::new(),
            neologism_candidates: Vec::new(),
            lexicalized_terms: Vec::new(),
        }
    }

    pub fn add_concept_anchor(&mut self, anchor: ConceptAnchor) -> Result<()> {
        self.concept_anchors.push(anchor);
        Ok(())
    }

    pub fn progress_to_neologism(&mut self) -> Result<()> {
        if self.stage == StageLevel::Phase1ConceptAnchoring {
            self.stage = StageLevel::Phase2NeuralMetaphor;
            Ok(())
        } else {
            Err(crate::error::NeoError::NeologismGenFailed(
                "Must complete concept anchoring first".to_string(),
            ))
        }
    }

    pub fn add_neologism_candidate(&mut self, candidate: NeologismCandidate) -> Result<()> {
        if self.stage >= StageLevel::Phase2NeuralMetaphor {
            self.neologism_candidates.push(candidate);
            Ok(())
        } else {
            Err(crate::error::NeoError::NeologismGenFailed(
                "Must be in neologism generation phase".to_string(),
            ))
        }
    }

    pub fn lexicalize_term(&mut self, term: String) -> Result<()> {
        if self.stage >= StageLevel::Phase3FullValidation {
            self.lexicalized_terms.push(term);
            self.stage = StageLevel::Phase4Lexicalization;
            Ok(())
        } else {
            Err(crate::error::NeoError::NeologismGenFailed(
                "Must complete validation first".to_string(),
            ))
        }
    }
}

impl Default for StagedFokuspfad {
    fn default() -> Self {
        Self::new()
    }
}
