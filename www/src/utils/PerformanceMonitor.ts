export class PerformanceMonitor {
    private frameStartTime: number = 0;
    private frameEndTime: number = 0;
    private frameTimes: number[] = [];
    private readonly maxFrameHistory = 60; // Track last 60 frames
    
    // Performance metrics
    private lastFPSUpdate: number = 0;
    private currentFPS: number = 0;
    private frameCount: number = 0;
    
    // Memory usage tracking
    private memoryBaseline: number = 0;
    private peakMemory: number = 0;
    
    // WebAssembly performance
    private wasmExecutionTimes: number[] = [];
    private renderTimes: number[] = [];
    
    constructor() {
        this.initializeMemoryBaseline();
        this.lastFPSUpdate = performance.now();
    }

    private initializeMemoryBaseline(): void {
        // Initialize memory baseline if Performance API is available
        if ('memory' in performance) {
            const memory = (performance as any).memory;
            this.memoryBaseline = memory.usedJSHeapSize || 0;
        }
    }

    public frameStart(): void {
        this.frameStartTime = performance.now();
    }

    public frameEnd(): void {
        this.frameEndTime = performance.now();
        const frameTime = this.frameEndTime - this.frameStartTime;
        
        // Track frame times
        this.frameTimes.push(frameTime);
        if (this.frameTimes.length > this.maxFrameHistory) {
            this.frameTimes.shift();
        }
        
        this.frameCount++;
        
        // Update FPS every second
        const now = performance.now();
        if (now - this.lastFPSUpdate >= 1000) {
            this.currentFPS = this.frameCount * 1000 / (now - this.lastFPSUpdate);
            this.frameCount = 0;
            this.lastFPSUpdate = now;
            
            // Track peak memory usage
            this.updateMemoryStats();
        }
    }

    private updateMemoryStats(): void {
        if ('memory' in performance) {
            const memory = (performance as any).memory;
            const currentMemory = memory.usedJSHeapSize || 0;
            this.peakMemory = Math.max(this.peakMemory, currentMemory);
        }
    }

    public measureWasmExecution<T>(fn: () => T): T {
        const start = performance.now();
        const result = fn();
        const end = performance.now();
        
        this.wasmExecutionTimes.push(end - start);
        if (this.wasmExecutionTimes.length > 100) {
            this.wasmExecutionTimes.shift();
        }
        
        return result;
    }

    public measureRenderTime<T>(fn: () => T): T {
        const start = performance.now();
        const result = fn();
        const end = performance.now();
        
        this.renderTimes.push(end - start);
        if (this.renderTimes.length > 100) {
            this.renderTimes.shift();
        }
        
        return result;
    }

    public getFPS(): number {
        return this.currentFPS;
    }

    public getAverageFrameTime(): number {
        if (this.frameTimes.length === 0) return 0;
        return this.frameTimes.reduce((sum, time) => sum + time, 0) / this.frameTimes.length;
    }

    public getMinFrameTime(): number {
        return this.frameTimes.length > 0 ? Math.min(...this.frameTimes) : 0;
    }

    public getMaxFrameTime(): number {
        return this.frameTimes.length > 0 ? Math.max(...this.frameTimes) : 0;
    }

    public getFrameTimeStandardDeviation(): number {
        if (this.frameTimes.length < 2) return 0;
        
        const mean = this.getAverageFrameTime();
        const variance = this.frameTimes.reduce((sum, time) => sum + Math.pow(time - mean, 2), 0) / this.frameTimes.length;
        
        return Math.sqrt(variance);
    }

    public getAverageWasmExecutionTime(): number {
        if (this.wasmExecutionTimes.length === 0) return 0;
        return this.wasmExecutionTimes.reduce((sum, time) => sum + time, 0) / this.wasmExecutionTimes.length;
    }

    public getAverageRenderTime(): number {
        if (this.renderTimes.length === 0) return 0;
        return this.renderTimes.reduce((sum, time) => sum + time, 0) / this.renderTimes.length;
    }

    public getMemoryUsage(): {
        current: number;
        peak: number;
        baseline: number;
        growth: number;
    } {
        let currentMemory = 0;
        
        if ('memory' in performance) {
            const memory = (performance as any).memory;
            currentMemory = memory.usedJSHeapSize || 0;
        }
        
        return {
            current: currentMemory,
            peak: this.peakMemory,
            baseline: this.memoryBaseline,
            growth: currentMemory - this.memoryBaseline
        };
    }

    public getPerformanceReport(): {
        fps: number;
        frameTime: {
            average: number;
            min: number;
            max: number;
            stdDev: number;
        };
        wasmExecution: {
            average: number;
            samples: number;
        };
        rendering: {
            average: number;
            samples: number;
        };
        memory: {
            current: number;
            peak: number;
            baseline: number;
            growth: number;
        };
        performance: {
            rating: 'excellent' | 'good' | 'fair' | 'poor';
            bottleneck: string;
            suggestions: string[];
        };
    } {
        const frameTime = {
            average: this.getAverageFrameTime(),
            min: this.getMinFrameTime(),
            max: this.getMaxFrameTime(),
            stdDev: this.getFrameTimeStandardDeviation()
        };

        const memory = this.getMemoryUsage();
        const wasmTime = this.getAverageWasmExecutionTime();
        const renderTime = this.getAverageRenderTime();

        // Performance analysis
        const performance = this.analyzePerformance(this.currentFPS, frameTime, wasmTime, renderTime, memory);

        return {
            fps: this.currentFPS,
            frameTime,
            wasmExecution: {
                average: wasmTime,
                samples: this.wasmExecutionTimes.length
            },
            rendering: {
                average: renderTime,
                samples: this.renderTimes.length
            },
            memory,
            performance
        };
    }

    private analyzePerformance(
        fps: number,
        frameTime: { average: number; max: number; stdDev: number },
        wasmTime: number,
        renderTime: number,
        memory: { growth: number; current: number }
    ): {
        rating: 'excellent' | 'good' | 'fair' | 'poor';
        bottleneck: string;
        suggestions: string[];
    } {
        const suggestions: string[] = [];
        let bottleneck = 'none';
        let rating: 'excellent' | 'good' | 'fair' | 'poor' = 'excellent';

        // FPS analysis
        if (fps < 30) {
            rating = 'poor';
            suggestions.push('Frame rate is below 30 FPS - consider reducing simulation complexity');
        } else if (fps < 45) {
            rating = 'fair';
            suggestions.push('Frame rate is below 45 FPS - some optimizations recommended');
        } else if (fps < 55) {
            rating = 'good';
        }

        // Frame time consistency
        if (frameTime.stdDev > 5) {
            suggestions.push('Frame times are inconsistent - consider frame rate limiting');
            if (bottleneck === 'none') bottleneck = 'frame_consistency';
        }

        // WebAssembly performance
        if (wasmTime > 8) {
            suggestions.push('WebAssembly execution time is high - optimize simulation logic');
            bottleneck = 'wasm_execution';
            if (rating !== 'poor') rating = 'fair';
        }

        // Rendering performance
        if (renderTime > 12) {
            suggestions.push('Rendering time is high - consider reducing visual complexity');
            if (bottleneck === 'none') bottleneck = 'rendering';
            if (rating !== 'poor') rating = 'fair';
        }

        // Memory growth analysis
        const memoryGrowthMB = memory.growth / (1024 * 1024);
        if (memoryGrowthMB > 50) {
            suggestions.push('Memory usage is growing - possible memory leak detected');
            if (bottleneck === 'none') bottleneck = 'memory_leak';
            if (rating !== 'poor') rating = 'fair';
        }

        // Overall performance bottleneck detection
        if (wasmTime > renderTime * 2) {
            bottleneck = 'simulation_logic';
        } else if (renderTime > wasmTime * 2) {
            bottleneck = 'rendering';
        }

        return { rating, bottleneck, suggestions };
    }

    public startProfiling(): void {
        // Clear previous profiling data
        this.frameTimes = [];
        this.wasmExecutionTimes = [];
        this.renderTimes = [];
        this.frameCount = 0;
        this.lastFPSUpdate = performance.now();
        this.initializeMemoryBaseline();
        
        console.log('🔍 Performance profiling started');
    }

    public stopProfiling(): void {
        const report = this.getPerformanceReport();
        
        console.log('📊 Performance Profiling Report:');
        console.log(`FPS: ${report.fps.toFixed(1)}`);
        console.log(`Average Frame Time: ${report.frameTime.average.toFixed(2)}ms`);
        console.log(`WebAssembly Execution: ${report.wasmExecution.average.toFixed(2)}ms`);
        console.log(`Rendering Time: ${report.rendering.average.toFixed(2)}ms`);
        console.log(`Memory Growth: ${(report.memory.growth / (1024 * 1024)).toFixed(2)}MB`);
        console.log(`Performance Rating: ${report.performance.rating.toUpperCase()}`);
        
        if (report.performance.bottleneck !== 'none') {
            console.log(`Primary Bottleneck: ${report.performance.bottleneck}`);
        }
        
        if (report.performance.suggestions.length > 0) {
            console.log('Optimization Suggestions:');
            report.performance.suggestions.forEach(suggestion => {
                console.log(`  • ${suggestion}`);
            });
        }
    }

    public logRealTimeStats(): void {
        const report = this.getPerformanceReport();
        
        // Create a compact real-time display
        const stats = [
            `FPS: ${report.fps.toFixed(1)}`,
            `Frame: ${report.frameTime.average.toFixed(1)}ms`,
            `WASM: ${report.wasmExecution.average.toFixed(1)}ms`,
            `Render: ${report.rendering.average.toFixed(1)}ms`,
            `Mem: ${(report.memory.current / (1024 * 1024)).toFixed(1)}MB`
        ].join(' | ');
        
        console.log(`⚡ ${stats}`);
    }
}