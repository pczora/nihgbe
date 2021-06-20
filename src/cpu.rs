use super::mem;

const ZERO_FLAG: u8 = 0b10000000;
const SUBTRACT_FLAG: u8 = 0b01000000;
const HALFCARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

const OPCODE_NOP: u8 = 0x00;
const OPCODE_DEC_B: u8 = 0x05;
const OPCODE_DEC_C: u8 = 0x0d;
const OPCODE_DEC_E: u8 = 0x1d;
const OPCODE_DEC_H: u8 = 0x25;
const OPCODE_INC_C: u8 = 0x0c;
const OPCODE_XOR_A: u8 = 0xAF;
const OPCODE_LD_HL: u8 = 0x21;
const OPCODE_LDD_HL_A: u8 = 0x32;
const OPCODE_LDH_ADDR_A: u8 = 0xe0;
const OPCODE_LD_ADDR_HL_IMMEDIATE: u8 = 0x36;

const OPCODE_RRA: u8 = 0x1f;

const OPCODE_DI: u8 = 0xf3;

// Jumps & Calls
const OPCODE_JP: u8 = 0xc3;
const OPCODE_JR_NZ: u8 = 0x20;
const OPCODE_CALL: u8 = 0xcd;

// 8 bit loads
const OPCODE_LD_A_IMMEDIATE: u8 = 0x3e;
const OPCODE_LD_B_IMMEDIATE: u8 = 0x06;
const OPCODE_LD_C_IMMEDIATE: u8 = 0x0e;
const OPCODE_LD_E_IMMEDIATE: u8 = 0x16;
const OPCODE_LD_A_ADDR_DE: u8 = 0x1a;
const OPCODE_LD_ADDR_C_A: u8 = 0xe2;
const OPCODE_LD_ADDR_HL_A: u8 = 0x77;
const OPCODE_LD_C_A: u8 = 0x4f;

const OPCODE_LD_ADDRESS_A: u8 = 0xea;

// 16 bit loads
const OPCODE_LD_SP_IMMEDIATE: u8 = 0x31;
const OPCODE_LD_DE_IMMEDIATE: u8 = 0x11;
const OPCODE_PUSH_BC: u8 = 0xc5;

const OPCODE_LDH_A_ADDR: u8 = 0xf0;

const OPCODE_CP_A_D: u8 = 0xfe;

const OPCODE_PREFIX: u8 = 0xcb;

#[derive(Copy, Clone)]
pub struct CPU {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
}

#[derive(Copy, Clone)]
pub struct Register {
    high_byte: u8,
    low_byte: u8,
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

enum Registers {
    A,
    Flags,
    B,
    C,
    D,
    E,
    HL,
    SP,
    PC,
}

impl CPU {
    fn set_af(&self, value: u16) -> CPU {
        CPU {
            af: init_16bit_register(value),
            ..*self
        }
    }
    fn set_a(&self, value: u8) -> CPU {
        CPU {
            af: self.af.set_high_byte(value),
            ..*self
        }
    }
    fn get_a(&self) -> u8 {
        return self.af.get_high_byte();
    }
    fn set_f(&self, value: u8) -> CPU {
        CPU {
            af: self.af.set_low_byte(value),
            ..*self
        }
    }
    fn get_f(&self) -> u8 {
        return self.af.get_low_byte();
    }
    fn set_bc(&self, value: u16) -> CPU {
        CPU {
            bc: init_16bit_register(value),
            ..*self
        }
    }
    fn get_b(&self) -> u8 {
        return self.bc.get_high_byte();
    }
    fn get_c(&self) -> u8 {
        return self.bc.get_low_byte();
    }
    fn set_b(&self, value: u8) -> CPU {
        CPU {
            bc: self.bc.set_high_byte(value),
            ..*self
        }
    }
    fn set_c(&self, value: u8) -> CPU {
        CPU {
            bc: self.bc.set_low_byte(value),
            ..*self
        }
    }
    fn set_de(&self, value: u16) -> CPU {
        CPU {
            de: init_16bit_register(value),
            ..*self
        }
    }
    fn set_d(&self, value: u8) -> CPU {
        CPU {
            de: self.de.set_high_byte(value),
            ..*self
        }
    }
    fn set_e(&self, value: u8) -> CPU {
        CPU {
            de: self.de.set_low_byte(value),
            ..*self
        }
    }
    fn get_d(&self) -> u8 {
        return self.de.get_high_byte();
    }
    fn get_e(&self) -> u8 {
        return self.de.get_low_byte();
    }
    fn set_pc(&self, value: u16) -> CPU {
        CPU {
            pc: init_16bit_register(value),
            ..*self
        }
    }
    fn increment_pc(&self, value: u16) -> CPU {
        self.set_pc(self.pc.get_16bit_value().wrapping_add(value))
    }

