import { NetworkTopologyData, NetworkNode, NetworkConnection } from '../types/simulation';

export class NetworkRenderer {
    private ctx: CanvasRenderingContext2D;
    private canvas: HTMLCanvasElement;
    private width: number = 0;
    private height: number = 0;
    private animationFrame: number = 0;
    
    // Layout parameters
    private readonly layerSpacing = 150;
    private readonly nodeSpacing = 40;
    private readonly maxNodeRadius = 15;
    private readonly minNodeRadius = 5;
    
    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        const context = canvas.getContext('2d');
        if (!context) {
            throw new Error('Failed to get 2D rendering context for network');
        }
        this.ctx = context;
        
        // Set canvas properties
        this.canvas.style.position = 'absolute';
        this.canvas.style.top = '0';
        this.canvas.style.left = '0';
        this.canvas.style.pointerEvents = 'none';
        this.canvas.style.background = 'rgba(0, 0, 0, 0.1)';
        
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

    public show(): void {
        this.canvas.style.display = 'block';
    }

    public hide(): void {
        this.canvas.style.display = 'none';
    }

    public render(topologyData: NetworkTopologyData): void {
        if (!topologyData || !topologyData.nodes.length) {
            return;
        }

        // Clear previous frame
        this.ctx.clearRect(0, 0, this.width, this.height);

        // Organize nodes by layers
        const layers = this.organizeNodesByLayers(topologyData.nodes);
        const layoutNodes = this.calculateLayout(layers);

        // Render connections first (background layer)
        this.renderConnections(topologyData.connections, layoutNodes);

        // Render nodes (foreground layer)
        this.renderNodes(layoutNodes);

        // Render information panel
        this.renderInfoPanel(topologyData);

        // Animate activations
        this.animateActivations(layoutNodes);
    }

    private organizeNodesByLayers(nodes: NetworkNode[]): Map<string, NetworkNode[]> {
        const layers = new Map<string, NetworkNode[]>();
        
        for (const node of nodes) {
            const nodeType = node.node_type;
            if (!layers.has(nodeType)) {
                layers.set(nodeType, []);
            }
            layers.get(nodeType)!.push(node);
        }
        
        return layers;
    }

    private calculateLayout(layers: Map<string, NetworkNode[]>): Map<number, { x: number; y: number; node: NetworkNode }> {
        const layoutNodes = new Map<number, { x: number; y: number; node: NetworkNode }>();
        
        // Define layer order and positions
        const layerOrder = ['input', 'hidden', 'output'];
        const totalLayers = layerOrder.length;
        
        layerOrder.forEach((layerType, layerIndex) => {
            const layerNodes = layers.get(layerType) || [];
            const layerX = (this.width / (totalLayers + 1)) * (layerIndex + 1);
            
            // Arrange nodes vertically within the layer
            layerNodes.forEach((node, nodeIndex) => {
                const layerY = (this.height / (layerNodes.length + 1)) * (nodeIndex + 1);
                
                layoutNodes.set(node.id, {
                    x: layerX,
                    y: layerY,
                    node: node
                });
            });
        });
        
        return layoutNodes;
    }

    private renderConnections(connections: NetworkConnection[], layoutNodes: Map<number, { x: number; y: number; node: NetworkNode }>): void {
        for (const connection of connections) {
            const fromNode = layoutNodes.get(connection.from);
            const toNode = layoutNodes.get(connection.to);
            
            if (!fromNode || !toNode) continue;
            
            // Connection strength determines visual properties
            const weight = connection.weight;
            const absWeight = Math.abs(weight);
            const alpha = Math.min(absWeight * 0.8 + 0.2, 1.0);
            
            // Color based on weight polarity
            const color = weight >= 0 ? '#00ff88' : '#ff4444';
            
            // Line width based on connection strength
            const lineWidth = Math.max(0.5, Math.min(4, absWeight * 3));
            
            // Draw connection with gradient
            const gradient = this.ctx.createLinearGradient(fromNode.x, fromNode.y, toNode.x, toNode.y);
            gradient.addColorStop(0, color + Math.floor(alpha * 255).toString(16).padStart(2, '0'));
            gradient.addColorStop(1, color + '22');
            
            this.ctx.strokeStyle = gradient;
            this.ctx.lineWidth = lineWidth;
            
            // Add curve to connections for better visibility
            this.drawCurvedConnection(fromNode.x, fromNode.y, toNode.x, toNode.y);
            
            // Draw weight label for strong connections
            if (absWeight > 0.7) {
                const midX = (fromNode.x + toNode.x) / 2;
                const midY = (fromNode.y + toNode.y) / 2;
                
                this.ctx.fillStyle = '#ffffff88';
                this.ctx.font = '8px monospace';
                this.ctx.textAlign = 'center';
                this.ctx.fillText(weight.toFixed(2), midX, midY);
            }
        }
    }

    private drawCurvedConnection(fromX: number, fromY: number, toX: number, toY: number): void {
        const controlPointOffset = Math.abs(toX - fromX) * 0.3;
        
        this.ctx.beginPath();
        this.ctx.moveTo(fromX, fromY);
        this.ctx.bezierCurveTo(
            fromX + controlPointOffset, fromY,
            toX - controlPointOffset, toY,
            toX, toY
        );
        this.ctx.stroke();
    }

