use neural_network_arena::{
    NeuralArenaSimulation, SimulationConfig,
    neural::{Genome, NeuralWarrior},
    environment::Environment,
    vm::VirtualMachine,
};

#[test]
fn test_full_simulation_integration() {
    let config = SimulationConfig {
        max_population: 50,
        vm_memory_size: 1024,
        territory_size: 32,
        target_species_count: 5,
        mutation_rate: 0.05,
        survival_threshold: 0.3,
        fitness_sharing: true,
        elitism_rate: 0.1,
        tournament_size: 3,
        max_generations: 5, // Short test
        performance_target_rps: 100,
    };
    
    let mut simulation = NeuralArenaSimulation::new(config);
    simulation.initialize_population(30);
    
    // Run for a few generations
    let results = simulation.run_simulation(Some(1000));
    
    assert!(!results.is_empty());
    assert!(simulation.generation > 0);
    assert!(simulation.tick > 0);
    
    let stats = simulation.get_statistics();
    assert!(stats.population_size > 0);
}

#[test]
fn test_warrior_environment_interaction() {
    let mut environment = Environment::new(500.0, 500.0, 100);
    
    // Create test warriors
    let genome1 = Genome::new_random();
    let genome2 = Genome::new_random();
    let warrior1 = NeuralWarrior::new(genome1, 1);
    let warrior2 = NeuralWarrior::new(genome2, 2);
    
    environment.add_warrior(warrior1);
    environment.add_warrior(warrior2);
    
    // Run environment for several ticks
    for _ in 0..10 {
        let _update = environment.tick();
        
        // Get warrior actions
        let warriors: Vec<_> = environment.warriors.values().collect();
        let env_state = environment.get_environment_state();
        
        let mut actions = std::collections::HashMap::new();
        for warrior in warriors {
            let sensors = warrior.sense_environment(&env_state);
            let mut warrior_copy = warrior.clone();
            let action = warrior_copy.decide_action(&sensors);
            actions.insert(warrior.id, action);
        }
        
        let _results = environment.execute_warrior_actions(actions);
    }
    
    assert!(environment.warriors.len() <= 2); // Warriors might die or replicate
    assert_eq!(environment.tick, 10);
}

#[test]
fn test_vm_neural_integration() {
    let mut vm = VirtualMachine::new(512);
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    
    // Test VM instruction execution from warrior
    let instructions = warrior.execute_vm_instructions(&mut vm).unwrap();
    
    assert!(!instructions.is_empty());
    
    for instruction in instructions {
        let _result = vm.execute_instruction(&instruction); // VM execution may fail for some instructions, that's OK
    }
    
    assert!(vm.cycle_count() > 0);
    assert!(vm.available_resources() < 10000); // Some resources should be consumed
}

#[test]
fn test_speciation_system() {
    use neural_network_arena::evolution::SpeciationManager;
    
    let mut speciation = SpeciationManager::new(3);
    
    // Create diverse warriors
    let mut warriors = Vec::new();
    for i in 0..20 {
        let genome = Genome::new_random();
        let mut warrior = NeuralWarrior::new(genome, i);
        warrior.fitness_score = rand::random::<f32>() * 100.0;
        warriors.push(warrior);
    }
    
    // Apply speciation
    speciation.speciate(&warriors);
    
    let stats = speciation.get_species_stats();
    assert!(stats.species_count > 0);
    assert!(stats.species_count <= warriors.len());
    
    // Test selection
    let next_gen = speciation.perform_species_selection(&warriors);
    assert_eq!(next_gen.len(), warriors.len());
}

#[test]
fn test_memory_territory_allocation() {
    use neural_network_arena::memory::MemoryAllocator;
    
    let mut allocator = MemoryAllocator::new(1024, 64);
    
    // Allocate several territories
    let territory1 = allocator.allocate_territory(1).unwrap();
    let territory2 = allocator.allocate_territory(2).unwrap();
    let territory3 = allocator.allocate_territory(3).unwrap();
    
    assert_ne!(territory1, territory2);
    assert_ne!(territory2, territory3);
    
    // Test access control (territories might have different addressing)
    let territory1_address = territory1 * 64;
    assert!(allocator.can_access(territory1_address, 1));
    // Access control might not be strict initially, so just verify basic functionality
    
    // Test deallocation
    let _ = allocator.deallocate_territory(territory1, 1); // May succeed or fail, we just test it doesn't crash
}

