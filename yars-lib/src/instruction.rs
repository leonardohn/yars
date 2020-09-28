use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InstructionFormat {
    //  /* | 31        25 | 24 20 | 19 15 | 14  12 | 11        7 | 6    0 | */
    R,  /* | ---funct7--- | -rs2- | -rs1- | funct3 | ----rd----- | opcode | */
    R4, /* | rs3 | funct2 | -rs2- | -rs1- | funct3 | ----rd----- | opcode | */
    I,  /* | -----imm[11:0]------ | -rs1- | funct3 | ----rd----- | opcode | */
    S,  /* | -imm[11:5]-- | -rs2- | -rs1- | funct3 | -imm[4:0]-- | opcode | */
    B,  /* | imm[12|10:5] | -rs2- | -rs1- | funct3 | imm[4:1|11] | opcode | */
    U,  /* | -------------imm[31:12]-------------- | ----rd----- | opcode | */
    J,  /* | --------imm[20|10:1|11|19:12]-------- | ----rd----- | opcode | */
}

const INSTRUCTION_FORMATS: [Option<InstructionFormat>; 32] = [
    /* 00000 */ Some(InstructionFormat::I),
    /* 00001 */ Some(InstructionFormat::I),
    /* 00010 */ None,
    /* 00011 */ Some(InstructionFormat::I),
    /* 00100 */ Some(InstructionFormat::I),
    /* 00101 */ Some(InstructionFormat::U),
    /* 00110 */ Some(InstructionFormat::I),
    /* 00111 */ None,
    /* 01000 */ Some(InstructionFormat::S),
    /* 01001 */ Some(InstructionFormat::S),
    /* 01010 */ None,
    /* 01011 */ Some(InstructionFormat::R),
    /* 01100 */ Some(InstructionFormat::R),
    /* 01101 */ Some(InstructionFormat::U),
    /* 01110 */ Some(InstructionFormat::R),
    /* 01111 */ None,
    /* 10000 */ Some(InstructionFormat::R4),
    /* 10001 */ Some(InstructionFormat::R4),
    /* 10010 */ Some(InstructionFormat::R4),
    /* 10011 */ Some(InstructionFormat::R4),
    /* 10100 */ Some(InstructionFormat::R),
    /* 10101 */ None,
    /* 10110 */ None,
    /* 10111 */ None,
    /* 11000 */ Some(InstructionFormat::B),
    /* 11001 */ Some(InstructionFormat::I),
    /* 11010 */ None,
    /* 11011 */ Some(InstructionFormat::J),
    /* 11100 */ Some(InstructionFormat::I),
    /* 11101 */ None,
    /* 11110 */ None,
    /* 11111 */ None,
];

impl InstructionFormat {
    pub fn from_opcode(opcode: u8) -> Option<Self> {
        match opcode & 3 {
            3 => INSTRUCTION_FORMATS[(opcode as usize >> 2) & 0x1F],
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FenceKind {
    R = 0b10,
    W = 0b01,
    RW = 0b11,
}

impl fmt::Display for FenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::R => write!(f, "r"),
            Self::W => write!(f, "w"),
            Self::RW => write!(f, "rw"),
        }
    }
}

