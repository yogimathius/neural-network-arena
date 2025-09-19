use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OpCode {
    Activate,
    Mutate,
    Replicate,
    Move,
    Sense,
    Noop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub opcode: OpCode,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: f32,
}

impl Instruction {
    pub fn new(opcode: OpCode, arg1: usize, arg2: usize, arg3: impl Into<f32>) -> Self {
        Self {
            opcode,
            arg1,
            arg2,
            arg3: arg3.into(),
        }
    }

    pub fn cost(&self) -> u32 {
        match self.opcode {
            OpCode::Activate => 1,
            OpCode::Mutate => 5,
            OpCode::Replicate => 10,
            OpCode::Move => 2,
            OpCode::Sense => 1,
            OpCode::Noop => 0,
        }
    }
}
