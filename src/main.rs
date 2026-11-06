struct CPU {
    pc: u32,
    registers: [i32; 32],
    memory: Vec<u8>,
}

impl CPU {
    fn new(memory: Vec<u8>) -> Self {
        let cpu = CPU {
            pc: 0,
            registers: [0; 32],
            memory,
        };

        cpu
    }

    fn step(&mut self) {
        let pc_usize = self.pc as usize;
        let mut inst_bytes = [0u8; 4];

        inst_bytes.copy_from_slice(&self.memory[pc_usize..pc_usize + 4]);
        let instruction: u32 = u32::from_le_bytes(inst_bytes);

        let opcode: u32 = instruction & 0x7f;
        let rd: u32 = (instruction >> 7) & 0x1f;
        let funct3: u32 = (instruction >> 12) & 0x07;
        let rs1: u32 = (instruction >> 15) & 0x1f;
        let rs2: u32 = (instruction >> 20) & 0x1f;
        let funct7: u32 = (instruction >> 25) & 0x7f;

        match opcode {
            0x33 => {
                match funct3 {
                    0x0 => match funct7 {
                        0x00 => {
                            // ADD
                            let x = self.registers[rs1 as usize];
                            let y = self.registers[rs2 as usize];
                            let total = x.wrapping_add(y);

                            self.registers[rd as usize] = total;
                        }
                        0x20 => {
                            // SUB
                            let x = self.registers[rs1 as usize];
                            let y = self.registers[rs2 as usize];
                            let total = x.wrapping_sub(y);

                            self.registers[rd as usize] = total;
                        }
                        _ => {
                            // Incorrect funct7
                            println!("Incorrect funct7 provided (either 0x00 or 0x20)");
                        }
                    },
                    0x4 => {
                        // XOR
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        self.registers[rd as usize] = x ^ y;
                    }
                    0x6 => {
                        // OR
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        self.registers[rd as usize] = x | y;
                    }
                    0x7 => {
                        // AND
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        self.registers[rd as usize] = x & y;
                    }
                    0x1 => {
                        // SLR
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        self.registers[rd as usize] = x << y;
                    }
                    0x5 => match funct7 {
                        0x00 => {
                            // SRL
                            let x = self.registers[rs1 as usize];
                            let y = self.registers[rs2 as usize];

                            self.registers[rd as usize] = (x as u32 >> y) as i32;
                        }
                        0x20 => {
                            // SRA
                            let x = self.registers[rs1 as usize];
                            let y = self.registers[rs2 as usize];

                            self.registers[rd as usize] = x >> y;
                        }
                        _ => {
                            // Incorrect funct7
                            println!("Incorrect funct7 provided (either 0x00 or 0x20)");
                        }
                    },
                    0x2 => {
                        // SLT
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        self.registers[rd as usize] = if x < y { 1 } else { 0 };
                    }
                    0x3 => {
                        // SLTU
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        self.registers[rd as usize] = if (x as u32) < (y as u32) { 1 } else { 0 };
                    }
                    _ => {
                        // Impossible funct3 state
                        unreachable!("funct3 is unable to exceed 3 bits");
                    }
                }
                self.pc += 4;
            }
            0x13 => {
                let immu = (funct7 << 5) | rs2;
                let imm = ((immu << 20) as i32) >> 20;
                match funct3 {
                    0x0 => {
                        // ADDI
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = x.wrapping_add(imm);
                    }
                    0x4 => {
                        // XORI
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = x ^ imm;
                    }
                    0x6 => {
                        // ORI
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = x | imm;
                    }
                    0x7 => {
                        // ANDI
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = x & imm;
                    }
                    0x1 => {
                        // SLLI
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = x << imm;
                    }
                    0x5 => match funct7 {
                        0x00 => {
                            // SRLI
                            let x = self.registers[rs1 as usize];

                            self.registers[rd as usize] = (x as u32 >> imm) as i32;
                        }
                        0x20 => {
                            // SRAI
                            let x = self.registers[rs1 as usize];

                            self.registers[rd as usize] = x >> imm;
                        }
                        _ => {
                            // Incorrect funct7
                            println!("Incorrect funct7 provided (either 0x00 or 0x20)");
                        }
                    },
                    0x2 => {
                        // SLT
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = if x < imm { 1 } else { 0 };
                    }
                    0x3 => {
                        // SLTU
                        let x = self.registers[rs1 as usize];

                        self.registers[rd as usize] = if (x as u32) < (imm as u32) { 1 } else { 0 };
                    }
                    _ => {
                        // Impossible funct3 state
                        unreachable!("funct3 is unable to exceed 3 bits");
                    }
                }
                self.pc += 4;
            }
            0x03 => {
                let immu = (funct7 << 5) | rs2;
                let imm = ((immu << 20) as i32) >> 20;
                match funct3 {
                    0x0 => {
                        // LB
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let mut e_byte = [0u8; 1];
                        e_byte.copy_from_slice(&self.memory[e_add..e_add + 1]);

                        self.registers[rd as usize] = i8::from_le_bytes(e_byte) as i32;
                    }
                    0x1 => {
                        // LH
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let mut e_half = [0u8; 2];
                        e_half.copy_from_slice(&self.memory[e_add..e_add + 2]);

                        self.registers[rd as usize] = i16::from_le_bytes(e_half) as i32;
                    }
                    0x2 => {
                        // LW
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let mut e_whole = [0u8; 4];
                        e_whole.copy_from_slice(&self.memory[e_add..e_add + 4]);

                        self.registers[rd as usize] = i32::from_le_bytes(e_whole);
                    }
                    0x4 => {
                        // LBU
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        self.registers[rd as usize] = self.memory[e_add] as i32;
                    }
                    0x5 => {
                        // LHU
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let mut e_half = [0u8; 2];
                        e_half.copy_from_slice(&self.memory[e_add..e_add + 2]);

                        self.registers[rd as usize] = u16::from_le_bytes(e_half) as i32;
                    }
                    _ => {
                        // Incorrect funct3
                        println!("Incorrect funct3 provided (either 0x0, 0x1, 0x2, 0x4, or 0x5)");
                    }
                }
                self.pc += 4;
            }
            0x23 => {
                let immu = (funct7 << 5) | rd;
                let imm = ((immu << 20) as i32) >> 20;
                match funct3 {
                    0x0 => {
                        // SB
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let y = self.registers[rs2 as usize] & 0x00FF;
                        let bytes = y.to_ne_bytes();
                        for i in 0..1 {
                            self.memory[e_add + i] = bytes[i];
                        }
                    }
                    0x1 => {
                        // SH
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let y = self.registers[rs2 as usize];
                        let bytes = y.to_ne_bytes();
                        for i in 0..2 {
                            self.memory[e_add + i] = bytes[i];
                        }
                    }
                    0x2 => {
                        // SW
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let y = self.registers[rs2 as usize];
                        let bytes = y.to_ne_bytes();
                        for i in 0..4 {
                            self.memory[e_add + i] = bytes[i];
                        }
                    }
                    _ => {
                        // Incorrect funct3
                        println!("Incorrect funct3 provided (either 0x0, 0x1, or 0x2)");
                    }
                }
                self.pc += 4;
            }
            0x63 => {
                let imm1 = (instruction >> 31) & 0x01;
                let imm2 = (instruction >> 7) & 0x01;
                let imm3 = (instruction >> 25) & 0x3F;
                let imm4 = (instruction >> 8) & 0x0F;

                let immu = (imm1 << 12) | (imm2 << 11) | (imm3 << 5) | (imm4 << 1);
                let imm = (((immu << 19) as i32) >> 19) as u32;
                match funct3 {
                    0x0 => {
                        // BEQ
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x == y {
                            self.pc = self.pc.wrapping_add(imm);
                        } else {
                            self.pc += 4;
                        }
                    }
                    0x1 => {
                        // BNE
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x != y {
                            self.pc = self.pc.wrapping_add(imm);
                        } else {
                            self.pc += 4;
                        }
                    }
                    0x4 => {
                        // BLT
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x < y {
                            self.pc = self.pc.wrapping_add(imm);
                        } else {
                            self.pc += 4;
                        }
                    }
                    0x5 => {
                        // BGE
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x >= y {
                            self.pc = self.pc.wrapping_add(imm);
                        } else {
                            self.pc += 4;
                        }
                    }
                    0x6 => {
                        // BLTU
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if (x as u32) < (y as u32) {
                            self.pc = self.pc.wrapping_add(imm);
                        } else {
                            self.pc += 4;
                        }
                    }
                    0x7 => {
                        // BGEU
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if (x as u32) >= (y as u32) {
                            self.pc = self.pc.wrapping_add(imm);
                        } else {
                            self.pc += 4;
                        }
                    }
                    _ => {
                        // Incorrect funct3
                        println!(
                            "Incorrect funct3 provided (either 0x0, 0x1, 0x4, 0x5, 0x6, or 0x7)"
                        );
                    }
                }
            }
            0x6f => {
                // JAL
                let imm1 = (instruction >> 31) & 0x01;
                let imm2 = (instruction >> 21) & 0x3FF;
                let imm3 = (instruction >> 20) & 0x01;
                let imm4 = (instruction >> 12) & 0xFF;

                let immu = (imm1 << 20) | (imm4 << 12) | (imm3 << 11) | (imm2 << 1);
                let imm = ((immu << 11) as i32) >> 11;

                self.registers[rd as usize] = (self.pc + 4) as i32;
                self.pc = self.pc.wrapping_add(imm as u32);
            }
            0x67 => {
                // JALR
                let immu = (funct7 << 5) | rs2;
                let imm = ((immu << 20) as i32) >> 20;

                let x = self.registers[rs1 as usize];
                let target_u = imm.wrapping_add(x);
                let target = target_u & !1;

                self.registers[rd as usize] = (self.pc + 4) as i32;
                self.pc = target as u32;
            }
            0x37 => {
                // LUI
                let imm = (((((funct7 << 5) | rs2) << 5) | rs1) << 3) | funct3;

                self.registers[rd as usize] = (imm << 12) as i32;
                self.pc += 4;
            }
            0x17 => {
                // AUIPC
                let imm = (((((funct7 << 5) | rs2) << 5) | rs1) << 3) | funct3;

                self.registers[rd as usize] = (self.pc as i32) + (imm << 12) as i32;
                self.pc += 4;
            }
            _ => {
                // Incorrect Opcode
                println!(
                    "Incorrect opcode provided (either 0x33, 0x13, 0x03, 0x23, 0x63, 0x6f, 0x67, 0x37, or 0x17)"
                );
            }
        }
    }
}

