use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::register::{IntRegister, IntRegisterSet};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProcessorError {
    Ebreak,
    Ecall,
    IllegalAccess,
    IllegalFetch,
    InvalidOpcode,
    MisalignedFetch,
}

#[derive(Debug)]
pub struct Processor {
    pc: u32,
    cycles: usize,
    memory: Memory,
    registers: IntRegisterSet,
}

impl Processor {
    pub fn new(memory: Memory) -> Self {
        let pc = 0;
        let cycles = 0;
        let mut registers = IntRegisterSet::new();
        registers.write(IntRegister::SP, memory.size() - 1);

        Self {
            pc,
            cycles,
            memory,
            registers,
        }
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn reset_cycles(&mut self) {
        self.cycles = 0;
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn registers(&self) -> &IntRegisterSet {
        &self.registers
    }

    pub fn fetch(&self) -> Result<Instruction, ProcessorError> {
        if self.pc >= self.memory.size() {
            return Err(ProcessorError::IllegalFetch);
        }

        if self.pc & 0b11 != 0b00 {
            return Err(ProcessorError::MisalignedFetch);
        }

        let opcode = self.memory.read_word(self.pc);
        Instruction::try_from(opcode).map_err(|_| ProcessorError::InvalidOpcode)
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<(), ProcessorError> {
        use Instruction::*;
        match inst {
            LUI { rd, imm } => {
                self.registers.write(rd, (imm as u32) << 12);
                self.cycles += 1;
                Ok(())
            }
            LB { rd, rs1, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.memory.read_byte(addr) as i32 as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            LH { rd, rs1, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.memory.read_halfword(addr) as i32 as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            LW { rd, rs1, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.memory.read_word(addr);
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            LBU { rd, rs1, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.memory.read_byte(addr) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            LHU { rd, rs1, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.memory.read_halfword(addr) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SB { rs1, rs2, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.registers.read(rs2) as u8;
                self.memory.write_byte(addr, val);
                self.cycles += 1;
                Ok(())
            }
            SH { rs1, rs2, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.registers.read(rs2) as u16;
                self.memory.write_halfword(addr, val);
                self.cycles += 1;
                Ok(())
            }
            SW { rs1, rs2, imm } => {
                let addr = self.registers.read(rs1).wrapping_add(imm as i32 as u32);

                if addr >= self.memory.size() {
                    return Err(ProcessorError::IllegalAccess);
                }

                let val = self.registers.read(rs2);
                self.memory.write_word(addr, val);
                self.cycles += 1;
                Ok(())
            }
            SLLI { rd, rs1, shamt } => {
                let v1 = self.registers.read(rs1);
                let val = v1 << shamt;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SRLI { rd, rs1, shamt } => {
                let v1 = self.registers.read(rs1);
                let val = v1 >> shamt;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SRAI { rd, rs1, shamt } => {
                let v1 = self.registers.read(rs1);
                let val = (v1 >> shamt) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SLL { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 << v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SRL { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 >> v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SRA { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2);
                let val = (v1 >> v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            ADDI { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = imm as i32;
                let val = v1.wrapping_add(v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            ADD { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let val = v1.wrapping_add(v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SUB { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let val = v1.wrapping_sub(v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            ORI { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = imm as i32 as u32;
                let val = v1 | v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            XORI { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = imm as i32 as u32;
                let val = v1 ^ v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            ANDI { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = imm as i32 as u32;
                let val = v1 & v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            OR { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 | v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            XOR { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 ^ v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            AND { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 & v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SLTI { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = imm as i32;
                let val = (v1 < v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SLTIU { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = imm as i32 as u32;
                let val = (v1 < v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SLT { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let val = (v1 < v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            SLTU { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = (v1 < v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            BEQ { rs1, rs2, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let v3 = imm as i32 as u32;

                if v1 == v2 {
                    self.pc = self.pc.wrapping_add(v3);
                }

                self.cycles += 1;
                Ok(())
            }
            BNE { rs1, rs2, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let v3 = imm as i32 as u32;

                if v1 != v2 {
                    self.pc = self.pc.wrapping_add(v3);
                }

                self.cycles += 1;
                Ok(())
            }
            BLT { rs1, rs2, imm } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let v3 = imm as i32 as u32;

                if v1 > v2 {
                    self.pc = self.pc.wrapping_add(v3);
                }

                self.cycles += 1;
                Ok(())
            }
            BGE { rs1, rs2, imm } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let v3 = imm as i32 as u32;

                if v1 <= v2 {
                    self.pc = self.pc.wrapping_add(v3);
                }

                self.cycles += 1;
                Ok(())
            }
            BLTU { rs1, rs2, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let v3 = imm as i32 as u32;

                if v1 > v2 {
                    self.pc = self.pc.wrapping_add(v3);
                }

                self.cycles += 1;
                Ok(())
            }
            BGEU { rs1, rs2, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let v3 = imm as i32 as u32;

                if v1 <= v2 {
                    self.pc = self.pc.wrapping_add(v3);
                }

                self.cycles += 1;
                Ok(())
            }
            JAL { rd, imm } => {
                let val = self.pc.wrapping_add(imm as u32);
                self.registers.write(rd, self.pc.wrapping_add(4));
                self.pc = val;
                self.cycles += 1;
                Ok(())
            }
            AUIPC { rd, imm } => {
                let val = self.pc.wrapping_add((imm as u32) << 12);
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            JALR { rd, rs1, imm } => {
                let v1 = self.registers.read(rs1);
                let v2 = imm as i32 as u32;
                let val = v1.wrapping_add(v2) & !0b1;
                self.registers.write(rd, self.pc.wrapping_add(4));
                self.pc = val;
                self.cycles += 1;
                Ok(())
            }
            FENCE { .. } => {
                self.cycles += 1;
                Ok(())
            }
            FENCETSO => {
                self.cycles += 1;
                Ok(())
            }
            ECALL => {
                self.cycles += 1;
                Err(ProcessorError::Ecall)
            }
            EBREAK => {
                self.cycles += 1;
                Err(ProcessorError::Ebreak)
            }
            MUL { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let val = v1.wrapping_mul(v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            MULH { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i64;
                let v2 = self.registers.read(rs2) as i64;
                let val = ((v1.wrapping_mul(v2) as u64) >> 32) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            MULHSU { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i64;
                let v2 = self.registers.read(rs2) as u64 as i64;
                let val = ((v1.wrapping_mul(v2) as u64) >> 32) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            MULHU { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as u64;
                let v2 = self.registers.read(rs2) as u64;
                let val = (v1.wrapping_mul(v2) >> 32) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            DIV { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let val = (v1 / v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            DIVU { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 / v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            REM { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1) as i32;
                let v2 = self.registers.read(rs2) as i32;
                let val = (v1 % v2) as u32;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
            REMU { rd, rs1, rs2 } => {
                let v1 = self.registers.read(rs1);
                let v2 = self.registers.read(rs2);
                let val = v1 % v2;
                self.registers.write(rd, val);
                self.cycles += 1;
                Ok(())
            }
        }
    }
}
