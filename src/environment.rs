use crate::neural::{NeuralWarrior, Action, EnvironmentState, Resource, Territory};
use crate::neural::warrior::ResourceType;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub width: f32,
    pub height: f32,
    pub warriors: HashMap<u32, NeuralWarrior>,
    pub resources: Vec<Resource>,
    pub territories: Vec<Territory>,
    pub barriers: Vec<MemoryBarrier>,
    pub safe_zones: Vec<SafeZone>,
    pub tick: u64,
    pub resource_spawn_timer: u32,
    pub environmental_pressure: f32,
    pub carrying_capacity: usize,
    pub resource_config: ResourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub spawn_rate: f32,
    pub max_resources: usize,
    pub energy_range: (f32, f32),
    pub computational_bonus: f32,
    pub territory_control_bonus: f32,
    pub scarcity_events: bool,
    pub abundance_events: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBarrier {
    pub position: (f32, f32),
    pub width: f32,
    pub height: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeZone {
    pub center: (f32, f32),
    pub radius: f32,
    pub protection_level: f32,
    pub resource_bonus: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentEvent {
    pub event_type: EventType,
    pub duration: u32,
    pub intensity: f32,
    pub affected_area: Option<(f32, f32, f32)>, // center_x, center_y, radius
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    ResourceScarcity,
    ResourceAbundance,
    MemoryCompaction,
    TerritorialShift,
    PopulationPressure,
    EnergeticStorm,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            spawn_rate: 0.1,
            max_resources: 200,
            energy_range: (5.0, 25.0),
            computational_bonus: 1.5,
            territory_control_bonus: 2.0,
            scarcity_events: true,
            abundance_events: true,
        }
    }
}

impl Environment {
    pub fn new(width: f32, height: f32, carrying_capacity: usize) -> Self {
        let mut env = Self {
            width,
            height,
            warriors: HashMap::new(),
            resources: Vec::new(),
            territories: Vec::new(),
            barriers: Vec::new(),
            safe_zones: Vec::new(),
            tick: 0,
            resource_spawn_timer: 0,
            environmental_pressure: 0.0,
            carrying_capacity,
            resource_config: ResourceConfig::default(),
        };
        
        env.initialize_terrain();
        env.spawn_initial_resources();
        env
    }
    
    pub fn tick(&mut self) -> EnvironmentUpdate {
        self.tick += 1;
        self.resource_spawn_timer += 1;
        
        let mut update = EnvironmentUpdate::new(self.tick);
        
        // Update environmental pressure based on population
        self.update_environmental_pressure();
        
        // Spawn resources
        if self.should_spawn_resources() {
            let spawned = self.spawn_resources();
            update.resources_spawned = spawned;
            self.resource_spawn_timer = 0;
        }
        
        // Age and process warriors
        for warrior in self.warriors.values_mut() {
            warrior.age_tick();
        }
        
        // Remove dead warriors
        let initial_count = self.warriors.len();
        self.warriors.retain(|_, warrior| warrior.is_alive());
        update.warriors_died = initial_count - self.warriors.len();
        
        // Decay unused resources
        self.decay_resources();
        
        // Update territories
        self.update_territories();
        
        // Trigger random environmental events
        if rand::random::<f32>() < 0.02 {
            let event = self.generate_environmental_event();
            self.apply_environmental_event(&event);
            update.environmental_event = Some(event);
        }
        
        update
    }
    
    pub fn execute_warrior_actions(&mut self, actions: HashMap<u32, Action>) -> ActionResults {
        let mut results = ActionResults::new();
        
        for (warrior_id, action) in actions {
            if let Some(warrior) = self.warriors.get_mut(&warrior_id) {
                let result = self.execute_action(warrior_id, action);
                results.add_result(warrior_id, result);
            }
        }
        
        // Process combat interactions
        self.process_combat(&mut results);
        
        // Process resource collection
        self.process_resource_collection(&mut results);
        
        results
    }
    