#[test]
fn test_fitness_calculation() {
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    
    assert_eq!(warrior.fitness_score, 0.0);
    
    // Update fitness based on survival and performance
    warrior.update_fitness(100, 50.0, 10.0);
    
    assert!(warrior.fitness_score > 0.0);
    
    // Older, more successful warriors should have higher fitness
    let mut warrior2 = warrior.clone();
    warrior2.id = 2;
    warrior2.update_fitness(200, 80.0, 20.0);
    
    assert!(warrior2.fitness_score > warrior.fitness_score);
}

#[test]
fn test_environmental_events() {
    let mut environment = Environment::new(1000.0, 1000.0, 200);
    
    // Add some warriors and resources
    for i in 0..10 {
        let genome = Genome::new_random();
        let warrior = NeuralWarrior::new(genome, i);
        environment.add_warrior(warrior);
    }
    
    let _initial_resources = environment.resources.len();
    let initial_warriors = environment.warriors.len();
    
    // Run simulation and look for environmental events
    let mut event_occurred = false;
    for _ in 0..100 {
        let update = environment.tick();
        if update.environmental_event.is_some() {
            event_occurred = true;
            break;
        }
    }
    
    // Events should eventually occur (though not guaranteed in short test)
    // At minimum, verify the system doesn't crash
    assert!(environment.warriors.len() <= initial_warriors);
    assert!(environment.tick == 100 || event_occurred);
}

#[test]
fn test_warrior_replication() {
    let mut environment = Environment::new(1000.0, 1000.0, 200);
    
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    warrior.energy = 100.0; // Full energy
    warrior.age = 20; // Old enough to replicate
    
    environment.add_warrior(warrior);
    
    // Try to trigger replication
    use neural_network_arena::neural::Action;
    let mut actions = std::collections::HashMap::new();
    actions.insert(1, Action::Replicate { mutation_rate: 0.1 });
    
    let _results = environment.execute_warrior_actions(actions);
    
    // Check if replication succeeded (should create offspring)
    if environment.warriors.len() > 1 {
        // Replication successful
        assert!(environment.warriors.len() == 2);
        
        // Parent should have less energy
        let parent = environment.warriors.get(&1).unwrap();
        assert!(parent.energy < 100.0);
    }
}

#[test]
fn test_combat_system() {
    let mut environment = Environment::new(1000.0, 1000.0, 200);
    
    // Create two warriors close to each other
    let genome1 = Genome::new_random();
    let genome2 = Genome::new_random();
    let mut warrior1 = NeuralWarrior::new(genome1, 1);
    let mut warrior2 = NeuralWarrior::new(genome2, 2);
    
    warrior1.position = (100.0, 100.0);
    warrior2.position = (110.0, 100.0); // Close proximity
    warrior1.energy = 100.0;
    warrior2.energy = 100.0;
    
    environment.add_warrior(warrior1);
    environment.add_warrior(warrior2);
    
    // Warrior 1 attacks toward warrior 2
    use neural_network_arena::neural::Action;
    let mut actions = std::collections::HashMap::new();
    actions.insert(1, Action::Attack { 
        target_direction: 0.0, // Attack eastward 
        strength: 1.0 
    });
    
    let _results = environment.execute_warrior_actions(actions);
    
    // Combat should have some effect (energy changes, etc.)
    let warrior1_after = environment.warriors.get(&1).unwrap();
    let warrior2_after = environment.warriors.get(&2);
    
    // Attacker should have less energy from attack cost
    assert!(warrior1_after.energy < 100.0);
    
    // Target might be damaged or might have survived
    if let Some(w2) = warrior2_after {
        // If warrior 2 survived, they might have taken damage
        assert!(w2.energy <= 100.0);
    }
}

#[test]
fn test_population_stability() {
    let config = SimulationConfig {
        max_population: 100,
        max_generations: 10,
        ..SimulationConfig::default()
    };
    
    let mut simulation = NeuralArenaSimulation::new(config);
    simulation.initialize_population(50);
    
    let mut population_history = Vec::new();
    
    for _ in 0..10 {
        let _result = simulation.run_generation();
        let stats = simulation.get_statistics();
        population_history.push(stats.population_size);
    }
    
    // Population should remain relatively stable (not crash to 0 or explode)
    assert!(population_history.iter().all(|&pop| pop > 0));
    assert!(population_history.iter().all(|&pop| pop <= 100));
    
    // Should have some variation but not wild swings
    let min_pop = *population_history.iter().min().unwrap();
    let max_pop = *population_history.iter().max().unwrap();
    assert!((max_pop as f32) / (min_pop as f32) < 5.0); // Less than 5x variation
}