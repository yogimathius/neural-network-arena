pub mod evolution;
pub mod memory;
pub mod neural;
pub mod vm;
pub mod environment;
pub mod simulation;
pub mod wasm_api;

pub use vm::VirtualMachine;
pub use environment::Environment;
pub use simulation::{NeuralArenaSimulation, SimulationConfig};
