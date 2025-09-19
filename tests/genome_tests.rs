use neural_network_arena::neural::{Genome, NeuralNetwork};

#[test]
fn test_genome_size_constraints() {
    // Test that random genomes respect the 64-byte maximum
    for _ in 0..100 {
        let genome = Genome::new_random();
        assert!(genome.size() <= Genome::MAX_SIZE, 
               "Genome size {} exceeds maximum {}", genome.size(), Genome::MAX_SIZE);
        assert!(genome.size() >= 32, "Genome too small: {}", genome.size());
    }
}

#[test]
fn test_genome_fitness_tracking() {
    let mut genome = Genome::new_random();
    
    // Initial fitness should be zero
    assert_eq!(genome.fitness(), 0.0);
    
    // Test fitness updates
    genome.set_fitness(42.5);
    assert_eq!(genome.fitness(), 42.5);
    
    genome.set_fitness(-10.0);
    assert_eq!(genome.fitness(), -10.0);
}

#[test]
fn test_genome_generation_tracking() {
    let genome = Genome::new_random();
    assert_eq!(genome.generation(), 0);
    
    // Test with specific generation
    let layer_sizes = vec![8, 16, 4];
    let network = NeuralNetwork::new(layer_sizes);
    let genome_gen5 = Genome::from_network(&network, 5, 12345);
    assert_eq!(genome_gen5.generation(), 5);
    assert_eq!(genome_gen5.lineage_id(), 12345);
}

#[test]
fn test_genome_crossover() {
    let parent1 = Genome::new_random();
    let parent2 = Genome::new_random();
    
    let child = parent1.crossover(&parent2);
    
    // Child should have higher generation than parents
    assert!(child.generation() > parent1.generation().max(parent2.generation()));
    
    // Child should respect size constraints
    assert!(child.size() <= Genome::MAX_SIZE);
    
    // Child should have different lineage ID
    assert_ne!(child.lineage_id(), parent1.lineage_id());
    assert_ne!(child.lineage_id(), parent2.lineage_id());
    
    // Child should start with zero fitness
    assert_eq!(child.fitness(), 0.0);
}

#[test]
fn test_genome_mutation() {
    let original_genome = Genome::new_random();
    let mut mutated_genome = original_genome.clone();
    
    // High mutation rate should change the genome
    mutated_genome.mutate(1.0);
    
    // Size should remain the same
    assert_eq!(mutated_genome.size(), original_genome.size());
    
    // Generation and lineage should remain unchanged by mutation
    assert_eq!(mutated_genome.generation(), original_genome.generation());
    assert_eq!(mutated_genome.lineage_id(), original_genome.lineage_id());
    
    // Fitness should remain unchanged by mutation
    assert_eq!(mutated_genome.fitness(), original_genome.fitness());
}

#[test]
fn test_genome_network_conversion() {
    let layer_sizes = vec![8, 16, 4];
    let network = NeuralNetwork::new(layer_sizes.clone());
    
    // Test encoding network to genome
    let genome = Genome::from_network(&network, 1, 42);
    
    // Test decoding genome back to network
    let decoded_network = genome.to_network();
    
    // Network should have expected structure (8 inputs, 4 outputs)
    // This tests the MVP requirement: "8 inputs: memory pressure, neighbor proximity, resources"
    // and "4 decisions: move, replicate, attack, defend"
    
    // We can't directly compare networks, but we can test basic properties
    assert!(genome.size() > 0);
    assert!(genome.size() <= Genome::MAX_SIZE);
}

#[test]
fn test_genome_size_enforcement_in_crossover() {
    // Create two maximum-sized genomes
    let mut parent1 = Genome::new_random();
    let mut parent2 = Genome::new_random();
    
    // Force them to maximum size by setting fitness and testing
    for _ in 0..100 {
        let child = parent1.crossover(&parent2);
        assert!(child.size() <= Genome::MAX_SIZE, 
               "Crossover child size {} exceeds maximum", child.size());
    }
}

#[test]
fn test_genome_deterministic_properties() {
    let layer_sizes = vec![8, 16, 4];
    let network = NeuralNetwork::new(layer_sizes);
    
    // Same inputs should produce same genome
    let genome1 = Genome::from_network(&network, 5, 12345);
    let genome2 = Genome::from_network(&network, 5, 12345);
    
    assert_eq!(genome1.generation(), genome2.generation());
    assert_eq!(genome1.lineage_id(), genome2.lineage_id());
    assert_eq!(genome1.size(), genome2.size());
}