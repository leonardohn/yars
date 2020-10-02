use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Memory {
    memory: Box<[u8]>,
    offset: u32,
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Self {
            memory: vec![0u8; size as usize].into_boxed_slice(),
            offset: 0,
        }
    }

    pub fn size(&self) -> u32 {
        self.memory.len() as u32
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let len = program.len();
        self.offset = len as u32;
        self.memory[..len].copy_from_slice(program);
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
