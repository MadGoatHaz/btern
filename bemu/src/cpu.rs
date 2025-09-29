// cpu.rs - Defines the CPU structure and its primary operations.

use btern_core::{add_words, neg_word, word_to_i64, trits_to_i64, i64_to_word, Word, Tryte, Trit, Instruction, Opcode};

const MEMORY_TRYTES: usize = 19683; // 3^9 Trytes

pub struct Cpu {
    /// General-Purpose Registers R0-R26.
    gpr: [Word; 27],

    /// Program Counter.
    pc: Word,

    /// Simulated main memory.
    memory: Vec<Tryte>,
}

impl Cpu {
    /// Converts a raw byte (which should be -1, 0, or 1) into a Trit.
    fn byte_to_trit(byte: u8) -> Result<Trit, String> {
        let val = byte as i8;
        match Trit::from_i8(val) {
            Ok(t) => Ok(t),
            Err(_) => Err(format!("Invalid trit value in program binary: {}", val)),
        }
    }

    /// Loads a raw byte program into memory.
    /// Assumes the byte stream contains sequential i8 representations of Trits.
    pub fn load_program(&mut self, program_bytes: &[u8]) -> Result<(), String> {
        let trits_per_tryte = 9;
        let mut current_tryte_idx = 0;
        let mut current_trit_in_tryte = 0;

        if program_bytes.len() % trits_per_tryte != 0 {
            return Err(format!(
                "Program size is not a multiple of 9 trits (1 Tryte). Size: {} bytes",
                program_bytes.len()
            ));
        }

        for byte in program_bytes {
            if current_tryte_idx >= self.memory.len() {
                return Err("Program exceeds maximum memory size.".to_string());
            }

            let trit = Self::byte_to_trit(*byte)?;
            
            // Write the trit to the current Tryte in memory
            self.memory[current_tryte_idx][current_trit_in_tryte] = trit;

            current_trit_in_tryte += 1;

            if current_trit_in_tryte == trits_per_tryte {
                current_tryte_idx += 1;
                current_trit_in_tryte = 0;
            }
        }

        println!("Successfully loaded {} Trytes into memory.", current_tryte_idx);
        Ok(())
    }

    /// Creates a new, initialized CPU instance.
    /// Creates a new, initialized CPU instance.
    pub fn new() -> Self {
        println!("Initializing btern CPU...");
        Self {
            // R0 is not special-cased here, but in the instruction logic.
            // All registers default to a word of Zeros.
            gpr: [[Trit::Z; 27]; 27],
            pc: [Trit::Z; 27],
            memory: vec![[Trit::Z; 9]; MEMORY_TRYTES], // Trit::Z is imported from btern_core
        }
    }

    /// Runs the main fetch-decode-execute cycle.
    pub fn run(&mut self) -> Result<(), String> {
        let mut running = true;
        while running {
            // 1. Fetch
            let instruction_word = self.fetch()?;
            
            // 2. Decode
            let instruction = self.decode(&instruction_word)?;

            // 3. Execute
            running = self.execute(&instruction)?;

            // For now, we manually halt if we hit NOP after one cycle.
            if instruction.opcode == Opcode::NOP {
                running = false;
            }
        }
        Ok(())
    }

    /// Fetches a Word (3 trytes) from memory at the address in the PC.
    fn fetch(&self) -> Result<Word, String> {
        // Convert the 27-trit PC into a memory index.
        // We only use the lower 9 trits of the PC for addressing (3^9 Trytes).
        // A full Word conversion is performed, but checked against memory size.
        let pc_value = word_to_i64(&self.pc);
        
        if pc_value < 0 {
            return Err(format!("Negative PC address: {}", pc_value));
        }

        let pc_address = pc_value as usize;
        
        if pc_address + 2 >= self.memory.len() {
            return Err(format!("Memory access out of bounds at PC={}", pc_address));
        }

        // An instruction is one Word (27 trits), which is 3 Trytes.
        let mut instruction_word = [Trit::Z; 27];
        let tryte1 = &self.memory[pc_address];
        let tryte2 = &self.memory[pc_address + 1];
        let tryte3 = &self.memory[pc_address + 2];

        // This copy logic will be more sophisticated.
        instruction_word[0..9].copy_from_slice(tryte1);
        instruction_word[9..18].copy_from_slice(tryte2);
        instruction_word[18..27].copy_from_slice(tryte3);

        Ok(instruction_word)
    }