    pub fn add_warrior(&mut self, warrior: NeuralWarrior) -> bool {
        if self.warriors.len() >= self.carrying_capacity {
            return false;
        }
        
        // Ensure warrior is positioned within bounds
        let mut positioned_warrior = warrior;
        positioned_warrior.position.0 = positioned_warrior.position.0.clamp(0.0, self.width);
        positioned_warrior.position.1 = positioned_warrior.position.1.clamp(0.0, self.height);
        
        self.warriors.insert(positioned_warrior.id, positioned_warrior);
        true
    }
    
    pub fn remove_warrior(&mut self, warrior_id: u32) -> Option<NeuralWarrior> {
        self.warriors.remove(&warrior_id)
    }
    
    pub fn get_environment_state(&self) -> EnvironmentState {
        EnvironmentState {
            warriors: self.warriors.values().cloned().collect(),
            resources: self.resources.clone(),
            territories: self.territories.clone(),
            tick: self.tick,
        }
    }
    
    pub fn get_statistics(&self) -> EnvironmentStats {
        let alive_warriors = self.warriors.len();
        let total_energy: f32 = self.warriors.values().map(|w| w.energy).sum();
        let average_age: f32 = if alive_warriors > 0 {
            self.warriors.values().map(|w| w.age as f32).sum::<f32>() / alive_warriors as f32
        } else {
            0.0
        };
        let max_lineage = self.warriors.values().map(|w| w.lineage_depth).max().unwrap_or(0);
        
        EnvironmentStats {
            tick: self.tick,
            alive_warriors,
            total_resources: self.resources.len(),
            total_energy,
            average_age,
            max_lineage_depth: max_lineage,
            environmental_pressure: self.environmental_pressure,
            carrying_capacity_usage: alive_warriors as f32 / self.carrying_capacity as f32,
        }
    }
    
    fn initialize_terrain(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Create memory barriers
        for _ in 0..10 {
            self.barriers.push(MemoryBarrier {
                position: (rng.gen_range(0.0..self.width), rng.gen_range(0.0..self.height)),
                width: rng.gen_range(20.0..100.0),
                height: rng.gen_range(20.0..100.0),
                strength: rng.gen_range(0.5..1.0),
            });
        }
        
        // Create safe zones
        for _ in 0..5 {
            self.safe_zones.push(SafeZone {
                center: (rng.gen_range(0.0..self.width), rng.gen_range(0.0..self.height)),
                radius: rng.gen_range(30.0..80.0),
                protection_level: rng.gen_range(0.7..1.0),
                resource_bonus: rng.gen_range(1.2..2.0),
            });
        }
        
        // Create territories
        for i in 0..15 {
            self.territories.push(Territory {
                center: (rng.gen_range(0.0..self.width), rng.gen_range(0.0..self.height)),
                radius: rng.gen_range(40.0..120.0),
                owner_id: None,
                resource_multiplier: rng.gen_range(0.8..1.5),
            });
        }
    }
    
    fn spawn_initial_resources(&mut self) {
        for _ in 0..100 {
            self.spawn_single_resource();
        }
    }
    
    fn should_spawn_resources(&self) -> bool {
        self.resource_spawn_timer > 10 && 
        self.resources.len() < self.resource_config.max_resources &&
        rand::random::<f32>() < self.resource_config.spawn_rate
    }
    
    fn spawn_resources(&mut self) -> usize {
        let spawn_count = rand::thread_rng().gen_range(1..=5);
        let mut spawned = 0;
        
        for _ in 0..spawn_count {
            if self.resources.len() < self.resource_config.max_resources {
                self.spawn_single_resource();
                spawned += 1;
            }
        }
        
        spawned
    }
    
    fn spawn_single_resource(&mut self) {
        let mut rng = rand::thread_rng();
        let position = (rng.gen_range(0.0..self.width), rng.gen_range(0.0..self.height));
        
        // Check if position is in a safe zone for bonus
        let mut energy_value = rng.gen_range(self.resource_config.energy_range.0..=self.resource_config.energy_range.1);
        let resource_type = if rng.gen_bool(0.7) {
            ResourceType::Energy
        } else if rng.gen_bool(0.5) {
            energy_value *= self.resource_config.computational_bonus;
            ResourceType::Computational
        } else {
            energy_value *= self.resource_config.territory_control_bonus;
            ResourceType::Territory
        };
        
        // Apply safe zone bonus
        for safe_zone in &self.safe_zones {
            let distance = ((position.0 - safe_zone.center.0).powi(2) + 
                           (position.1 - safe_zone.center.1).powi(2)).sqrt();
            if distance < safe_zone.radius {
                energy_value *= safe_zone.resource_bonus;
                break;
            }
        }
        
        self.resources.push(Resource {
            position,
            energy_value,
            resource_type,
        });
    }
    
