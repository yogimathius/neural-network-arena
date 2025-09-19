use neural_network_arena::neural::{NeuralWarrior, Genome};
use neural_network_arena::neural::warrior::SensorType;
use neural_network_arena::environment::Environment;

#[test]
fn test_all_eight_sensors_exist() {
    // Test that all 8 required sensor types exist (MVP requirement)
    let sensor_types = vec![
        SensorType::Energy,
        SensorType::NeighborProximity,
        SensorType::ResourceDensity,
        SensorType::TerritoryPressure,
        SensorType::Population,
        SensorType::Threat,
        SensorType::Age,
        SensorType::LineageDepth,
    ];
    
    assert_eq!(sensor_types.len(), 8, "MVP requires exactly 8 sensor inputs");
}

#[test]
fn test_warrior_sensor_readings() {
    let mut environment = Environment::new(1000.0, 1000.0, 100);
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    
    // Place warrior in environment
    warrior.position = (100.0, 100.0);
    warrior.energy = 75.0;
    warrior.age = 50;
    
    let warrior_id = warrior.id;
    environment.add_warrior(warrior);
    
    // Get warrior back from environment for sensor testing
    let warrior = environment.warriors.get(&warrior_id).unwrap();
    
    // Test that each sensor returns a valid reading (0.0 to 1.0)
    let sensor_types = [
        SensorType::Energy,
        SensorType::NeighborProximity,
        SensorType::ResourceDensity,
        SensorType::TerritoryPressure,
        SensorType::Population,
        SensorType::Threat,
        SensorType::Age,
        SensorType::LineageDepth,
    ];
    
    for sensor_type in &sensor_types {
        let reading = warrior.get_sensor_reading(*sensor_type, &environment);
        assert!(reading >= 0.0 && reading <= 1.0, 
               "Sensor {:?} reading {} out of bounds [0.0, 1.0]", sensor_type, reading);
    }
}

#[test]
fn test_energy_sensor_accuracy() {
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    let environment = Environment::new(1000.0, 1000.0, 100);
    
    // Test different energy levels
    warrior.energy = 0.0;
    let reading_zero = warrior.get_sensor_reading(SensorType::Energy, &environment);
    assert_eq!(reading_zero, 0.0);
    
    warrior.energy = 100.0;
    let reading_full = warrior.get_sensor_reading(SensorType::Energy, &environment);
    assert_eq!(reading_full, 1.0);
    
    warrior.energy = 50.0;
    let reading_half = warrior.get_sensor_reading(SensorType::Energy, &environment);
    assert_eq!(reading_half, 0.5);
}

#[test]
fn test_neighbor_proximity_sensor() {
    let mut environment = Environment::new(1000.0, 1000.0, 100);
    let genome = Genome::new_random();
    
    // Create first warrior
    let mut warrior1 = NeuralWarrior::new(genome.clone(), 1);
    warrior1.position = (100.0, 100.0);
    
    // Create second warrior nearby
    let mut warrior2 = NeuralWarrior::new(genome, 2);
    warrior2.position = (110.0, 100.0); // 10 units away
    
    environment.add_warrior(warrior1.clone());
    environment.add_warrior(warrior2);
    
    // Test proximity sensor - should detect nearby warrior
    let proximity_reading = warrior1.get_sensor_reading(SensorType::NeighborProximity, &environment);
    
    // Should be high (close to 1.0) when neighbor is nearby
    assert!(proximity_reading > 0.5, "Should detect nearby warrior");
    
    // Test when alone
    let mut environment_alone = Environment::new(1000.0, 1000.0, 100);
    environment_alone.add_warrior(warrior1.clone());
    
    let alone_reading = warrior1.get_sensor_reading(SensorType::NeighborProximity, &environment_alone);
    assert!(alone_reading < 0.1, "Should read low when alone");
}

#[test]
fn test_age_sensor() {
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    let environment = Environment::new(1000.0, 1000.0, 100);
    
    // Test young warrior
    warrior.age = 0;
    let young_reading = warrior.get_sensor_reading(SensorType::Age, &environment);
    assert!(young_reading < 0.1);
    
    // Test older warrior
    warrior.age = 1000; // Assuming max age around 1000
    let old_reading = warrior.get_sensor_reading(SensorType::Age, &environment);
    assert!(old_reading > 0.8);
}

#[test]
fn test_lineage_depth_sensor() {
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    let environment = Environment::new(1000.0, 1000.0, 100);
    
    // Test first generation
    warrior.lineage_depth = 0;
    let first_gen_reading = warrior.get_sensor_reading(SensorType::LineageDepth, &environment);
    assert_eq!(first_gen_reading, 0.0);
    
    // Test deeper lineage
    warrior.lineage_depth = 10;
    let deep_reading = warrior.get_sensor_reading(SensorType::LineageDepth, &environment);
    assert!(deep_reading > 0.0);
}

#[test]
fn test_sensor_readings_change_with_environment() {
    let mut environment = Environment::new(1000.0, 1000.0, 100);
    let genome = Genome::new_random();
    let mut warrior = NeuralWarrior::new(genome, 1);
    
    // Position the test warrior where it can detect the others
    warrior.position = (50.0, 50.0);
    
    environment.add_warrior(warrior.clone());
    
    // Get initial readings
    let initial_population = warrior.get_sensor_reading(SensorType::Population, &environment);
    let initial_resource = warrior.get_sensor_reading(SensorType::ResourceDensity, &environment);
    
    // Add more warriors nearby to increase population density
    for i in 2..=10 {
        let genome = Genome::new_random();
        let mut new_warrior = NeuralWarrior::new(genome, i);
        // Place warriors close to the first warrior for population detection
        new_warrior.position = (50.0 + (i as f32 * 10.0), 50.0);
        environment.add_warrior(new_warrior);
    }
    
    // Population sensor should detect increased population
    let new_population = warrior.get_sensor_reading(SensorType::Population, &environment);
    assert!(new_population > initial_population, 
           "Population sensor should increase with more warriors");
    
    // Sensors should return valid values
    assert!(new_population >= 0.0 && new_population <= 1.0);
    assert!(initial_resource >= 0.0 && initial_resource <= 1.0);
}