    /// Decodes a 27-trit instruction Word into an Instruction struct.
    fn decode(&self, instruction_word: &Word) -> Result<Instruction, String> {
        // Opcode: 6 trits (21..26)
        let opcode_val = trits_to_i64(&instruction_word[21..27]);
        
        // Rd: 3 trits (18..20)
        let rd_val = trits_to_i64(&instruction_word[18..21]);
        
        // Rs1: 3 trits (15..17)
        let rs1_val = trits_to_i64(&instruction_word[15..18]);
        
        // Rs2: 3 trits (12..14)
        let rs2_val = trits_to_i64(&instruction_word[12..15]);
        
        // Imm/Offset: 12 trits (0..11)
        let imm_val = trits_to_i64(&instruction_word[0..12]);

        // Validate register indices (0 to 26)
        if rd_val < 0 || rd_val > 26 || rs1_val < 0 || rs1_val > 26 || rs2_val < 0 || rs2_val > 26 {
            return Err(format!("Invalid register index detected during decode: Rd={}, Rs1={}, Rs2={}", rd_val, rs1_val, rs2_val));
        }

        // Convert opcode integer to Opcode enum
        let opcode = match opcode_val as u8 {
            0 => Opcode::NOP,
            1 => Opcode::ADD,
            2 => Opcode::ADDI,
            3 => Opcode::SUB,
            4 => Opcode::SUBI,
            5 => Opcode::LDW,
            6 => Opcode::STW,
            7 => Opcode::JMP,
            8 => Opcode::CALL,
            9 => Opcode::RET,
            10 => Opcode::BRZ,
            63 => Opcode::HALT,
            _ => return Err(format!("Unknown opcode: {}", opcode_val)),
        };

        Ok(Instruction {
            opcode,
            rd: rd_val as usize,
            rs1: rs1_val as usize,
            rs2: rs2_val as usize,
            imm: imm_val,
        })
    }

