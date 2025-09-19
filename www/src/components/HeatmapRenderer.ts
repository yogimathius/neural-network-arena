import { MemoryHeatmapData } from '../types/simulation';

export class HeatmapRenderer {
    private ctx: CanvasRenderingContext2D;
    private canvas: HTMLCanvasElement;
    private width: number = 0;
    private height: number = 0;
    private imageData: ImageData | null = null;
    
    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        const context = canvas.getContext('2d');
        if (!context) {
            throw new Error('Failed to get 2D rendering context for heatmap');
        }
        this.ctx = context;
        
        // Set initial canvas properties
        this.canvas.style.position = 'absolute';
        this.canvas.style.top = '0';
        this.canvas.style.left = '0';
        this.canvas.style.pointerEvents = 'none'; // Allow clicks to pass through
        this.canvas.style.mixBlendMode = 'multiply'; // Blend with background
        
        this.setupCanvas();
    }

    private setupCanvas(): void {
        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.getBoundingClientRect();
        
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;
        
        this.ctx.scale(dpr, dpr);
        this.canvas.style.width = rect.width + 'px';
        this.canvas.style.height = rect.height + 'px';
        
        this.width = rect.width;
        this.height = rect.height;
        
        // Pre-allocate ImageData for performance
        this.imageData = this.ctx.createImageData(this.width, this.height);
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
        this.imageData = this.ctx.createImageData(width, height);
    }

    public show(): void {
        this.canvas.style.display = 'block';
    }

    public hide(): void {
        this.canvas.style.display = 'none';
    }

    public render(heatmapData: MemoryHeatmapData): void {
        if (!this.imageData || !heatmapData) {
            return;
        }

        // Clear previous frame
        this.ctx.clearRect(0, 0, this.width, this.height);

        // Scale heatmap data to canvas dimensions
        const scaleX = this.width / heatmapData.width;
        const scaleY = this.height / heatmapData.height;

        // Render using direct pixel manipulation for performance
        if (scaleX >= 1 && scaleY >= 1) {
            // Upscaling - use nearest neighbor for crisp pixels
            this.renderUpscaled(heatmapData, scaleX, scaleY);
        } else {
            // Downscaling - use sampling
            this.renderDownscaled(heatmapData, scaleX, scaleY);
        }

        // Apply the heatmap with intensity legend
        this.renderLegend();
    }

    private renderUpscaled(heatmapData: MemoryHeatmapData, scaleX: number, scaleY: number): void {
        if (!this.imageData) return;

        const data = this.imageData.data;
        
        for (let canvasY = 0; canvasY < this.height; canvasY++) {
            for (let canvasX = 0; canvasX < this.width; canvasX++) {
                // Map canvas coordinates back to heatmap coordinates
                const heatmapX = Math.floor(canvasX / scaleX);
                const heatmapY = Math.floor(canvasY / scaleY);
                
                if (heatmapX < heatmapData.width && heatmapY < heatmapData.height) {
                    const heatmapIndex = heatmapY * heatmapData.width + heatmapX;
                    const intensity = heatmapData.data[heatmapIndex] || 0;
                    
                    // Convert to pixel coordinates
                    const pixelIndex = (canvasY * this.width + canvasX) * 4;
                    
                    // Apply heat colormap
                    const color = this.intensityToColor(intensity);
                    data[pixelIndex] = color.r;     // Red
                    data[pixelIndex + 1] = color.g; // Green
                    data[pixelIndex + 2] = color.b; // Blue
                    data[pixelIndex + 3] = color.a; // Alpha
                }
            }
        }

        this.ctx.putImageData(this.imageData, 0, 0);
    }

    private renderDownscaled(heatmapData: MemoryHeatmapData, scaleX: number, scaleY: number): void {
        // For downscaling, we sample multiple heatmap pixels per canvas pixel
        if (!this.imageData) return;

        const data = this.imageData.data;
        
        for (let canvasY = 0; canvasY < this.height; canvasY++) {
            for (let canvasX = 0; canvasX < this.width; canvasX++) {
                // Sample area in heatmap
                const startHeatmapX = Math.floor(canvasX / scaleX);
                const endHeatmapX = Math.floor((canvasX + 1) / scaleX);
                const startHeatmapY = Math.floor(canvasY / scaleY);
                const endHeatmapY = Math.floor((canvasY + 1) / scaleY);
                
                // Average intensity over the sampled area
                let totalIntensity = 0;
                let sampleCount = 0;
                
                for (let hy = startHeatmapY; hy <= endHeatmapY && hy < heatmapData.height; hy++) {
                    for (let hx = startHeatmapX; hx <= endHeatmapX && hx < heatmapData.width; hx++) {
                        const heatmapIndex = hy * heatmapData.width + hx;
                        totalIntensity += heatmapData.data[heatmapIndex] || 0;
                        sampleCount++;
                    }
                }
                
                const avgIntensity = sampleCount > 0 ? totalIntensity / sampleCount : 0;
                
                // Convert to pixel coordinates
                const pixelIndex = (canvasY * this.width + canvasX) * 4;
                
                // Apply heat colormap
                const color = this.intensityToColor(avgIntensity);
                data[pixelIndex] = color.r;     // Red
                data[pixelIndex + 1] = color.g; // Green
                data[pixelIndex + 2] = color.b; // Blue
                data[pixelIndex + 3] = color.a; // Alpha
            }
        }

        this.ctx.putImageData(this.imageData, 0, 0);
    }

    private intensityToColor(intensity: number): { r: number; g: number; b: number; a: number } {
        // Clamp intensity to [0, 1]
        const clamped = Math.max(0, Math.min(1, intensity));
        
        // Heat colormap: black -> red -> yellow -> white
        let r: number, g: number, b: number;
        
        if (clamped < 0.25) {
            // Black to red
            const t = clamped * 4;
            r = Math.floor(255 * t);
            g = 0;
            b = 0;
        } else if (clamped < 0.5) {
            // Red to yellow
            const t = (clamped - 0.25) * 4;
            r = 255;
            g = Math.floor(255 * t);
            b = 0;
        } else if (clamped < 0.75) {
            // Yellow to orange-white
            const t = (clamped - 0.5) * 4;
            r = 255;
            g = 255;
            b = Math.floor(128 * t);
        } else {
            // Orange-white to white
            const t = (clamped - 0.75) * 4;
            r = 255;
            g = 255;
            b = 128 + Math.floor(127 * t);
        }
        
        // Alpha based on intensity for smooth blending
        const alpha = Math.floor(255 * Math.min(clamped * 1.5, 0.8));
        
        return { r, g, b, a: alpha };
    }

    private renderLegend(): void {
        // Render a small intensity legend in the top-right corner
        const legendWidth = 20;
        const legendHeight = 100;
        const legendX = this.width - legendWidth - 10;
        const legendY = 10;
        
        // Create gradient for legend
        const gradient = this.ctx.createLinearGradient(0, legendY + legendHeight, 0, legendY);
        gradient.addColorStop(0, 'rgba(0, 0, 0, 0.8)');     // Low intensity
        gradient.addColorStop(0.25, 'rgba(255, 0, 0, 0.8)');   // Red
        gradient.addColorStop(0.5, 'rgba(255, 255, 0, 0.8)');  // Yellow
        gradient.addColorStop(0.75, 'rgba(255, 255, 128, 0.8)'); // Orange-white
        gradient.addColorStop(1, 'rgba(255, 255, 255, 0.8)');   // High intensity
        
        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(legendX, legendY, legendWidth, legendHeight);
        
        // Legend border
        this.ctx.strokeStyle = '#ffffff66';
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(legendX, legendY, legendWidth, legendHeight);
        
        // Legend labels
        this.ctx.fillStyle = '#ffffff';
        this.ctx.font = '10px monospace';
        this.ctx.textAlign = 'left';
        this.ctx.fillText('Memory', legendX + legendWidth + 5, legendY + 10);
        this.ctx.fillText('Usage', legendX + legendWidth + 5, legendY + 22);
        this.ctx.fillText('High', legendX + legendWidth + 5, legendY + 35);
        this.ctx.fillText('Med', legendX + legendWidth + 5, legendY + legendHeight / 2 + 5);
        this.ctx.fillText('Low', legendX + legendWidth + 5, legendY + legendHeight - 5);
    }

    public renderStaticOverlay(): void {
        // Render memory access pattern overlay
        this.ctx.strokeStyle = '#ff000033';
        this.ctx.lineWidth = 0.5;
        
        // Grid showing memory pages
        const pageSize = 32;
        for (let x = 0; x < this.width; x += pageSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.height);
            this.ctx.stroke();
        }
        
        for (let y = 0; y < this.height; y += pageSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.width, y);
            this.ctx.stroke();
        }
    }
}