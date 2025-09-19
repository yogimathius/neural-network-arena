import { Application } from './components/Application';
import { SimulationConfig } from './types/simulation';

// Application entry point
async function main() {
    console.log('🧠 Neural Network Arena - WebAssembly Frontend Starting...');
    
    try {
        // Show loading screen
        const loadingScreen = document.getElementById('loadingScreen')!;
        const mainContainer = document.getElementById('mainContainer')!;
        
        // Default simulation configuration
        const config: SimulationConfig = {
            max_population: 200,
            vm_memory_size: 2048,
            territory_size: 64,
            target_species_count: 8,
            mutation_rate: 0.05,
            survival_threshold: 0.3,
            fitness_sharing: true,
            elitism_rate: 0.1,
            tournament_size: 3,
            max_generations: 1000,
            performance_target_rps: 1000,
        };
        
        // Initialize application
        const app = new Application(config);
        await app.initialize();
        
        // Hide loading screen and show main interface
        setTimeout(() => {
            loadingScreen.style.display = 'none';
            mainContainer.style.display = 'grid';
            console.log('✅ Neural Network Arena fully loaded!');
        }, 2000);
        
    } catch (error) {
        console.error('❌ Failed to initialize Neural Network Arena:', error);
        alert('Failed to load Neural Network Arena. Please check the console for details.');
    }
}

// Start the application
main();