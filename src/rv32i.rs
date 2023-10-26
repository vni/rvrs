struct RV32i {
    regs: [u32; 32], // x0 - x31
    pc: u32,
}

impl RV32i {
    pub fn new() -> Self {
        RV32i {
            regs: [0u32; 32],
            pc: 0u32,
        }
    }

    pub fn process_instruction() {
        // add
        // sub
        // or
        // xor
        // and

        // slt
        // sltu

        // sll
        // srl
        // sra

        // addi
        // andi
        // ori
        // xori

        // slti

        // slli
        // srli
        // srai

        // jal
        // jalr

        // lui
        // auipc

        // beq
        // bneq
        // blt
        // bltu
        // bgt
        // bgtu
    }
}