    fn set_sp(&self, value: u16) -> CPU {
        CPU {
            sp: init_16bit_register(value),
            ..*self
        }
    }
    fn decrement_sp(&self, value: u16) -> CPU {
        return CPU {
            sp: self.sp.set_16bit_value(self.sp.get_16bit_value() - value),
            ..*self
        };
    }
    fn set_hl(&self, value: u16) -> CPU {
        CPU {
            hl: init_16bit_register(value),
            ..*self
        }
    }
    fn decrement_hl(&self, value: u16) -> CPU {
        CPU {
            hl: self.hl.set_16bit_value(self.hl.get_16bit_value() - value),
            ..*self
        }
    }
    fn get_hl(&self) -> u16 {
        return self.hl.get_16bit_value();
    }

    pub fn execute(&self, mem: &mut mem::Mem) -> CPU {
        let opcode = mem.read(self.pc.get_16bit_value());
        print!("{:#04x?}\t", self.pc.get_16bit_value(),);
        match opcode {
            OPCODE_LD_SP_IMMEDIATE => {
                let data = self.get_16_bit_arg(mem);
                print!("LD SP, {:#04x?}\n", data);
                return self.set_sp(data).increment_pc(3);
            }
            OPCODE_LD_DE_IMMEDIATE => {
                let data = self.get_16_bit_arg(mem);
                print!("LD DE, {:#04x?}\n", data);
                return self.set_de(data).increment_pc(3);
            }
            OPCODE_PREFIX => {
                // TODO: Implement! Careful, getting the next byte and
                // running the instruction cannot be interrupted
                let data = self.get_8_bit_arg(mem);
                print!("PREFIX {:#04x?}\n", data);
                match data {
                    // BIT 7, h
                    0x7c => {
                        let bit_set = ((1 << 15) & &self.hl.get_16bit_value()) == 1;
                        self.set_zero(!bit_set);
                    }
                    _ => {
                        panic!("Invalid 16 byte opcode")
                    }
                }
                return self.increment_pc(2);
            }
            OPCODE_NOP => {
                print!("NOP\n");
                return self.increment_pc(1);
            }
            OPCODE_JP => {
                let jp_dest = self.get_16_bit_arg(mem);
                print!("JP {:#04x?}\n", jp_dest);
                return self.set_pc(jp_dest);
            }
            OPCODE_XOR_A => {
                print!("XOR A\n");
                let current_a = self.get_a();
                return self.set_a(current_a ^ current_a).increment_pc(1);
            }
            OPCODE_LD_HL => {
                let data = self.get_16_bit_arg(mem);
                print!("LD HL, {:#04x?}\n", data);
                return self.set_hl(data).increment_pc(3);
            }
            OPCODE_LD_A_IMMEDIATE => {
                print!("LD A, ");
                return self.ld_8_bit_immediate(mem, Registers::A);
            }
            OPCODE_LD_B_IMMEDIATE => {
                print!("LD B, ");
                return self.ld_8_bit_immediate(mem, Registers::B);
            }
            OPCODE_LD_C_IMMEDIATE => {
                print!("LD C, ");
                return self.ld_8_bit_immediate(mem, Registers::C);
            }
            OPCODE_LD_E_IMMEDIATE => {
                print!("LD E, ");
                return self.ld_8_bit_immediate(mem, Registers::E);
            }
            OPCODE_LDH_A_ADDR => {
                let data = self.get_8_bit_arg(mem) as u16;
                print!("LD A, ({:#04x?})\n", 0xff00 + data);
                return self.set_a(mem.read(0xff00 + data)).increment_pc(2);
            }
            OPCODE_LDD_HL_A => {
                print!("LDD (HL), A \n");
                mem.write(self.hl.get_16bit_value(), self.get_a());
                return self.dec(Registers::HL);
            }
            OPCODE_DEC_B => {
                print!("DEC B \n");
                return self.dec(Registers::B);
            }
            OPCODE_DEC_C => {
                print!("DEC C \n");
                return self.dec(Registers::C);
            }
            OPCODE_DEC_E => {
                print!("DEC E \n");
                return self.dec(Registers::E);
            }
            OPCODE_DEC_H => {
                //TODO: Implement
                //TODO: Half carry flag?
                print!("DEC H \n");
                unimplemented!();
            }
            OPCODE_JR_NZ => {
                let data = self.get_8_bit_arg(mem) as i8;
                let pc = self.pc.get_16bit_value() as i16;
                // Jump relative to the byte _after_ JR
                let target = pc.wrapping_add(data as i16 + 2);
                print!("JR NZ {:#04x?} \t Target: {:#04x?}\n", data, target);
                if !self.get_zero() {
                    return self.set_pc(target as u16);
                } else {
                    return self.increment_pc(2);
                }
            }
            OPCODE_RRA => {
                print!("RRA \n");
                let old_carry: u8 = if self.get_carry() { 1 } else { 0 };
                let old_a = self.af.get_high_byte();
                let new_a = (old_a >> 1) | (old_carry << 7);
                return self
                    .set_carry((old_a & 1) == 1)
                    .set_zero(new_a == 0)
                    .increment_pc(1);
            }
            OPCODE_DI => {
                //TODO: Implement (Disable interrupts)
                print!("DI \n");
                return self.increment_pc(1);
            }
            OPCODE_LDH_ADDR_A => {
                let data = self.get_8_bit_arg(mem) as u16;
                print!("LDH ({:#04x?}), A \n", 0xff00 + data);
                mem.write(0xff00 + data, self.get_a());
                return self.increment_pc(2);
            }
            OPCODE_CP_A_D => {
                return self.cp_immediate(mem);
            }
            OPCODE_LD_ADDR_HL_IMMEDIATE => {
                let data = self.get_8_bit_arg(mem);
                print!("LD (HL), {:#04x?} \n", data);
                mem.write(self.hl.get_16bit_value(), data);
                return self.increment_pc(2);
            }
            OPCODE_LD_ADDRESS_A => {
                let data = self.get_16_bit_arg(mem);
                print!("LD ({:#04x?}), A\n", data);
                mem.write(data, self.af.get_high_byte());
                return self.increment_pc(3);
            }
            OPCODE_LD_ADDR_C_A => {
                print!("LD (C), A\n");
                mem.write(self.get_c() as u16 + 0xff00, self.get_a());
                return self.increment_pc(1);
            }
            OPCODE_INC_C => {
                print!("INC C\n");
                return self.inc(Registers::C);
            }
            OPCODE_LD_ADDR_HL_A => {
                print!("LD (HL), A\n");
                mem.write(self.hl.get_16bit_value(), self.get_a());
                return self.increment_pc(1);
            }
            OPCODE_LD_A_ADDR_DE => {
                print!("LD A, (DE)\n");
                return self
                    .set_a(mem.read(self.de.get_16bit_value()))
                    .increment_pc(1);
            }
            OPCODE_CALL => {
                let data = self.get_16_bit_arg(mem);
                print!("CALL {:#04x?}\n", data);
                return self.push(mem, self.pc.get_16bit_value() + 3).set_pc(data);
            }
            OPCODE_LD_C_A => {
                print!("LD C, A\n");
                return self.set_c(self.get_a()).increment_pc(1);
            }
            OPCODE_PUSH_BC => {
                print!("PUSH BC\n");
                self.push_two_bytes(mem, self.get_b(), self.get_c());
                return self.increment_pc(1);
            }
            _ => {
                print!("Unknown opcopde: {:#04x?}\n", opcode);
                return CPU { ..*self };
            }
        }
    }

