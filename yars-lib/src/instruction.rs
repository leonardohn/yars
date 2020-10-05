use std::convert::TryFrom;
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

impl TryFrom<u8> for FenceKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b10 => Ok(Self::R),
            0b01 => Ok(Self::W),
            0b11 => Ok(Self::RW),
            _ => Err(()),
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
            LUI { rd, imm } => write!(f, "lui      {}, {}", rd, imm),
            LB { rd, rs1, imm } => write!(f, "lb       {}, {}({})", rd, imm, rs1),
            LH { rd, rs1, imm } => write!(f, "lh       {}, {}({})", rd, imm, rs1),
            LW { rd, rs1, imm } => write!(f, "lw       {}, {}({})", rd, imm, rs1),
            LBU { rd, rs1, imm } => write!(f, "lbu      {}, {}({})", rd, imm, rs1),
            LHU { rd, rs1, imm } => write!(f, "lhu      {}, {}({})", rd, imm, rs1),
            SB { rs1, rs2, imm } => write!(f, "sb      {}, {}({})", rs1, imm, rs2),
            SH { rs1, rs2, imm } => write!(f, "sh      {}, {}({})", rs1, imm, rs2),
            SW { rs1, rs2, imm } => write!(f, "sw      {}, {}({})", rs1, imm, rs2),
            SLLI { rd, rs1, shamt } => write!(f, "slli     {}, {}, {}", rd, rs1, shamt),
            SRLI { rd, rs1, shamt } => write!(f, "srli     {}, {}, {}", rd, rs1, shamt),
            SRAI { rd, rs1, shamt } => write!(f, "srai     {}, {}, {}", rd, rs1, shamt),
            SLL { rd, rs1, rs2 } => write!(f, "sll      {}, {}, {}", rd, rs1, rs2),
            SRL { rd, rs1, rs2 } => write!(f, "srl      {}, {}, {}", rd, rs1, rs2),
            SRA { rd, rs1, rs2 } => write!(f, "sra      {}, {}, {}", rd, rs1, rs2),
            ADDI { rd, rs1, imm } => write!(f, "addi     {}, {}, {}", rd, rs1, imm),
            ADD { rd, rs1, rs2 } => write!(f, "add      {}, {}, {}", rd, rs1, rs2),
            SUB { rd, rs1, rs2 } => write!(f, "sub      {}, {}, {}", rd, rs1, rs2),
            ORI { rd, rs1, imm } => write!(f, "ori      {}, {}, {}", rd, rs1, imm),
            XORI { rd, rs1, imm } => write!(f, "xori     {}, {}, {}", rd, rs1, imm),
            ANDI { rd, rs1, imm } => write!(f, "andi     {}, {}, {}", rd, rs1, imm),
            OR { rd, rs1, rs2 } => write!(f, "or       {}, {}, {}", rd, rs1, rs2),
            XOR { rd, rs1, rs2 } => write!(f, "xor      {}, {}, {}", rd, rs1, rs2),
            AND { rd, rs1, rs2 } => write!(f, "and      {}, {}, {}", rd, rs1, rs2),
            SLTI { rd, rs1, imm } => write!(f, "slti     {}, {}, {}", rd, rs1, imm),
            SLTIU { rd, rs1, imm } => write!(f, "sltiu    {}, {}, {}", rd, rs1, imm),
            SLT { rd, rs1, rs2 } => write!(f, "slt      {}, {}, {}", rd, rs1, rs2),
            SLTU { rd, rs1, rs2 } => write!(f, "sltu     {}, {}, {}", rd, rs1, rs2),
            BEQ { rs1, rs2, imm } => write!(f, "beq      {}, {}, {}", rs1, rs2, imm),
            BNE { rs1, rs2, imm } => write!(f, "bne      {}, {}, {}", rs1, rs2, imm),
            BLT { rs1, rs2, imm } => write!(f, "blt      {}, {}, {}", rs1, rs2, imm),
            BGE { rs1, rs2, imm } => write!(f, "bge      {}, {}, {}", rs1, rs2, imm),
            BLTU { rs1, rs2, imm } => write!(f, "bltu     {}, {}, {}", rs1, rs2, imm),
            BGEU { rs1, rs2, imm } => write!(f, "bgeu     {}, {}, {}", rs1, rs2, imm),
            JAL { rd, imm } => write!(f, "jal      {}, {}", rd, imm),
            AUIPC { rd, imm } => write!(f, "auipc    {}, {}", rd, imm),
            JALR { rd, rs1, imm } => write!(f, "jalr     {}, {}({})", rd, imm, rs1),
            FENCE { pred, succ } => write!(f, "fence    {}, {}", pred, succ),
            FENCETSO => write!(f, "fence.tso"),
            ECALL => write!(f, "ecall"),
            EBREAK => write!(f, "ebreak"),
            FENCEI => write!(f, "fence.i"),
            CSRRWI { rd, uimm, csr } => write!(f, "csrrwi   {}, {}, {}", rd, csr, uimm),
            CSRRSI { rd, uimm, csr } => write!(f, "csrrsi   {}, {}, {}", rd, csr, uimm),
            CSRRCI { rd, uimm, csr } => write!(f, "csrrci   {}, {}, {}", rd, csr, uimm),
            CSRRW { rd, rs1, csr } => write!(f, "csrrw    {}, {}, {}", rd, csr, rs1),
            CSRRS { rd, rs1, csr } => write!(f, "csrrs    {}, {}, {}", rd, csr, rs1),
            CSRRC { rd, rs1, csr } => write!(f, "csrrc    {}, {}, {}", rd, csr, rs1),
            MUL { rd, rs1, rs2 } => write!(f, "mul      {}, {}, {}", rd, rs1, rs2),
            MULH { rd, rs1, rs2 } => write!(f, "mulh     {}, {}, {}", rd, rs1, rs2),
            MULHSU { rd, rs1, rs2 } => write!(f, "mulhsu   {}, {}, {}", rd, rs1, rs2),
            MULHU { rd, rs1, rs2 } => write!(f, "mulhu    {}, {}, {}", rd, rs1, rs2),
            DIV { rd, rs1, rs2 } => write!(f, "div      {}, {}, {}", rd, rs1, rs2),
            DIVU { rd, rs1, rs2 } => write!(f, "divu     {}, {}, {}", rd, rs1, rs2),
            REM { rd, rs1, rs2 } => write!(f, "rem      {}, {}, {}", rd, rs1, rs2),
            REMU { rd, rs1, rs2 } => write!(f, "remu     {}, {}, {}", rd, rs1, rs2),
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = ();

    fn try_from(inst: u32) -> Result<Self, Self::Error> {
        let opcode = (inst & 0x7F) as u8;
        let format = InstructionFormat::from_opcode(opcode).ok_or(())?;

        match format {
            InstructionFormat::R => {
                let rd = ((inst >> 7) & 0b11111) as u8;
                let rs1 = ((inst >> 15) & 0b11111) as u8;
                let rs2 = ((inst >> 20) & 0b11111) as u8;
                let funct3 = ((inst >> 12) & 0b111) as u8;
                let funct7 = ((inst >> 25) & 0b1111111) as u8;
                let fn3_opcode = (funct3 << 5) | (opcode >> 2);

                match fn3_opcode {
                    0b000_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::ADD { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::MUL { rd, rs1, rs2 }),
                        0b0100000 => Ok(Instruction::SUB { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b001_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::SLL { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::MULH { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b010_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::SLT { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::MULHSU { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b011_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::SLTU { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::MULHU { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b100_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::XOR { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::DIV { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b101_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::SRL { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::DIVU { rd, rs1, rs2 }),
                        0b0100000 => Ok(Instruction::SRA { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b110_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::OR { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::REM { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    0b111_01100 => match funct7 {
                        0b0000000 => Ok(Instruction::AND { rd, rs1, rs2 }),
                        0b0000001 => Ok(Instruction::REMU { rd, rs1, rs2 }),
                        _ => Err(()),
                    },
                    _ => Err(()),
                }
            }
            InstructionFormat::R4 => Err(()),
            InstructionFormat::I => {
                let rd = ((inst >> 7) & 0b11111) as u8;
                let rs1 = ((inst >> 15) & 0b11111) as u8;
                let funct3 = ((inst >> 12) & 0b111) as u8;
                let imm = ((inst as i32) >> 20) as i16;
                let csr = imm as u16 & 0b111111111111;
                let fn3_opcode = (funct3 << 5) | (opcode >> 2);

                match fn3_opcode {
                    0b000_11001 => Ok(Instruction::JALR { rd, rs1, imm }),
                    0b000_00000 => Ok(Instruction::LB { rd, rs1, imm }),
                    0b001_00000 => Ok(Instruction::LH { rd, rs1, imm }),
                    0b010_00000 => Ok(Instruction::LW { rd, rs1, imm }),
                    0b100_00000 => Ok(Instruction::LBU { rd, rs1, imm }),
                    0b101_00000 => Ok(Instruction::LHU { rd, rs1, imm }),
                    0b000_00100 => Ok(Instruction::ADDI { rd, rs1, imm }),
                    0b010_00100 => Ok(Instruction::SLTI { rd, rs1, imm }),
                    0b011_00100 => Ok(Instruction::SLTIU { rd, rs1, imm }),
                    0b100_00100 => Ok(Instruction::XORI { rd, rs1, imm }),
                    0b110_00100 => Ok(Instruction::ORI { rd, rs1, imm }),
                    0b111_00100 => Ok(Instruction::ANDI { rd, rs1, imm }),
                    0b000_00011 => {
                        let fm = ((imm >> 8) & 0b1111) as u8;
                        let pred = ((imm >> 4) & 0b1111) as u8;
                        let succ = (imm & 0b1111) as u8;
                        let pred = FenceKind::try_from(pred)?;
                        let succ = FenceKind::try_from(succ)?;

                        match (fm, pred, succ) {
                            (0b1000, FenceKind::RW, FenceKind::RW) => Ok(Instruction::FENCETSO),
                            (0b0000, pred, succ) => Ok(Instruction::FENCE { pred, succ }),
                            _ => Err(()),
                        }
                    }
                    0b000_11100 => match imm {
                        0 => Ok(Instruction::ECALL),
                        1 => Ok(Instruction::EBREAK),
                        _ => Err(()),
                    },
                    0b001_00011 => Ok(Instruction::FENCEI),
                    0b001_11100 => Ok(Instruction::CSRRW { rd, rs1, csr }),
                    0b010_11100 => Ok(Instruction::CSRRS { rd, rs1, csr }),
                    0b011_11100 => Ok(Instruction::CSRRC { rd, rs1, csr }),
                    0b101_11100 => Ok(Instruction::CSRRWI { rd, uimm: rs1, csr }),
                    0b110_11100 => Ok(Instruction::CSRRSI { rd, uimm: rs1, csr }),
                    0b111_11100 => Ok(Instruction::CSRRCI { rd, uimm: rs1, csr }),
                    _ => Err(()),
                }
            }
            InstructionFormat::S => {
                let funct3 = ((inst >> 12) & 0b111) as u8;
                let rs1 = ((inst >> 15) & 0b11111) as u8;
                let rs2 = ((inst >> 20) & 0b11111) as u8;
                let imm115 = (inst >> 25) & 0b1111111;
                let imm40 = (inst >> 7) & 0b11111;
                let imm = (imm115 << 5) | imm40;
                let imm = ((imm as i16) << 4) >> 4;
                let fn3_opcode = (funct3 << 5) | (opcode >> 2);

                match fn3_opcode {
                    0b000_01000 => Ok(Instruction::SB { rs1, rs2, imm }),
                    0b001_01000 => Ok(Instruction::SH { rs1, rs2, imm }),
                    0b010_01000 => Ok(Instruction::SW { rs1, rs2, imm }),
                    _ => Err(()),
                }
            }
            InstructionFormat::B => {
                let funct3 = ((inst >> 12) & 0b111) as u8;
                let rs1 = ((inst >> 15) & 0b11111) as u8;
                let rs2 = ((inst >> 20) & 0b11111) as u8;
                let imm12 = (inst >> 31) & 0b1;
                let imm105 = (inst >> 25) & 0b111111;
                let imm41 = (inst >> 8) & 0b1111;
                let imm11 = (inst >> 7) & 0b1;
                let imm = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
                let imm = ((imm as i16) << 3) >> 3;
                let fn3_opcode = (funct3 << 5) | (opcode >> 2);

                match fn3_opcode {
                    0b000_11000 => Ok(Instruction::BEQ { rs1, rs2, imm }),
                    0b001_11000 => Ok(Instruction::BNE { rs1, rs2, imm }),
                    0b100_11000 => Ok(Instruction::BLT { rs1, rs2, imm }),
                    0b101_11000 => Ok(Instruction::BGE { rs1, rs2, imm }),
                    0b110_11000 => Ok(Instruction::BLTU { rs1, rs2, imm }),
                    0b111_11000 => Ok(Instruction::BGEU { rs1, rs2, imm }),
                    _ => Err(()),
                }
            }
            InstructionFormat::U => {
                let rd = ((inst >> 7) & 0b11111) as u8;
                let imm = (inst & !0b111111111111) as i32;

                match opcode >> 2 {
                    0b01101 => Ok(Instruction::LUI { rd, imm }),
                    0b00101 => Ok(Instruction::AUIPC { rd, imm }),
                    _ => Err(()),
                }
            }
            InstructionFormat::J => {
                let rd = ((inst >> 7) & 0b11111) as u8;
                let imm20 = (inst >> 31) & 0b1;
                let imm101 = (inst >> 21) & 0b1111111111;
                let imm11 = (inst >> 20) & 0b1;
                let imm1912 = (inst >> 12) & 0b11111111;
                let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1);
                let imm = ((imm as i32) << 11) >> 11;

                match opcode >> 2 {
                    0b11011 => Ok(Instruction::JAL { rd, imm }),
                    _ => Err(()),
                }
            }
        }
    }
}