    private renderNodes(layoutNodes: Map<number, { x: number; y: number; node: NetworkNode }>): void {
        for (const [nodeId, nodeLayout] of layoutNodes) {
            const node = nodeLayout.node;
            const x = nodeLayout.x;
            const y = nodeLayout.y;
            
            // Node size based on activation
            const activation = Math.abs(node.activation);
            const radius = this.minNodeRadius + (this.maxNodeRadius - this.minNodeRadius) * activation;
            
            // Color based on node type and activation
            let color: string;
            switch (node.node_type) {
                case 'input':
                    color = '#0088ff';
                    break;
                case 'output':
                    color = '#ff8800';
                    break;
                case 'hidden':
                default:
                    color = '#00ff88';
                    break;
            }
            
            // Activation glow effect
            if (activation > 0.3) {
                const glowRadius = radius + activation * 10;
                const gradient = this.ctx.createRadialGradient(x, y, 0, x, y, glowRadius);
                gradient.addColorStop(0, color + '66');
                gradient.addColorStop(1, color + '00');
                
                this.ctx.fillStyle = gradient;
                this.ctx.beginPath();
                this.ctx.arc(x, y, glowRadius, 0, Math.PI * 2);
                this.ctx.fill();
            }
            
            // Node body
            this.ctx.fillStyle = color;
            this.ctx.beginPath();
            this.ctx.arc(x, y, radius, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Node border
            this.ctx.strokeStyle = '#ffffff';
            this.ctx.lineWidth = 2;
            this.ctx.stroke();
            
            // Activation value
            if (activation > 0.1) {
                this.ctx.fillStyle = '#ffffff';
                this.ctx.font = '10px monospace';
                this.ctx.textAlign = 'center';
                this.ctx.fillText(node.activation.toFixed(2), x, y + 3);
            }
            
            // Node ID below the node
            this.ctx.fillStyle = '#ffffff88';
            this.ctx.font = '8px monospace';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(`#${nodeId}`, x, y + radius + 12);
        }
    }

    private renderInfoPanel(topologyData: NetworkTopologyData): void {
        // Background panel
        const panelWidth = 200;
        const panelHeight = 120;
        const panelX = 10;
        const panelY = 10;
        
        this.ctx.fillStyle = 'rgba(10, 10, 35, 0.9)';
        this.ctx.fillRect(panelX, panelY, panelWidth, panelHeight);
        
        this.ctx.strokeStyle = '#00ff88';
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(panelX, panelY, panelWidth, panelHeight);
        
        // Panel title
        this.ctx.fillStyle = '#00ff88';
        this.ctx.font = 'bold 12px monospace';
        this.ctx.textAlign = 'left';
        this.ctx.fillText('🧠 Network Topology', panelX + 10, panelY + 20);
        
        // Network statistics
        const inputNodes = topologyData.nodes.filter(n => n.node_type === 'input').length;
        const hiddenNodes = topologyData.nodes.filter(n => n.node_type === 'hidden').length;
        const outputNodes = topologyData.nodes.filter(n => n.node_type === 'output').length;
        const totalConnections = topologyData.connections.length;
        
        // Calculate average activation
        const avgActivation = topologyData.nodes.reduce((sum, n) => sum + Math.abs(n.activation), 0) / topologyData.nodes.length;
        
        this.ctx.fillStyle = '#ffffff';
        this.ctx.font = '10px monospace';
        
        this.ctx.fillText(`Input Nodes: ${inputNodes}`, panelX + 10, panelY + 40);
        this.ctx.fillText(`Hidden Nodes: ${hiddenNodes}`, panelX + 10, panelY + 55);
        this.ctx.fillText(`Output Nodes: ${outputNodes}`, panelX + 10, panelY + 70);
        this.ctx.fillText(`Connections: ${totalConnections}`, panelX + 10, panelY + 85);
        this.ctx.fillText(`Avg Activation: ${avgActivation.toFixed(3)}`, panelX + 10, panelY + 100);
    }

    private animateActivations(layoutNodes: Map<number, { x: number; y: number; node: NetworkNode }>): void {
        // Create pulsing animation for highly active nodes
        this.animationFrame = (this.animationFrame + 1) % 60;
        const pulseIntensity = Math.sin(this.animationFrame * 0.2) * 0.5 + 0.5;
        
        for (const [nodeId, nodeLayout] of layoutNodes) {
            const node = nodeLayout.node;
            const activation = Math.abs(node.activation);
            
            if (activation > 0.8) {
                const x = nodeLayout.x;
                const y = nodeLayout.y;
                const pulseRadius = 20 + pulseIntensity * 10;
                
                // Animated pulse ring
                this.ctx.strokeStyle = `rgba(255, 255, 255, ${0.3 - pulseIntensity * 0.2})`;
                this.ctx.lineWidth = 2;
                this.ctx.beginPath();
                this.ctx.arc(x, y, pulseRadius, 0, Math.PI * 2);
                this.ctx.stroke();
            }
        }
    }

    public renderActivationFlow(connections: NetworkConnection[], layoutNodes: Map<number, { x: number; y: number; node: NetworkNode }>): void {
        // Animated signal flow visualization
        const flowTime = Date.now() * 0.005;
        
        for (const connection of connections) {
            const fromNode = layoutNodes.get(connection.from);
            const toNode = layoutNodes.get(connection.to);
            
            if (!fromNode || !toNode || Math.abs(connection.weight) < 0.3) continue;
            
            // Calculate signal position along connection
            const t = (Math.sin(flowTime + connection.from * 0.1) + 1) / 2;
            const signalX = fromNode.x + (toNode.x - fromNode.x) * t;
            const signalY = fromNode.y + (toNode.y - fromNode.y) * t;
            
            // Draw signal
            this.ctx.fillStyle = connection.weight > 0 ? '#00ff88aa' : '#ff4444aa';
            this.ctx.beginPath();
            this.ctx.arc(signalX, signalY, 3, 0, Math.PI * 2);
            this.ctx.fill();
        }
    }
}