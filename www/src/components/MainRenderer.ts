import { SimulationState, VisualizationSettings, WarriorData, ResourceData, TerritoryData } from '../types/simulation';

export class MainRenderer {
    private ctx: CanvasRenderingContext2D;
    private canvas: HTMLCanvasElement;
    private width: number = 0;
    private height: number = 0;
    
    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        const context = canvas.getContext('2d');
        if (!context) {
            throw new Error('Failed to get 2D rendering context');
        }
        this.ctx = context;
        
        // Enable high DPI rendering
        const dpr = window.devicePixelRatio || 1;
        const rect = canvas.getBoundingClientRect();
        
        canvas.width = rect.width * dpr;
        canvas.height = rect.height * dpr;
        
        this.ctx.scale(dpr, dpr);
        canvas.style.width = rect.width + 'px';
        canvas.style.height = rect.height + 'px';
        
        this.width = rect.width;
        this.height = rect.height;
    }

    public resize(width: number, height: number): void {
        this.width = width;
        this.height = height;
        
        const dpr = window.devicePixelRatio || 1;
        
        this.canvas.width = width * dpr;
        this.canvas.height = height * dpr;
        this.canvas.style.width = width + 'px';
        this.canvas.style.height = height + 'px';
        
        this.ctx.scale(dpr, dpr);
    }

    public render(state: SimulationState, settings: VisualizationSettings): void {
        this.ctx.clearRect(0, 0, this.width, this.height);
        
        // Set background
        this.ctx.fillStyle = '#0a0a23';
        this.ctx.fillRect(0, 0, this.width, this.height);
        
        // Calculate scale based on territory size (assuming 64x64 territory)
        const scale = Math.min(this.width / 64, this.height / 64) * 0.8;
        const offsetX = (this.width - 64 * scale) / 2;
        const offsetY = (this.height - 64 * scale) / 2;
        
        // Render territories first (background layer)
        if (settings.showTerritories) {
            this.renderTerritories(state.territories, scale, offsetX, offsetY);
        }
        
        // Render resources
        if (settings.showResources) {
            this.renderResources(state.resources, scale, offsetX, offsetY);
        }
        
        // Render warriors (foreground layer)
        this.renderWarriors(state.warriors, scale, offsetX, offsetY);
        
        // Render UI overlay
        this.renderOverlay(state);
    }

    private renderTerritories(territories: TerritoryData[], scale: number, offsetX: number, offsetY: number): void {
        for (const territory of territories) {
            const x = territory.center_x * scale + offsetX;
            const y = territory.center_y * scale + offsetY;
            const radius = territory.radius * scale;
            
            // Territory boundary
            this.ctx.beginPath();
            this.ctx.arc(x, y, radius, 0, Math.PI * 2);
            this.ctx.strokeStyle = territory.owner_id ? '#0088ff44' : '#ffffff22';
            this.ctx.lineWidth = territory.owner_id ? 2 : 1;
            this.ctx.stroke();
            
            // Territory fill based on resource multiplier
            const alpha = Math.min(territory.resource_multiplier * 0.1, 0.3);
            this.ctx.fillStyle = `rgba(0, 255, 136, ${alpha})`;
            this.ctx.fill();
            
            // Owner indicator
            if (territory.owner_id) {
                this.ctx.fillStyle = '#0088ff';
                this.ctx.beginPath();
                this.ctx.arc(x, y, 3, 0, Math.PI * 2);
                this.ctx.fill();
            }
        }
    }

    private renderResources(resources: ResourceData[], scale: number, offsetX: number, offsetY: number): void {
        for (const resource of resources) {
            const x = resource.x * scale + offsetX;
            const y = resource.y * scale + offsetY;
            
            // Resource glow effect
            const gradient = this.ctx.createRadialGradient(x, y, 0, x, y, 8);
            gradient.addColorStop(0, '#ffff00aa');
            gradient.addColorStop(1, '#ffff0000');
            
            this.ctx.fillStyle = gradient;
            this.ctx.beginPath();
            this.ctx.arc(x, y, 8, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Resource core
            const size = Math.max(2, Math.min(6, resource.energy_value / 10));
            this.ctx.fillStyle = '#ffff00';
            this.ctx.beginPath();
            this.ctx.arc(x, y, size, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Resource type indicator
            this.ctx.fillStyle = '#ffffff';
            this.ctx.font = '8px monospace';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(resource.resource_type.charAt(0).toUpperCase(), x, y - 10);
        }
    }

    private renderWarriors(warriors: WarriorData[], scale: number, offsetX: number, offsetY: number): void {
        // Sort warriors by fitness for rendering order (lowest first)
        const sortedWarriors = [...warriors].sort((a, b) => a.fitness - b.fitness);
        
        for (const warrior of sortedWarriors) {
            const x = warrior.x * scale + offsetX;
            const y = warrior.y * scale + offsetY;
            
            // Energy-based color and size
            const energyRatio = Math.min(warrior.energy / 100, 1);
            const size = 3 + energyRatio * 4;
            
            let color: string;
            if (energyRatio > 0.7) {
                color = '#00ff88'; // High energy - green
            } else if (energyRatio > 0.4) {
                color = '#0088ff'; // Medium energy - blue
            } else {
                color = '#ff8800'; // Low energy - orange
            }
            
            // Warrior glow based on fitness
            if (warrior.fitness > 50) {
                const glowRadius = size + warrior.fitness / 20;
                const gradient = this.ctx.createRadialGradient(x, y, 0, x, y, glowRadius);
                gradient.addColorStop(0, color + '66');
                gradient.addColorStop(1, color + '00');
                
                this.ctx.fillStyle = gradient;
                this.ctx.beginPath();
                this.ctx.arc(x, y, glowRadius, 0, Math.PI * 2);
                this.ctx.fill();
            }
            
            // Warrior body
            this.ctx.fillStyle = color;
            this.ctx.beginPath();
            this.ctx.arc(x, y, size, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Warrior border (species indicator)
            if (warrior.species_id !== undefined) {
                this.ctx.strokeStyle = `hsl(${(warrior.species_id * 60) % 360}, 70%, 50%)`;
                this.ctx.lineWidth = 1;
                this.ctx.stroke();
            }
            
            // Action indicator
            if (warrior.action && warrior.action !== 'idle') {
                this.ctx.fillStyle = '#ffffff';
                this.ctx.font = '10px monospace';
                this.ctx.textAlign = 'center';
                this.ctx.fillText(this.getActionSymbol(warrior.action), x, y - size - 5);
            }
            
            // Age indicator (lineage depth as small dots)
            if (warrior.lineage_depth > 0) {
                const dots = Math.min(warrior.lineage_depth, 5);
                for (let i = 0; i < dots; i++) {
                    this.ctx.fillStyle = '#ffffff44';
                    this.ctx.beginPath();
                    this.ctx.arc(x - size + i * 2, y + size + 3, 0.5, 0, Math.PI * 2);
                    this.ctx.fill();
                }
            }
        }
    }

    private getActionSymbol(action: string): string {
        switch (action.toLowerCase()) {
            case 'move': return '→';
            case 'attack': return '⚔';
            case 'defend': return '🛡';
            case 'collect': return '💎';
            case 'reproduce': return '🧬';
            case 'explore': return '👁';
            default: return '•';
        }
    }

    private renderOverlay(state: SimulationState): void {
        // Performance grid overlay
        this.ctx.strokeStyle = '#ffffff11';
        this.ctx.lineWidth = 0.5;
        
        const gridSize = 32;
        for (let x = 0; x < this.width; x += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.height);
            this.ctx.stroke();
        }
        
        for (let y = 0; y < this.height; y += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.width, y);
            this.ctx.stroke();
        }
        
        // Population density heatmap overlay (subtle)
        const densityMap = this.calculatePopulationDensity(state.warriors);
        this.renderDensityOverlay(densityMap);
    }

    private calculatePopulationDensity(warriors: WarriorData[]): number[][] {
        const gridSize = 8;
        const density: number[][] = Array(gridSize).fill(null).map(() => Array(gridSize).fill(0));
        
        for (const warrior of warriors) {
            const gridX = Math.floor((warrior.x / 64) * gridSize);
            const gridY = Math.floor((warrior.y / 64) * gridSize);
            
            if (gridX >= 0 && gridX < gridSize && gridY >= 0 && gridY < gridSize) {
                density[gridY][gridX] += warrior.energy;
            }
        }
        
        return density;
    }

    private renderDensityOverlay(densityMap: number[][]): void {
        const gridSize = densityMap.length;
        const cellWidth = this.width / gridSize;
        const cellHeight = this.height / gridSize;
        
        for (let y = 0; y < gridSize; y++) {
            for (let x = 0; x < gridSize; x++) {
                const density = densityMap[y][x];
                if (density > 0) {
                    const alpha = Math.min(density / 500, 0.2);
                    this.ctx.fillStyle = `rgba(0, 255, 136, ${alpha})`;
                    this.ctx.fillRect(x * cellWidth, y * cellHeight, cellWidth, cellHeight);
                }
            }
        }
    }
}