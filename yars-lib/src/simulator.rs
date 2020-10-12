use crate::memory::{Memory, ProgramError};
use crate::processor::{Processor, ProcessorError};
use crate::register::IntRegister;
use std::convert::TryFrom;
use std::io::Write;
use std::path::Path;

pub struct Simulator<W: Write> {
    processor: Processor,
    logger: Option<W>,
}

impl<W: Write> Simulator<W> {
    pub fn new<P: AsRef<Path>>(
        program: P,
        memsize: u32,
        pc: Option<u32>,
        logger: Option<W>,
    ) -> Result<Self, ProgramError> {
        let mut memory = Memory::new(memsize);
        let def_pc = memory.load_program(program)?;
        let mut processor = Processor::new(memory);
        processor.set_pc(if let Some(pc) = pc { pc } else { def_pc });
        Ok(Self { processor, logger })
    }

    pub fn step(&mut self) -> Result<(), ProcessorError> {
        let pc = self.processor.pc();
        let inst = self.processor.fetch()?;
        self.processor.execute(inst)?;

        if pc == self.processor.pc() {
            self.processor.set_pc(pc.wrapping_add(4));
        }

        if let Some(logger) = &mut self.logger {
            let raw_inst = self.processor.memory().read_word(pc);
            let registers = self.processor.registers();

            let rd_id = ((raw_inst >> 7) & 0b11111) as u8;
            let rs1_id = ((raw_inst >> 15) & 0b11111) as u8;
            let rs2_id = ((raw_inst >> 20) & 0b11111) as u8;

            let rd = registers.read(IntRegister::try_from(rd_id).unwrap());
            let rs1 = registers.read(IntRegister::try_from(rs1_id).unwrap());
            let rs2 = registers.read(IntRegister::try_from(rs2_id).unwrap());

            writeln!(
                logger,
                "[PC={:08X}] [{:08X}] [x{:02}={:08X}] \
                 [x{:02}={:08X}] [x{:02}={:08X}] {}",
                pc, raw_inst, rd_id, rd, rs1_id, rs1, rs2_id, rs2, inst
            )
            .unwrap();
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ProcessorError> {
        loop {
            match self.step() {
                Ok(()) => continue,
                Err(ProcessorError::Ecall) | Err(ProcessorError::Ebreak) => break Ok(()),
                e => break e,
            }
        }
    }
}
