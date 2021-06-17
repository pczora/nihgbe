use super::mem;

const ZERO_FLAG: u8 = 0b10000000;
const SUBTRACT_FLAG: u8 = 0b01000000;
const HALFCARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

const OPCODE_NOP: u8 = 0x0000;
const OPCODE_JP: u8 = 0x00c3;
const OPCODE_JR_NZ: u8 = 0x0020;
const OPCODE_DEC_B: u8 = 0x0005;
const OPCODE_DEC_C: u8 = 0x000d;
const OPCODE_DEC_E: u8 = 0x001d;
const OPCODE_DEC_H: u8 = 0x0025;
const OPCODE_XOR_A: u8 = 0x00AF;
const OPCODE_LD_HL: u8 = 0x0021;
const OPCODE_LDD_HL_A: u8 = 0x0032;
const OPCODE_LDH_ADDR_A: u8 = 0x00e0;
const OPCODE_LD_ADDR_HL_IMMEDIATE: u8 = 0x0036;

const OPCODE_RRA: u8 = 0x001f;

const OPCODE_DI: u8 = 0x00f3;

// 8 bit loads
const OPCODE_LD_A_IMMEDIATE: u8 = 0x003e;
const OPCODE_LD_B_IMMEDIATE: u8 = 0x0006;
const OPCODE_LD_C_IMMEDIATE: u8 = 0x000e;
const OPCODE_LD_E_IMMEDIATE: u8 = 0x0016;

const OPCODE_LD_ADDRESS_A: u8 = 0x00ea;

const OPCODE_LDH_A_ADDR: u8 = 0x00f0;

const OPCODE_CP_A_D: u8 = 0x00fe;

pub struct CPU {
    a: u8,
    flags: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    hl: u16,
    sp: u16,
    pc: u16,
}

enum Register {
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
    pub fn execute(&mut self, mem: &mut mem::Mem, num_instructions: u16) {
        if num_instructions == 0 {
            return;
        };
        let opcode = mem.read(self.pc);
        print!("{:#04x?}\t{}\t", self.pc, num_instructions);
        match opcode {
            OPCODE_NOP => {
                print!("NOP\n");
                self.pc += 1;
            }
            OPCODE_JP => {
                let jp_dest = self.get_16_bit_arg(mem);
                print!("JP {:#04x?}\n", jp_dest);
                self.pc = jp_dest;
            }
            OPCODE_XOR_A => {
                print!("XOR A\n");
                self.a ^= self.a;
                self.pc += 1;
            }
            OPCODE_LD_HL => {
                let data = self.get_16_bit_arg(mem);
                print!("LD HL, {:#04x?}\n", data);
                self.hl = data;
                self.pc += 3;
            }
            OPCODE_LD_A_IMMEDIATE => {
                print!("LD A, ");
                self.ld_8_bit_immediate(mem, Register::A);
            }
            OPCODE_LD_B_IMMEDIATE => {
                print!("LD B, ");
                self.ld_8_bit_immediate(mem, Register::B);
            }
            OPCODE_LD_C_IMMEDIATE => {
                print!("LD C, ");
                self.ld_8_bit_immediate(mem, Register::C);
            }
            OPCODE_LD_E_IMMEDIATE => {
                print!("LD E, ");
                self.ld_8_bit_immediate(mem, Register::E);
            }
            OPCODE_LDH_A_ADDR => {
                let data = self.get_8_bit_arg(mem) as u16;
                print!("LD A, ({:#04x?})\n", 0xff00 + data);
                self.a = mem.read(0xff00 + data);
                self.pc += 2
            }
            OPCODE_LDD_HL_A => {
                print!("LDD (HL), A \n");
                mem.write(self.hl, self.a);
                self.pc += 1
            }
            OPCODE_DEC_B => {
                print!("DEC B \n");
                self.dec(Register::B);
            }
            OPCODE_DEC_C => {
                print!("DEC C \n");
                self.dec(Register::C);
            }
            OPCODE_DEC_E => {
                print!("DEC E \n");
                self.dec(Register::E);
            }
            OPCODE_DEC_H => {
                //TODO: Implement
                //TODO: Half carry flag?
                print!("DEC H \n");
                unimplemented!();
            }
            OPCODE_JR_NZ => {
                let data = self.get_8_bit_arg(mem) as i8;
                let pc = self.pc as i16;
                // Jump relative to the byte _after_ JR
                let target = pc.wrapping_add(data as i16 + 2);
                print!("JR NZ {:#04x?} \t Target: {:#04x?}\n", data, target);
                if !self.get_zero() {
                    self.pc = target as u16;
                } else {
                    self.pc += 2;
                }
            }
            OPCODE_RRA => {
                print!("RRA \n");
                let old_carry: u8 = if self.get_carry() { 1 } else { 0 };

                if (self.a & 1) == 1 {
                    self.set_carry(true);
                }
                self.a = (self.a >> 1) | (old_carry << 7);
                if self.a == 0 {
                    self.set_zero(true);
                }
                self.pc += 1;
            }
            OPCODE_DI => {
                //TODO: Implement (Disable interrupts)
                print!("DI \n");
                self.pc += 1;
            }
            OPCODE_LDH_ADDR_A => {
                let data = self.get_8_bit_arg(mem) as u16;
                print!("LDH ({:#04x?}), A \n", 0xff00 + data);
                mem.write(0xff00 + data, self.a);
                self.pc += 2
            }
            OPCODE_CP_A_D => {
                self.cp_immediate(mem);
            }
            OPCODE_LD_ADDR_HL_IMMEDIATE => {
                let data = self.get_8_bit_arg(mem);
                print!("LD (HL), {:#04x?} \n", data);
                mem.write(self.hl, data);
                self.pc += 2
            }
            OPCODE_LD_ADDRESS_A => {
                let data = self.get_16_bit_arg(mem);
                print!("LD ({:#04x?}), A\n", data);
                mem.write(data, self.a);
                self.pc += 3
            }
            _ => {
                panic!("Invalid or unimplemented op code {:#04x?}", opcode)
            }
        }
        self.execute(mem, num_instructions - 1);
    }

