# btern Daily

This living document outlines the required work practices, tracks completed milestones, and maintains the current roadmap for the btern computing architecture project.

---

## I. Required Work Practices

These practices must be strictly followed by all collaborators to ensure project consistency and maintainability.

| Practice | Requirement |
| :--- | :--- |
| **1. Living Document Update** | Immediately update this "btern Daily" document with any and all work completed on the project once finished. |
| **2. Code Quality** | Maintain high code quality, including clear variable naming, comprehensive comments, and adherence to Rust best practices. |
| **3. Test-Driven Development (TBD)** | Future critical features should be developed with unit or integration tests. |

---

## II. Completed Work (Alpha State)

The core toolchain is functional and verified, achieving the Minimal Viable Architecture (MVA) state.

### Core Architecture (`btern_core`)
*   Balanced ternary data types (`Trit`, `Word`, `Tryte`) implemented.
*   Core arithmetic logic (`add_words`, `neg_word`, `i64_to_word`, `word_to_i64`) implemented.
*   Instruction encoding logic defined.

### Emulator (`bemu`)
*   CPU structure, memory, and the Fetch-Decode-Execute (FDE) cycle implemented.
*   All initial Instruction Set Architecture (ISA) categories implemented:
    *   ALU: `ADD`, `ADDI`, `SUB`, `SUBI`.
    *   Memory: `LDW`, `STW`.
    *   Control Flow: `JMP`, `CALL`, `RET`, `BRZ`, `HALT`.
*   Verified execution of a test program (R3 = 15).

### Assembler (`basm`)
*   Initial instruction encoding and machine code generation implemented.
*   Successfully generated executable binary (`test_program.bin`).

---

## III. Project Roadmap

The roadmap focuses on evolving the toolchain from a minimal prototype to a fully functional platform.

### Milestone 2: Assembler Feature Parity (Focus: `basm` development)
| Task | Description | Status |
| :--- | :--- | :--- |
| Implement full assembly parser | Replace hardcoded test program with a parser capable of reading `.basm` files. | Pending |
| Implement symbol table & labels | Enable branching and function calls using symbolic names. | Pending |
| Implement assembler directives | Support data definition and memory allocation (`.word`, `.tryte`). | Pending |
| Implement proper binary encoding | Optimize output by packing 4 trits into 1 byte (BCT) for file size efficiency. | Pending |

### Milestone 3: Advanced Emulator Features (Focus: `bemu` expansion)
| Task | Description | Status |
| :--- | :--- | :--- |
| Implement external I/O | Add support for basic terminal input/output (e.g., `PUT`, `GET` instructions). | Pending |
| Implement remaining ISA | Add shifting and logical operations. | Pending |
| Implement basic debugging | Add features like breakpoints and single-step execution. | Pending |