    /// Loads an 8 bit immediate value to a register
    ///
    /// Mnemonic: ld r, n
    fn ld_8_bit_immediate(&self, mem: &mem::Mem, reg: Registers) -> CPU {
        let data = self.get_8_bit_arg(mem);
        print!("{:#04x?}\n", data);
        match reg {
            Registers::A => {
                return self.set_a(data).increment_pc(2);
            }
            Registers::B => {
                return self.set_b(data).increment_pc(2);
            }
            Registers::C => {
                return self.set_c(data).increment_pc(2);
            }
            Registers::D => {
                return self.set_d(data).increment_pc(2);
            }
            Registers::E => {
                return self.set_e(data).increment_pc(2);
            }
            _ => {
                panic!("Register does not support 8 bit loads");
            }
        }
    }

    /// Compares immediate value with reg A and sets flags accordingly
    ///
    /// Mnemonic: cp n
    fn cp_immediate(&self, mem: &mem::Mem) -> CPU {
        let data = self.get_8_bit_arg(mem);
        print!("CP {:#04x?}\n", data);
        return self.compare(self.get_a(), data).increment_pc(2);
    }

    /// Compares two values and sets flags accordingly
    fn compare(&self, a: u8, b: u8) -> CPU {
        //TODO: Set half carry
        if (a as i8 - b as i8) == 0 {
            return self.set_zero(true).set_subtract(true);
        } else if (a as i8 - b as i8) < 0 {
            return self.set_carry(true).set_zero(false);
        } else {
            return self.set_carry(false).set_zero(false);
        }
    }

