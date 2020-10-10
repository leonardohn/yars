use goblin::elf::header::{EM_RISCV, ET_EXEC};
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::Elf;
use goblin::error::Error;
use std::convert::TryInto;
use std::path::Path;

pub enum ProgramError {
    OutOfMemory,
    UnsupportedBinary,
    Goblin(Error),
}

#[derive(Clone, Debug)]
pub struct Memory {
    memory: Box<[u8]>,
    entry: u32,
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Self {
            memory: vec![0u8; size as usize].into_boxed_slice(),
            entry: 0,
        }
    }

    pub fn size(&self) -> u32 {
        self.memory.len() as u32
    }

    pub fn entry(&self) -> u32 {
        self.entry
    }

    pub fn set_entry(&mut self, entry: u32) {
        self.entry = entry;
    }

    pub fn load_program<P: AsRef<Path>>(&mut self, program: P) -> Result<(), ProgramError> {
        let buffer = std::fs::read(program).map_err(|e| ProgramError::Goblin(Error::IO(e)))?;
        let binary = Elf::parse(&buffer).map_err(|e| ProgramError::Goblin(e))?;

        if binary.header.e_machine != EM_RISCV || binary.header.e_type != ET_EXEC || binary.is_64 {
            return Err(ProgramError::UnsupportedBinary);
        }

        self.entry = binary.entry as u32;

        for ph in binary.program_headers {
            if ph.p_type == PT_LOAD {
                let vm_range = ph.vm_range();
                let file_range = ph.file_range();

                if vm_range.end >= self.memory.len() {
                    return Err(ProgramError::OutOfMemory);
                }

                for i in self.memory[vm_range].iter_mut() {
                    *i = 0;
                }

                self.memory[file_range.clone()].copy_from_slice(&buffer[file_range]);
            }
        }

        Ok(())
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        let addr = address as usize;
        let array = self.memory[addr..addr + 2].try_into().unwrap();
        u16::from_le_bytes(array)
    }

    pub fn read_word(&self, address: u32) -> u32 {
        let addr = address as usize;
        let array = self.memory[addr..addr + 4].try_into().unwrap();
        u32::from_le_bytes(array)
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        let addr = address as usize;
        let slice = &u16::to_le_bytes(value)[..];
        self.memory[addr..addr + 2].copy_from_slice(slice);
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        let addr = address as usize;
        let slice = &u32::to_le_bytes(value)[..];
        self.memory[addr..addr + 4].copy_from_slice(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_word_and_read_bytes() {
        let mut mem = Memory::new(4);
        mem.write_word(0, 0x00FF0FF0);
        assert_eq!(mem.read_byte(0), 0xF0);
        assert_eq!(mem.read_byte(1), 0x0F);
        assert_eq!(mem.read_byte(2), 0xFF);
        assert_eq!(mem.read_byte(3), 0x00);
    }

    #[test]
    fn write_bytes_and_read_word() {
        let mut mem = Memory::new(4);
        mem.write_byte(0, 0xF0);
        mem.write_byte(1, 0x0F);
        mem.write_byte(2, 0xFF);
        mem.write_byte(3, 0x00);
        assert_eq!(mem.read_word(0), 0x00FF0FF0);
    }

    #[test]
    #[should_panic]
    fn panic_on_read_out_of_bounds() {
        Memory::new(3).read_word(0);
    }

    #[test]
    #[should_panic]
    fn panic_on_write_out_of_bounds() {
        Memory::new(3).write_word(0, 0xFFFFFFFF);
    }
}