    fn update_environmental_pressure(&mut self) {
        let population_ratio = self.warriors.len() as f32 / self.carrying_capacity as f32;
        let resource_scarcity = 1.0 - (self.resources.len() as f32 / self.resource_config.max_resources as f32);
        
        self.environmental_pressure = (population_ratio + resource_scarcity) / 2.0;
        self.environmental_pressure = self.environmental_pressure.clamp(0.0, 1.0);
    }
    
    fn decay_resources(&mut self) {
        // Remove resources that have been around too long or in low-activity areas
        self.resources.retain(|resource| {
            if rand::random::<f32>() < 0.002 {
                // Random decay
                false
            } else {
                // Check for nearby activity
                let nearby_warriors = self.warriors.values().any(|warrior| {
                    let distance = ((warrior.position.0 - resource.position.0).powi(2) + 
                                   (warrior.position.1 - resource.position.1).powi(2)).sqrt();
                    distance < 100.0
                });
                
                // Resources in active areas are more likely to persist
                nearby_warriors || rand::random::<f32>() < 0.99
            }
        });
    }
    
    fn update_territories(&mut self) {
        for territory in &mut self.territories {
            // Find warriors in territory
            let nearby_warriors: Vec<&NeuralWarrior> = self.warriors.values()
                .filter(|warrior| {
                    let distance = ((warrior.position.0 - territory.center.0).powi(2) + 
                                   (warrior.position.1 - territory.center.1).powi(2)).sqrt();
                    distance < territory.radius
                })
                .collect();
            
            // Determine territory control based on strongest presence
            if let Some(dominant_warrior) = nearby_warriors.iter()
                .max_by(|a, b| a.energy.partial_cmp(&b.energy).unwrap()) {
                territory.owner_id = Some(dominant_warrior.id);
            } else {
                territory.owner_id = None;
            }
        }
    }
    
    fn execute_action(&mut self, warrior_id: u32, action: Action) -> ActionResult {
        match action {
            Action::Move { direction, intensity } => {
                self.execute_move(warrior_id, direction, intensity)
            },
            Action::Attack { target_direction, strength } => {
                self.execute_attack(warrior_id, target_direction, strength)
            },
            Action::Defend { shield_strength } => {
                self.execute_defend(warrior_id, shield_strength)
            },
            Action::Replicate { mutation_rate } => {
                self.execute_replicate(warrior_id, mutation_rate)
            },
            Action::Sense { sensor_type } => {
                self.execute_sense(warrior_id, sensor_type)
            },
            Action::Rest => {
                if let Some(warrior) = self.warriors.get_mut(&warrior_id) {
                    warrior.gain_energy(2.0);
                    ActionResult::Success("Rested and recovered energy".to_string())
                } else {
                    ActionResult::Failed("Warrior not found".to_string())
                }
            },
        }
    }
    
    fn execute_move(&mut self, warrior_id: u32, direction: f32, intensity: f32) -> ActionResult {
        let move_distance = intensity * 10.0;
        let energy_cost = intensity * 2.0;
        
        let (new_x, new_y) = {
            let warrior = match self.warriors.get(&warrior_id) {
                Some(w) => w,
                None => return ActionResult::Failed("Warrior not found".to_string()),
            };
            
            if warrior.energy < energy_cost {
                return ActionResult::Failed("Insufficient energy for movement".to_string());
            }
            
            let new_x = (warrior.position.0 + direction.cos() * move_distance).clamp(0.0, self.width);
            let new_y = (warrior.position.1 + direction.sin() * move_distance).clamp(0.0, self.height);
            (new_x, new_y)
        };
        
        // Check for barriers
        for barrier in &self.barriers {
            if new_x >= barrier.position.0 && new_x <= barrier.position.0 + barrier.width &&
               new_y >= barrier.position.1 && new_y <= barrier.position.1 + barrier.height {
                if let Some(warrior) = self.warriors.get_mut(&warrior_id) {
                    warrior.consume_energy(energy_cost * barrier.strength);
                }
                return ActionResult::Partial("Movement blocked by barrier".to_string());
            }
        }
        
        if let Some(warrior) = self.warriors.get_mut(&warrior_id) {
            warrior.position = (new_x, new_y);
            warrior.consume_energy(energy_cost);
            ActionResult::Success(format!("Moved to ({:.1}, {:.1})", new_x, new_y))
        } else {
            ActionResult::Failed("Warrior not found".to_string())
        }
    }
    
