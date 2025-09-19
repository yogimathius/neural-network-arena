use super::{Genome, NeuralNetwork};
use crate::vm::{Instruction, OpCode, VirtualMachine};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralWarrior {
    pub id: u32,
    pub genome: Genome,
    pub network: NeuralNetwork,
    pub position: (f32, f32),
    pub energy: f32,
    pub age: u32,
    pub territory_id: Option<usize>,
    pub action_history: VecDeque<Action>,
    pub fitness_score: f32,
    pub lineage_depth: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Move { direction: f32, intensity: f32 },
    Attack { target_direction: f32, strength: f32 },
    Defend { shield_strength: f32 },
    Replicate { mutation_rate: f32 },
    Sense { sensor_type: SensorType },
    Rest,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SensorType {
    Energy,
    NeighborProximity,
    ResourceDensity,
    TerritoryPressure,
    Population,
    Threat,
    Age,
    LineageDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSensors {
    pub energy_level: f32,
    pub neighbor_proximity: f32,
    pub resource_density: f32,
    pub territory_pressure: f32,
    pub population_density: f32,
    pub threat_level: f32,
    pub age_normalized: f32,
    pub lineage_depth_normalized: f32,
}

impl NeuralWarrior {
    pub fn new(genome: Genome, id: u32) -> Self {
        let network = genome.to_network();
        let position = (
            rand::random::<f32>() * 1000.0,
            rand::random::<f32>() * 1000.0,
        );
        
        Self {
            id,
            genome,
            network,
            position,
            energy: 100.0,
            age: 0,
            territory_id: None,
            action_history: VecDeque::with_capacity(10),
            fitness_score: 0.0,
            lineage_depth: 0,
        }
    }
    
    pub fn from_parents(parent1: &Self, parent2: &Self, id: u32) -> Self {
        let child_genome = parent1.genome.crossover(&parent2.genome);
        let lineage_depth = parent1.lineage_depth.max(parent2.lineage_depth) + 1;
        
        let mut warrior = Self::new(child_genome, id);
        warrior.lineage_depth = lineage_depth;
        warrior
    }
    
    pub fn sense_environment(&self, environment: &EnvironmentState) -> EnvironmentSensors {
        EnvironmentSensors {
            energy_level: self.energy / 100.0,
            neighbor_proximity: self.calculate_neighbor_proximity(environment),
            resource_density: self.calculate_resource_density(environment),
            territory_pressure: self.calculate_territory_pressure(environment),
            population_density: self.calculate_population_density(environment),
            threat_level: self.calculate_threat_level(environment),
            age_normalized: (self.age as f32).min(1000.0) / 1000.0,
            lineage_depth_normalized: (self.lineage_depth as f32).min(50.0) / 50.0,
        }
    }
    
    pub fn decide_action(&mut self, sensors: &EnvironmentSensors) -> Action {
        let sensor_inputs = vec![
            sensors.energy_level,
            sensors.neighbor_proximity,
            sensors.resource_density,
            sensors.territory_pressure,
            sensors.population_density,
            sensors.threat_level,
            sensors.age_normalized,
            sensors.lineage_depth_normalized,
        ];
        
        let outputs = self.network.forward(&sensor_inputs);
        let action = self.interpret_neural_output(&outputs);
        
        if self.action_history.len() >= 10 {
            self.action_history.pop_front();
        }
        self.action_history.push_back(action);
        
        action
    }
    
    pub fn execute_vm_instructions(&mut self, vm: &mut VirtualMachine) -> Result<Vec<Instruction>, String> {
        let mut instructions = Vec::new();
        let sensor_data = self.get_vm_sensor_data();
        
        // Load sensor data into VM memory
        for (i, &value) in sensor_data.iter().enumerate() {
            if i < 8 {
                let instruction = Instruction::new(OpCode::Replicate, 1000 + i, i, value);
                instructions.push(instruction);
            }
        }
        
        // Generate neural processing instructions
        instructions.push(Instruction::new(OpCode::Activate, 0, 8, 0.0));
        instructions.push(Instruction::new(OpCode::Activate, 1, 9, 0.0));
        instructions.push(Instruction::new(OpCode::Activate, 2, 10, 0.0));
        instructions.push(Instruction::new(OpCode::Activate, 3, 11, 0.0));
        
        // Add mutation for evolution
        if rand::random::<f32>() < 0.01 {
            instructions.push(Instruction::new(OpCode::Mutate, 8, 8, 0.1));
        }
        
        Ok(instructions)
    }
    
    pub fn update_fitness(&mut self, survival_time: u32, resources_acquired: f32, combat_success: f32) {
        let survival_component = (survival_time as f32).ln().max(0.0);
        let resource_component = resources_acquired.sqrt();
        let combat_component = combat_success * 2.0;
        let age_bonus = if self.age > 100 { 10.0 } else { 0.0 };
        let lineage_bonus = (self.lineage_depth as f32) * 0.5;
        
        self.fitness_score = survival_component + resource_component + combat_component + age_bonus + lineage_bonus;
    }
    
    pub fn can_replicate(&self) -> bool {
        self.energy > 80.0 && self.age > 10
    }
    
    pub fn consume_energy(&mut self, amount: f32) {
        self.energy = (self.energy - amount).max(0.0);
    }
    
    pub fn gain_energy(&mut self, amount: f32) {
        self.energy = (self.energy + amount).min(100.0);
    }
    
    pub fn is_alive(&self) -> bool {
        self.energy > 0.0
    }
    
    pub fn age_tick(&mut self) {
        self.age += 1;
        self.consume_energy(0.1); // Aging costs energy
    }
    
    fn calculate_neighbor_proximity(&self, environment: &EnvironmentState) -> f32 {
        let mut closest_distance = f32::INFINITY;
        
        for other_warrior in &environment.warriors {
            if other_warrior.id != self.id {
                let distance = self.distance_to(other_warrior);
                if distance < closest_distance {
                    closest_distance = distance;
                }
            }
        }
        
        if closest_distance == f32::INFINITY {
            0.0
        } else {
            (100.0 / (closest_distance + 1.0)).min(1.0)
        }
    }
    
    fn calculate_resource_density(&self, environment: &EnvironmentState) -> f32 {
        let nearby_resources = environment.resources.iter()
            .filter(|resource| self.distance_to_point(resource.position) < 50.0)
            .count();
        
        (nearby_resources as f32 / 10.0).min(1.0)
    }
    
    fn calculate_territory_pressure(&self, _environment: &EnvironmentState) -> f32 {
        // Simplified territory pressure based on energy and position
        let boundary_distance = self.position.0.min(self.position.1)
            .min(1000.0 - self.position.0)
            .min(1000.0 - self.position.1);
        
        if boundary_distance < 50.0 {
            1.0 - (boundary_distance / 50.0)
        } else {
            0.0
        }
    }
    
    fn calculate_population_density(&self, environment: &EnvironmentState) -> f32 {
        let nearby_population = environment.warriors.iter()
            .filter(|warrior| warrior.id != self.id && self.distance_to(warrior) < 100.0)
            .count();
        
        (nearby_population as f32 / 20.0).min(1.0)
    }
    
    fn calculate_threat_level(&self, environment: &EnvironmentState) -> f32 {
        let mut max_threat = 0.0;
        
        for other_warrior in &environment.warriors {
            if other_warrior.id != self.id {
                let distance = self.distance_to(other_warrior);
                let energy_ratio = other_warrior.energy / (self.energy + 1.0);
                let threat = (energy_ratio / (distance + 1.0)).min(1.0);
                
                if threat > max_threat {
                    max_threat = threat;
                }
            }
        }
        
        max_threat
    }
    
    fn distance_to(&self, other: &NeuralWarrior) -> f32 {
        let dx = self.position.0 - other.position.0;
        let dy = self.position.1 - other.position.1;
        (dx * dx + dy * dy).sqrt()
    }
    
    fn distance_to_point(&self, point: (f32, f32)) -> f32 {
        let dx = self.position.0 - point.0;
        let dy = self.position.1 - point.1;
        (dx * dx + dy * dy).sqrt()
    }
    
    fn interpret_neural_output(&self, outputs: &[f32]) -> Action {
        if outputs.len() < 4 {
            return Action::Rest;
        }
        
        let action_type = outputs.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        
        match action_type {
            0 => Action::Move {
                direction: outputs[0] * std::f32::consts::PI * 2.0,
                intensity: outputs[0].abs().min(1.0),
            },
            1 => Action::Attack {
                target_direction: outputs[1] * std::f32::consts::PI * 2.0,
                strength: outputs[1].abs().min(1.0),
            },
            2 => Action::Defend {
                shield_strength: outputs[2].abs().min(1.0),
            },
            3 => {
                if self.can_replicate() {
                    Action::Replicate {
                        mutation_rate: (outputs[3].abs() * 0.2).min(0.5),
                    }
                } else {
                    Action::Rest
                }
            },
            _ => Action::Rest,
        }
    }
    
    fn get_vm_sensor_data(&self) -> Vec<f32> {
        vec![
            self.energy / 100.0,
            self.position.0 / 1000.0,
            self.position.1 / 1000.0,
            self.age as f32 / 1000.0,
            self.fitness_score / 100.0,
            self.lineage_depth as f32 / 50.0,
            if self.territory_id.is_some() { 1.0 } else { 0.0 },
            self.action_history.len() as f32 / 10.0,
        ]
    }

    pub fn get_sensor_reading(&self, sensor_type: SensorType, environment: &crate::environment::Environment) -> f32 {
        // Convert Environment to EnvironmentState for sensor calculations
        let env_state = EnvironmentState {
            warriors: environment.warriors.values().cloned().collect(),
            resources: Vec::new(), // Environment has resources but different structure
            territories: Vec::new(), // Environment has territories but different structure  
            tick: 0,
        };

        match sensor_type {
            SensorType::Energy => self.energy / 100.0,
            SensorType::NeighborProximity => self.calculate_neighbor_proximity(&env_state),
            SensorType::ResourceDensity => self.calculate_resource_density(&env_state),
            SensorType::TerritoryPressure => self.calculate_territory_pressure(&env_state),
            SensorType::Population => self.calculate_population_density(&env_state),
            SensorType::Threat => self.calculate_threat_level(&env_state),
            SensorType::Age => {
                // Normalize age to 0.0-1.0 range, assuming max age of 1000
                (self.age as f32 / 1000.0).min(1.0)
            },
            SensorType::LineageDepth => {
                // Normalize lineage depth to 0.0-1.0 range, assuming max depth of 50
                (self.lineage_depth as f32 / 50.0).min(1.0)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub warriors: Vec<NeuralWarrior>,
    pub resources: Vec<Resource>,
    pub territories: Vec<Territory>,
    pub tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub position: (f32, f32),
    pub energy_value: f32,
    pub resource_type: ResourceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Energy,
    Computational,
    Territory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Territory {
    pub center: (f32, f32),
    pub radius: f32,
    pub owner_id: Option<u32>,
    pub resource_multiplier: f32,
}