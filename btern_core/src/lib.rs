// lib.rs - Core data types and math functions for the btern architecture.

use std::fmt;
use std::ops::Neg;

// --- Trit Module ---

/// Represents a single balanced ternary digit {-1, 0, +1}.
/// Using a C-style enum with explicit discriminants for clarity.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(i8)]
pub enum Trit {
    N = -1, // Negative
    #[default]
    Z = 0,  // Zero
    P = 1,  // Positive
}

impl Trit {
    /// Returns the signed integer value of the Trit (-1, 0, or 1).
    pub fn to_i8(self) -> i8 {
        self as i8
    }

    /// Converts an integer into a Trit. Returns an error if the value is invalid.
    pub fn from_i8(val: i8) -> Result<Self, &'static str> {
        match val {
            -1 => Ok(Trit::N),
            0 => Ok(Trit::Z),
            1 => Ok(Trit::P),
            _ => Err("Invalid integer value for Trit; must be -1, 0, or 1."),
        }
    }

    /// Converts a Trit into its 2-bit Binary Coded Ternary (BCT) representation.
    /// -1 (N) -> 00
    ///  0 (Z) -> 01
    /// +1 (P) -> 10
    pub fn to_bct(self) -> u8 {
        match self {
            Trit::N => 0b00,
            Trit::Z => 0b01,
            Trit::P => 0b10,
        }
    }

    /// Creates a Trit from its 2-bit BCT representation.
    pub fn from_bct(bct: u8) -> Result<Self, &'static str> {
        match bct & 0b11 { // Mask to ensure we only look at 2 bits
            0b00 => Ok(Trit::N),
            0b01 => Ok(Trit::Z),
            0b10 => Ok(Trit::P),
            _ => Err("Invalid BCT value; must be 00, 01, or 10."),
        }
    }
}

/// Implement the Neg trait for Trit, allowing us to use the `-` operator.
/// e.g., -Trit::P == Trit::N
impl Neg for Trit {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Trit::N => Trit::P,
            Trit::Z => Trit::Z,
            Trit::P => Trit::N,
        }
    }
}

/// Custom display for printing Trits.
impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trit::N => write!(f, "-"),
            Trit::Z => write!(f, "0"),
            Trit::P => write!(f, "+"),
        }
    }
}

// --- Types Module ---

/// A Word is 27 trits, the native data size of the btern processor's registers.
pub type Word = [Trit; 27];

/// A Tryte is 9 trits, the fundamental addressable unit of memory.
pub type Tryte = [Trit; 9];

// --- Instruction Set Definition ---

/// Defines the instruction opcodes.
/// Opcodes are 6 trits (3^6 = 729 possible instructions).
/// We assign small positive integers for easy encoding.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    NOP = 0,
    ADD = 1,    // Rd = Rs1 + Rs2 (3-Reg)
    ADDI = 2,   // Rd = Rs1 + Imm (Reg-Imm)
    SUB = 3,    // Rd = Rs1 - Rs2 (3-Reg)
    SUBI = 4,   // Rd = Rs1 - Imm (Reg-Imm)
    LDW = 5,    // Rd = Mem[Rs1 + Offset] (I-Type)
    STW = 6,    // Mem[Rs1 + Offset] = Rs2 (I-Type)
    JMP = 7,    // PC = PC + Offset (J-Type)
    CALL = 8,   // R26 = PC + 1; PC = PC + Offset (J-Type)
    RET = 9,    // PC = R26 (Reg)
    BRZ = 10,   // if (Rcond == 0) PC = PC + Offset (B-Type)
    // Placeholder for other instructions...
    HALT = 63, // Arbitrary high value for termination
}

/// Represents a decoded instruction.
#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rd: usize,      // Destination Register Index (0-26)
    pub rs1: usize,     // Source 1 Register Index (0-26)
    pub rs2: usize,     // Source 2 Register Index (0-26)
    pub imm: i64,       // Immediate/Offset value (12 trits, signed)
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            opcode: Opcode::NOP,
            rd: 0,
            rs1: 0,
            rs2: 0,
            imm: 0,
        }
    }
}

// --- Math Module ---

/// Performs balanced ternary addition on three trits (A, B, and Carry_in).
/// Returns a tuple (sum_trit, carry_out_trit).
/// The sum of three trits is in the range [-3, 3].
/// The resulting sum trit S is in [-1, 1] and the carry C is in [-1, 1].
/// A + B + C_in = 3 * C_out + S
pub fn add_trits(a: Trit, b: Trit, carry_in: Trit) -> (Trit, Trit) {
    let sum_val = a.to_i8() + b.to_i8() + carry_in.to_i8();
    
    // Determine the carry-out (C_out)
    let carry_out_val = match sum_val {
        -3..=-2 => -1, // Sum is -3 or -2, C_out must be -1
        -1..=1 => 0,   // Sum is -1, 0, or 1, C_out is 0
        2..=3 => 1,    // Sum is 2 or 3, C_out must be 1
        _ => unreachable!(),
    };

    // Determine the sum trit (S)
    // S = Sum_val - 3 * C_out_val
    let sum_trit_val = sum_val - 3 * carry_out_val;
    
    // Conversion to Trit is safe as sum_trit_val is guaranteed to be -1, 0, or 1
    let sum_trit = Trit::from_i8(sum_trit_val).unwrap();
    let carry_out_trit = Trit::from_i8(carry_out_val).unwrap();

    (sum_trit, carry_out_trit)
}

