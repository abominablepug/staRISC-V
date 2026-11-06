struct CPU {
    pc: u32,
    registers: [i32; 32],
    memory: Vec<u8>,
}

impl CPU {
    fn new(memory: Vec<u8>) -> Self {
        let mut cpu = CPU {
            pc: 0,
            registers: [0; 32],
            memory,
        };

        cpu.registers[1] = 1;
        cpu.registers[2] = 2;
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
            }
            0x13 => {
                let immu = (funct7 << 5) & rs2;
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
            }
            0x03 => {
                let immu = (funct7 << 5) & rs2;
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
            }
            0x23 => {
                let immu = (funct7 << 5) & rs2;
                let imm = ((immu << 20) as i32) >> 20;
                match funct3 {
                    0x0 => {
                        // SB
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let y = self.registers[rs2 as usize] & 0x000F;
                        let bytes = y.to_ne_bytes();
                        for i in 0..4 {
                            self.memory[e_add + i] = bytes[i];
                        }
                    }
                    0x1 => {
                        // SH
                        let x = self.registers[rs1 as usize];
                        let e_add = imm.wrapping_add(x) as usize;

                        let y = self.registers[rs2 as usize] & 0x00FF;
                        let bytes = y.to_ne_bytes();
                        for i in 0..4 {
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
            }
            0x63 => {
                let immu = (funct7 << 5) & rs2;
                let imm = (((immu << 20) as i32) >> 20) as u32;
                match funct3 {
                    0x0 => {
                        // BEQ
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x == y {
                            self.pc += imm
                        }
                    }
                    0x1 => {
                        // BNE
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x != y {
                            self.pc += imm
                        }
                    }
                    0x4 => {
                        // BLT
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x < y {
                            self.pc += imm
                        }
                    }
                    0x5 => {
                        // BGE
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if x >= y {
                            self.pc += imm
                        }
                    }
                    0x6 => {
                        // BLTU
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if (x as u32) < (y as u32) {
                            self.pc += imm
                        }
                    }
                    0x7 => {
                        // BGEU
                        let x = self.registers[rs1 as usize];
                        let y = self.registers[rs2 as usize];

                        if (x as u32) >= (y as u32) {
                            self.pc += imm
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
                let imm1 = ((((funct7 >> 6) << 5) | rs1) << 3) | funct3; // 20-12
                let imm2 = ((((rs2 & 0x0001) << 6) | (funct7 & 0x003f)) << 4) | (rs2 >> 1); // 11-1
                let imm = ((((imm1 << 11) | imm2) << 12) as i32) >> 12;

                self.registers[rd as usize] = (self.pc + 4) as i32;
                self.pc += imm as u32;
            }
            0x67 => {
                // JALR
                let immu = (funct7 << 5) & rs2;
                let imm = ((immu << 20) as i32) >> 20;

                let x = self.registers[rs1 as usize];
                let target_u = imm.wrapping_add(x);
                let target = target_u & 0xfffd;

                self.registers[rd as usize] = (self.pc + 4) as i32;
                self.pc = target as u32;
            }
            0x37 => { // LUI
            }
            0x17 => { // AUIPC
            }
            0x73 => {
                match funct3 {
                    0x0 => { // ECALL
                    }
                    0x1 => { // EBREAK
                    }
                    _ => { // Incorrect funct3
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    // ADD x3, x1, x2 = 0x002081B3
    let memory: Vec<u8> = vec![0xB3, 0x81, 0x20, 0x00];

    let mut cpu = CPU::new(memory);

    loop {
        cpu.step();
        cpu.pc += 4;
        cpu.registers[0] = 0;
    }
}
