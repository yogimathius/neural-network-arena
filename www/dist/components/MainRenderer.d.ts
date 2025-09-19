import { SimulationState, VisualizationSettings } from '../types/simulation';
export declare class MainRenderer {
    private ctx;
    private canvas;
    private width;
    private height;
    constructor(canvas: HTMLCanvasElement);
    resize(width: number, height: number): void;
    render(state: SimulationState, settings: VisualizationSettings): void;
    private renderTerritories;
    private renderResources;
    private renderWarriors;
    private getActionSymbol;
    private renderOverlay;
    private calculatePopulationDensity;
    private renderDensityOverlay;
}
