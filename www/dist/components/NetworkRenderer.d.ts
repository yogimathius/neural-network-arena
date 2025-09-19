import { NetworkTopologyData, NetworkNode, NetworkConnection } from '../types/simulation';
export declare class NetworkRenderer {
    private ctx;
    private canvas;
    private width;
    private height;
    private animationFrame;
    private readonly layerSpacing;
    private readonly nodeSpacing;
    private readonly maxNodeRadius;
    private readonly minNodeRadius;
    constructor(canvas: HTMLCanvasElement);
    private setupCanvas;
    resize(width: number, height: number): void;
    show(): void;
    hide(): void;
    render(topologyData: NetworkTopologyData): void;
    private organizeNodesByLayers;
    private calculateLayout;
    private renderConnections;
    private drawCurvedConnection;
    private renderNodes;
    private renderInfoPanel;
    private animateActivations;
    renderActivationFlow(connections: NetworkConnection[], layoutNodes: Map<number, {
        x: number;
        y: number;
        node: NetworkNode;
    }>): void;
}
