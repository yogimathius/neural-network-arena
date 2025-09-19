use super::territory::{Territory, TerritoryError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemoryAllocator {
    #[allow(dead_code)]
    total_size: usize,
    territories: Vec<Territory>,
    owner_territories: HashMap<u32, Vec<usize>>,
    free_territories: Vec<usize>,
}

#[derive(Debug, thiserror::Error)]
pub enum AllocationError {
    #[error("Not enough free memory: requested {requested}, available {available}")]
    InsufficientMemory { requested: usize, available: usize },
    #[error("Territory error: {0}")]
    Territory(#[from] TerritoryError),
    #[error("Invalid territory ID: {id}")]
    InvalidTerritory { id: usize },
}

type AllocationResult<T> = Result<T, AllocationError>;

impl MemoryAllocator {
    pub fn new(total_size: usize, territory_size: usize) -> Self {
        let territory_count = total_size / territory_size;
        let mut territories = Vec::with_capacity(territory_count);
        let mut free_territories = Vec::with_capacity(territory_count);

        for i in 0..territory_count {
            territories.push(Territory::new(i * territory_size, territory_size));
            free_territories.push(i);
        }

        Self {
            total_size,
            territories,
            owner_territories: HashMap::new(),
            free_territories,
        }
    }

    pub fn allocate_territory(&mut self, owner_id: u32) -> AllocationResult<usize> {
        let territory_id =
            self.free_territories
                .pop()
                .ok_or(AllocationError::InsufficientMemory {
                    requested: 1,
                    available: 0,
                })?;

        self.territories[territory_id].allocate_to(owner_id)?;

        self.owner_territories
            .entry(owner_id)
            .or_default()
            .push(territory_id);

        Ok(territory_id)
    }

    pub fn deallocate_territory(
        &mut self,
        territory_id: usize,
        owner_id: u32,
    ) -> AllocationResult<()> {
        if territory_id >= self.territories.len() {
            return Err(AllocationError::InvalidTerritory { id: territory_id });
        }

        let territory = &mut self.territories[territory_id];
        if territory.owner() != Some(owner_id) {
            return Err(AllocationError::Territory(TerritoryError::AccessDenied));
        }

        territory.release();
        self.free_territories.push(territory_id);

        if let Some(owner_territories) = self.owner_territories.get_mut(&owner_id) {
            owner_territories.retain(|&id| id != territory_id);
        }

        Ok(())
    }

    pub fn can_access(&self, address: usize, requester_id: u32) -> bool {
        self.find_territory_for_address(address)
            .map(|territory| territory.can_access(requester_id))
            .unwrap_or(false)
    }

    pub fn get_territory(&self, territory_id: usize) -> Option<&Territory> {
        self.territories.get(territory_id)
    }

    pub fn get_territories_for_owner(&self, owner_id: u32) -> Vec<&Territory> {
        self.owner_territories
            .get(&owner_id)
            .map(|ids| ids.iter().map(|&id| &self.territories[id]).collect())
            .unwrap_or_default()
    }

    pub fn available_territories(&self) -> usize {
        self.free_territories.len()
    }

    pub fn total_territories(&self) -> usize {
        self.territories.len()
    }

    pub fn memory_utilization(&self) -> f32 {
        let used = self.territories.len() - self.free_territories.len();
        used as f32 / self.territories.len() as f32
    }

    fn find_territory_for_address(&self, address: usize) -> Option<&Territory> {
        self.territories
            .iter()
            .find(|t| t.contains_address(address))
    }
}
