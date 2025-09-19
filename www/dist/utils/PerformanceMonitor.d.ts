export declare class PerformanceMonitor {
    private frameStartTime;
    private frameEndTime;
    private frameTimes;
    private readonly maxFrameHistory;
    private lastFPSUpdate;
    private currentFPS;
    private frameCount;
    private memoryBaseline;
    private peakMemory;
    private wasmExecutionTimes;
    private renderTimes;
    constructor();
    private initializeMemoryBaseline;
    frameStart(): void;
    frameEnd(): void;
    private updateMemoryStats;
    measureWasmExecution<T>(fn: () => T): T;
    measureRenderTime<T>(fn: () => T): T;
    getFPS(): number;
    getAverageFrameTime(): number;
    getMinFrameTime(): number;
    getMaxFrameTime(): number;
    getFrameTimeStandardDeviation(): number;
    getAverageWasmExecutionTime(): number;
    getAverageRenderTime(): number;
    getMemoryUsage(): {
        current: number;
        peak: number;
        baseline: number;
        growth: number;
    };
    getPerformanceReport(): {
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
    };
    private analyzePerformance;
    startProfiling(): void;
    stopProfiling(): void;
    logRealTimeStats(): void;
}
