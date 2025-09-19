use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Territory {
    start_address: usize,
    size: usize,
    owner_id: Option<u32>,
    resource_density: f32,
    protection_level: u8,
}

impl Territory {
    pub fn new(start_address: usize, size: usize) -> Self {
        Self {
            start_address,
            size,
            owner_id: None,
            resource_density: rand::random::<f32>(),
            protection_level: 0,
        }
    }

    pub fn allocate_to(&mut self, owner_id: u32) -> Result<(), TerritoryError> {
        if self.owner_id.is_some() {
            return Err(TerritoryError::AlreadyOwned);
        }
        self.owner_id = Some(owner_id);
        Ok(())
    }

    pub fn release(&mut self) {
        self.owner_id = None;
        self.protection_level = 0;
    }

    pub fn contains_address(&self, address: usize) -> bool {
        address >= self.start_address && address < self.start_address + self.size
    }

    pub fn can_access(&self, requester_id: u32) -> bool {
        match self.owner_id {
            None => true,
            Some(owner) => owner == requester_id || self.protection_level == 0,
        }
    }

    pub fn start_address(&self) -> usize {
        self.start_address
    }

    pub fn end_address(&self) -> usize {
        self.start_address + self.size
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn owner(&self) -> Option<u32> {
        self.owner_id
    }

    pub fn resource_density(&self) -> f32 {
        self.resource_density
    }

    pub fn set_protection_level(&mut self, level: u8) {
        self.protection_level = level.min(3);
    }

    pub fn protection_level(&self) -> u8 {
        self.protection_level
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TerritoryError {
    #[error("Territory is already owned")]
    AlreadyOwned,
    #[error("Access denied to territory")]
    AccessDenied,
}
