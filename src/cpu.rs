use super::mem;

const ZERO_FLAG: u8 = 0b10000000;
const SUBTRACT_FLAG: u8 = 0b01000000;
const HALFCARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

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

const CPU_FREQUENCY_HZ: i32 = 4_194_304;

impl CPU {
    fn get_16bit_register(&self, reg: &Registers) -> u16 {
        match reg {
            Registers::AF => self.af.get_16bit_value(),
            Registers::BC => self.bc.get_16bit_value(),
            Registers::DE => self.de.get_16bit_value(),
            Registers::HL => self.hl.get_16bit_value(),
            Registers::SP => self.sp.get_16bit_value(),
            Registers::PC => self.pc.get_16bit_value(),
            _ => panic!("Cannot get 8 bit register as u16"),
        }
    }

    fn get_8bit_register(&self, reg: &Registers) -> u8 {
        match reg {
            Registers::A => self.af.get_high_byte(),
            Registers::Flags => self.af.get_low_byte(),
            Registers::B => self.bc.get_high_byte(),
            Registers::C => self.bc.get_low_byte(),
            Registers::D => self.de.get_high_byte(),
            Registers::E => self.de.get_low_byte(),
            Registers::H => self.hl.get_high_byte(),
            Registers::L => self.hl.get_low_byte(),
            _ => panic!("Cannot get 16 bit register as u8"),
        }
    }

    fn set_16bit_register(&self, reg: &Registers, value: u16) -> CPU {
        match reg {
            Registers::AF => {
                return CPU {
                    af: init_16bit_register(value),
                    ..*self
                };
            }
            Registers::BC => {
                return CPU {
                    bc: init_16bit_register(value),
                    ..*self
                };
            }
            Registers::DE => {
                return CPU {
                    de: init_16bit_register(value),
                    ..*self
                };
            }
            Registers::HL => {
                return CPU {
                    hl: init_16bit_register(value),
                    ..*self
                };
            }
            Registers::SP => {
                return CPU {
                    sp: init_16bit_register(value),
                    ..*self
                };
            }
            Registers::PC => {
                return CPU {
                    pc: init_16bit_register(value),
                    ..*self
                };
            }
            _ => {
                panic!("Cannot set 16 bit value for 8 bit register")
            }
        }
    }
    fn set_8bit_register(&self, reg: &Registers, value: u8) -> CPU {
        match reg {
            Registers::A => {
                return CPU {
                    af: init_register(value, self.af.low_byte),
                    ..*self
                };
            }
            Registers::Flags => {
                return CPU {
                    af: init_register(self.af.high_byte, value),
                    ..*self
                };
            }
            Registers::B => {
                return CPU {
                    bc: init_register(value, self.bc.low_byte),
                    ..*self
                };
            }
            Registers::C => {
                return CPU {
                    bc: init_register(self.bc.high_byte, value),
                    ..*self
                };
            }
            Registers::D => {
                return CPU {
                    de: init_register(value, self.de.low_byte),
                    ..*self
                };
            }
            Registers::E => {
                return CPU {
                    de: init_register(self.de.high_byte, value),
                    ..*self
                };
            }
            Registers::H => {
                return CPU {
                    hl: init_register(value, self.hl.low_byte),
                    ..*self
                };
            }
            Registers::L => {
                return CPU {
                    hl: init_register(self.hl.high_byte, value),
                    ..*self
                };
            }
            _ => panic!("Cannot set 16 bit register with 8 bit value"),
        }
    }
    fn increment_pc(&self, value: u16) -> CPU {
        self.set_16bit_register(
            &Registers::PC,
            self.get_16bit_register(&Registers::PC).wrapping_add(value),
        )
    }

    fn increment_sp(&self, value: u16) -> CPU {
        self.set_16bit_register(
            &Registers::SP,
            self.get_16bit_register(&Registers::SP).wrapping_add(value),
        )
    }

