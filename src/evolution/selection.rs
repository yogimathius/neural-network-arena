use crate::neural::Genome;
use rand::Rng;

pub trait SelectionStrategy {
    fn select<'a>(&self, population: &'a [Genome]) -> &'a Genome;
}

#[derive(Debug)]
pub struct TournamentSelection {
    tournament_size: usize,
}

impl TournamentSelection {
    pub fn new(tournament_size: usize) -> Self {
        Self { tournament_size }
    }
}

impl SelectionStrategy for TournamentSelection {
    fn select<'a>(&self, population: &'a [Genome]) -> &'a Genome {
        let mut rng = rand::thread_rng();
        let mut best = &population[rng.gen_range(0..population.len())];

        for _ in 1..self.tournament_size {
            let candidate = &population[rng.gen_range(0..population.len())];
            if candidate.fitness() > best.fitness() {
                best = candidate;
            }
        }

        best
    }
}

#[derive(Debug)]
pub struct RouletteWheelSelection;

impl SelectionStrategy for RouletteWheelSelection {
    fn select<'a>(&self, population: &'a [Genome]) -> &'a Genome {
        let total_fitness: f32 = population.iter().map(|g| g.fitness().max(0.0)).sum();

        if total_fitness == 0.0 {
            let mut rng = rand::thread_rng();
            return &population[rng.gen_range(0..population.len())];
        }

        let mut rng = rand::thread_rng();
        let mut wheel_pos = rng.gen::<f32>() * total_fitness;

        for genome in population {
            wheel_pos -= genome.fitness().max(0.0);
            if wheel_pos <= 0.0 {
                return genome;
            }
        }

        &population[population.len() - 1]
    }
}

#[derive(Debug)]
pub struct ElitistSelection {
    #[allow(dead_code)]
    elite_count: usize,
}

impl ElitistSelection {
    pub fn new(elite_count: usize) -> Self {
        Self { elite_count }
    }
}

impl SelectionStrategy for ElitistSelection {
    fn select<'a>(&self, population: &'a [Genome]) -> &'a Genome {
        population
            .iter()
            .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
            .unwrap()
    }
}
