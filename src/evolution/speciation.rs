use crate::neural::{Genome, NeuralWarrior};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Species {
    pub id: u32,
    pub representative: Genome,
    pub members: Vec<u32>, // warrior IDs
    pub average_fitness: f32,
    pub generations_since_improvement: u32,
    pub best_fitness: f32,
    pub fitness_history: Vec<f32>,
    pub stagnation_threshold: u32,
}

#[derive(Debug)]
pub struct SpeciationManager {
    pub species: HashMap<u32, Species>,
    pub compatibility_threshold: f32,
    pub species_counter: u32,
    pub target_species_count: usize,
    pub compatibility_weights: CompatibilityWeights,
}

#[derive(Debug, Clone)]
pub struct CompatibilityWeights {
    pub genome_size_weight: f32,
    pub lineage_weight: f32,
    pub fitness_weight: f32,
    pub age_weight: f32,
}

impl Default for CompatibilityWeights {
    fn default() -> Self {
        Self {
            genome_size_weight: 1.0,
            lineage_weight: 1.5,
            fitness_weight: 0.5,
            age_weight: 0.3,
        }
    }
}

impl SpeciationManager {
    pub fn new(target_species_count: usize) -> Self {
        Self {
            species: HashMap::new(),
            compatibility_threshold: 3.0,
            species_counter: 0,
            target_species_count,
            compatibility_weights: CompatibilityWeights::default(),
        }
    }
    
    pub fn speciate(&mut self, warriors: &[NeuralWarrior]) {
        // Clear existing species memberships
        for species in self.species.values_mut() {
            species.members.clear();
        }
        
        // Assign each warrior to a species
        for warrior in warriors {
            let species_id = self.find_compatible_species(warrior)
                .unwrap_or_else(|| self.create_new_species(warrior));
            
            if let Some(species) = self.species.get_mut(&species_id) {
                species.members.push(warrior.id);
            }
        }
        
        // Update species statistics
        self.update_species_statistics(warriors);
        
        // Remove empty species
        self.remove_empty_species();
        
        // Adjust compatibility threshold to maintain target species count
        self.adjust_compatibility_threshold();
    }
    
    pub fn perform_species_selection(&self, warriors: &[NeuralWarrior]) -> Vec<NeuralWarrior> {
        let mut selected = Vec::new();
        let total_fitness = self.calculate_total_adjusted_fitness(warriors);
        
        for species in self.species.values() {
            if species.members.is_empty() {
                continue;
            }
            
            let species_fitness = self.calculate_species_fitness(species, warriors);
            let offspring_count = ((species_fitness / total_fitness) * warriors.len() as f32) as usize;
            
            let species_warriors: Vec<&NeuralWarrior> = warriors.iter()
                .filter(|w| species.members.contains(&w.id))
                .collect();
            
            // Tournament selection within species
            for _ in 0..offspring_count {
                if let Some(parent1) = self.tournament_selection_within_species(&species_warriors, 3) {
                    let parent2 = self.tournament_selection_within_species(&species_warriors, 3)
                        .unwrap_or(parent1);
                    
                    let mut child = if parent1.id != parent2.id {
                        NeuralWarrior::from_parents(parent1, parent2, self.generate_warrior_id())
                    } else {
                        // Asexual reproduction with mutation
                        let mut child = parent1.clone();
                        child.id = self.generate_warrior_id();
                        child.genome.mutate(0.1);
                        child.network = child.genome.to_network();
                        child
                    };
                    
                    // Species-specific mutation rates
                    let mutation_rate = self.calculate_species_mutation_rate(species);
                    child.genome.mutate(mutation_rate);
                    child.network = child.genome.to_network();
                    
                    selected.push(child);
                }
            }
        }
        
        // Fill remaining slots with best performers
        while selected.len() < warriors.len() {
            if let Some(best) = self.get_best_warrior(warriors) {
                let mut child = best.clone();
                child.id = self.generate_warrior_id();
                child.genome.mutate(0.05);
                child.network = child.genome.to_network();
                selected.push(child);
            } else {
                break;
            }
        }
        
        selected.truncate(warriors.len());
        selected
    }
    
    pub fn get_species_stats(&self) -> SpeciesStats {
        SpeciesStats {
            species_count: self.species.len(),
            average_species_size: if self.species.is_empty() {
                0.0
            } else {
                self.species.values().map(|s| s.members.len()).sum::<usize>() as f32 / self.species.len() as f32
            },
            stagnant_species: self.species.values()
                .filter(|s| s.generations_since_improvement > s.stagnation_threshold)
                .count(),
            compatibility_threshold: self.compatibility_threshold,
        }
    }
    
    fn find_compatible_species(&self, warrior: &NeuralWarrior) -> Option<u32> {
        for (species_id, species) in &self.species {
            if self.calculate_compatibility_distance(warrior, &species.representative) < self.compatibility_threshold {
                return Some(*species_id);
            }
        }
        None
    }
    
    fn create_new_species(&mut self, warrior: &NeuralWarrior) -> u32 {
        self.species_counter += 1;
        let species_id = self.species_counter;
        
        let species = Species {
            id: species_id,
            representative: warrior.genome.clone(),
            members: vec![warrior.id],
            average_fitness: warrior.fitness_score,
            generations_since_improvement: 0,
            best_fitness: warrior.fitness_score,
            fitness_history: vec![warrior.fitness_score],
            stagnation_threshold: 15,
        };
        
        self.species.insert(species_id, species);
        species_id
    }
    