    /// Executes a decoded instruction. Returns true if the CPU should continue running.
    fn execute(&mut self, instruction: &Instruction) -> Result<bool, String> {
        match instruction.opcode {
            Opcode::NOP => {
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::HALT => {
                self.print_register_state();
                Ok(false)
            }
            Opcode::ADD => {
                self.op_add(instruction.rd, instruction.rs1, instruction.rs2);
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::ADDI => {
                self.op_addi(instruction.rd, instruction.rs1, instruction.imm);
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::SUB => {
                self.op_sub(instruction.rd, instruction.rs1, instruction.rs2);
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::SUBI => {
                self.op_subi(instruction.rd, instruction.rs1, instruction.imm);
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::LDW => {
                self.op_ldw(instruction.rd, instruction.rs1, instruction.imm)?;
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::STW => {
                self.op_stw(instruction.rs1, instruction.imm, instruction.rs2)?;
                self.pc = self.next_pc();
                Ok(true)
            }
            Opcode::JMP => {
                self.op_jmp(instruction.imm);
                Ok(true)
            }
            Opcode::CALL => {
                self.op_call(instruction.imm);
                Ok(true)
            }
            Opcode::RET => {
                self.op_ret();
                Ok(true)
            }
            Opcode::BRZ => {
                self.op_brz(instruction.rs1, instruction.imm);
                Ok(true)
            }
        }
    }

    /// Increments the Program Counter by 3 Trytes (1 Word).
    fn next_pc(&self) -> Word {
        // PC always points to the start of an instruction (Word-aligned).
        // Since one instruction is 3 Trytes, we add 3 to the PC value.
        let current_pc_value = word_to_i64(&self.pc);
        btern_core::i64_to_word(current_pc_value + 3)
    }

    /// Executes the ADD instruction. Rd = Rs1 + Rs2.
    /// Assumes registers are addressed by indices 0-26.
    pub fn op_add(&mut self, rd_idx: usize, rs1_idx: usize, rs2_idx: usize) {
        // R0 is the hardwired zero register. Writes to R0 are discarded.
        if rd_idx == 0 {
            return;
        }

        let rs1 = self.gpr[rs1_idx];
        let rs2 = self.gpr[rs2_idx];

        let result = add_words(&rs1, &rs2);

        self.gpr[rd_idx] = result;
    }

    /// Executes the ADDI instruction. Rd = Rs1 + Imm.
    pub fn op_addi(&mut self, rd_idx: usize, rs1_idx: usize, imm: i64) {
        if rd_idx == 0 {
            return;
        }

        let rs1 = self.gpr[rs1_idx];
        let imm_word = i64_to_word(imm);

        let result = add_words(&rs1, &imm_word);

        self.gpr[rd_idx] = result;
    }

    /// Executes the SUB instruction. Rd = Rs1 - Rs2. (A - B = A + (-B))
    pub fn op_sub(&mut self, rd_idx: usize, rs1_idx: usize, rs2_idx: usize) {
        if rd_idx == 0 {
            return;
        }

        let rs1 = self.gpr[rs1_idx];
        let rs2_neg = neg_word(&self.gpr[rs2_idx]);

        let result = add_words(&rs1, &rs2_neg);

        self.gpr[rd_idx] = result;
    }

    /// Executes the SUBI instruction. Rd = Rs1 - Imm. (A - B = A + (-B))
    pub fn op_subi(&mut self, rd_idx: usize, rs1_idx: usize, imm: i64) {
        if rd_idx == 0 {
            return;
        }

        let rs1 = self.gpr[rs1_idx];
        let imm_word_neg = neg_word(&i64_to_word(imm));

        let result = add_words(&rs1, &imm_word_neg);

        self.gpr[rd_idx] = result;
    }

    // --- Memory Access Operations ---

    /// Calculates the effective Tryte address (EA = Rs1 + Imm) and validates it.
    fn calculate_effective_address(&self, rs1_idx: usize, imm: i64) -> Result<usize, String> {
        let rs1_value = word_to_i64(&self.gpr[rs1_idx]);
        let effective_address_value = rs1_value + imm;

        if effective_address_value < 0 {
            return Err(format!("Memory access error: Effective address is negative ({})", effective_address_value));
        }

        let ea = effective_address_value as usize;

        // Check bounds for a 3-tryte Word access
        if ea + 2 >= self.memory.len() {
            return Err(format!("Memory access out of bounds at EA={}", ea));
        }

        Ok(ea)
    }

    /// Executes the LDW instruction. Rd = Mem[Rs1 + Offset].
    pub fn op_ldw(&mut self, rd_idx: usize, rs1_idx: usize, offset: i64) -> Result<(), String> {
        if rd_idx == 0 {
            return Ok(()); // Write to R0 is discarded
        }

        let ea = self.calculate_effective_address(rs1_idx, offset)?;

        let mut loaded_word = [Trit::Z; 27];
        
        // Load 3 Trytes (1 Word)
        loaded_word[0..9].copy_from_slice(&self.memory[ea]);
        loaded_word[9..18].copy_from_slice(&self.memory[ea + 1]);
        loaded_word[18..27].copy_from_slice(&self.memory[ea + 2]);

        self.gpr[rd_idx] = loaded_word;
        Ok(())
    }

    /// Executes the STW instruction. Mem[Rs1 + Offset] = Rs2.
    pub fn op_stw(&mut self, rs1_idx: usize, offset: i64, rs2_idx: usize) -> Result<(), String> {
        let ea = self.calculate_effective_address(rs1_idx, offset)?;
        let data_word = self.gpr[rs2_idx];

        // Store 3 Trytes (1 Word)
        self.memory[ea].copy_from_slice(&data_word[0..9]);
        self.memory[ea + 1].copy_from_slice(&data_word[9..18]);
        self.memory[ea + 2].copy_from_slice(&data_word[18..27]);

        Ok(())
    }

    // --- Control Flow Operations ---

    /// JMP: PC = PC + Offset (Relative jump)
    pub fn op_jmp(&mut self, offset: i64) {
        let current_pc_value = word_to_i64(&self.pc);
        self.pc = i64_to_word(current_pc_value + offset);
    }

    /// CALL: R26 = PC + 3; PC = PC + Offset (R26 is LR, R25 is SP by convention)
    pub fn op_call(&mut self, offset: i64) {
        // Store return address (PC + 3 Trytes) in R26 (Link Register)
        let return_address_value = word_to_i64(&self.pc) + 3;
        self.gpr[26] = i64_to_word(return_address_value);

        // Jump to target address
        self.op_jmp(offset);
    }

    /// RET: PC = R26
    pub fn op_ret(&mut self) {
        // Load return address from R26 (Link Register) into PC
        self.pc = self.gpr[26];
    }

    /// BRZ: Branch if Rs1 == 0.
    pub fn op_brz(&mut self, rs1_idx: usize, offset: i64) {
        // Check if the value in Rs1 is zero
        let is_zero = self.gpr[rs1_idx].iter().all(|&t| t == Trit::Z);
        
        if is_zero {
            // Branch taken: PC = PC + Offset
            self.op_jmp(offset);
        } else {
            // Branch not taken: PC = PC + 3 (next instruction)
            self.pc = self.next_pc();
        }
    }

    /// Prints the state of the general-purpose registers (R0-R26).
    pub fn print_register_state(&self) {
        println!("\n--- Register State ---");
        for i in 0..27 {
            let val_i64 = word_to_i64(&self.gpr[i]);
            let val_trits: String = self.gpr[i].iter().map(|t| t.to_string()).collect();
            
            println!("R{:02}: {:<27} ({})", i, val_trits, val_i64);
        }
        println!("----------------------");
    }
}