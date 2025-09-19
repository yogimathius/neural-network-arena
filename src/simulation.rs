use crate::environment::{Environment, EnvironmentUpdate, ActionResults};
use crate::evolution::{SpeciationManager, SpeciesStats};
use crate::neural::{Genome, NeuralWarrior, Action};
use crate::vm::VirtualMachine;
use crate::memory::MemoryAllocator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NeuralArenaSimulation {
    pub environment: Environment,
    pub vm: VirtualMachine,
    pub memory_allocator: MemoryAllocator,
    pub speciation_manager: SpeciationManager,
    pub simulation_config: SimulationConfig,
    pub statistics: SimulationStatistics,
    pub generation: u32,
    pub tick: u64,
    pub is_running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub max_population: usize,
    pub vm_memory_size: usize,
    pub territory_size: usize,
    pub target_species_count: usize,
    pub mutation_rate: f32,
    pub survival_threshold: f32,
    pub fitness_sharing: bool,
    pub elitism_rate: f32,
    pub tournament_size: usize,
    pub max_generations: u32,
    pub performance_target_rps: u32, // rounds per second
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_population: 200,
            vm_memory_size: 2048,
            territory_size: 64,
            target_species_count: 8,
            mutation_rate: 0.05,
            survival_threshold: 0.3,
            fitness_sharing: true,
            elitism_rate: 0.1,
            tournament_size: 3,
            max_generations: 1000,
            performance_target_rps: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStatistics {
    pub generation: u32,
    pub tick: u64,
    pub population_size: usize,
    pub species_count: usize,
    pub average_fitness: f32,
    pub max_fitness: f32,
    pub diversity_score: f32,
    pub survival_rate: f32,
    pub average_age: f32,
    pub max_lineage_depth: u32,
    pub computational_efficiency: f32,
    pub rounds_per_second: f32,
    pub resource_utilization: f32,
    pub environmental_pressure: f32,
}

#[derive(Debug, Clone)]
pub struct GenerationResult {
    pub generation: u32,
    pub survivors: Vec<NeuralWarrior>,
    pub extinct_lineages: Vec<u32>,
    pub new_species: usize,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub simulation_time_ms: u128,
    pub rounds_per_second: f32,
    pub vm_cycles_executed: u64,
    pub memory_allocations: usize,
    pub species_operations: usize,
}

impl NeuralArenaSimulation {
    pub fn new(config: SimulationConfig) -> Self {
        let environment = Environment::new(1000.0, 1000.0, config.max_population);
        let vm = VirtualMachine::new(config.vm_memory_size);
        let memory_allocator = MemoryAllocator::new(config.vm_memory_size, config.territory_size);
        let speciation_manager = SpeciationManager::new(config.target_species_count);
        
        Self {
            environment,
            vm,
            memory_allocator,
            speciation_manager,
            simulation_config: config,
            statistics: SimulationStatistics::default(),
            generation: 0,
            tick: 0,
            is_running: false,
        }
    }
    
    pub fn initialize_population(&mut self, initial_population: usize) {
        for _ in 0..initial_population.min(self.simulation_config.max_population) {
            let genome = Genome::new_random();
            let warrior = NeuralWarrior::new(genome, rand::random());
            self.environment.add_warrior(warrior);
        }
        
        self.is_running = true;
    }
    
    pub fn run_simulation(&mut self, max_ticks: Option<u64>) -> Vec<GenerationResult> {
        let mut generation_results = Vec::new();
        
        while self.is_running {
            if let Some(max_ticks) = max_ticks {
                if self.tick >= max_ticks {
                    break;
                }
            }
            
            if self.generation >= self.simulation_config.max_generations {
                break;
            }
            
            let generation_result = self.run_generation();
            generation_results.push(generation_result);
            
            // Check termination conditions
            if self.environment.warriors.is_empty() {
                println!("Simulation ended: Population extinct");
                break;
            }
            
            if self.statistics.max_fitness > 1000.0 {
                println!("Simulation ended: Fitness threshold reached");
                break;
            }
        }
        
        generation_results
    }
    
    pub fn run_generation(&mut self) -> GenerationResult {
        let start_time = std::time::Instant::now();
        let mut performance_metrics = PerformanceMetrics {
            simulation_time_ms: 0,
            rounds_per_second: 0.0,
            vm_cycles_executed: 0,
            memory_allocations: 0,
            species_operations: 0,
        };
        
        self.generation += 1;
        let generation_ticks = 1000; // Each generation lasts 1000 ticks
        
        // Run generation simulation
        for _ in 0..generation_ticks {
            self.tick += 1;
            
            // Environment update
            let env_update = self.environment.tick();
            
            // Get current warriors
            let warriors: Vec<NeuralWarrior> = self.environment.warriors.values().cloned().collect();
            if warriors.is_empty() {
                break;
            }
            
            // Execute neural networks and VM instructions
            let warrior_actions = self.execute_neural_decisions(&warriors, &mut performance_metrics);
            
            // Execute actions in environment
            let action_results = self.environment.execute_warrior_actions(warrior_actions);
            
            // Update fitness based on survival and performance
            self.update_fitness_scores(&action_results);
            
            performance_metrics.vm_cycles_executed += self.vm.cycle_count();
        }
        
        // Collect survivors
        let survivors: Vec<NeuralWarrior> = self.environment.warriors.values().cloned().collect();
        
        // Apply speciation and evolution
        let initial_species_count = self.speciation_manager.species.len();
        self.speciation_manager.speciate(&survivors);
        performance_metrics.species_operations += 1;
        
        let new_species = self.speciation_manager.species.len().saturating_sub(initial_species_count);
        
        // Evolve population
        let next_generation = if survivors.len() > 10 {
            self.speciation_manager.perform_species_selection(&survivors)
        } else {
            // Emergency population boost
            self.create_emergency_population(&survivors)
        };
        
        // Replace population
        self.environment.warriors.clear();
        for warrior in &next_generation {
            self.environment.add_warrior(warrior.clone());
        }
        
        // Update statistics
        self.update_statistics(&survivors);
        
        // Calculate performance metrics
        let elapsed = start_time.elapsed();
        performance_metrics.simulation_time_ms = elapsed.as_millis();
        performance_metrics.rounds_per_second = (generation_ticks as f32 / elapsed.as_secs_f32())
            .min(self.simulation_config.performance_target_rps as f32);
        
        GenerationResult {
            generation: self.generation,
            survivors,
            extinct_lineages: Vec::new(), // TODO: Track extinct lineages
            new_species,
            performance_metrics,
        }
    }
    
    pub fn single_tick(&mut self) -> EnvironmentUpdate {
        self.tick += 1;
        
        // Environment update
        let env_update = self.environment.tick();
        
        // Get current warriors
        let warriors: Vec<NeuralWarrior> = self.environment.warriors.values().cloned().collect();
        
        if !warriors.is_empty() {
            let mut perf_metrics = PerformanceMetrics {
                simulation_time_ms: 0,
                rounds_per_second: 0.0,
                vm_cycles_executed: 0,
                memory_allocations: 0,
                species_operations: 0,
            };
            
            // Execute neural decisions
            let warrior_actions = self.execute_neural_decisions(&warriors, &mut perf_metrics);
            
            // Execute actions
            let action_results = self.environment.execute_warrior_actions(warrior_actions);
            
            // Update fitness
            self.update_fitness_scores(&action_results);
        }
        
        env_update
    }
    
    pub fn get_statistics(&self) -> &SimulationStatistics {
        &self.statistics
    }
    
    pub fn get_species_stats(&self) -> SpeciesStats {
        self.speciation_manager.get_species_stats()
    }
    
    pub fn pause(&mut self) {
        self.is_running = false;
    }
    
    pub fn resume(&mut self) {
        self.is_running = true;
    }
    
    pub fn reset(&mut self) {
        self.environment = Environment::new(1000.0, 1000.0, self.simulation_config.max_population);
        self.vm = VirtualMachine::new(self.simulation_config.vm_memory_size);
        self.memory_allocator = MemoryAllocator::new(
            self.simulation_config.vm_memory_size, 
            self.simulation_config.territory_size
        );
        self.speciation_manager = SpeciationManager::new(self.simulation_config.target_species_count);
        self.generation = 0;
        self.tick = 0;
        self.statistics = SimulationStatistics::default();
    }
    
    fn execute_neural_decisions(&mut self, warriors: &[NeuralWarrior], performance_metrics: &mut PerformanceMetrics) -> HashMap<u32, Action> {
        let mut warrior_actions = HashMap::new();
        let environment_state = self.environment.get_environment_state();
        
        for warrior in warriors {
            // Sense environment
            let sensors = warrior.sense_environment(&environment_state);
            
            // Make decision
            let mut warrior_copy = warrior.clone();
            let action = warrior_copy.decide_action(&sensors);
            
            // Execute VM instructions for neural processing
            if let Ok(instructions) = warrior_copy.execute_vm_instructions(&mut self.vm) {
                for instruction in instructions {
                    if let Err(_) = self.vm.execute_instruction(&instruction) {
                        // VM instruction failed - continue with basic neural decision
                        break;
                    }
                    performance_metrics.vm_cycles_executed += 1;
                }
            }
            
            // Allocate memory territory if needed
            if warrior.territory_id.is_none() && rand::random::<f32>() < 0.1 {
                if let Ok(_territory_id) = self.memory_allocator.allocate_territory(warrior.id) {
                    performance_metrics.memory_allocations += 1;
                }
            }
            
            warrior_actions.insert(warrior.id, action);
        }
        
        warrior_actions
    }
    
    fn update_fitness_scores(&mut self, _action_results: &ActionResults) {
        for warrior in self.environment.warriors.values_mut() {
            // Calculate fitness based on survival, energy, age, and lineage
            let survival_time = warrior.age;
            let resources_acquired = warrior.energy;
            let combat_success = 0.0; // TODO: Track combat success
            
            warrior.update_fitness(survival_time, resources_acquired, combat_success);
        }
    }
    
    fn create_emergency_population(&self, survivors: &[NeuralWarrior]) -> Vec<NeuralWarrior> {
        let mut emergency_population = Vec::new();
        let target_size = self.simulation_config.max_population / 4; // Quarter population for recovery
        
        if survivors.is_empty() {
            // Complete extinction - create new random population
            for i in 0..target_size {
                let genome = Genome::new_random();
                let warrior = NeuralWarrior::new(genome, i as u32);
                emergency_population.push(warrior);
            }
        } else {
            // Clone and mutate best survivors
            let best_survivors: Vec<&NeuralWarrior> = {
                let mut sorted = survivors.iter().collect::<Vec<_>>();
                sorted.sort_by(|a, b| b.fitness_score.partial_cmp(&a.fitness_score).unwrap());
                sorted.into_iter().take(5).collect()
            };
            
            for i in 0..target_size {
                let parent = best_survivors[i % best_survivors.len()];
                let mut child = parent.clone();
                child.id = rand::random();
                child.age = 0;
                child.fitness_score = 0.0;
                child.genome.mutate(0.2); // Higher mutation rate for recovery
                child.network = child.genome.to_network();
                child.lineage_depth += 1;
                emergency_population.push(child);
            }
        }
        
        emergency_population
    }
    
    fn update_statistics(&mut self, survivors: &[NeuralWarrior]) {
        let population_size = survivors.len();
        
        if population_size == 0 {
            self.statistics = SimulationStatistics {
                generation: self.generation,
                tick: self.tick,
                population_size: 0,
                species_count: 0,
                average_fitness: 0.0,
                max_fitness: 0.0,
                diversity_score: 0.0,
                survival_rate: 0.0,
                average_age: 0.0,
                max_lineage_depth: 0,
                computational_efficiency: 0.0,
                rounds_per_second: 0.0,
                resource_utilization: 0.0,
                environmental_pressure: self.environment.environmental_pressure,
            };
            return;
        }
        
        let total_fitness: f32 = survivors.iter().map(|w| w.fitness_score).sum();
        let average_fitness = total_fitness / population_size as f32;
        let max_fitness = survivors.iter().map(|w| w.fitness_score).fold(0.0, f32::max);
        
        let total_age: u32 = survivors.iter().map(|w| w.age).sum();
        let average_age = total_age as f32 / population_size as f32;
        
        let max_lineage_depth = survivors.iter().map(|w| w.lineage_depth).max().unwrap_or(0);
        
        // Calculate diversity as variance in fitness scores
        let fitness_variance = if population_size > 1 {
            let variance_sum: f32 = survivors.iter()
                .map(|w| (w.fitness_score - average_fitness).powi(2))
                .sum();
            variance_sum / (population_size - 1) as f32
        } else {
            0.0
        };
        
        let diversity_score = fitness_variance.sqrt();
        
        let species_count = self.speciation_manager.species.len();
        let survival_rate = population_size as f32 / self.simulation_config.max_population as f32;
        let resource_utilization = self.environment.resources.len() as f32 / 200.0; // Assuming max 200 resources
        
        self.statistics = SimulationStatistics {
            generation: self.generation,
            tick: self.tick,
            population_size,
            species_count,
            average_fitness,
            max_fitness,
            diversity_score,
            survival_rate,
            average_age,
            max_lineage_depth,
            computational_efficiency: self.vm.available_resources() as f32 / 10000.0,
            rounds_per_second: 0.0, // Updated in performance metrics
            resource_utilization,
            environmental_pressure: self.environment.environmental_pressure,
        };
    }
}

impl Default for SimulationStatistics {
    fn default() -> Self {
        Self {
            generation: 0,
            tick: 0,
            population_size: 0,
            species_count: 0,
            average_fitness: 0.0,
            max_fitness: 0.0,
            diversity_score: 0.0,
            survival_rate: 0.0,
            average_age: 0.0,
            max_lineage_depth: 0,
            computational_efficiency: 0.0,
            rounds_per_second: 0.0,
            resource_utilization: 0.0,
            environmental_pressure: 0.0,
        }
    }
}