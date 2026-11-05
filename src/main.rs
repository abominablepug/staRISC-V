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

        let opcode = instruction & 0x7f;
        let rd = (instruction >> 7) & 0x1f;
        let funct3 = (instruction >> 12) & 0x07;
        let rs1 = (instruction >> 15) & 0x1f;
        let rs2 = (instruction >> 20) & 0x1f;
        let funct7 = (instruction >> 25) & 0x7f;

        match opcode {
            0x33 => {
                match funct3 {
                    0x0 => match funct7 {
                        0x00 => { // ADD
                        }
                        0x20 => { // SUB
                        }
                        _ => { // Incorrect funct7
                        }
                    },
                    0x4 => { // XOR
                    }
                    0x6 => { // OR
                    }
                    0x7 => { // AND
                    }
                    0x1 => { // SLR
                    }
                    0x5 => match funct7 {
                        0x00 => { // SRL
                        }
                        0x20 => { // SRA
                        }
                        _ => { // Incorrect funct7
                        }
                    },
                    0x2 => { // SLT
                    }
                    0x3 => { // SLTU
                    }
                    _ => { // Incorrect funct3
                    }
                }
            }
            0x13 => {
                match funct3 {
                    0x0 => { // ADDI
                    }
                    0x4 => { // XORI
                    }
                    0x6 => { // ORI
                    }
                    0x7 => { // ANDI
                    }
                    0x1 => { // SLLI
                    }
                    0x5 => match funct7 {
                        0x00 => { // SRLI
                        }
                        0x20 => { // SRAI
                        }
                        _ => { // Incorrect funct7
                        }
                    },
                    0x2 => { // SLT
                    }
                    0x3 => { // SLTU
                    }
                    _ => { // Incorrect funct3
                    }
                }
            }
            0x03 => {
                match funct3 {
                    0x0 => { // LB
                    }
                    0x1 => { // LH
                    }
                    0x2 => { // LW
                    }
                    0x4 => { // LBU
                    }
                    0x5 => { // LHU
                    }
                    _ => { // Incorrect funct3
                    }
                }
            }
            0x23 => {
                match funct3 {
                    0x0 => { // SB
                    }
                    0x1 => { // SH
                    }
                    0x2 => { // SW
                    }
                    _ => { // Incorrect funct3
                    }
                }
            }
            0x63 => {
                match funct3 {
                    0x0 => { // BEQ
                    }
                    0x1 => { // BNE
                    }
                    0x4 => { // BLT
                    }
                    0x5 => { // BGE
                    }
                    0x6 => { // BLTU
                    }
                    0x7 => { // BGEU
                    }
                    _ => { // Incorrect funct3
                    }
                }
            }
            0x6f => { // JAL
            }
            0x67 => { // JALR
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