fn main() {
    // 256 bytes of RAM initialized to 0
    let mut memory: Vec<u8> = vec![0; 256];

    let program: [u8; 112] = [
        0xB7, 0xE0, 0xCD, 0xAB, 0x93, 0x80, 0x00, 0xFF, 0x13, 0x01, 0x80, 0x00, 0xB3, 0xD1, 0x20,
        0x40, 0x33, 0xD2, 0x20, 0x00, 0xB3, 0xA2, 0x41, 0x00, 0x33, 0xB3, 0x41, 0x00, 0x93, 0x03,
        0x00, 0x08, 0x23, 0xA0, 0x13, 0x00, 0x03, 0x84, 0x03, 0x00, 0x83, 0xC4, 0x03, 0x00, 0x03,
        0x95, 0x23, 0x00, 0x83, 0xD5, 0x23, 0x00, 0x13, 0x06, 0x50, 0x05, 0xA3, 0x80, 0xC3, 0x00,
        0x83, 0xA6, 0x03, 0x00, 0x13, 0x07, 0x30, 0x00, 0x93, 0x07, 0x00, 0x00, 0xB3, 0x87, 0xE7,
        0x00, 0x13, 0x07, 0xF7, 0xFF, 0xE3, 0x1C, 0x07, 0xFE, 0x6F, 0x08, 0xC0, 0x00, 0x93, 0x08,
        0x30, 0x06, 0x6F, 0x00, 0x00, 0x01, 0x17, 0x19, 0x00, 0x00, 0xB3, 0x09, 0xF9, 0x40, 0x67,
        0x0A, 0x18, 0x00, 0x6F, 0x00, 0x00, 0x00,
    ];
    memory[0..program.len()].copy_from_slice(&program);
    let mut cpu = CPU::new(memory);

    let mut cycles = 0;
    println!("--- Executing Stress Test ---");
    loop {
        if cpu.pc == 0x6C || cycles > 50 {
            println!("Halted at PC: 0x{:02X} after {} cycles\n", cpu.pc, cycles);
            break;
        }
        cpu.step();
        cpu.registers[0] = 0;
        cycles += 1;
    }

    println!("--- Register Verification ---");
    println!(
        "x1  (LUI + ADDI) : {:11} (Expected: -1412571152 / 0xABCDDFF0)",
        cpu.registers[1]
    );
    println!(
        "x3  (SRA)        : {:11} (Expected:    -5517857 / 0xFFABCDDF)",
        cpu.registers[3]
    );
    println!(
        "x4  (SRL)        : {:11} (Expected:    11259359 / 0x00ABCDDF)",
        cpu.registers[4]
    );
    println!(
        "x5  (SLT)        : {:11} (Expected:           1)",
        cpu.registers[5]
    );
    println!(
        "x6  (SLTU)       : {:11} (Expected:           0)",
        cpu.registers[6]
    );
    println!(
        "x8  (LB)         : {:11} (Expected:         -16)",
        cpu.registers[8]
    );
    println!(
        "x9  (LBU)        : {:11} (Expected:         240)",
        cpu.registers[9]
    );
    println!(
        "x10 (LH)         : {:11} (Expected:      -21555)",
        cpu.registers[10]
    );
    println!(
        "x11 (LHU)        : {:11} (Expected:       43981)",
        cpu.registers[11]
    );
    println!(
        "x13 (SB + LW)    : {:11} (Expected: -1412606480 / 0xABCD55F0)",
        cpu.registers[13]
    );
    println!(
        "x14 (Loop Cnt)   : {:11} (Expected:           0)",
        cpu.registers[14]
    );
    println!(
        "x15 (Loop Sum)   : {:11} (Expected:           6)",
        cpu.registers[15]
    );
    println!(
        "x16 (JAL Link)   : {:11} (Expected:          88 / 0x58)",
        cpu.registers[16]
    );
    println!(
        "x17 (Post-Ret)   : {:11} (Expected:          99)",
        cpu.registers[17]
    );
    println!(
        "x18 (AUIPC)      : {:11} (Expected:        4192 / 0x1060)",
        cpu.registers[18]
    );
    println!(
        "x19 (Subroutine) : {:11} (Expected:        4186)",
        cpu.registers[19]
    );
    println!(
        "x20 (JALR Link)  : {:11} (Expected:         108 / 0x6C)",
        cpu.registers[20]
    );
}
