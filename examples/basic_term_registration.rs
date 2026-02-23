use neuro_hybridstil::{
    config::NeoConfig,
    hybridstyler::{WordMathEngine, HybridValidator},
    knotenlexikon::{
        lexicon::{NodeId, TermEntry, TermCategory},
        KnotenLexikon,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = NeoConfig::default();
    let lexicon = KnotenLexikon::new();

    let mut knotenpfad_entry = TermEntry::new(
        "Knotenpfad".to_string(),
        "NodePath".to_string(),
        "Ordered sequence of nodes carrying a signal or activation through a network".to_string(),
        TermCategory::PathType,
    )
    .with_multilingual("en".to_string(), "node path".to_string())
    .with_multilingual("la".to_string(), "via nodorum".to_string())
    .with_examples(vec![
        "A Knotenpfad through visual processing layers".to_string(),
        "Spike propagation follows a Knotenpfad".to_string(),
    ]);

    let wm_engine = WordMathEngine::new();
    let analysis = wm_engine.analyze_german_form(&knotenpfad_entry.german_form)?;
    knotenpfad_entry.word_math_compliance.overall_score = analysis.structural_well_formedness;
    knotenpfad_entry.phonotactic_score = analysis.phonotactic_score;

    let validator = HybridValidator::default();
    let report = validator.validate_term(&knotenpfad_entry)?;

    println!("Knotenpfad Validation Report:");
    println!("  Valid: {}", report.is_valid);
    println!("  Word-Math Score: {:.2}", report.word_math_score);
    println!("  Hybridlast: {:.2}", report.hybridlast_score);

    if !report.issues.is_empty() {
        println!("  Issues:");
        for issue in &report.issues {
            println!("    - {}", issue);
        }
    }

    let node_id = lexicon.register(knotenpfad_entry)?;
    println!("\nRegistered Knotenpfad with ID: {}", node_id.0);

    let retrieved = lexicon.lookup_by_code_label("NodePath")?;
    println!("Retrieved: {} ({})", retrieved.german_form, retrieved.code_label);

    Ok(())
}
