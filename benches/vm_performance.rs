use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neural_network_arena::{
    NeuralArenaSimulation, SimulationConfig,
    vm::{VirtualMachine, Instruction, OpCode},
    neural::{Genome, NeuralWarrior},
    environment::Environment,
};

fn benchmark_vm_execution(c: &mut Criterion) {
    let vm = VirtualMachine::new(1024);

    c.bench_function("vm_single_round", |b| {
        b.iter(|| {
            let mut vm_copy = vm.clone();
            let _ = vm_copy.execute_round_robin_cycle();
        })
    });

    c.bench_function("vm_1000_rounds", |b| {
        b.iter(|| {
            let mut vm_copy = vm.clone();
            for _ in 0..1000 {
                let _ = vm_copy.execute_round_robin_cycle();
            }
        })
    });
    
    // Test instruction execution performance
    let instruction = Instruction::new(OpCode::Activate, 0, 1, 0.5);
    c.bench_function("vm_instruction_execution", |b| {
        b.iter(|| {
            let mut vm_copy = vm.clone();
            let _ = vm_copy.execute_instruction(black_box(&instruction));
        })
    });
}

fn benchmark_neural_network(c: &mut Criterion) {
    let genome = Genome::new_random();
    let network = genome.to_network();
    let inputs = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

    c.bench_function("neural_forward_pass", |b| {
        b.iter(|| {
            network.forward(black_box(&inputs));
        })
    });
    
    c.bench_function("warrior_decision_making", |b| {
        let mut warrior = NeuralWarrior::new(genome.clone(), 1);
        let environment = Environment::new(1000.0, 1000.0, 100);
        let env_state = environment.get_environment_state();
        let sensors = warrior.sense_environment(&env_state);
        
        b.iter(|| {
            warrior.decide_action(black_box(&sensors));
        })
    });
}

fn benchmark_environment(c: &mut Criterion) {
    c.bench_function("environment_single_tick", |b| {
        let mut environment = Environment::new(1000.0, 1000.0, 200);
        
        // Add some warriors
        for i in 0..50 {
            let genome = Genome::new_random();
            let warrior = NeuralWarrior::new(genome, i);
            environment.add_warrior(warrior);
        }
        
        b.iter(|| {
            environment.tick();
        })
    });
    
    c.bench_function("environment_with_actions", |b| {
        let mut environment = Environment::new(1000.0, 1000.0, 200);
        
        // Add warriors
        for i in 0..30 {
            let genome = Genome::new_random();
            let warrior = NeuralWarrior::new(genome, i);
            environment.add_warrior(warrior);
        }
        
        b.iter(|| {
            let _ = environment.tick();
            
            // Simulate warrior actions
            let warriors: Vec<_> = environment.warriors.values().collect();
            let env_state = environment.get_environment_state();
            let mut actions = std::collections::HashMap::new();
            
            for warrior in warriors {
                let sensors = warrior.sense_environment(&env_state);
                let mut warrior_copy = warrior.clone();
                let action = warrior_copy.decide_action(&sensors);
                actions.insert(warrior.id, action);
            }
            
            environment.execute_warrior_actions(black_box(actions));
        })
    });
}

fn benchmark_full_simulation(c: &mut Criterion) {
    c.bench_function("simulation_single_tick", |b| {
        let config = SimulationConfig {
            max_population: 100,
            vm_memory_size: 1024,
            ..SimulationConfig::default()
        };
        let mut simulation = NeuralArenaSimulation::new(config);
        simulation.initialize_population(50);
        
        b.iter(|| {
            simulation.single_tick();
        })
    });
    
    c.bench_function("simulation_generation_small", |b| {
        let config = SimulationConfig {
            max_population: 50,
            vm_memory_size: 512,
            target_species_count: 3,
            ..SimulationConfig::default()
        };
        
        b.iter(|| {
            let mut simulation = NeuralArenaSimulation::new(config.clone());
            simulation.initialize_population(25);
            simulation.run_generation();
        })
    });
}

fn benchmark_performance_targets(c: &mut Criterion) {
    // Test if we can achieve 1000+ simulation rounds per second
    c.bench_function("1000_simulation_rounds", |b| {
        let config = SimulationConfig {
            max_population: 100,
            vm_memory_size: 1024,
            performance_target_rps: 1000,
            ..SimulationConfig::default()
        };
        let mut simulation = NeuralArenaSimulation::new(config);
        simulation.initialize_population(50);
        
        b.iter(|| {
            // Run 1000 ticks to simulate 1 second at target rate
            for _ in 0..1000 {
                simulation.single_tick();
            }
        })
    });
    
    // Test population scalability
    c.bench_function("population_scalability_500", |b| {
        let config = SimulationConfig {
            max_population: 500,
            vm_memory_size: 2048,
            ..SimulationConfig::default()
        };
        let mut simulation = NeuralArenaSimulation::new(config);
        simulation.initialize_population(250);
        
        b.iter(|| {
            simulation.single_tick();
        })
    });
}

criterion_group!(
    benches, 
    benchmark_vm_execution,
    benchmark_neural_network,
    benchmark_environment,
    benchmark_full_simulation,
    benchmark_performance_targets
);
criterion_main!(benches);
