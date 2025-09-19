import { SimulationConfig, WasmSimulation, SimulationState, VisualizationSettings } from '../types/simulation';
import { MainRenderer } from './MainRenderer';
import { HeatmapRenderer } from './HeatmapRenderer';
import { NetworkRenderer } from './NetworkRenderer';
import { ControlPanel } from './ControlPanel';
import { PerformanceMonitor } from '../utils/PerformanceMonitor';

export class Application {
    private wasmModule: any;
    private simulation: WasmSimulation | null = null;
    private mainRenderer: MainRenderer;
    private heatmapRenderer: HeatmapRenderer;
    private networkRenderer: NetworkRenderer;
    private controlPanel: ControlPanel;
    private performanceMonitor: PerformanceMonitor;
    
    private isRunning: boolean = false;
    private animationId: number | null = null;
    private lastFrameTime: number = 0;
    private frameCount: number = 0;
    
    private settings: VisualizationSettings = {
        showHeatmap: false,
        showTopology: false,
        showTerritories: true,
        showResources: true,
        showTrails: false,
        simulationSpeed: 1.0,
    };

    constructor(private config: SimulationConfig) {
        // Get canvas elements
        const mainCanvas = document.getElementById('mainCanvas') as HTMLCanvasElement;
        const heatmapCanvas = document.getElementById('heatmapCanvas') as HTMLCanvasElement;
        const networkCanvas = document.getElementById('networkCanvas') as HTMLCanvasElement;
        
        // Initialize renderers
        this.mainRenderer = new MainRenderer(mainCanvas);
        this.heatmapRenderer = new HeatmapRenderer(heatmapCanvas);
        this.networkRenderer = new NetworkRenderer(networkCanvas);
        this.controlPanel = new ControlPanel();
        this.performanceMonitor = new PerformanceMonitor();
        
        this.setupEventListeners();
    }

    async initialize(): Promise<void> {
        try {
            console.log('🔄 Loading WebAssembly module...');
            
            // Import the WebAssembly module (this path will be updated after wasm-pack build)
            this.wasmModule = await import('../../pkg');
            
            console.log('✅ WebAssembly module loaded');
            
            // Create simulation instance
            const configJson = JSON.stringify(this.config);
            this.simulation = new this.wasmModule.WasmSimulation(configJson);
            
            // Initialize population
            this.simulation?.initialize_population(100);
            
            console.log('✅ Simulation initialized');
            
            // Setup canvas sizes
            this.setupCanvases();
            
            // Start render loop
            this.startRenderLoop();
            
        } catch (error) {
            console.error('Failed to initialize WebAssembly:', error);
            throw new Error(`WebAssembly initialization failed: ${error}`);
        }
    }

    private setupCanvases(): void {
        const container = document.querySelector('.visualization-area') as HTMLElement;
        const width = container.clientWidth;
        const height = container.clientHeight;
        
        // Setup main canvas
        this.mainRenderer.resize(width, height);
        
        // Setup heatmap canvas
        this.heatmapRenderer.resize(width, height);
        this.heatmapRenderer.hide();
        
        // Setup network canvas
        this.networkRenderer.resize(width, height);
        this.networkRenderer.hide();
        
        console.log(`📐 Canvases initialized: ${width}x${height}`);
    }

    private setupEventListeners(): void {
        // Window resize
        window.addEventListener('resize', () => {
            this.setupCanvases();
        });
        
        // Control panel events
        this.controlPanel.onStart(() => this.start());
        this.controlPanel.onPause(() => this.pause());
        this.controlPanel.onReset(() => this.reset());
        this.controlPanel.onStep(() => this.step());
        this.controlPanel.onGeneration(() => this.runGeneration());
        
        this.controlPanel.onSpeedChange((speed) => {
            this.settings.simulationSpeed = speed;
        });
        
        // Visualization toggles
        this.controlPanel.onHeatmapToggle(() => this.toggleHeatmap());
        this.controlPanel.onTopologyToggle(() => this.toggleTopology());
        this.controlPanel.onTerritoriesToggle(() => this.toggleTerritories());
        this.controlPanel.onResourcesToggle(() => this.toggleResources());
        
        // Export functions
        this.controlPanel.onExportJson(() => this.exportData('json'));
        this.controlPanel.onExportCsv(() => this.exportData('csv'));
    }

    private startRenderLoop(): void {
        const render = (currentTime: number) => {
            this.performanceMonitor.frameStart();
            
            // Update simulation if running
            if (this.isRunning && this.simulation) {
                const deltaTime = currentTime - this.lastFrameTime;
                const targetFrameTime = 1000 / (60 * this.settings.simulationSpeed);
                
                if (deltaTime >= targetFrameTime) {
                    const state = this.simulation.step();
                    this.updateVisualization(state);
                    this.updateUI(state);
                    this.lastFrameTime = currentTime;
                }
            }
            
            this.performanceMonitor.frameEnd();
            this.frameCount++;
            
            // Update FPS display
            if (this.frameCount % 60 === 0) {
                const fps = this.performanceMonitor.getFPS();
                document.getElementById('fps')!.textContent = fps.toFixed(1);
            }
            
            this.animationId = requestAnimationFrame(render);
        };
        
        this.animationId = requestAnimationFrame(render);
    }