    fn execute_attack(&mut self, attacker_id: u32, target_direction: f32, strength: f32) -> ActionResult {
        let attacker_pos = match self.warriors.get(&attacker_id) {
            Some(w) => w.position,
            None => return ActionResult::Failed("Attacker not found".to_string()),
        };
        
        let energy_cost = strength * 5.0;
        if let Some(attacker) = self.warriors.get_mut(&attacker_id) {
            if attacker.energy < energy_cost {
                return ActionResult::Failed("Insufficient energy for attack".to_string());
            }
            attacker.consume_energy(energy_cost);
        }
        
        // Find target in attack direction
        let attack_range = strength * 30.0;
        let target_x = attacker_pos.0 + target_direction.cos() * attack_range;
        let target_y = attacker_pos.1 + target_direction.sin() * attack_range;
        
        for (target_id, target) in self.warriors.iter_mut() {
            if *target_id == attacker_id {
                continue;
            }
            
            let distance = ((target.position.0 - target_x).powi(2) + 
                           (target.position.1 - target_y).powi(2)).sqrt();
            
            if distance < 20.0 {
                let damage = strength * 15.0;
                target.consume_energy(damage);
                return ActionResult::Success(format!("Hit target {} for {:.1} damage", target_id, damage));
            }
        }
        
        ActionResult::Failed("No target in range".to_string())
    }
    
    fn execute_defend(&mut self, warrior_id: u32, shield_strength: f32) -> ActionResult {
        let energy_cost = shield_strength * 3.0;
        
        if let Some(warrior) = self.warriors.get_mut(&warrior_id) {
            if warrior.energy < energy_cost {
                return ActionResult::Failed("Insufficient energy for defense".to_string());
            }
            
            warrior.consume_energy(energy_cost);
            ActionResult::Success(format!("Defending with {:.1} strength", shield_strength))
        } else {
            ActionResult::Failed("Warrior not found".to_string())
        }
    }
    
    fn execute_replicate(&mut self, parent_id: u32, mutation_rate: f32) -> ActionResult {
        let parent = match self.warriors.get(&parent_id) {
            Some(w) => w.clone(),
            None => return ActionResult::Failed("Parent not found".to_string()),
        };
        
        if !parent.can_replicate() {
            return ActionResult::Failed("Cannot replicate - insufficient energy or too young".to_string());
        }
        
        if self.warriors.len() >= self.carrying_capacity {
            return ActionResult::Failed("Environment at carrying capacity".to_string());
        }
        
        // Create offspring
        let mut child = parent.clone();
        child.id = rand::random();
        child.energy = parent.energy * 0.6; // Child gets part of parent's energy
        child.age = 0;
        child.fitness_score = 0.0;
        child.genome.mutate(mutation_rate);
        child.network = child.genome.to_network();
        child.lineage_depth = parent.lineage_depth + 1;
        
        // Consume parent energy
        if let Some(parent_mut) = self.warriors.get_mut(&parent_id) {
            parent_mut.consume_energy(40.0);
        }
        
        // Place child nearby
        let offset_distance = 20.0;
        let offset_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        child.position.0 = (child.position.0 + offset_angle.cos() * offset_distance).clamp(0.0, self.width);
        child.position.1 = (child.position.1 + offset_angle.sin() * offset_distance).clamp(0.0, self.height);
        
        let child_id = child.id;
        self.warriors.insert(child_id, child);
        
        ActionResult::Success(format!("Created offspring {}", child_id))
    }
    
