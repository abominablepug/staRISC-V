fn main() {
    let mut pc: u32 = 0;
    let mut registers: [i32; 32] = [0; 32];
    registers[0] = 0;
    registers[1] = 1;
    registers[2] = 2;

    // ADD x3, x1, x2 = 0x002081B3
    let mut memory: Vec<u8> = vec![0xB3, 0x81, 0x20, 0x00];

    let pc_usize = pc as usize;
    let mut inst_bytes = [0u8; 4];
    inst_bytes.copy_from_slice(&memory[pc_usize..pc_usize + 4]);
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
                0x0 => {
                    if funct7 == 0x00 {
                        // ADD
                        print!(
                            "Added register {} and {} to get {}, storing in register {}",
                            rs1,
                            rs2,
                            registers[rs1 as usize] + registers[rs2 as usize],
                            rd
                        );
                    } else { // SUB
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    // TODO: ADDI, ADD, SUB, AND, OR, XOR, SLT
}