    fn decrement_sp(&self, value: u16) -> CPU {
        self.set_16bit_register(
            &Registers::SP,
            self.get_16bit_register(&Registers::SP).wrapping_sub(value),
        )
    }

    pub fn execute(&self, mem: &mut mem::Mem) -> (CPU, u8) {
        let opcode = mem.read(self.pc.get_16bit_value());
        print!("{:#04x?}\t", self.pc.get_16bit_value());
        match opcode {
            0x31 => self.load_16bit_immediate(mem, &Registers::SP),
            0x11 => self.load_16bit_immediate(mem, &Registers::DE),
            0xcb => {
                // TODO: Implement! Careful, getting the next byte and
                // running the instruction cannot be interrupted
                let data = self.get_8bit_arg(mem);
                match data {
                    // BIT 7, h
                    0x7c => {
                        print!("BIT 7,H\n");
                        let bit_is_zero = (1 & self.get_8bit_register(&Registers::H)) == 0;
                        return (self.set_zero(bit_is_zero).increment_pc(2), 4 + 8);
                    }
                    // RL C
                    0x11 => (self.prefixed_rotate_left(&Registers::C), 4 + 8),
                    // RL A
                    0x17 => (self.prefixed_rotate_left(&Registers::A), 4 + 8),
                    _ => {
                        print!("Invalid or unimplemented 16 byte opcode: {:#04x?}", data);
                        panic!()
                    }
                }
            }
            0x00 => {
                print!("NOP\n");
                return (self.increment_pc(1), 4);
            }
            0xc3 => {
                let jp_dest = self.get_16bit_arg(mem);
                print!("JP {:#04x?}\n", jp_dest);
                return (self.set_16bit_register(&Registers::PC, jp_dest), 16);
            }
            0xAF => {
                print!("XOR A\n");
                let current_a = self.get_8bit_register(&Registers::A);
                return (
                    self.set_8bit_register(&Registers::A, current_a ^ current_a)
                        .increment_pc(1),
                    4,
                );
            }
            0x21 => {
                let data = self.get_16bit_arg(mem);
                print!("LD HL, {:#04x?}\n", data);
                return (
                    self.set_16bit_register(&Registers::HL, data)
                        .increment_pc(3),
                    12,
                );
            }
            0x2e => self.ld_8bit_immediate(mem, &Registers::L),
            0x3e => self.ld_8bit_immediate(mem, &Registers::A),
            0x06 => self.ld_8bit_immediate(mem, &Registers::B),
            0x0e => self.ld_8bit_immediate(mem, &Registers::C),
            0x16 => self.ld_8bit_immediate(mem, &Registers::D),
            0x1e => self.ld_8bit_immediate(mem, &Registers::E),
            0xf0 => {
                let data = self.get_8bit_arg(mem) as u16;
                print!("LD A, ({:#04x?})\n", 0xff00 + data);
                return (
                    self.set_8bit_register(&Registers::A, mem.read(0xff00 + data))
                        .increment_pc(2),
                    12,
                );
            }
            0x32 => {
                print!("LDD (HL), A\n");
                mem.write(
                    self.get_16bit_register(&Registers::HL),
                    self.get_8bit_register(&Registers::A),
                );
                return (
                    self.set_16bit_register(
                        &Registers::HL,
                        self.get_16bit_register(&Registers::HL).wrapping_sub(1),
                    )
                    .increment_pc(1),
                    8,
                );
            }
            0x22 => self.load_increment_hl_a(mem),
            0x23 => self.inc_16bit_register(&Registers::HL),
            0x13 => self.inc_16bit_register(&Registers::DE),
            0x3d => self.dec_8bit_register(&Registers::A),
            0x05 => self.dec_8bit_register(&Registers::B),
            0x0d => self.dec_8bit_register(&Registers::C),
            0x15 => self.dec_8bit_register(&Registers::D),
            0x1d => self.dec_8bit_register(&Registers::E),
            0x25 => self.dec_8bit_register(&Registers::H),
            0x18 => self.jr(mem),
            0x20 => self.jr_nonzero(mem),
            0x28 => self.jr_zero(mem),
            0x1f => {
                print!("RRA \n");
                let old_carry: u8 = if self.get_carry() { 1 } else { 0 };
                let old_a = self.get_8bit_register(&Registers::A);
                let new_a = (old_a >> 1) | (old_carry << 7);
                return (
                    self.set_carry((old_a & 1) == 1)
                        .set_zero(new_a == 0)
                        .increment_pc(1),
                    4,
                );
            }
            0xf3 => {
                //TODO: Implement (Disable interrupts)
                print!("DI \n");
                return (self.increment_pc(1), 4);
            }
            0xe0 => {
                let data = self.get_8bit_arg(mem) as u16;
                print!("LDH ({:#04x?}), A \n", 0xff00 + data);
                mem.write(0xff00 + data, self.get_8bit_register(&Registers::A) as u8);
                return (self.increment_pc(2), 12);
            }
            0xfe => {
                return self.cp_immediate(mem);
            }
            0x36 => {
                let data = self.get_8bit_arg(mem);
                print!("LD (HL), {:#04x?} \n", data);
                mem.write(self.get_16bit_register(&Registers::HL), data);
                return (self.increment_pc(2), 12);
            }
            0xea => {
                let data = self.get_16bit_arg(mem);
                print!("LD ({:#04x?}), A\n", data);
                mem.write(data, self.get_8bit_register(&Registers::A));
                return (self.increment_pc(3), 16);
            }
            0xe2 => {
                print!("LD (C), A\n");
                mem.write(
                    self.get_8bit_register(&Registers::C) as u16 + 0xff00,
                    self.get_8bit_register(&Registers::A),
                );
                return (self.increment_pc(1), 8);
            }
            0x04 => return self.inc_8bit_register(&Registers::B),
            0x14 => return self.inc_8bit_register(&Registers::D),
            0x24 => return self.inc_8bit_register(&Registers::H),
            0x0c => return self.inc_8bit_register(&Registers::C),
            0x1c => return self.inc_8bit_register(&Registers::E),
            0x2c => return self.inc_8bit_register(&Registers::L),
            0x3c => return self.inc_8bit_register(&Registers::A),
            0x77 => {
                print!("LD (HL), A\n");
                mem.write(
                    self.get_16bit_register(&Registers::HL),
                    self.get_8bit_register(&Registers::A),
                );
                return (self.increment_pc(1), 8);
            }
            0x1a => {
                print!("LD A, (DE)\n");
                return (
                    self.set_8bit_register(
                        &Registers::A,
                        mem.read(self.get_16bit_register(&Registers::DE)),
                    )
                    .increment_pc(1),
                    8,
                );
            }
            0xcd => {
                let data = self.get_16bit_arg(mem);
                print!("CALL {:#04x?}\n", data);
                return (
                    self.push(mem, self.get_16bit_register(&Registers::PC) + 3)
                        .set_16bit_register(&Registers::PC, data),
                    24,
                );
            }
            0x4f => self.load(&Registers::C, &Registers::A),
            0x7b => self.load(&Registers::A, &Registers::E),
            0xc5 => {
                print!("PUSH BC\n");
                (
                    self.push(mem, self.get_16bit_register(&Registers::BC))
                        .increment_pc(1),
                    16,
                )
            }
            0xc9 => self.ret(mem),
            0x17 => self.rla(),                    // RLA
            0xc1 => self.pop(&Registers::BC, mem), // POP BC
            0x67 => self.load_register_from_register(&Registers::H, &Registers::A),
            0x78 => self.load_register_from_register(&Registers::A, &Registers::B),
            0x7c => self.load_register_from_register(&Registers::A, &Registers::H),
            0x7d => self.load_register_from_register(&Registers::A, &Registers::L),
            0x57 => self.load_register_from_register(&Registers::D, &Registers::A),
            0x90 => self.sub(&Registers::B),
            0xbe => { // CP (HL)
                print!("CP (HL)\n");
                let value = mem.read(self.get_16bit_register(&Registers::HL));
                (self.compare(self.get_8bit_register(&Registers::A), value).increment_pc(1), 8)
            },
            0x86 => { // ADD (HL)
                print!("ADD A, (HL)\n");
                let value = mem.read(self.get_16bit_register(&Registers::HL));
                self.add(&Registers::A, value)
            }
            _ => {
                print!("Unknown opcode: {:#04x?}\n", opcode);
                panic!();
            }
        }
    }

