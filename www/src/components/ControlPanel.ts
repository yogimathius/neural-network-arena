export class ControlPanel {
    private startButton: HTMLButtonElement;
    private pauseButton: HTMLButtonElement;
    private resetButton: HTMLButtonElement;
    private stepButton: HTMLButtonElement;
    private generationButton: HTMLButtonElement;
    private speedSlider: HTMLInputElement;
    private speedValue: HTMLSpanElement;
    
    // Visualization toggles
    private heatmapButton: HTMLButtonElement;
    private topologyButton: HTMLButtonElement;
    private territoriesButton: HTMLButtonElement;
    private resourcesButton: HTMLButtonElement;
    
    // Export buttons
    private exportJsonButton: HTMLButtonElement;
    private exportCsvButton: HTMLButtonElement;
    
    // Event handlers
    private onStartCallback?: () => void;
    private onPauseCallback?: () => void;
    private onResetCallback?: () => void;
    private onStepCallback?: () => void;
    private onGenerationCallback?: () => void;
    private onSpeedChangeCallback?: (speed: number) => void;
    private onHeatmapToggleCallback?: () => void;
    private onTopologyToggleCallback?: () => void;
    private onTerritoriesToggleCallback?: () => void;
    private onResourcesToggleCallback?: () => void;
    private onExportJsonCallback?: () => void;
    private onExportCsvCallback?: () => void;
    
    constructor() {
        // Get DOM elements
        this.startButton = this.getElement('startBtn') as HTMLButtonElement;
        this.pauseButton = this.getElement('pauseBtn') as HTMLButtonElement;
        this.resetButton = this.getElement('resetBtn') as HTMLButtonElement;
        this.stepButton = this.getElement('stepBtn') as HTMLButtonElement;
        this.generationButton = this.getElement('generationBtn') as HTMLButtonElement;
        this.speedSlider = this.getElement('speedSlider') as HTMLInputElement;
        this.speedValue = this.getElement('speedValue') as HTMLSpanElement;
        
        this.heatmapButton = this.getElement('heatmapBtn') as HTMLButtonElement;
        this.topologyButton = this.getElement('topologyBtn') as HTMLButtonElement;
        this.territoriesButton = this.getElement('territoriesBtn') as HTMLButtonElement;
        this.resourcesButton = this.getElement('resourcesBtn') as HTMLButtonElement;
        
        this.exportJsonButton = this.getElement('exportJsonBtn') as HTMLButtonElement;
        this.exportCsvButton = this.getElement('exportCsvBtn') as HTMLButtonElement;
        
        this.setupEventListeners();
        this.initializeUI();
    }
    
    private getElement(id: string): HTMLElement {
        const element = document.getElementById(id);
        if (!element) {
            throw new Error(`Element with id '${id}' not found`);
        }
        return element;
    }
    
    private setupEventListeners(): void {
        // Simulation controls
        this.startButton.addEventListener('click', () => {
            this.onStartCallback?.();
            this.updateButtonStates('running');
        });
        
        this.pauseButton.addEventListener('click', () => {
            this.onPauseCallback?.();
            this.updateButtonStates('paused');
        });
        
        this.resetButton.addEventListener('click', () => {
            this.onResetCallback?.();
            this.updateButtonStates('reset');
        });
        
        this.stepButton.addEventListener('click', () => {
            this.onStepCallback?.();
        });
        
        this.generationButton.addEventListener('click', () => {
            this.onGenerationCallback?.();
        });
        
        // Speed control
        this.speedSlider.addEventListener('input', () => {
            const speed = parseFloat(this.speedSlider.value);
            this.speedValue.textContent = speed.toFixed(1) + 'x';
            this.onSpeedChangeCallback?.(speed);
        });
        
        // Visualization toggles
        this.heatmapButton.addEventListener('click', () => {
            this.onHeatmapToggleCallback?.();
            this.toggleButtonState(this.heatmapButton);
            // Ensure topology is disabled when heatmap is enabled
            if (this.heatmapButton.classList.contains('active')) {
                this.topologyButton.classList.remove('active');
            }
        });
        
        this.topologyButton.addEventListener('click', () => {
            this.onTopologyToggleCallback?.();
            this.toggleButtonState(this.topologyButton);
            // Ensure heatmap is disabled when topology is enabled
            if (this.topologyButton.classList.contains('active')) {
                this.heatmapButton.classList.remove('active');
            }
        });
        
        this.territoriesButton.addEventListener('click', () => {
            this.onTerritoriesToggleCallback?.();
            this.toggleButtonState(this.territoriesButton);
        });
        
        this.resourcesButton.addEventListener('click', () => {
            this.onResourcesToggleCallback?.();
            this.toggleButtonState(this.resourcesButton);
        });
        
        // Export functions
        this.exportJsonButton.addEventListener('click', () => {
            this.onExportJsonCallback?.();
            this.showExportFeedback(this.exportJsonButton, 'JSON');
        });
        
        this.exportCsvButton.addEventListener('click', () => {
            this.onExportCsvCallback?.();
            this.showExportFeedback(this.exportCsvButton, 'CSV');
        });
        
        // Keyboard shortcuts
        document.addEventListener('keydown', (event) => {
            this.handleKeyboardShortcuts(event);
        });
    }
    
    private initializeUI(): void {
        // Set initial button states
        this.updateButtonStates('reset');
        
        // Initialize toggle states (territories and resources enabled by default)
        this.territoriesButton.classList.add('active');
        this.resourcesButton.classList.add('active');
        
        // Add CSS classes for active states
        this.addToggleStyles();
    }
    
    private addToggleStyles(): void {
        const style = document.createElement('style');
        style.textContent = `
            .control-button.active {
                background: linear-gradient(45deg, #00cc66, #00ff88) !important;
                box-shadow: 0 0 10px rgba(0, 255, 136, 0.3);
            }
            
            .control-button.export-feedback {
                background: linear-gradient(45deg, #cc6600, #ff8800) !important;
                transform: scale(0.95);
                transition: all 0.2s ease;
            }
            
            .control-button:disabled {
                background: linear-gradient(45deg, #666, #888) !important;
                cursor: not-allowed;
                opacity: 0.6;
            }
        `;
        document.head.appendChild(style);
    }
    
    private updateButtonStates(state: 'running' | 'paused' | 'reset'): void {
        switch (state) {
            case 'running':
                this.startButton.disabled = true;
                this.pauseButton.disabled = false;
                this.resetButton.disabled = false;
                this.stepButton.disabled = true;
                this.generationButton.disabled = true;
                break;
                
            case 'paused':
                this.startButton.disabled = false;
                this.pauseButton.disabled = true;
                this.resetButton.disabled = false;
                this.stepButton.disabled = false;
                this.generationButton.disabled = false;
                break;
                
            case 'reset':
                this.startButton.disabled = false;
                this.pauseButton.disabled = true;
                this.resetButton.disabled = true;
                this.stepButton.disabled = false;
                this.generationButton.disabled = false;
                break;
        }
    }
    
    private toggleButtonState(button: HTMLButtonElement): void {
        button.classList.toggle('active');
    }
    
    private showExportFeedback(button: HTMLButtonElement, format: string): void {
        const originalText = button.textContent;
        button.textContent = `Exported ${format}!`;
        button.classList.add('export-feedback');
        
        setTimeout(() => {
            button.textContent = originalText;
            button.classList.remove('export-feedback');
        }, 1500);
    }
    
    private handleKeyboardShortcuts(event: KeyboardEvent): void {
        // Prevent shortcuts when typing in input fields
        if (event.target instanceof HTMLInputElement) {
            return;
        }
        
        switch (event.key.toLowerCase()) {
            case ' ':
            case 'enter':
                event.preventDefault();
                if (!this.startButton.disabled) {
                    this.startButton.click();
                } else if (!this.pauseButton.disabled) {
                    this.pauseButton.click();
                }
                break;
                
            case 'r':
                event.preventDefault();
                this.resetButton.click();
                break;
                
            case 's':
                event.preventDefault();
                this.stepButton.click();
                break;
                
            case 'g':
                event.preventDefault();
                this.generationButton.click();
                break;
                
            case 'h':
                event.preventDefault();
                this.heatmapButton.click();
                break;
                
            case 't':
                event.preventDefault();
                this.topologyButton.click();
                break;
                
            case 'm':
                event.preventDefault();
                this.territoriesButton.click();
                break;
                
            case 'n':
                event.preventDefault();
                this.resourcesButton.click();
                break;
                
            case 'j':
                event.preventDefault();
                this.exportJsonButton.click();
                break;
                
            case 'c':
                event.preventDefault();
                this.exportCsvButton.click();
                break;
                
            case 'arrowup':
            case '+':
                event.preventDefault();
                this.adjustSpeed(0.1);
                break;
                
            case 'arrowdown':
            case '-':
                event.preventDefault();
                this.adjustSpeed(-0.1);
                break;
        }
    }
    
    private adjustSpeed(delta: number): void {
        const currentSpeed = parseFloat(this.speedSlider.value);
        const newSpeed = Math.max(0.1, Math.min(5.0, currentSpeed + delta));
        
        this.speedSlider.value = newSpeed.toString();
        this.speedValue.textContent = newSpeed.toFixed(1) + 'x';
        this.onSpeedChangeCallback?.(newSpeed);
    }
    
    // Public methods to register event callbacks
    public onStart(callback: () => void): void {
        this.onStartCallback = callback;
    }
    
    public onPause(callback: () => void): void {
        this.onPauseCallback = callback;
    }
    
    public onReset(callback: () => void): void {
        this.onResetCallback = callback;
    }
    
    public onStep(callback: () => void): void {
        this.onStepCallback = callback;
    }
    
    public onGeneration(callback: () => void): void {
        this.onGenerationCallback = callback;
    }
    
    public onSpeedChange(callback: (speed: number) => void): void {
        this.onSpeedChangeCallback = callback;
    }
    
    public onHeatmapToggle(callback: () => void): void {
        this.onHeatmapToggleCallback = callback;
    }
    
    public onTopologyToggle(callback: () => void): void {
        this.onTopologyToggleCallback = callback;
    }
    
    public onTerritoriesToggle(callback: () => void): void {
        this.onTerritoriesToggleCallback = callback;
    }
    
    public onResourcesToggle(callback: () => void): void {
        this.onResourcesToggleCallback = callback;
    }
    
    public onExportJson(callback: () => void): void {
        this.onExportJsonCallback = callback;
    }
    
    public onExportCsv(callback: () => void): void {
        this.onExportCsvCallback = callback;
    }
    
    // Public methods to control UI state
    public setSimulationState(running: boolean): void {
        this.updateButtonStates(running ? 'running' : 'paused');
    }
    
    public updateMetrics(metrics: {
        generation: number;
        population: number;
        species: number;
        fps: number;
    }): void {
        // These are updated directly by Application.ts, but we can add validation here
        const generationEl = document.getElementById('generation');
        const populationEl = document.getElementById('population');
        const speciesEl = document.getElementById('species');
        const fpsEl = document.getElementById('fps');
        
        if (generationEl) generationEl.textContent = metrics.generation.toString();
        if (populationEl) populationEl.textContent = metrics.population.toString();
        if (speciesEl) speciesEl.textContent = metrics.species.toString();
        if (fpsEl) fpsEl.textContent = metrics.fps.toFixed(1);
    }
    
    public showNotification(message: string, type: 'success' | 'warning' | 'error' = 'success'): void {
        // Create floating notification
        const notification = document.createElement('div');
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background: ${type === 'success' ? 'linear-gradient(45deg, #00cc66, #00ff88)' : 
                         type === 'warning' ? 'linear-gradient(45deg, #cc6600, #ff8800)' :
                         'linear-gradient(45deg, #cc0066, #ff0088)'};
            color: white;
            padding: 12px 20px;
            border-radius: 5px;
            font-family: monospace;
            font-size: 12px;
            z-index: 10000;
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
            animation: slideIn 0.3s ease-out;
        `;
        
        // Add animation styles if not already present
        if (!document.querySelector('#notification-styles')) {
            const style = document.createElement('style');
            style.id = 'notification-styles';
            style.textContent = `
                @keyframes slideIn {
                    from { transform: translateX(100%); opacity: 0; }
                    to { transform: translateX(0); opacity: 1; }
                }
                @keyframes slideOut {
                    from { transform: translateX(0); opacity: 1; }
                    to { transform: translateX(100%); opacity: 0; }
                }
            `;
            document.head.appendChild(style);
        }
        
        document.body.appendChild(notification);
        
        // Auto-remove after 3 seconds
        setTimeout(() => {
            notification.style.animation = 'slideOut 0.3s ease-in';
            setTimeout(() => {
                if (document.body.contains(notification)) {
                    document.body.removeChild(notification);
                }
            }, 300);
        }, 3000);
    }
}