    private updateVisualization(state: any): void {
        if (!state) return;
        
        try {
            const simulationState = state as SimulationState;
            
            // Always render main simulation view
            this.mainRenderer.render(simulationState, this.settings);
            
            // Render additional overlays if enabled
            if (this.settings.showHeatmap && this.simulation) {
                const heatmapData = this.simulation.get_memory_heatmap();
                this.heatmapRenderer.render(heatmapData);
            }
            
            if (this.settings.showTopology && this.simulation && simulationState.warriors.length > 0) {
                const topologyData = this.simulation.get_network_topology(simulationState.warriors[0].id);
                this.networkRenderer.render(topologyData);
            }
            
        } catch (error) {
            console.warn('Visualization update error:', error);
        }
    }

    private updateUI(state: any): void {
        if (!state) return;
        
        try {
            const simulationState = state as SimulationState;
            
            // Update header stats
            document.getElementById('generation')!.textContent = simulationState.generation.toString();
            document.getElementById('population')!.textContent = simulationState.population_size.toString();
            document.getElementById('species')!.textContent = simulationState.species_count.toString();
            
            // Update metrics
            document.getElementById('avgFitness')!.textContent = simulationState.average_fitness.toFixed(1);
            document.getElementById('maxFitness')!.textContent = simulationState.max_fitness.toFixed(1);
            document.getElementById('diversity')!.textContent = simulationState.diversity_score.toFixed(2);
            document.getElementById('pressure')!.textContent = simulationState.environmental_pressure.toFixed(2);
            
        } catch (error) {
            console.warn('UI update error:', error);
        }
    }

    // Control methods
    public start(): void {
        if (this.simulation) {
            this.simulation.start();
            this.isRunning = true;
            console.log('▶️ Simulation started');
        }
    }

    public pause(): void {
        if (this.simulation) {
            this.simulation.pause();
            this.isRunning = false;
            console.log('⏸️ Simulation paused');
        }
    }

    public reset(): void {
        if (this.simulation) {
            this.simulation.reset();
            this.simulation.initialize_population(100);
            this.isRunning = false;
            console.log('🔄 Simulation reset');
        }
    }

    public step(): void {
        if (this.simulation) {
            const state = this.simulation.step();
            this.updateVisualization(state);
            this.updateUI(state);
            console.log('⏭️ Single step executed');
        }
    }

    public runGeneration(): void {
        if (this.simulation) {
            const state = this.simulation.run_generation();
            this.updateVisualization(state);
            this.updateUI(state);
            console.log('🧬 Generation completed');
        }
    }

    // Visualization toggles
    public toggleHeatmap(): void {
        this.settings.showHeatmap = !this.settings.showHeatmap;
        if (this.settings.showHeatmap) {
            this.heatmapRenderer.show();
            this.settings.showTopology = false;
            this.networkRenderer.hide();
        } else {
            this.heatmapRenderer.hide();
        }
        console.log('🔥 Heatmap:', this.settings.showHeatmap ? 'ON' : 'OFF');
    }

    public toggleTopology(): void {
        this.settings.showTopology = !this.settings.showTopology;
        if (this.settings.showTopology) {
            this.networkRenderer.show();
            this.settings.showHeatmap = false;
            this.heatmapRenderer.hide();
        } else {
            this.networkRenderer.hide();
        }
        console.log('🕸️ Network topology:', this.settings.showTopology ? 'ON' : 'OFF');
    }

    public toggleTerritories(): void {
        this.settings.showTerritories = !this.settings.showTerritories;
        console.log('🗺️ Territories:', this.settings.showTerritories ? 'ON' : 'OFF');
    }

    public toggleResources(): void {
        this.settings.showResources = !this.settings.showResources;
        console.log('💎 Resources:', this.settings.showResources ? 'ON' : 'OFF');
    }

    // Export functionality
    public exportData(format: string): void {
        if (this.simulation) {
            const data = this.simulation.export_data(format);
            
            const blob = new Blob([data], { 
                type: format === 'json' ? 'application/json' : 'text/csv' 
            });
            const url = URL.createObjectURL(blob);
            
            const link = document.createElement('a');
            link.href = url;
            link.download = `neural-arena-data.${format}`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);
            
            console.log(`💾 Data exported as ${format.toUpperCase()}`);
        }
    }

    public destroy(): void {
        if (this.animationId) {
            cancelAnimationFrame(this.animationId);
        }
        this.isRunning = false;
        console.log('🗑️ Application destroyed');
    }
}