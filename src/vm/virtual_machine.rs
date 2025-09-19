use super::instruction::{Instruction, OpCode};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryTerritory {
    id: usize,
    owner_program: usize,
    start_address: usize,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    memory: Vec<f32>,
    memory_size: usize,
    cycle_count: u64,
    available_resources: u32,
    programs: HashMap<usize, Vec<Instruction>>,
    program_counters: HashMap<usize, usize>,
    current_program: usize,
    territories: HashMap<usize, MemoryTerritory>,
    next_territory_id: usize,
    allocated_memory: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum VmError {
    #[error("Memory access out of bounds: index {index}, size {size}")]
    OutOfBounds { index: usize, size: usize },
    #[error("Insufficient resources: required {required}, available {available}")]
    InsufficientResources { required: u32, available: u32 },
    #[error("Program {id} not found")]
    ProgramNotFound { id: usize },
    #[error("Territory {id} not found")]
    TerritoryNotFound { id: usize },
    #[error("Insufficient memory for territory allocation: requested {requested}, available {available}")]
    InsufficientMemory { requested: usize, available: usize },
    #[error("Territory access out of bounds: offset {offset}, territory size {size}")]
    TerritoryBoundsViolation { offset: usize, size: usize },
}

type VmResult<T> = Result<T, VmError>;

impl VirtualMachine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: vec![0.0; memory_size],
            memory_size,
            cycle_count: 0,
            available_resources: 10000,
            programs: HashMap::new(),
            program_counters: HashMap::new(),
            current_program: 0,
            territories: HashMap::new(),
            next_territory_id: 0,
            allocated_memory: 0,
        }
    }

    pub fn memory_size(&self) -> usize {
        self.memory_size
    }

    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    pub fn available_resources(&self) -> u32 {
        self.available_resources
    }

    pub fn load_program(&mut self, id: usize, program: Vec<Instruction>) -> VmResult<()> {
        self.programs.insert(id, program);
        self.program_counters.insert(id, 0);
        Ok(())
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) -> VmResult<()> {
        let cost = instruction.cost();
        if self.available_resources < cost {
            return Err(VmError::InsufficientResources {
                required: cost,
                available: self.available_resources,
            });
        }

        if instruction.arg1 >= self.memory_size || instruction.arg2 >= self.memory_size {
            return Err(VmError::OutOfBounds {
                index: instruction.arg1.max(instruction.arg2),
                size: self.memory_size,
            });
        }

        match instruction.opcode {
            OpCode::Activate => self.execute_activate(instruction),
            OpCode::Mutate => self.execute_mutate(instruction),
            OpCode::Replicate => self.execute_replicate(instruction),
            OpCode::Move => self.execute_move(instruction),
            OpCode::Sense => self.execute_sense(instruction),
            OpCode::Noop => Ok(()),
        }?;

        self.available_resources -= cost;
        self.cycle_count += 1;
        Ok(())
    }

    pub fn execute_round_robin_cycle(&mut self) -> VmResult<()> {
        if self.programs.is_empty() {
            return Ok(());
        }

        let program_ids: Vec<usize> = self.programs.keys().cloned().collect();

        for &program_id in &program_ids {
            if let Some(program) = self.programs.get(&program_id).cloned() {
                let pc = *self.program_counters.get(&program_id).unwrap_or(&0);

                if pc < program.len() {
                    self.current_program = program_id;
                    self.execute_instruction(&program[pc])?;
                    self.program_counters.insert(program_id, pc + 1);
                }
            }
        }

        Ok(())
    }

    pub fn execute_single_cycle(&mut self, _vm: &mut VirtualMachine) {
        if self.execute_round_robin_cycle().is_err() {}
    }

    fn execute_activate(&mut self, instruction: &Instruction) -> VmResult<()> {
        let input = self.memory[instruction.arg1];
        let output = self.activation_function(input);
        self.memory[instruction.arg2] = output;
        Ok(())
    }

    fn execute_mutate(&mut self, instruction: &Instruction) -> VmResult<()> {
        let mutation_rate = instruction.arg3;
        let current_value = self.memory[instruction.arg1];
        let mutation = (rand::random::<f32>() - 0.5) * mutation_rate;
        self.memory[instruction.arg2] = (current_value + mutation).clamp(-1.0, 1.0);
        Ok(())
    }

    fn execute_replicate(&mut self, instruction: &Instruction) -> VmResult<()> {
        let source_value = self.memory[instruction.arg1];
        self.memory[instruction.arg2] = source_value;
        Ok(())
    }

    fn execute_move(&mut self, _instruction: &Instruction) -> VmResult<()> {
        Ok(())
    }

    fn execute_sense(&mut self, instruction: &Instruction) -> VmResult<()> {
        let sensor_value = self.get_sensor_data(instruction.arg1)?;
        self.memory[instruction.arg2] = sensor_value;
        Ok(())
    }

    fn activation_function(&self, x: f32) -> f32 {
        (2.0 / (1.0 + (-2.0 * x).exp())) - 1.0
    }

    fn get_sensor_data(&self, sensor_id: usize) -> VmResult<f32> {
        match sensor_id {
            0 => Ok(self.available_resources as f32 / 10000.0),
            1 => Ok(self.memory_size as f32 / 1024.0),
            _ => Ok(rand::random::<f32>()),
        }
    }

    // Territory Management Methods
    pub fn allocate_territory(&mut self, owner_program: usize, size: usize) -> VmResult<usize> {
        // Check if we have enough available memory
        let available_memory = self.memory_size - self.allocated_memory;
        if size > available_memory {
            return Err(VmError::InsufficientMemory { 
                requested: size, 
                available: available_memory 
            });
        }

        // Create new territory
        let territory_id = self.next_territory_id;
        let territory = MemoryTerritory {
            id: territory_id,
            owner_program,
            start_address: self.allocated_memory,
            size,
        };

        // Update allocations
        self.territories.insert(territory_id, territory);
        self.allocated_memory += size;
        self.next_territory_id += 1;

        Ok(territory_id)
    }

    pub fn has_territory(&self, territory_id: usize) -> bool {
        self.territories.contains_key(&territory_id)
    }

    pub fn territory_size(&self, territory_id: usize) -> VmResult<usize> {
        self.territories.get(&territory_id)
            .map(|t| t.size)
            .ok_or(VmError::TerritoryNotFound { id: territory_id })
    }

    pub fn territory_owner(&self, territory_id: usize) -> VmResult<usize> {
        self.territories.get(&territory_id)
            .map(|t| t.owner_program)
            .ok_or(VmError::TerritoryNotFound { id: territory_id })
    }

    pub fn territory_start_address(&self, territory_id: usize) -> VmResult<usize> {
        self.territories.get(&territory_id)
            .map(|t| t.start_address)
            .ok_or(VmError::TerritoryNotFound { id: territory_id })
    }

    pub fn write_territory_memory(&mut self, territory_id: usize, offset: usize, value: f32) -> VmResult<()> {
        let territory = self.territories.get(&territory_id)
            .ok_or(VmError::TerritoryNotFound { id: territory_id })?;

        // Check bounds
        if offset >= territory.size {
            return Err(VmError::TerritoryBoundsViolation { 
                offset, 
                size: territory.size 
            });
        }

        // Write to memory
        let memory_index = territory.start_address + offset;
        self.memory[memory_index] = value;
        Ok(())
    }

    pub fn read_territory_memory(&self, territory_id: usize, offset: usize) -> VmResult<f32> {
        let territory = self.territories.get(&territory_id)
            .ok_or(VmError::TerritoryNotFound { id: territory_id })?;

        // Check bounds
        if offset >= territory.size {
            return Err(VmError::TerritoryBoundsViolation { 
                offset, 
                size: territory.size 
            });
        }

        // Read from memory
        let memory_index = territory.start_address + offset;
        Ok(self.memory[memory_index])
    }

    pub fn cross_territory_access_denied(&self, territory1: usize, territory2: usize) -> bool {
        // Different territories should be isolated from each other
        match (self.territories.get(&territory1), self.territories.get(&territory2)) {
            (Some(t1), Some(t2)) => t1.owner_program != t2.owner_program,
            _ => true, // If either territory doesn't exist, deny access
        }
    }
}
