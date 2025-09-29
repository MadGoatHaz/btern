// main.rs - The entry point for the btern emulator (bemu).

use std::fs;

// Declare the modules we'll be using.
mod cpu;

use cpu::Cpu;

const PROGRAM_FILE: &str = "test_program.bin";

fn main() {
    println!("Starting btern Virtual Machine (bemu)...");
    
    // Create a new instance of our CPU.
    let mut btern_cpu = Cpu::new();

    // Load the program into memory.
    let program_bytes = match fs::read(PROGRAM_FILE) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading program file {}: {}", PROGRAM_FILE, e);
            std::process::exit(1);
        }
    };

    if let Err(e) = btern_cpu.load_program(&program_bytes) {
        eprintln!("Error loading program: {}", e);
        std::process::exit(1);
    }

    // Run the simulation.
    match btern_cpu.run() {
        Ok(_) => println!("\nbemu simulation finished successfully."),
        Err(e) => {
            eprintln!("\nAn error occurred during execution: {}", e);
            std::process::exit(1);
        }
    }
}