use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Register {
    pub high_byte: u8,
    pub low_byte: u8,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x?}{:02x?}", self.high_byte, self.low_byte)
    }
}
pub fn init_register(high_byte: u8, low_byte: u8) -> Register {
    return Register {
        high_byte,
        low_byte,
    };
}

pub fn init_16bit_register(value: u16) -> Register {
    return Register {
        high_byte: (value >> 8) as u8,
        low_byte: (value & 0x00ff) as u8,
    };
}

impl Register {
    pub fn get_high_byte(&self) -> u8 {
        return self.high_byte;
    }
    pub fn get_low_byte(&self) -> u8 {
        return self.low_byte;
    }
    pub fn set_high_byte(&self, byte: u8) -> Register {
        return Register {
            high_byte: byte,
            ..*self
        };
    }
    pub fn get_16bit_value(&self) -> u16 {
        return (self.high_byte as u16) << 8 | self.low_byte as u16;
    }
    pub fn set_low_byte(&self, byte: u8) -> Register {
        return Register {
            low_byte: byte,
            ..*self
        };
    }
    pub fn set_16bit_value(&self, bytes: u16) -> Register {
        return init_16bit_register(bytes);
    }
}

pub enum Registers {
    A,
    Flags,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Registers::AF => write!(f, "AF"),
            Registers::BC => write!(f, "BC"),
            Registers::DE => write!(f, "DE"),
            Registers::HL => write!(f, "HL"),
            Registers::SP => write!(f, "SP"),
            Registers::PC => write!(f, "PC"),
            Registers::A => write!(f, "A"),
            Registers::Flags => write!(f, "Flags"),
            Registers::B => write!(f, "B"),
            Registers::C => write!(f, "C"),
            Registers::D => write!(f, "D"),
            Registers::E => write!(f, "E"),
            Registers::H => write!(f, "H"),
            Registers::L => write!(f, "L"),
        }
    }
}