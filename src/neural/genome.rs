use super::network::NeuralNetwork;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    data: Vec<u8>,
    fitness: f32,
    generation: u32,
    lineage_id: u32,
}

impl Genome {
    pub const MAX_SIZE: usize = 64;

    pub fn new_random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(32..=Self::MAX_SIZE);
        let data = (0..size).map(|_| rng.gen()).collect();

        Self {
            data,
            fitness: 0.0,
            generation: 0,
            lineage_id: rng.gen(),
        }
    }

    pub fn from_network(network: &NeuralNetwork, generation: u32, lineage_id: u32) -> Self {
        let mut data = Vec::new();
        data.extend_from_slice(&(network.parameter_count() as u16).to_le_bytes());

        Self {
            data,
            fitness: 0.0,
            generation,
            lineage_id,
        }
    }

    pub fn to_network(&self) -> NeuralNetwork {
        let layer_sizes = vec![8, 16, 4];
        NeuralNetwork::new(layer_sizes)
    }

    pub fn crossover(&self, other: &Self) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let crossover_point = rng.gen_range(1..self.data.len().min(other.data.len()));

        let mut child_data = self.data[..crossover_point].to_vec();
        child_data.extend_from_slice(&other.data[crossover_point..]);

        if child_data.len() > Self::MAX_SIZE {
            child_data.truncate(Self::MAX_SIZE);
        }

        Self {
            data: child_data,
            fitness: 0.0,
            generation: self.generation.max(other.generation) + 1,
            lineage_id: rng.gen(),
        }
    }

    pub fn mutate(&mut self, rate: f32) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for byte in &mut self.data {
            if rng.gen::<f32>() < rate {
                *byte = rng.gen();
            }
        }
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }

    pub fn set_fitness(&mut self, fitness: f32) {
        self.fitness = fitness;
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn lineage_id(&self) -> u32 {
        self.lineage_id
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