    /// Decrements a given register value and sets the flags appropriately
    fn dec(&self, reg: Registers) -> CPU {
        //TODO: Half carry flag?
        match reg {
            Registers::A => {
                let new_a = self.get_a().wrapping_sub(1);
                return self
                    .set_a(new_a)
                    .set_zero(new_a == 0)
                    .set_subtract(true)
                    .increment_pc(1);
            }
            Registers::B => {
                let new_b = self.get_b().wrapping_sub(1);
                return self
                    .set_a(new_b)
                    .set_zero(new_b == 0)
                    .set_subtract(true)
                    .increment_pc(1);
            }
            Registers::C => {
                let new_c = self.get_c().wrapping_sub(1);
                return self
                    .set_c(new_c)
                    .set_zero(new_c == 0)
                    .set_subtract(true)
                    .increment_pc(1);
            }
            Registers::D => {
                let new_d = self.get_d().wrapping_sub(1);
                return self
                    .set_d(new_d)
                    .set_zero(new_d == 0)
                    .set_subtract(true)
                    .increment_pc(1);
            }
            Registers::E => {
                let new_e = self.get_e().wrapping_sub(1);
                return self
                    .set_e(new_e)
                    .set_zero(new_e == 0)
                    .set_subtract(true)
                    .increment_pc(1);
            }
            Registers::HL => {
                let new_hl = self.get_hl().wrapping_sub(1);
                return self
                    .set_hl(new_hl)
                    .set_zero(new_hl == 0)
                    .set_subtract(true)
                    .increment_pc(1);
            }
            _ => {
                panic!("Register does not support 8 bit loads");
            }
        }
    }

