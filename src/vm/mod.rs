pub mod instruction;
pub mod virtual_machine;

pub use instruction::{Instruction, OpCode};
pub use virtual_machine::VirtualMachine;
