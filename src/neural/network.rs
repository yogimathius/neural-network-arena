use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetwork {
    weights: Vec<f32>,
    biases: Vec<f32>,
    layer_sizes: Vec<usize>,
}

impl NeuralNetwork {
    pub fn new(layer_sizes: Vec<usize>) -> Self {
        let total_weights = layer_sizes.windows(2).map(|pair| pair[0] * pair[1]).sum();
        let total_biases = layer_sizes.iter().skip(1).sum();

        Self {
            weights: vec![0.0; total_weights],
            biases: vec![0.0; total_biases],
            layer_sizes,
        }
    }

    pub fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        let mut activations = inputs.to_vec();
        let mut weight_idx = 0;
        let mut bias_idx = 0;

        for layer_idx in 1..self.layer_sizes.len() {
            let prev_size = self.layer_sizes[layer_idx - 1];
            let curr_size = self.layer_sizes[layer_idx];
            let mut next_activations = vec![0.0; curr_size];

            #[allow(clippy::needless_range_loop)]
            for j in 0..curr_size {
                let mut sum = self.biases[bias_idx + j];
                #[allow(clippy::needless_range_loop)]
                for i in 0..prev_size {
                    sum += activations[i] * self.weights[weight_idx + i * curr_size + j];
                }
                next_activations[j] = self.activation_function(sum);
            }

            weight_idx += prev_size * curr_size;
            bias_idx += curr_size;
            activations = next_activations;
        }

        activations
    }

    pub fn parameter_count(&self) -> usize {
        self.weights.len() + self.biases.len()
    }

    pub fn mutate(&mut self, mutation_rate: f32, mutation_strength: f32) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for weight in &mut self.weights {
            if rng.gen::<f32>() < mutation_rate {
                *weight += rng.gen_range(-mutation_strength..mutation_strength);
                *weight = weight.clamp(-1.0, 1.0);
            }
        }

        for bias in &mut self.biases {
            if rng.gen::<f32>() < mutation_rate {
                *bias += rng.gen_range(-mutation_strength..mutation_strength);
                *bias = bias.clamp(-1.0, 1.0);
            }
        }
    }

    fn activation_function(&self, x: f32) -> f32 {
        (2.0 / (1.0 + (-2.0 * x).exp())) - 1.0
    }
}