    fn load_register_from_register(&self, to: &Registers, from: &Registers) -> (CPU, u8) {
        print!("LD {}, {}\n", to, from);
        (
            self.set_8bit_register(to, self.get_8bit_register(from))
                .increment_pc(1),
            4,
        )
    }

    fn jr_nonzero(&self, mem: &mut mem::Mem) -> (CPU, u8) {
        print!("JR NZ ");
        return if !self.get_zero() {
            (self.reljump(mem), 12)
        } else {
            print!("\n");
            (self.increment_pc(2), 8)
        };
    }

    fn jr_zero(&self, mem: &mut mem::Mem) -> (CPU, u8) {
        print!("JR Z ");
        return if self.get_zero() {
            (self.reljump(mem), 12)
        } else {
            print!("\n");
            (self.increment_pc(2), 8)
        };
    }

    fn jr(&self, mem: &mut mem::Mem) -> (CPU, u8) {
        print!("JR ");
        return (self.reljump(mem), 12);
    }

    fn reljump(&self, mem: &mut mem::Mem) -> CPU {
        let data = self.get_8bit_arg(mem) as i8;
        let pc = self.get_16bit_register(&Registers::PC) as i16;
        // Jump relative to the byte _after_ JR
        let target = pc.wrapping_add(data as i16 + 2);
        print!("{:#04x?} \t Target: {:#04x?}\n", data, target);
        return self.set_16bit_register(&Registers::PC, target as u16);
    }

