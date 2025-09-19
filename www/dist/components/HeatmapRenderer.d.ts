import { MemoryHeatmapData } from '../types/simulation';
export declare class HeatmapRenderer {
    private ctx;
    private canvas;
    private width;
    private height;
    private imageData;
    constructor(canvas: HTMLCanvasElement);
    private setupCanvas;
    resize(width: number, height: number): void;
    show(): void;
    hide(): void;
    render(heatmapData: MemoryHeatmapData): void;
    private renderUpscaled;
    private renderDownscaled;
    private intensityToColor;
    private renderLegend;
    renderStaticOverlay(): void;
}