    /// Loads an 8 bit immediate value to a register
    ///
    /// Mnemonic: ld r, n
    fn ld_8_bit_immediate(&mut self, mem: &mem::Mem, reg: Register) {
        let data = self.get_8_bit_arg(mem);
        print!("{:#04x?}\n", data);
        match reg {
            Register::A => {
                self.a = data;
            }
            Register::B => {
                self.b = data;
            }
            Register::C => {
                self.c = data;
            }
            Register::D => {
                self.d = data;
            }
            Register::E => {
                self.e = data;
            }
            _ => {
                panic!("Register does not support 8 bit loads");
            }
        }
        self.pc += 2;
    }

    /// Compares immediate value with reg A and sets flags accordingly
    ///
    /// Mnemonic: cp n
    fn cp_immediate(&mut self, mem: &mem::Mem) {
        let data = self.get_8_bit_arg(mem);
        print!("CP {:#04x?}\n", data);
        self.compare(self.a, data);
        self.pc += 2;
    }

    /// Compares two values and sets flags accordingly
    fn compare(&mut self, a: u8, b: u8) {
        //TODO: Set half carry
        self.set_subtract(true);
        if (a as i8 - b as i8) == 0 {
            self.set_zero(true);
        } else if (a as i8 - b as i8) < 0 {
            self.set_carry(true);
            self.set_zero(false);
        } else {
            self.set_carry(false);
            self.set_zero(false);
        }
    }

    /// Decrements a given register value and sets the flags appropriately
    fn dec(&mut self, reg: Register) {
        //TODO: Half carry flag?
        match reg {
            Register::A => {
                self.a = self.a.wrapping_sub(1);
                self.set_zero(self.a == 0);
            }
            Register::B => {
                self.b = self.b.wrapping_sub(1);
                self.set_zero(self.b == 0);
            }
            Register::C => {
                self.c = self.c.wrapping_sub(1);
                self.set_zero(self.c == 0);
            }
            Register::D => {
                self.d = self.d.wrapping_sub(1);
                self.set_zero(self.d == 0);
            }
            Register::E => {
                self.d = self.e.wrapping_sub(1);
                self.set_zero(self.e == 0);
            }
            _ => {
                panic!("Register does not support 8 bit loads");
            }
        }
        self.set_subtract(true);
        self.pc += 1;
    }

    fn get_8_bit_arg(&self, mem: &mem::Mem) -> u8 {
        return mem.read(self.pc + 1);
    }

    fn get_16_bit_arg(&self, mem: &mem::Mem) -> u16 {
        return ((mem.read(self.pc + 2) as u16) << 8) | (mem.read(self.pc + 1) as u16);
    }

    fn get_carry(&self) -> bool {
        self.get_flag(CARRY_FLAG)
    }

    fn set_carry(&mut self, value: bool) {
        self.set_flag(CARRY_FLAG, value);
    }

    fn get_subtract(&self) -> bool {
        self.get_flag(SUBTRACT_FLAG)
    }

    fn set_subtract(&mut self, value: bool) {
        self.set_flag(SUBTRACT_FLAG, value);
    }

    fn get_zero(&self) -> bool {
        self.get_flag(ZERO_FLAG)
    }

    fn set_zero(&mut self, value: bool) {
        self.set_flag(ZERO_FLAG, value);
    }

    fn get_flag(&self, flag: u8) -> bool {
        (self.flags & flag) > 0
    }

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value == true {
            self.flags |= flag;
        } else {
            self.flags ^= flag;
        }
    }
}

pub fn init_cpu() -> CPU {
    CPU {
        a: 0,
        flags: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        hl: 0,
        sp: 0xFFFE,
        pc: 0x0100,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zero() {
        let mut cpu = init_cpu();
        cpu.set_zero(true);
        assert!(cpu.flags & ZERO_FLAG == 128)
    }

    #[test]
    fn test_unset_zero() {
        let mut cpu = CPU {
            a: 0,
            flags: 0b10000000,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        };
        cpu.set_zero(false);
        assert!(cpu.flags & ZERO_FLAG == 0)
    }
}