    fn execute_sense(&mut self, _warrior_id: u32, _sensor_type: crate::neural::warrior::SensorType) -> ActionResult {
        // Sensing is passive and handled in the warrior's decision making
        ActionResult::Success("Sensed environment".to_string())
    }
    
    fn generate_environmental_event(&self) -> EnvironmentEvent {
        let mut rng = rand::thread_rng();
        let event_types = [
            EventType::ResourceScarcity,
            EventType::ResourceAbundance, 
            EventType::MemoryCompaction,
            EventType::TerritorialShift,
            EventType::PopulationPressure,
            EventType::EnergeticStorm,
        ];
        
        let event_type = event_types[rng.gen_range(0..event_types.len())];
        
        EnvironmentEvent {
            event_type,
            duration: rng.gen_range(5..20),
            intensity: rng.gen_range(0.3..0.8),
            affected_area: Some((
                rng.gen_range(0.0..self.width),
                rng.gen_range(0.0..self.height),
                rng.gen_range(50.0..200.0),
            )),
        }
    }
    
    fn apply_environmental_event(&mut self, event: &EnvironmentEvent) {
        match event.event_type {
            EventType::ResourceScarcity => {
                let remove_count = (self.resources.len() as f32 * event.intensity * 0.3) as usize;
                for _ in 0..remove_count {
                    if !self.resources.is_empty() {
                        let idx = rand::thread_rng().gen_range(0..self.resources.len());
                        self.resources.remove(idx);
                    }
                }
            },
            EventType::ResourceAbundance => {
                let spawn_count = (event.intensity * 20.0) as usize;
                for _ in 0..spawn_count {
                    if self.resources.len() < self.resource_config.max_resources {
                        self.spawn_single_resource();
                    }
                }
            },
            EventType::PopulationPressure => {
                for warrior in self.warriors.values_mut() {
                    warrior.consume_energy(event.intensity * 5.0);
                }
            },
            _ => {
                // Other events affect specific areas or have complex logic
            },
        }
    }
    
    fn process_combat(&mut self, _results: &mut ActionResults) {
        // Combat resolution happens during action execution
        // This could be expanded for more complex combat interactions
    }
    
    fn process_resource_collection(&mut self, results: &mut ActionResults) {
        let warrior_positions: Vec<(u32, (f32, f32))> = self.warriors.iter()
            .map(|(id, warrior)| (*id, warrior.position))
            .collect();
        
        for (warrior_id, position) in warrior_positions {
            // Find nearby resources
            let mut collected_resources = Vec::new();
            
            for (i, resource) in self.resources.iter().enumerate() {
                let distance = ((position.0 - resource.position.0).powi(2) + 
                               (position.1 - resource.position.1).powi(2)).sqrt();
                
                if distance < 15.0 {
                    collected_resources.push(i);
                    
                    if let Some(warrior) = self.warriors.get_mut(&warrior_id) {
                        warrior.gain_energy(resource.energy_value);
                        results.add_result(warrior_id, ActionResult::Success(
                            format!("Collected {} energy", resource.energy_value)
                        ));
                    }
                }
            }
            
            // Remove collected resources (in reverse order to maintain indices)
            for &index in collected_resources.iter().rev() {
                self.resources.remove(index);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentUpdate {
    pub tick: u64,
    pub resources_spawned: usize,
    pub warriors_died: usize,
    pub environmental_event: Option<EnvironmentEvent>,
}

impl EnvironmentUpdate {
    pub fn new(tick: u64) -> Self {
        Self {
            tick,
            resources_spawned: 0,
            warriors_died: 0,
            environmental_event: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionResults {
    pub results: HashMap<u32, ActionResult>,
}

impl ActionResults {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }
    
    pub fn add_result(&mut self, warrior_id: u32, result: ActionResult) {
        self.results.insert(warrior_id, result);
    }
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    Success(String),
    Partial(String),
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct EnvironmentStats {
    pub tick: u64,
    pub alive_warriors: usize,
    pub total_resources: usize,
    pub total_energy: f32,
    pub average_age: f32,
    pub max_lineage_depth: u32,
    pub environmental_pressure: f32,
    pub carrying_capacity_usage: f32,
}