/// Performs balanced ternary addition of two Words (27 trits).
/// Returns the resulting Word.
pub fn add_words(a: &Word, b: &Word) -> Word {
    let mut result = [Trit::Z; 27];
    let mut carry = Trit::Z;

    // Iterate from the LSB (index 0) to the MSB (index 26)
    for i in 0..27 {
        let (sum, new_carry) = add_trits(a[i], b[i], carry);
        result[i] = sum;
        carry = new_carry;
    }

    // NOTE: If carry != Trit::Z after the loop, it indicates an overflow.
    
    result
}

/// Performs trit-wise negation of a Word.
pub fn neg_word(word: &Word) -> Word {
    let mut result = [Trit::Z; 27];
    for i in 0..27 {
        result[i] = -word[i];
    }
    result
}

/// Converts a slice of balanced trits into a signed i64 integer.
/// The trits must be ordered from LSB (index 0) to MSB.
pub fn trits_to_i64(trits: &[Trit]) -> i64 {
    let mut value: i64 = 0;
    let mut power_of_three: i64 = 1;

    for trit in trits.iter() {
        let trit_val = trit.to_i8() as i64;
        value += trit_val * power_of_three;
        power_of_three *= 3;
    }
    value
}

/// Converts a balanced ternary Word (27 trits) into a signed i64 integer.
pub fn word_to_i64(word: &Word) -> i64 {
    trits_to_i64(word)
}

/// Converts a signed i64 integer into a balanced ternary Word (27 trits).
/// This is the inverse of word_to_i64.
pub fn i64_to_word(mut value: i64) -> Word {
    let mut word = [Trit::Z; 27];
    let mut i = 0;

    while value != 0 && i < 27 {
        // The remainder when dividing by 3 will be 0, 1, or 2 (unbalanced ternary).
        let rem = value % 3;
        
        // Convert unbalanced remainder (0, 1, 2) to balanced trit (-1, 0, 1)
        let trit_val = match rem {
            0 => 0,
            1 => 1,
            2 => -1, // 2 mod 3 is equivalent to -1 mod 3, carry is +1
            _ => unreachable!(),
        };

        word[i] = Trit::from_i8(trit_val as i8).unwrap();
        
        // Calculate the next value for iteration by handling the carry/borrow
        value = (value - trit_val) / 3;
        
        i += 1;
    }

    word
}

/// Converts a signed i64 integer into a balanced ternary Word of a specific size (e.g., 3 trits for register index).
/// This is used for encoding small fields within a Word.
fn i64_to_trits_fixed_size(mut value: i64, size: usize) -> Vec<Trit> {
    let mut trits = vec![Trit::Z; size];
    let mut i = 0;

    while value != 0 && i < size {
        let rem = value % 3;
        
        let trit_val = match rem {
            0 => 0,
            1 => 1,
            2 => -1,
            _ => unreachable!(),
        };

        trits[i] = Trit::from_i8(trit_val as i8).unwrap();
        
        value = (value - trit_val) / 3;
        
        i += 1;
    }

    trits
}

/// Encodes an Instruction struct into a 27-trit Word.
/// Format (LSB to MSB): [Imm/Offset: 12 | Rs2: 3 | Rs1: 3 | Rd: 3 | Opcode: 6]
pub fn encode_instruction(inst: &Instruction) -> Word {
    let mut word = [Trit::Z; 27];
    let mut current_idx = 0;

    // 1. Imm/Offset (12 trits, indices 0..11)
    let imm_trits = i64_to_trits_fixed_size(inst.imm, 12);
    word[current_idx..current_idx + 12].copy_from_slice(&imm_trits);
    current_idx += 12;

    // 2. Rs2 (3 trits, indices 12..14)
    let rs2_trits = i64_to_trits_fixed_size(inst.rs2 as i64, 3);
    word[current_idx..current_idx + 3].copy_from_slice(&rs2_trits);
    current_idx += 3;

    // 3. Rs1 (3 trits, indices 15..17)
    let rs1_trits = i64_to_trits_fixed_size(inst.rs1 as i64, 3);
    word[current_idx..current_idx + 3].copy_from_slice(&rs1_trits);
    current_idx += 3;

    // 4. Rd (3 trits, indices 18..20)
    let rd_trits = i64_to_trits_fixed_size(inst.rd as i64, 3);
    word[current_idx..current_idx + 3].copy_from_slice(&rd_trits);
    current_idx += 3;

    // 5. Opcode (6 trits, indices 21..26)
    let opcode_trits = i64_to_trits_fixed_size(inst.opcode as i64, 6);
    word[current_idx..current_idx + 6].copy_from_slice(&opcode_trits);
    // current_idx += 6; // Should equal 27 now

    word
}