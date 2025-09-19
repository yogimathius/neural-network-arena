use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::{NeuralArenaSimulation, SimulationConfig};
use crate::neural::{NeuralWarrior, Action};
use crate::environment::EnvironmentStats;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Use `wee_alloc` as the global allocator for smaller WASM binary size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Macro for console.log! functionality
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Neural Network Arena!");
}

// WebAssembly-compatible wrapper for the simulation
#[wasm_bindgen]
pub struct WasmSimulation {
    simulation: NeuralArenaSimulation,
    is_running: bool,
    animation_frame_id: Option<i32>,
}

// Serializable data structures for JavaScript
#[derive(Serialize, Deserialize)]
pub struct WarriorData {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub energy: f32,
    pub age: u32,
    pub fitness: f32,
    pub lineage_depth: u32,
    pub species_id: Option<u32>,
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceData {
    pub x: f32,
    pub y: f32,
    pub energy_value: f32,
    pub resource_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct TerritoryData {
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
    pub owner_id: Option<u32>,
    pub resource_multiplier: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SimulationState {
    pub warriors: Vec<WarriorData>,
    pub resources: Vec<ResourceData>,
    pub territories: Vec<TerritoryData>,
    pub generation: u32,
    pub tick: u64,
    pub population_size: usize,
    pub species_count: usize,
    pub average_fitness: f32,
    pub max_fitness: f32,
    pub diversity_score: f32,
    pub environmental_pressure: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryHeatmapData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>, // Flattened 2D array of memory usage intensities (0.0 to 1.0)
}

#[derive(Serialize, Deserialize)]
pub struct NetworkTopologyData {
    pub nodes: Vec<NetworkNode>,
    pub connections: Vec<NetworkConnection>,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub activation: f32,
    pub node_type: String, // "input", "hidden", "output"
}

#[derive(Serialize, Deserialize)]
pub struct NetworkConnection {
    pub from: u32,
    pub to: u32,
    pub weight: f32,
}

#[wasm_bindgen]
impl WasmSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<WasmSimulation, JsValue> {
        // Set panic hook for better error messages
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        let config: SimulationConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Config parse error: {}", e)))?;
        
        let simulation = NeuralArenaSimulation::new(config);
        
        log!("Neural Network Arena WebAssembly module initialized!");
        
        Ok(WasmSimulation {
            simulation,
            is_running: false,
            animation_frame_id: None,
        })
    }
    
    #[wasm_bindgen]
    pub fn initialize_population(&mut self, size: usize) {
        self.simulation.initialize_population(size);
        log!("Population initialized with {} warriors", size);
    }
    
    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.is_running = true;
        log!("Simulation started");
    }
    
    #[wasm_bindgen]
    pub fn pause(&mut self) {
        self.is_running = false;
        log!("Simulation paused");
    }
    
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.simulation.reset();
        self.is_running = false;
        log!("Simulation reset");
    }
    