    /// Mnemonic: LD r, n
    fn load_16bit_immediate(&self, mem: &mem::Mem, reg: &Registers) -> (CPU, u8) {
        let data = self.get_16bit_arg(mem);
        print!("LD {}, {:#04x?}\n", reg, data);
        return (self.set_16bit_register(reg, data).increment_pc(3), 12);
    }

    /// Loads an 8 bit immediate value to a register
    ///
    /// Mnemonic: ld r, n
    fn ld_8bit_immediate(&self, mem: &mem::Mem, reg: &Registers) -> (CPU, u8) {
        let data = self.get_8bit_arg(mem);
        print!("LD {}, {:#04x?}\n", reg, data);
        (self.set_8bit_register(reg, data).increment_pc(2), 8)
    }

    /// Compares immediate value with reg A and sets flags accordingly
    ///
    /// Mnemonic: cp n
    fn cp_immediate(&self, mem: &mem::Mem) -> (CPU, u8) {
        let data = self.get_8bit_arg(mem);
        print!("CP {:#04x?}\n", data);
        return (
            self.compare(self.get_8bit_register(&Registers::A), data)
                .increment_pc(2),
            8,
        );
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
    fn dec_8bit_register(&self, reg: &Registers) -> (CPU, u8) {
        print!("DEC {}\n", reg);
        //TODO: Half carry flag?
        let new_value = self.get_8bit_register(reg).wrapping_sub(1);
        return (
            self.set_8bit_register(&reg, new_value)
                .set_zero(new_value == 0)
                .set_subtract(true)
                .increment_pc(1),
            4,
        );
    }

    fn dec_16bit_register(&self, reg: &Registers) -> CPU {
        //TODO: Half carry flag?
        let new_value = self.get_16bit_register(reg).wrapping_sub(1);
        print!("New value: {:#04x?} \n", new_value);
        return self
            .set_16bit_register(&reg, new_value)
            .set_zero(new_value == 0)
            .set_subtract(true)
            .increment_pc(1);
    }

    fn inc_8bit_register(&self, reg: &Registers) -> (CPU, u8) {
        print!("INC {}\n", reg);
        //TODO: Half carry flag?
        let new_value = (self.get_8bit_register(reg)).wrapping_add(1);
        return (
            self.set_8bit_register(reg, new_value)
                .set_zero(new_value == 0)
                .set_subtract(false)
                .increment_pc(1),
            4,
        );
    }

    fn inc_16bit_register(&self, reg: &Registers) -> (CPU, u8) {
        print!("INC {}\n", reg);
        //TODO: Half carry flag?
        let current_value = self.get_16bit_register(reg);
        let new_value = current_value.wrapping_add(1);
        return (
            self.set_16bit_register(reg, new_value)
                .set_zero(new_value == 0)
                .set_subtract(false)
                .increment_pc(1),
            8,
        );
    }

    fn push_two_bytes(&self, mem: &mut mem::Mem, byte1: u8, byte2: u8) -> CPU {
        let current_sp = self.get_16bit_register(&Registers::SP);
        mem.write(current_sp, byte1);
        mem.write(current_sp - 1, byte2);
        return self.decrement_sp(2);
    }
    fn push(&self, mem: &mut mem::Mem, value: u16) -> CPU {
        let (high_byte, low_byte) = self.byte_split(value);
        return self.push_two_bytes(mem, low_byte, high_byte);
    }

    fn byte_split(&self, value: u16) -> (u8, u8) {
        let high_byte = ((value as u16) >> 8) as u8;
        let low_byte = (value & 0x00ff) as u8;
        (high_byte, low_byte)
    }

    fn combine_bytes(&self, msb: u8, lsb: u8) -> u16 {
        ((msb as u16) << 8) | (lsb as u16)
    }

    fn get_8bit_arg(&self, mem: &mem::Mem) -> u8 {
        return mem.read(self.pc.get_16bit_value() + 1);
    }

    fn get_16bit_arg(&self, mem: &mem::Mem) -> u16 {
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
        (self.get_8bit_register(&Registers::Flags) & flag) > 0
    }

    fn set_flag(&self, flag: u8, value: bool) -> CPU {
        let current_value = self.get_8bit_register(&Registers::Flags);
        let new_state = if value == true {
            self.set_8bit_register(&Registers::Flags, current_value | flag)
        } else {
            self.set_8bit_register(&Registers::Flags, current_value & !flag)
        };
        return new_state;
    }

    fn prefixed_rotate_left(&self, reg: &Registers) -> CPU {
        print!("RL {}\n", reg);
        self.rotate_left(reg).increment_pc(2)
    }

    fn rla(&self) -> (CPU, u8) {
        print!("RLA\n");
        (self.rotate_left(&Registers::A).increment_pc(1), 4)
    }

    fn rotate_left(&self, reg: &Registers) -> CPU {
        let current_value = self.get_8bit_register(reg);
        let carry = (current_value & 0b10000000) == 128;
        let new_value = current_value << 1;
        self.set_carry(carry)
            .set_zero(new_value == 0)
            .set_8bit_register(reg, new_value)
    }

    fn load_increment_hl_a(&self, mem: &mut mem::Mem) -> (CPU, u8) {
        print!("LDI (HL), A\n");
        mem.write(
            self.get_16bit_register(&Registers::HL),
            self.get_8bit_register(&Registers::A),
        );
        return (
            self.set_16bit_register(
                &Registers::HL,
                self.get_16bit_register(&Registers::HL).wrapping_add(1),
            )
            .increment_pc(1),
            8,
        );
    }
    fn ret(&self, mem: &mem::Mem) -> (CPU, u8) {
        print!("RET\n");
        let address = self.stack_pop(mem);
        (
            self.set_16bit_register(&Registers::PC, address)
                .increment_sp(2),
            16,
        )
    }

    fn load(&self, dst: &Registers, src: &Registers) -> (CPU, u8) {
        print!("LD {}, {}\n", dst, src);
        return (
            self.set_8bit_register(dst, self.get_8bit_register(src))
                .increment_pc(1),
            4,
        );
    }

    fn pop(&self, reg: &Registers, mem: &mem::Mem) -> (CPU, u8) {
        print!("POP {}\n", reg);
        let value = self.stack_pop(mem);
        let new_cpu = match reg {
            Registers::BC => self
                .set_16bit_register(&Registers::BC, value)
                .increment_sp(2)
                .increment_pc(1),
            Registers::DE => todo!(),
            Registers::HL => todo!(),
            Registers::AF => todo!(),
            _ => panic! {"Invalid register to pop to"},
        };
        return (new_cpu, 12);
    }

    /// does not really pop (does not increment SP)
    fn stack_pop(&self, mem: &mem::Mem) -> u16 {
        let current_sp = self.get_16bit_register(&Registers::SP);
        let lsb = mem.read(current_sp + 2);
        let msb = mem.read(current_sp + 1);
        self.combine_bytes(msb, lsb)
    }

    fn sub(&self, rhs: &Registers) -> (CPU, u8) {
        // TODO: set half carry
        let current_a = self.get_8bit_register(&Registers::A);
        let rhs_value = self.get_8bit_register(rhs);
        let new_a = current_a - rhs_value;

        return (
            self.set_8bit_register(&Registers::A, new_a)
                .set_zero(new_a == 0)
                .set_carry(rhs_value > current_a)
                .increment_pc(1),
            4,
        );
    }
    fn add(&self, reg: &Registers, rhs: u8) -> (CPU, u8) {
        // TODO: set half carry & carry
        let current_value = self.get_8bit_register(reg);
        let new_value = current_value.wrapping_add(rhs);
        (self.set_8bit_register(reg, new_value).set_zero(new_value == 0).set_subtract(false).increment_pc(1), 8)
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
        let new_cpu = cpu.set_zero(true);
        assert_eq!(
            new_cpu.get_8bit_register(&Registers::Flags) & ZERO_FLAG,
            0b10000000
        )
    }

    #[test]
    fn test_get_zero() {
        let cpu = CPU {
            af: init_register(0, 0b10000000),
            bc: init_16bit_register(0),
            de: init_16bit_register(0),
            hl: init_16bit_register(0),
            sp: init_16bit_register(0),
            pc: init_16bit_register(0),
        };
        assert!(cpu.get_zero())
    }

    #[test]
    fn test_unset_zero() {
        let cpu = CPU {
            af: init_register(0, 0b10000000),
            bc: init_16bit_register(0),
            de: init_16bit_register(0),
            hl: init_16bit_register(0),
            sp: init_16bit_register(0),
            pc: init_16bit_register(0),
        };
        let new_cpu = cpu.set_zero(false);
        assert_eq!(new_cpu.get_8bit_register(&Registers::Flags) & ZERO_FLAG, 0)
    }

    #[test]
    fn test_set_carry() {
        let cpu = init_cpu();
        let new_cpu = cpu.set_carry(true);
        assert_eq!(
            new_cpu.get_8bit_register(&Registers::Flags) & CARRY_FLAG,
            CARRY_FLAG
        )
    }

    #[test]
    fn test_prefixed_rotate_left() {
        let cpu = init_cpu().set_8bit_register(&Registers::A, 0b10000000);
        let new_cpu = cpu.prefixed_rotate_left(&Registers::A);
        assert_eq!(new_cpu.get_8bit_register(&Registers::A), 0);
        assert!(new_cpu.get_carry());
        assert!(new_cpu.get_zero());
        assert_eq!(
            new_cpu.get_16bit_register(&Registers::PC),
            cpu.get_16bit_register(&Registers::PC) + 2
        );
    }

    #[test]
    fn test_combine_bytes() {
        let cpu = init_cpu();
        assert_eq!(
            0b0000000100000001,
            cpu.combine_bytes(0b00000001, 0b00000001)
        );
    }
}
