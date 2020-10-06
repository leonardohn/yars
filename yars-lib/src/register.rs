use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IntRegister {
    Zero = 0,
    RA,
    SP,
    GP,
    TP,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

impl fmt::Display for IntRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntRegister::Zero => write!(f, "zero"),
            IntRegister::RA => write!(f, "ra"),
            IntRegister::SP => write!(f, "sp"),
            IntRegister::GP => write!(f, "gp"),
            IntRegister::TP => write!(f, "tp"),
            IntRegister::T0 => write!(f, "t0"),
            IntRegister::T1 => write!(f, "t1"),
            IntRegister::T2 => write!(f, "t2"),
            IntRegister::S0 => write!(f, "s0"),
            IntRegister::S1 => write!(f, "s1"),
            IntRegister::A0 => write!(f, "a0"),
            IntRegister::A1 => write!(f, "a1"),
            IntRegister::A2 => write!(f, "a2"),
            IntRegister::A3 => write!(f, "a3"),
            IntRegister::A4 => write!(f, "a4"),
            IntRegister::A5 => write!(f, "a5"),
            IntRegister::A6 => write!(f, "a6"),
            IntRegister::A7 => write!(f, "a7"),
            IntRegister::S2 => write!(f, "s2"),
            IntRegister::S3 => write!(f, "s3"),
            IntRegister::S4 => write!(f, "s4"),
            IntRegister::S5 => write!(f, "s5"),
            IntRegister::S6 => write!(f, "s6"),
            IntRegister::S7 => write!(f, "s7"),
            IntRegister::S8 => write!(f, "s8"),
            IntRegister::S9 => write!(f, "s9"),
            IntRegister::S10 => write!(f, "s10"),
            IntRegister::S11 => write!(f, "s11"),
            IntRegister::T3 => write!(f, "t3"),
            IntRegister::T4 => write!(f, "t4"),
            IntRegister::T5 => write!(f, "t5"),
            IntRegister::T6 => write!(f, "t6"),
        }
    }
}

impl TryFrom<u8> for IntRegister {
    type Error = ();

    fn try_from(reg: u8) -> Result<Self, Self::Error> {
        match reg {
            0 => Ok(IntRegister::Zero),
            1 => Ok(IntRegister::RA),
            2 => Ok(IntRegister::SP),
            3 => Ok(IntRegister::GP),
            4 => Ok(IntRegister::TP),
            5 => Ok(IntRegister::T0),
            6 => Ok(IntRegister::T1),
            7 => Ok(IntRegister::T2),
            8 => Ok(IntRegister::S0),
            9 => Ok(IntRegister::S1),
            10 => Ok(IntRegister::A0),
            11 => Ok(IntRegister::A1),
            12 => Ok(IntRegister::A2),
            13 => Ok(IntRegister::A3),
            14 => Ok(IntRegister::A4),
            15 => Ok(IntRegister::A5),
            16 => Ok(IntRegister::A6),
            17 => Ok(IntRegister::A7),
            18 => Ok(IntRegister::S2),
            19 => Ok(IntRegister::S3),
            20 => Ok(IntRegister::S4),
            21 => Ok(IntRegister::S5),
            22 => Ok(IntRegister::S6),
            23 => Ok(IntRegister::S7),
            24 => Ok(IntRegister::S8),
            25 => Ok(IntRegister::S9),
            26 => Ok(IntRegister::S10),
            27 => Ok(IntRegister::S11),
            28 => Ok(IntRegister::T3),
            29 => Ok(IntRegister::T4),
            30 => Ok(IntRegister::T5),
            31 => Ok(IntRegister::T6),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IntRegisterSet {
    reg: [u32; 32],
}

impl IntRegisterSet {
    pub fn new() -> Self {
        let reg = [0; 32];
        Self { reg }
    }

    pub fn read(&self, reg: IntRegister) -> u32 {
        let reg = reg as usize;
        self.reg[reg]
    }

    pub fn write(&mut self, reg: IntRegister, val: u32) {
        let reg = reg as usize;
        if reg != 0 {
            self.reg[reg] = val;
        }
    }
}

impl fmt::Display for IntRegisterSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, r) in self.reg.chunks(4).enumerate() {
            let i = 4 * i as u8;
            let n = (i..i + 4)
                .map(|n| format!("{}", IntRegister::try_from(n).unwrap()))
                .collect::<Vec<_>>();

            writeln!(
                f,
                "{:>4}={:#010X} {:>4}={:#010X} {:>4}={:#010X} {:>4}={:#010X}",
                n[0], r[0], n[1], r[1], n[2], r[2], n[3], r[3],
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_int_register_set() {
        let mut rs = IntRegisterSet::new();
        rs.write(IntRegister::Zero, 1);
        rs.write(IntRegister::RA, 1);
        assert_eq!(rs.read(IntRegister::Zero), 0);
        assert_eq!(rs.read(IntRegister::RA), 1);
    }
}
