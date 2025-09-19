// TypeScript type definitions for WebAssembly simulation interface

export interface SimulationConfig {
  max_population: number;
  vm_memory_size: number;
  territory_size: number;
  target_species_count: number;
  mutation_rate: number;
  survival_threshold: number;
  fitness_sharing: boolean;
  elitism_rate: number;
  tournament_size: number;
  max_generations: number;
  performance_target_rps: number;
}

export interface WarriorData {
  id: number;
  x: number;
  y: number;
  energy: number;
  age: number;
  fitness: number;
  lineage_depth: number;
  species_id?: number;
  action: string;
}

export interface ResourceData {
  x: number;
  y: number;
  energy_value: number;
  resource_type: string;
}

export interface TerritoryData {
  center_x: number;
  center_y: number;
  radius: number;
  owner_id?: number;
  resource_multiplier: number;
}

export interface SimulationState {
  warriors: WarriorData[];
  resources: ResourceData[];
  territories: TerritoryData[];
  generation: number;
  tick: number;
  population_size: number;
  species_count: number;
  average_fitness: number;
  max_fitness: number;
  diversity_score: number;
  environmental_pressure: number;
}

export interface MemoryHeatmapData {
  width: number;
  height: number;
  data: number[]; // Flattened 2D array of memory usage intensities (0.0 to 1.0)
}

export interface NetworkNode {
  id: number;
  x: number;
  y: number;
  activation: number;
  node_type: string; // "input", "hidden", "output"
}

export interface NetworkConnection {
  from: number;
  to: number;
  weight: number;
}

export interface NetworkTopologyData {
  nodes: NetworkNode[];
  connections: NetworkConnection[];
}

export interface PerformanceMetrics {
  generation: number;
  tick: number;
  population_size: number;
  species_count: number;
  average_fitness: number;
  max_fitness: number;
  diversity_score: number;
  survival_rate: number;
  average_age: number;
  max_lineage_depth: number;
  computational_efficiency: number;
  rounds_per_second: number;
  resource_utilization: number;
  environmental_pressure: number;
}

export interface VisualizationSettings {
  showHeatmap: boolean;
  showTopology: boolean;
  showTerritories: boolean;
  showResources: boolean;
  showTrails: boolean;
  simulationSpeed: number;
}

// WebAssembly module interface
export interface WasmSimulation {
  new(config_json: string): WasmSimulation;
  initialize_population(size: number): void;
  start(): void;
  pause(): void;
  reset(): void;
  step(): any;
  run_generation(): any;
  get_simulation_state_json(): string;
  get_memory_heatmap(): any;
  get_network_topology(warrior_id: number): any;
  get_performance_metrics(): any;
  export_data(format: string): string;
  is_running(): boolean;
  get_generation(): number;
  get_tick(): number;
}