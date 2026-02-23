use crate::error::Result;
use crate::config::CompressionStrategy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStep {
    pub stage: u32,
    pub input: String,
    pub output: String,
    pub reduction_ratio: f64,
}

pub struct CompressionPath {
    strategy: CompressionStrategy,
    max_length: usize,
}

impl CompressionPath {
    pub fn new(strategy: CompressionStrategy, max_length: usize) -> Self {
        Self { strategy, max_length }
    }

    pub fn compress_phrase(&self, phrase: &str) -> Result<Vec<CompressionStep>> {
        let mut steps = Vec::new();
        let mut current = phrase.to_string();
        let mut stage = 0u32;

        let initial_len = current.len() as f64;

        while current.len() > self.max_length && stage < 5 {
            stage += 1;
            let previous = current.clone();
            current = self.apply_compression_strategy(&current, stage);

            let reduction_ratio = current.len() as f64 / previous.len() as f64;
            steps.push(CompressionStep {
                stage,
                input: previous,
                output: current.clone(),
                reduction_ratio,
            });

            if reduction_ratio > 0.95 {
                break;
            }
        }

        Ok(steps)
    }

    fn apply_compression_strategy(&self, phrase: &str, stage: u32) -> String {
        match self.strategy {
            CompressionStrategy::Minimal => self.minimal_compression(phrase, stage),
            CompressionStrategy::Balanced => self.balanced_compression(phrase, stage),
            CompressionStrategy::Descriptive => self.descriptive_compression(phrase, stage),
        }
    }

    fn minimal_compression(&self, phrase: &str, _stage: u32) -> String {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let mut result = String::new();
        for word in words {
            if let Some(first) = word.chars().next() {
                result.push(first);
            }
        }
        result
    }

    fn balanced_compression(&self, phrase: &str, stage: u32) -> String {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        if stage == 1 {
            words.iter().take(words.len() / 2).map(|w| *w).collect::<Vec<_>>().join(" ")
        } else {
            self.minimal_compression(phrase, stage)
        }
    }

    fn descriptive_compression(&self, phrase: &str, _stage: u32) -> String {
        phrase[..phrase.len().saturating_sub(1)].to_string()
    }
}

impl Default for CompressionPath {
    fn default() -> Self {
        Self::new(CompressionStrategy::Balanced, 25)
    }
}