type Register = u8;

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /* --- RV32I --- */
 
    // Load
    LUI { rd: Register, imm: i32 },
    LB { rd: Register, rs1: Register, imm: i16 },
    LH { rd: Register, rs1: Register, imm: i16 },
    LW { rd: Register, rs1: Register, imm: i16 },
    LBU { rd: Register, rs1: Register, imm: i16 },
    LHU { rd: Register, rs1: Register, imm: i16 },

    // Store
    SB { rs1: Register, rs2: Register, imm: i16 },
    SH { rs1: Register, rs2: Register, imm: i16 },
    SW { rs1: Register, rs2: Register, imm: i16 },

    // Shift
    SLLI { rd: Register, rs1: Register, shamt: u16 },
    SRLI { rd: Register, rs1: Register, shamt: u16 },
    SRAI { rd: Register, rs1: Register, shamt: u16 },
    SLL { rd: Register, rs1: Register, rs2: Register },
    SRL { rd: Register, rs1: Register, rs2: Register },
    SRA { rd: Register, rs1: Register, rs2: Register },

    // Arithmetic
    ADDI { rd: Register, rs1: Register, imm: i16 },
    ADD { rd: Register, rs1: Register, rs2: Register },
    SUB { rd: Register, rs1: Register, rs2: Register },

    // Logical
    ORI { rd: Register, rs1: Register, imm: i16 },
    XORI { rd: Register, rs1: Register, imm: i16 },
    ANDI { rd: Register, rs1: Register, imm: i16 },
    OR { rd: Register, rs1: Register, rs2: Register },
    XOR { rd: Register, rs1: Register, rs2: Register },
    AND { rd: Register, rs1: Register, rs2: Register },

    // Compare
    SLTI { rd: Register, rs1: Register, imm: i16 },
    SLTIU { rd: Register, rs1: Register, imm: i16 },
    SLT { rd: Register, rs1: Register, rs2: Register },
    SLTU { rd: Register, rs1: Register, rs2: Register },

    // Branch
    BEQ { rs1: Register, rs2: Register, imm: i16 },
    BNE { rs1: Register, rs2: Register, imm: i16 },
    BLT { rs1: Register, rs2: Register, imm: i16 },
    BGE { rs1: Register, rs2: Register, imm: i16 },
    BLTU { rs1: Register, rs2: Register, imm: i16 },
    BGEU { rs1: Register, rs2: Register, imm: i16 },

    // Jump
    JAL { rd: Register, imm: i32 },
    AUIPC { rd: Register, imm: i32 },
    JALR { rd: Register, rs1: Register, imm: i16 },

    // Sync
    FENCE { pred: FenceKind, succ: FenceKind },
    FENCETSO,

    // System
    ECALL,
    EBREAK,

    /* --- RVZifencei --- */

    FENCEI,

    /* --- RVZicsr --- */

    CSRRWI { rd: Register, uimm: u8, csr: u16 },
    CSRRSI { rd: Register, uimm: u8, csr: u16 },
    CSRRCI { rd: Register, uimm: u8, csr: u16 },
    CSRRW { rd: Register, rs1: Register, csr: u16 },
    CSRRS { rd: Register, rs1: Register, csr: u16 },
    CSRRC { rd: Register, rs1: Register, csr: u16 },

    /* --- RV32M --- */

    MUL { rd: Register, rs1: Register, rs2: Register },
    MULH { rd: Register, rs1: Register, rs2: Register },
    MULHSU { rd: Register, rs1: Register, rs2: Register },
    MULHU { rd: Register, rs1: Register, rs2: Register },
    DIV { rd: Register, rs1: Register, rs2: Register },
    DIVU { rd: Register, rs1: Register, rs2: Register },
    REM { rd: Register, rs1: Register, rs2: Register },
    REMU { rd: Register, rs1: Register, rs2: Register },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match self {
            LUI { rd, imm } => write!(f, "lui x{},{}", rd, imm),
            LB { rd, rs1, imm } => write!(f, "lb x{},{}(x{})", rd, imm, rs1),
            LH { rd, rs1, imm } => write!(f, "lh x{},{}(x{})", rd, imm, rs1),
            LW { rd, rs1, imm } => write!(f, "lw x{},{}(x{})", rd, imm, rs1),
            LBU { rd, rs1, imm } => write!(f, "lbu x{},{}(x{})", rd, imm, rs1),
            LHU { rd, rs1, imm } => write!(f, "lhu x{},{}(x{})", rd, imm, rs1),
            SB { rs1, rs2, imm } => write!(f, "sb x{},{}(x{})", rs1, imm, rs2),
            SH { rs1, rs2, imm } => write!(f, "sh x{},{}(x{})", rs1, imm, rs2),
            SW { rs1, rs2, imm } => write!(f, "sw x{},{}(x{})", rs1, imm, rs2),
            SLLI { rd, rs1, shamt } => write!(f, "slli x{},x{},{}", rd, rs1, shamt),
            SRLI { rd, rs1, shamt } => write!(f, "srli x{},x{},{}", rd, rs1, shamt),
            SRAI { rd, rs1, shamt } => write!(f, "srai x{},x{},{}", rd, rs1, shamt),
            SLL { rd, rs1, rs2 } => write!(f, "sll x{},x{},x{}", rd, rs1, rs2),
            SRL { rd, rs1, rs2 } => write!(f, "srl x{},x{},x{}", rd, rs1, rs2),
            SRA { rd, rs1, rs2 } => write!(f, "sra x{},x{},x{}", rd, rs1, rs2),
            ADDI { rd, rs1, imm } => write!(f, "addi x{},x{},{}", rd, rs1, imm),
            ADD { rd, rs1, rs2 } => write!(f, "add x{},x{},x{}", rd, rs1, rs2),
            SUB { rd, rs1, rs2 } => write!(f, "sub x{},x{},x{}", rd, rs1, rs2),
            ORI { rd, rs1, imm } => write!(f, "ori x{},x{},{}", rd, rs1, imm),
            XORI { rd, rs1, imm } => write!(f, "xori x{},x{},{}", rd, rs1, imm),
            ANDI { rd, rs1, imm } => write!(f, "andi x{},x{},{}", rd, rs1, imm),
            OR { rd, rs1, rs2 } => write!(f, "or x{},x{},x{}", rd, rs1, rs2),
            XOR { rd, rs1, rs2 } => write!(f, "xor x{},x{},x{}", rd, rs1, rs2),
            AND { rd, rs1, rs2 } => write!(f, "and x{},x{},x{}", rd, rs1, rs2),
            SLTI { rd, rs1, imm } => write!(f, "slti x{},x{},{}", rd, rs1, imm),
            SLTIU { rd, rs1, imm } => write!(f, "sltiu x{},x{},{}", rd, rs1, imm),
            SLT { rd, rs1, rs2 } => write!(f, "slt x{},x{},x{}", rd, rs1, rs2),
            SLTU { rd, rs1, rs2 } => write!(f, "sltu x{},x{},x{}", rd, rs1, rs2),
            BEQ { rs1, rs2, imm } => write!(f, "beq x{},x{},{}", rs1, rs2, imm),
            BNE { rs1, rs2, imm } => write!(f, "bne x{},x{},{}", rs1, rs2, imm),
            BLT { rs1, rs2, imm } => write!(f, "blt x{},x{},{}", rs1, rs2, imm),
            BGE { rs1, rs2, imm } => write!(f, "bge x{},x{},{}", rs1, rs2, imm),
            BLTU { rs1, rs2, imm } => write!(f, "bltu x{},x{},{}", rs1, rs2, imm),
            BGEU { rs1, rs2, imm } => write!(f, "bgeu x{},x{},{}", rs1, rs2, imm),
            JAL { rd, imm } => write!(f, "jal x{},{}", rd, imm),
            AUIPC { rd, imm } => write!(f, "auipc x{},{}", rd, imm),
            JALR { rd, rs1, imm } => write!(f, "jalr x{},{}(x{})", rd, imm, rs1),
            FENCE { pred, succ } => write!(f, "fence {},{}", pred, succ),
            FENCETSO => write!(f, "fence.tso"),
            ECALL => write!(f, "ecall"),
            EBREAK => write!(f, "ebreak"),
            FENCEI => write!(f, "fence.i"),
            CSRRWI { rd, uimm, csr } => write!(f, "csrrwi x{},{},{}", rd, csr, uimm),
            CSRRSI { rd, uimm, csr } => write!(f, "csrrsi x{},{},{}", rd, csr, uimm),
            CSRRCI { rd, uimm, csr } => write!(f, "csrrci x{},{},{}", rd, csr, uimm),
            CSRRW { rd, rs1, csr } => write!(f, "csrrw x{},{},x{}", rd, csr, rs1),
            CSRRS { rd, rs1, csr } => write!(f, "csrrs x{},{},x{}", rd, csr, rs1),
            CSRRC { rd, rs1, csr } => write!(f, "csrrc x{},{},x{}", rd, csr, rs1),
            MUL { rd, rs1, rs2 } => write!(f, "mul x{},x{},x{}", rd, rs1, rs2),
            MULH { rd, rs1, rs2 } => write!(f, "mulh x{},x{},x{}", rd, rs1, rs2),
            MULHSU { rd, rs1, rs2 } => write!(f, "mulhsu x{},x{},x{}", rd, rs1, rs2),
            MULHU { rd, rs1, rs2 } => write!(f, "mulhu x{},x{},x{}", rd, rs1, rs2),
            DIV { rd, rs1, rs2 } => write!(f, "div x{},x{},x{}", rd, rs1, rs2),
            DIVU { rd, rs1, rs2 } => write!(f, "divu x{},x{},x{}", rd, rs1, rs2),
            REM { rd, rs1, rs2 } => write!(f, "rem x{},x{},x{}", rd, rs1, rs2),
            REMU { rd, rs1, rs2 } => write!(f, "remu x{},x{},x{}", rd, rs1, rs2),
        }
    }
}
