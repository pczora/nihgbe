use super::mem;

const ZERO_FLAG: u8 = 0b10000000;
const CARRY_FLAG: u8 = 0b00010000;

const OPCODE_NOP: u8 = 0x0000;
const OPCODE_JP: u8 = 0x00C3;
const OPCODE_XOR_A: u8 = 0x00AF;
const OPCODE_LD_HL: u8 = 0x0021;
const OPCODE_LDD_HL_A: u8 = 0x0032;

// 8 bit loads
const OPCODE_LD_B: u8 = 0x0006;
const OPCODE_LD_C: u8 = 0x000e;

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

impl CPU {
    pub fn execute(&mut self, mem: &mem::Mem, num_instructions: u8) {
        if num_instructions == 0 {
            return;
        };
        let pc_usize = self.pc as usize;
        let opcode = mem.read(self.pc);
        print!("{:#04x?}\t", self.pc);
        match opcode {
            OPCODE_NOP => {
                print!("{}\n", "NOP");
                self.pc += 1;
            }
            OPCODE_JP => {
                let jp_dest = self.get_16_bit_arg(mem);
                print!("{} {:#04x?}\n", "JP", jp_dest);
                self.pc = jp_dest;
            }
            OPCODE_XOR_A => {
                print!("{}\n", "XOR A");
                self.a ^= self.a;
                self.pc += 1;
            }
            OPCODE_LD_HL => {
                let data = self.get_16_bit_arg(mem);
                print!("{} {:#04x?}\n", "LD HL,", data);
                self.hl = data;
                self.pc += 3;
            }
            OPCODE_LD_B => {
                let data = self.get_8_bit_arg(mem);
                print!("{} {:#04x?}\n", "LD B,", data);
                self.b = data;
                self.pc += 2
            }
            OPCODE_LD_C => {
                let data = self.get_8_bit_arg(mem);
                print!("{} {:#04x?}\n", "LD C,", data);
                self.c = data;
                self.pc += 2
            }
            OPCODE_LDD_HL_A => {
                print!("{} \n", "LDD (HL), A");
                // TODO: Actually write to RAM
            }
            _ => {
                panic!("Invalid or unimplemented op code {:#04x?}", opcode)
            }
        }
        self.execute(mem, num_instructions - 1);
    }

    fn get_8_bit_arg(&self, mem: &mem::Mem) -> u8 {
        return mem.read(self.pc + 1);
    }

    fn get_16_bit_arg(&self, mem: &mem::Mem) -> u16 {
        return ((mem.read(self.pc + 2) as u16) << 8) | (mem.read(self.pc + 1) as u16);
    }

    fn set_zero(&mut self) {
        self.flags = self.flags | ZERO_FLAG;
    }

    fn unset_zero(&mut self) {
        self.flags = self.flags ^ ZERO_FLAG;
    }

    fn set_carry(&mut self) {
        self.flags = self.flags | CARRY_FLAG;
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
        cpu.set_zero();
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
        cpu.unset_zero();
        assert!(cpu.flags & ZERO_FLAG == 0)
    }
}