    fn inc(&self, reg: Registers) -> CPU {
        //TODO: Half carry flag?
        match reg {
            Registers::A => {
                let new_a = self.get_a().wrapping_add(1);
                return self
                    .set_a(new_a)
                    .set_zero(new_a == 0)
                    .set_subtract(false)
                    .increment_pc(1);
            }
            Registers::B => {
                let new_b = self.get_b().wrapping_add(1);
                return self
                    .set_b(new_b)
                    .set_zero(new_b == 0)
                    .set_subtract(false)
                    .increment_pc(1);
            }
            Registers::C => {
                let new_c = self.get_c().wrapping_add(1);
                return self
                    .set_c(new_c)
                    .set_zero(new_c == 0)
                    .set_subtract(false)
                    .increment_pc(1);
            }
            Registers::D => {
                let new_d = self.get_d().wrapping_add(1);
                return self
                    .set_d(new_d)
                    .set_zero(new_d == 0)
                    .set_subtract(false)
                    .increment_pc(1);
            }
            Registers::E => {
                let new_e = self.get_e().wrapping_add(1);
                return self
                    .set_e(new_e)
                    .set_zero(new_e == 0)
                    .set_subtract(false)
                    .increment_pc(1);
            }
            _ => {
                panic!("Register does not support 8 bit loads");
            }
        }
    }

    fn push_two_bytes(&self, mem: &mut mem::Mem, byte1: u8, byte2: u8) -> CPU {
        let current_sp = self.sp.get_16bit_value();
        mem.write(current_sp, byte1);
        mem.write(current_sp - 1, byte2);
        return self.decrement_sp(2);
    }
    fn push(&self, mem: &mut mem::Mem, value: u16) -> CPU {
        let (high_byte, low_byte) = self.byte_split(value);
        return self.push_two_bytes(mem, high_byte, low_byte);
    }

    fn byte_split(&self, value: u16) -> (u8, u8) {
        let high_byte = ((value as u16) >> 8) as u8;
        let low_byte = (value & 0x00ff) as u8;
        (high_byte, low_byte)
    }

    fn get_8_bit_arg(&self, mem: &mem::Mem) -> u8 {
        return mem.read(self.pc.get_16bit_value() + 1);
    }

    fn get_16_bit_arg(&self, mem: &mem::Mem) -> u16 {
        let current_pc = self.pc.get_16bit_value();
        return ((mem.read(current_pc + 2) as u16) << 8) | (mem.read(current_pc + 1) as u16);
    }

    fn get_carry(&self) -> bool {
        self.get_flag(CARRY_FLAG)
    }

    fn set_carry(&self, value: bool) -> CPU {
        return self.set_flag(CARRY_FLAG, value);
    }

    fn get_subtract(&self) -> bool {
        self.get_flag(SUBTRACT_FLAG)
    }

    fn set_subtract(&self, value: bool) -> CPU {
        return self.set_flag(SUBTRACT_FLAG, value);
    }

    fn get_zero(&self) -> bool {
        self.get_flag(ZERO_FLAG)
    }

    fn set_zero(&self, value: bool) -> CPU {
        return self.set_flag(ZERO_FLAG, value);
    }

    fn get_flag(&self, flag: u8) -> bool {
        (self.get_f() & flag) > 0
    }

    fn set_flag(&self, flag: u8, value: bool) -> CPU {
        return if value == true {
            self.set_f(self.get_f() | flag)
        } else {
            self.set_f(self.get_f() ^ flag)
        };
    }
}

pub fn init_cpu() -> CPU {
    CPU {
        af: init_16bit_register(0),
        bc: init_16bit_register(0),
        de: init_16bit_register(0),
        hl: init_16bit_register(0),
        sp: init_16bit_register(0xfffe),
        pc: init_16bit_register(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zero() {
        let cpu = init_cpu();
        cpu.set_zero(true);
        assert_eq!(cpu.get_f() & ZERO_FLAG, 128)
    }

    #[test]
    fn test_unset_zero() {
        let cpu = CPU {
            af: init_register(0, 0b01000000),
            bc: init_16bit_register(0),
            de: init_16bit_register(0),
            hl: init_16bit_register(0),
            sp: init_16bit_register(0),
            pc: init_16bit_register(0),
        };
        cpu.set_zero(false);
        assert_eq!(cpu.get_f() & ZERO_FLAG, 0)
    }
}