    #[wasm_bindgen]
    pub fn step(&mut self) -> JsValue {
        let _update = self.simulation.single_tick();
        let state = self.get_simulation_state();
        serde_wasm_bindgen::to_value(&state).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn run_generation(&mut self) -> JsValue {
        let result = self.simulation.run_generation();
        let state = self.get_simulation_state();
        
        log!("Generation {} completed with {} survivors", 
             result.generation, result.survivors.len());
        
        serde_wasm_bindgen::to_value(&state).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn get_simulation_state_json(&self) -> String {
        let state = self.get_simulation_state();
        serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string())
    }
    
    #[wasm_bindgen]
    pub fn get_memory_heatmap(&self) -> JsValue {
        let heatmap = self.generate_memory_heatmap();
        serde_wasm_bindgen::to_value(&heatmap).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn get_network_topology(&self, warrior_id: u32) -> JsValue {
        let topology = self.generate_network_topology(warrior_id);
        serde_wasm_bindgen::to_value(&topology).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn get_performance_metrics(&self) -> JsValue {
        let stats = self.simulation.get_statistics();
        serde_wasm_bindgen::to_value(stats).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn export_data(&self, format: &str) -> String {
        match format {
            "json" => {
                let state = self.get_simulation_state();
                serde_json::to_string_pretty(&state).unwrap_or_else(|_| "{}".to_string())
            },
            "csv" => {
                self.export_csv_data()
            },
            _ => {
                log!("Unknown export format: {}", format);
                String::new()
            }
        }
    }
    
    #[wasm_bindgen]
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    #[wasm_bindgen]
    pub fn get_generation(&self) -> u32 {
        self.simulation.generation
    }
    
    #[wasm_bindgen]
    pub fn get_tick(&self) -> u64 {
        self.simulation.tick
    }
}

impl WasmSimulation {
    fn get_simulation_state(&self) -> SimulationState {
        let env_state = self.simulation.environment.get_environment_state();
        let stats = self.simulation.get_statistics();
        let species_stats = self.simulation.get_species_stats();
        
        // Convert warriors to serializable format
        let warriors: Vec<WarriorData> = env_state.warriors.iter().map(|warrior| {
            WarriorData {
                id: warrior.id,
                x: warrior.position.0,
                y: warrior.position.1,
                energy: warrior.energy,
                age: warrior.age,
                fitness: warrior.fitness_score,
                lineage_depth: warrior.lineage_depth,
                species_id: None, // TODO: Map warrior to species
                action: "idle".to_string(), // TODO: Get last action
            }
        }).collect();
        
        // Convert resources to serializable format
        let resources: Vec<ResourceData> = env_state.resources.iter().map(|resource| {
            ResourceData {
                x: resource.position.0,
                y: resource.position.1,
                energy_value: resource.energy_value,
                resource_type: format!("{:?}", resource.resource_type),
            }
        }).collect();
        
        // Convert territories to serializable format
        let territories: Vec<TerritoryData> = env_state.territories.iter().map(|territory| {
            TerritoryData {
                center_x: territory.center.0,
                center_y: territory.center.1,
                radius: territory.radius,
                owner_id: territory.owner_id,
                resource_multiplier: territory.resource_multiplier,
            }
        }).collect();
        
        SimulationState {
            warriors,
            resources,
            territories,
            generation: stats.generation,
            tick: stats.tick,
            population_size: stats.population_size,
            species_count: species_stats.species_count,
            average_fitness: stats.average_fitness,
            max_fitness: stats.max_fitness,
            diversity_score: stats.diversity_score,
            environmental_pressure: stats.environmental_pressure,
        }
    }
    
    fn generate_memory_heatmap(&self) -> MemoryHeatmapData {
        let width = 64;
        let height = 64;
        let mut data = vec![0.0; width * height];
        
        // Generate heatmap based on VM memory usage and territory allocation
        let memory_size = self.simulation.vm.memory_size();
        let territories = self.simulation.memory_allocator.total_territories();
        
        for i in 0..data.len() {
            // Map 2D heatmap coordinates to VM memory addresses
            let memory_address = (i * memory_size) / data.len();
            
            // Check if this memory region is allocated
            let intensity = if self.simulation.memory_allocator.can_access(memory_address, 999) {
                0.3 + (rand::random::<f32>() * 0.7) // Random activity for visualization
            } else {
                0.8 + (rand::random::<f32>() * 0.2) // High intensity for allocated regions
            };
            
            data[i] = intensity;
        }
        
        MemoryHeatmapData {
            width,
            height,
            data,
        }
    }
    
    fn generate_network_topology(&self, warrior_id: u32) -> NetworkTopologyData {
        let mut nodes = Vec::new();
        let mut connections = Vec::new();
        
        // Generate a simple neural network topology visualization
        // Input layer (8 nodes)
        for i in 0..8 {
            nodes.push(NetworkNode {
                id: i,
                x: 50.0,
                y: 50.0 + (i as f32 * 40.0),
                activation: rand::random::<f32>(),
                node_type: "input".to_string(),
            });
        }
        
        // Hidden layer (16 nodes)
        for i in 8..24 {
            nodes.push(NetworkNode {
                id: i,
                x: 200.0,
                y: 25.0 + ((i - 8) as f32 * 20.0),
                activation: rand::random::<f32>(),
                node_type: "hidden".to_string(),
            });
        }
        
        // Output layer (4 nodes)
        for i in 24..28 {
            nodes.push(NetworkNode {
                id: i,
                x: 350.0,
                y: 100.0 + ((i - 24) as f32 * 50.0),
                activation: rand::random::<f32>(),
                node_type: "output".to_string(),
            });
        }
        
        // Generate connections (simplified - full connectivity between layers)
        for input_id in 0..8 {
            for hidden_id in 8..24 {
                connections.push(NetworkConnection {
                    from: input_id,
                    to: hidden_id,
                    weight: (rand::random::<f32>() - 0.5) * 2.0,
                });
            }
        }
        
        for hidden_id in 8..24 {
            for output_id in 24..28 {
                connections.push(NetworkConnection {
                    from: hidden_id,
                    to: output_id,
                    weight: (rand::random::<f32>() - 0.5) * 2.0,
                });
            }
        }
        
        NetworkTopologyData {
            nodes,
            connections,
        }
    }
    
    fn export_csv_data(&self) -> String {
        let state = self.get_simulation_state();
        let mut csv = String::new();
        
        // CSV header
        csv.push_str("id,x,y,energy,age,fitness,lineage_depth\n");
        
        // Warrior data
        for warrior in &state.warriors {
            csv.push_str(&format!("{},{},{},{},{},{},{}\n",
                warrior.id, warrior.x, warrior.y, warrior.energy,
                warrior.age, warrior.fitness, warrior.lineage_depth
            ));
        }
        
        csv
    }
}