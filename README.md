# staRISC-V

staRisc-V is a 32-bit RISC-V (RV32I) instruction set emulator written in Rust capable of processing and executing binaries. The name is space-themed because I like space!

---

## Overview

The goal of staRISC-V was to learn more about RISC-V, ISA, and systems-level programming in Rust. Through the creation of the emulator, I was able to learn all this and more (including things like the difference between sign extend and zero extend) with the end result even letting me execute RISC-V assembly using my own simulated CPU. The exact specifics of the project can be seem below:

- Architecture: RISC-V 32-bit (`RV32I`)
- Registers: 32 registers (`x0` hardwired to zero) + the Program Counter (`pc`)
- Memory: 64 KB flat address space
- MMIO Support: Simulated UART output (`0x1000_0000`)
- Halt Mechanism: Clean termination upon halt instruction (`0x0000006f`)

---

## Supported Instructions

- Arithmetic & Logic (R-type): `ADD`, `SUB`, `SLL`, `SLT`, `SLTU`, `XOR`, `SRL`, `SRA`, `OR`, `AND`
- Immediate Operations (I-type): `ADDI`, `SLTI`, `SLTIU`, `XORI`, `ORI`, `ANDI`, `SLLI`, `SRLI`, `SRAI`
- Load & Store (I-type / S-type): `LB`, `LH`, `LW`, `LBU`, `LHU`, `SB`, `SH`, `SW`
- Branches (B-type): `BEQ`, `BNE`, `BLT`, `BGE`, `BLTU`, `BGEU`
- Upper Immediates (U-type): `LUI`, `AUIPC`
- Jumps (J-type / I-type): `JAL`, `JALR`

---

## How to Run

1. Compile `guest.rs` into an ELF:
   ```bash
   rustc --target riscv32i-unknown-none-elf -C link-arg=-Tlink.x guest.rs -o guest.elf
   ```

2. Extract the instruction bytes into a binary:
   ```bash
   rust-objcopy -O binary guest.elf guest.bin
   ```

3. Execute in the emulator:
   ```bash
   cargo run
   ```
Repeat steps 1 and 2 whenever the `guest.rs` file is changed.

---

## Example Output

This example uses the Fibonacci Sequence program I wrote.
<p align="center">
   <img width="502" height="522" alt="screenshot_2026-11-07_14-53-43" src="https://github.com/user-attachments/assets/e9857ba0-7cb5-4820-b0b2-7cbcefcc2b1e" />
</p>
Theoretically the size of the registers (32 bit) allows the program to calculate up to F<sub>46</sub>. Though for viewing purposes I reduced it.
