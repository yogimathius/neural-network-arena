use neural_network_arena::vm::{Instruction, OpCode, VirtualMachine};
use std::collections::HashMap;

#[test]
fn test_vm_creation() {
    let vm = VirtualMachine::new(1024);
    assert_eq!(vm.memory_size(), 1024);
    assert_eq!(vm.cycle_count(), 0);
}

#[test]
fn test_neural_activate_instruction() {
    let mut vm = VirtualMachine::new(1024);
    let instruction = Instruction::new(OpCode::Activate, 0, 1, 2.0);

    let result = vm.execute_instruction(&instruction);
    assert!(result.is_ok());
    assert_eq!(vm.cycle_count(), 1);
}

#[test]
fn test_neural_mutate_instruction() {
    let mut vm = VirtualMachine::new(1024);
    let instruction = Instruction::new(OpCode::Mutate, 0, 1, 0.1);

    let result = vm.execute_instruction(&instruction);
    assert!(result.is_ok());
}

#[test]
fn test_neural_replicate_instruction() {
    let mut vm = VirtualMachine::new(1024);
    let instruction = Instruction::new(OpCode::Replicate, 0, 1, 2.0);

    let result = vm.execute_instruction(&instruction);
    assert!(result.is_ok());
}

#[test]
fn test_memory_bounds_checking() {
    let mut vm = VirtualMachine::new(10);
    let instruction = Instruction::new(OpCode::Activate, 15, 20, 0.0);

    let result = vm.execute_instruction(&instruction);
    assert!(result.is_err());
}

#[test]
fn test_resource_consumption_tracking() {
    let mut vm = VirtualMachine::new(1024);
    let initial_resources = vm.available_resources();

    let instruction = Instruction::new(OpCode::Activate, 0, 1, 2.0);
    vm.execute_instruction(&instruction).unwrap();

    assert!(vm.available_resources() < initial_resources);
}

#[test]
fn test_round_robin_execution() {
    let mut vm = VirtualMachine::new(1024);

    let program1 = vec![
        Instruction::new(OpCode::Activate, 0, 1, 2.0),
        Instruction::new(OpCode::Mutate, 1, 2, 0.1),
    ];

    let program2 = vec![
        Instruction::new(OpCode::Replicate, 0, 1, 2.0),
        Instruction::new(OpCode::Activate, 2, 3, 4.0),
    ];

    vm.load_program(0, program1).unwrap();
    vm.load_program(1, program2).unwrap();

    vm.execute_round_robin_cycle().unwrap();

    assert_eq!(vm.cycle_count(), 2);
}

#[test]
fn test_memory_territory_allocation() {
    let mut vm = VirtualMachine::new(1024);
    
    // Test allocating territory for program 0
    let territory_size = 256;
    let territory_id = vm.allocate_territory(0, territory_size).unwrap();
    
    // Territory should be allocated
    assert!(vm.has_territory(territory_id));
    assert_eq!(vm.territory_size(territory_id).unwrap(), territory_size);
    assert_eq!(vm.territory_owner(territory_id).unwrap(), 0);
    
    // Test memory access within territory bounds
    let territory_start = vm.territory_start_address(territory_id).unwrap();
    vm.write_territory_memory(territory_id, 0, 42.0).unwrap();
    assert_eq!(vm.read_territory_memory(territory_id, 0).unwrap(), 42.0);
    
    // Test bounds checking - should fail to access outside territory
    assert!(vm.write_territory_memory(territory_id, territory_size, 100.0).is_err());
    assert!(vm.read_territory_memory(territory_id, territory_size).is_err());
}

#[test]
fn test_memory_territory_isolation() {
    let mut vm = VirtualMachine::new(1024);
    
    // Allocate territories for two different programs
    let territory1 = vm.allocate_territory(0, 128).unwrap();
    let territory2 = vm.allocate_territory(1, 128).unwrap();
    
    // Program 0 should not be able to access program 1's territory
    vm.write_territory_memory(territory1, 0, 100.0).unwrap();
    vm.write_territory_memory(territory2, 0, 200.0).unwrap();
    
    // Each program should only see its own data
    assert_eq!(vm.read_territory_memory(territory1, 0).unwrap(), 100.0);
    assert_eq!(vm.read_territory_memory(territory2, 0).unwrap(), 200.0);
    
    // Cross-territory access should be denied
    assert!(vm.cross_territory_access_denied(territory1, territory2));
}