    fn calculate_compatibility_distance(&self, warrior: &NeuralWarrior, representative: &Genome) -> f32 {
        let weights = &self.compatibility_weights;
        
        let size_diff = (warrior.genome.size() as f32 - representative.size() as f32).abs() * weights.genome_size_weight;
        let lineage_diff = (warrior.lineage_depth as f32 - representative.generation() as f32).abs() * weights.lineage_weight;
        let fitness_diff = (warrior.fitness_score - representative.fitness()).abs() * weights.fitness_weight;
        let age_diff = warrior.age as f32 * weights.age_weight;
        
        size_diff + lineage_diff + fitness_diff + age_diff
    }
    
    fn update_species_statistics(&mut self, warriors: &[NeuralWarrior]) {
        for species in self.species.values_mut() {
            if species.members.is_empty() {
                continue;
            }
            
            let species_warriors: Vec<&NeuralWarrior> = warriors.iter()
                .filter(|w| species.members.contains(&w.id))
                .collect();
            
            let total_fitness: f32 = species_warriors.iter().map(|w| w.fitness_score).sum();
            species.average_fitness = total_fitness / species_warriors.len() as f32;
            
            let max_fitness = species_warriors.iter()
                .map(|w| w.fitness_score)
                .fold(f32::NEG_INFINITY, f32::max);
            
            if max_fitness > species.best_fitness {
                species.best_fitness = max_fitness;
                species.generations_since_improvement = 0;
                
                // Update representative to best member
                if let Some(best_warrior) = species_warriors.iter()
                    .max_by(|a, b| a.fitness_score.partial_cmp(&b.fitness_score).unwrap()) {
                    species.representative = best_warrior.genome.clone();
                }
            } else {
                species.generations_since_improvement += 1;
            }
            
            species.fitness_history.push(species.average_fitness);
            if species.fitness_history.len() > 20 {
                species.fitness_history.remove(0);
            }
        }
    }
    
    fn remove_empty_species(&mut self) {
        let empty_species: Vec<u32> = self.species.iter()
            .filter(|(_, species)| species.members.is_empty())
            .map(|(id, _)| *id)
            .collect();
        
        for species_id in empty_species {
            self.species.remove(&species_id);
        }
    }
    
    fn adjust_compatibility_threshold(&mut self) {
        let current_count = self.species.len();
        
        if current_count < self.target_species_count {
            self.compatibility_threshold -= 0.1;
        } else if current_count > self.target_species_count {
            self.compatibility_threshold += 0.1;
        }
        
        self.compatibility_threshold = self.compatibility_threshold.clamp(0.5, 10.0);
    }
    
    fn calculate_total_adjusted_fitness(&self, warriors: &[NeuralWarrior]) -> f32 {
        self.species.values()
            .map(|species| self.calculate_species_fitness(species, warriors))
            .sum()
    }
    
    fn calculate_species_fitness(&self, species: &Species, warriors: &[NeuralWarrior]) -> f32 {
        let species_warriors: Vec<&NeuralWarrior> = warriors.iter()
            .filter(|w| species.members.contains(&w.id))
            .collect();
        
        if species_warriors.is_empty() {
            return 0.0;
        }
        
        let total_fitness: f32 = species_warriors.iter().map(|w| w.fitness_score).sum();
        let adjusted_fitness = total_fitness / species_warriors.len() as f32;
        
        // Apply stagnation penalty
        if species.generations_since_improvement > species.stagnation_threshold / 2 {
            adjusted_fitness * 0.5
        } else {
            adjusted_fitness
        }
    }
    
    fn tournament_selection_within_species<'a>(&self, species_warriors: &[&'a NeuralWarrior], tournament_size: usize) -> Option<&'a NeuralWarrior> {
        if species_warriors.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let mut best: Option<&NeuralWarrior> = None;
        let mut best_fitness = f32::NEG_INFINITY;
        
        for _ in 0..tournament_size.min(species_warriors.len()) {
            let candidate = species_warriors[rng.gen_range(0..species_warriors.len())];
            if candidate.fitness_score > best_fitness {
                best = Some(candidate);
                best_fitness = candidate.fitness_score;
            }
        }
        
        best
    }
    
    fn calculate_species_mutation_rate(&self, species: &Species) -> f32 {
        let base_rate = 0.05;
        let stagnation_bonus = (species.generations_since_improvement as f32 / species.stagnation_threshold as f32) * 0.1;
        let diversity_penalty = if species.members.len() < 5 { 0.02 } else { 0.0 };
        
        (base_rate + stagnation_bonus + diversity_penalty).min(0.5)
    }
    
    fn get_best_warrior<'a>(&self, warriors: &'a [NeuralWarrior]) -> Option<&'a NeuralWarrior> {
        warriors.iter().max_by(|a, b| a.fitness_score.partial_cmp(&b.fitness_score).unwrap())
    }
    
    fn generate_warrior_id(&self) -> u32 {
        rand::random()
    }
}

#[derive(Debug, Clone)]
pub struct SpeciesStats {
    pub species_count: usize,
    pub average_species_size: f32,
    pub stagnant_species: usize,
    pub compatibility_threshold: f32,
}