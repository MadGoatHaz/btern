# The btern Processor Project (BPP): The Future is Ternary

![btern Logo](https://madgoathaz.github.io/btern/btern.png)

[![GitHub Pages Status](https://github.com/MadGoatHaz/btern/actions/workflows/pages.yml/badge.svg)](https://madgoathaz.github.io/btern/)

**[View the Interactive Project Webpage Here](https://madgoathaz.github.io/btern/)**

A community-driven initiative to perfect digital computing—creating the ultimate architecture for AI and generations to come using balanced ternary logic.

## 🚀 Project Vision

The btern project aims to establish the definitive architecture for digital speed and efficiency by adopting balanced ternary logic (trits: -1, 0, +1). This system provides maximum information density and is inherently suited for advanced AI workloads, acting as the ideal classical partner for future quantum systems.

## ⚙️ Core Architectural Specification

| Unit | Size | Description |
| :--- | :--- | :--- |
| **Trit** | 1 trit | The fundamental unit, representing -1, 0, or +1. |
| **Tryte** | 9 trits | Smallest addressable memory unit (19,683 values). |
| **Word** | 27 trits | Native register size and instruction length (3 trytes). |
| **Registers** | 27 GPRs (R0-R26) | All 27 trits wide. R0 is conventionally the zero register. |

**Instruction Format (Alpha Implementation):**
All instructions are fixed at one word (27 trits) in length.
Format (LSB to MSB): `[Imm/Offset: 12 trits | Src2 Reg: 3 trits | Src1 Reg: 3 trits | Dest Reg: 3 trits | Opcode: 6 trits]`

## ✅ Alpha State Toolchain (Phase 1 Completed)

The core toolchain is built in Rust and verified to execute basic arithmetic and control flow instructions.

| Component | Status | Description |
| :--- | :--- | :--- |
| **bemu (Emulator)** | **Alpha** | Implemented CPU, memory, FDE cycle, and all initial ISA instructions (ALU, Memory, Control Flow). Verified execution of a test program (R3 = 15). |
| **basm (Assembler)** | **Alpha** | Implemented instruction encoding and successfully generated executable binary machine code. |
| **Documentation** | **Living** | The "btern Daily" document tracks progress, practices, and the evolving roadmap. |

## 🗺️ Development Roadmap (Current Focus: Phase 2)

### Milestone 2: Assembler Feature Parity
| Task | Description | Status |
| :--- | :--- | :--- |
| Implement full assembly parser | Replace hardcoded test program with a parser capable of reading `.basm` files. | Pending |
| Implement symbol table & labels | Enable branching and function calls using symbolic names. | Pending |
| Implement assembler directives | Support data definition and memory allocation (`.word`, `.tryte`). | Pending |
| Implement proper binary encoding | Optimize output by packing 4 trits into 1 byte (BCT) for file size efficiency. | Pending |

### Milestone 3: Advanced Emulator Features
| Task | Description | Status |
| :--- | :--- | :--- |
| Implement external I/O | Add support for basic terminal input/output (e.g., `PUT`, `GET` instructions). | Pending |
| Implement remaining ISA | Add shifting and logical operations. | Pending |
| Implement basic debugging | Add features like breakpoints and single-step execution. | Pending |

## 🤝 Contributing

We are an open-source project managed by a commitment to absolute transparency. Refer to the [`btern Daily.md`](btern Daily.md) for required work practices and the most current roadmap.