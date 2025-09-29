// main.rs - The entry point for the btern assembler (basm).

use btern_core::{Word, Opcode, Instruction, encode_instruction};
use std::fs::File;
use std::io::Write;

// Helper function to convert a Word ([Trit; 27]) into a raw byte vector (54 bytes, 2 bits per trit).
fn word_to_raw_bytes(word: &Word) -> Vec<u8> {
    // We use a simple representation: 1 byte per trit, storing the i8 value (-1, 0, or 1).
    let mut bytes = Vec::with_capacity(27);

    for trit in word.iter() {
        // Convert the Trit enum to its i8 representation (-1, 0, 1) and then cast to u8 for writing.
        // We rely on the emulator to cast it back to i8 and validate.
        bytes.push(trit.to_i8() as u8);
    }

    // A Word is 3 Trytes (27 trits), resulting in 27 bytes per instruction.
    bytes
}

fn main() -> Result<(), String> {
    println!("Starting btern Assembler (basm)...");

    // --- Test Program Definition ---
    
    // 1. ADDI R1, R0, 5 (R1 = 5)
    let inst1 = Instruction {
        opcode: Opcode::ADDI,
        rd: 1,
        rs1: 0,
        rs2: 0,
        imm: 5,
    };

    // 2. ADDI R2, R0, 10 (R2 = 10)
    let inst2 = Instruction {
        opcode: Opcode::ADDI,
        rd: 2,
        rs1: 0,
        rs2: 0,
        imm: 10,
    };

    // 3. ADD R3, R1, R2 (R3 = 15)
    let inst3 = Instruction {
        opcode: Opcode::ADD,
        rd: 3,
        rs1: 1,
        rs2: 2,
        imm: 0,
    };

    // 4. HALT
    let inst4 = Instruction {
        opcode: Opcode::HALT,
        rd: 0,
        rs1: 0,
        rs2: 0,
        imm: 0,
    };

    let program = vec![inst1, inst2, inst3, inst4];
    let mut raw_program_data = Vec::new();
    
    // --- Assembly and Encoding ---
    for (i, inst) in program.iter().enumerate() {
        let word = encode_instruction(inst);
        let raw_bytes = word_to_raw_bytes(&word);
        raw_program_data.extend_from_slice(&raw_bytes);
        println!("Instruction {}: {:?} -> {} bytes", i, inst.opcode, raw_bytes.len());
    }

    // --- Write to File ---
    let output_path = "test_program.bin";
    let mut file = File::create(output_path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&raw_program_data).map_err(|e| format!("Failed to write to file: {}", e))?;

    println!("Successfully assembled program to {}", output_path);
    
    Ok(())
}