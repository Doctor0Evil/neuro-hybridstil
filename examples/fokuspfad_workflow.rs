use neuro_hybridstil::{
    fokuspfad::{
        concept_anchor::ConceptAnchor,
        neologism_gen::{MetaphorFamily, NeologismGenerator},
        staging::StagedFokuspfad,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fokuspfad = StagedFokuspfad::new();

    let mut concept = ConceptAnchor::new(
        "concept_001".to_string(),
        "A pathway through nodes in a neural or neuromorphic system".to_string(),
    );
    concept.add_surface_form("de".to_string(), "Knotenpfad".to_string());
    concept.add_surface_form("en".to_string(), "node path".to_string());
    concept.add_surface_form("la".to_string(), "via nodorum".to_string());
    concept.mark_validated()?;

    fokuspfad.add_concept_anchor(concept)?;
    println!("Phase 1 Complete: Concept anchored");

    fokuspfad.progress_to_neologism()?;
    println!("Progressed to Phase 2: Neural Metaphor Generation");

    let mut generator = NeologismGenerator::new();
    let candidate = generator.generate_candidate(
        "Netz",
        "Pfad",
        MetaphorFamily::Topological,
        "A path composed of network elements".to_string(),
    )?;

    fokuspfad.add_neologism_candidate(candidate.clone())?;
    println!("Generated neologism: {}", candidate.german_form);

    Ok(())
}
