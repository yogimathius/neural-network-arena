use crate::neural::{Genome, NeuralNetwork};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Population {
    genomes: Vec<Genome>,
    generation: u32,
    population_size: usize,
    mutation_rate: f32,
    #[allow(dead_code)]
    mutation_strength: f32,
    tournament_size: usize,
}

impl Population {
    pub fn new(size: usize) -> Self {
        let genomes = (0..size).map(|_| Genome::new_random()).collect();

        Self {
            genomes,
            generation: 0,
            population_size: size,
            mutation_rate: 0.1,
            mutation_strength: 0.5,
            tournament_size: 3,
        }
    }

    pub fn evolve(&mut self) {
        let mut new_genomes = Vec::with_capacity(self.population_size);

        for _ in 0..self.population_size {
            let parent1 = self.tournament_selection();
            let parent2 = self.tournament_selection();

            let mut child = parent1.crossover(parent2);
            child.mutate(self.mutation_rate);

            new_genomes.push(child);
        }

        self.genomes = new_genomes;
        self.generation += 1;
    }

    pub fn tournament_selection(&self) -> &Genome {
        let mut rng = rand::thread_rng();
        let mut best_genome = &self.genomes[0];
        let mut best_fitness = best_genome.fitness();

        for _ in 1..self.tournament_size {
            let candidate_idx = rng.gen_range(0..self.genomes.len());
            let candidate = &self.genomes[candidate_idx];

            if candidate.fitness() > best_fitness {
                best_genome = candidate;
                best_fitness = candidate.fitness();
            }
        }

        best_genome
    }

    pub fn evaluate_fitness<F>(&mut self, fitness_fn: F)
    where
        F: Fn(&NeuralNetwork) -> f32,
    {
        for genome in &mut self.genomes {
            let network = genome.to_network();
            let fitness = fitness_fn(&network);
            genome.set_fitness(fitness);
        }
    }

    pub fn best_genome(&self) -> Option<&Genome> {
        self.genomes
            .iter()
            .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
    }

    pub fn average_fitness(&self) -> f32 {
        let total: f32 = self.genomes.iter().map(|g| g.fitness()).sum();
        total / self.genomes.len() as f32
    }

    pub fn diversity_score(&self) -> f32 {
        let mut total_distance = 0.0;
        let mut comparisons = 0;

        for i in 0..self.genomes.len() {
            for j in (i + 1)..self.genomes.len() {
                total_distance += self.genome_distance(&self.genomes[i], &self.genomes[j]);
                comparisons += 1;
            }
        }

        if comparisons > 0 {
            total_distance / comparisons as f32
        } else {
            0.0
        }
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn size(&self) -> usize {
        self.genomes.len()
    }

    pub fn genomes(&self) -> &[Genome] {
        &self.genomes
    }

    pub fn statistics(&self) -> PopulationStats {
        let fitnesses: Vec<f32> = self.genomes.iter().map(|g| g.fitness()).collect();
        let min_fitness = fitnesses.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_fitness = fitnesses.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg_fitness = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;

        let mut lineage_counts = HashMap::new();
        for genome in &self.genomes {
            *lineage_counts.entry(genome.lineage_id()).or_insert(0) += 1;
        }

        PopulationStats {
            generation: self.generation,
            population_size: self.genomes.len(),
            min_fitness,
            max_fitness,
            avg_fitness,
            diversity: self.diversity_score(),
            lineage_diversity: lineage_counts.len(),
        }
    }

    fn genome_distance(&self, a: &Genome, b: &Genome) -> f32 {
        if a.lineage_id() == b.lineage_id() {
            0.0
        } else {
            1.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct PopulationStats {
    pub generation: u32,
    pub population_size: usize,
    pub min_fitness: f32,
    pub max_fitness: f32,
    pub avg_fitness: f32,
    pub diversity: f32,
    pub lineage_diversity: usize,
}
