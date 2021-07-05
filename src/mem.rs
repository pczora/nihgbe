const INTERRUPT_ENABLE_REGISTER_START: u16 = 0xffff;
const HIGH_RAM_AREA_START: u16 = 0xff80;
const EMPTY_UNUSABLE_1_START: u16 = 0xff7f;
const IO_REGISTERS_START: u16 = 0xff00;
const EMPTY_UNUSABLE_0_START: u16 = 0xfea0;
const ECHO_INTERNAL_RAM_START: u16 = 0xe000;
const INTERNAL_RAM_START: u16 = 0xc000;
const CARTRIDGE_RAM_START: u16 = 0xa000;
const VRAM_START: u16 = 0x8000;

pub struct Mem {
    boot_rom: Vec<u8>,
    vram: Vec<u8>,
    cart: Vec<u8>,
    interrupt_enable_register: Vec<u8>,
    ram: Vec<u8>,
    io_regs: Vec<u8>,
    high_ram_area: Vec<u8>,
}

pub fn init_mem(boot_rom: Vec<u8>, cart: Vec<u8>) -> Mem {
    let ram_size = (INTERRUPT_ENABLE_REGISTER_START - INTERNAL_RAM_START) as usize;
    let io_regs_size = (EMPTY_UNUSABLE_1_START - IO_REGISTERS_START) as usize;
    let high_ram_area_size = (INTERRUPT_ENABLE_REGISTER_START - HIGH_RAM_AREA_START) as usize;
    let interrupt_enable_register_size = 1 as usize;
    let vram_size = (INTERNAL_RAM_START - VRAM_START) as usize;
    Mem {
        boot_rom,
        vram: vec![0; vram_size],
        cart,
        interrupt_enable_register: vec![0; interrupt_enable_register_size],
        ram: vec![0; ram_size],
        io_regs: vec![0; io_regs_size],
        high_ram_area: vec![0; high_ram_area_size],
    }
}

impl Mem {
    pub fn read_range(&self, range: std::ops::Range<u16>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for i in range {
            bytes.push(self.read(i))
        }
        return bytes;
    }

    pub fn read(&self, address: u16) -> u8 {
        let address_usize = address as usize;

        if address <= 0xff {
            // Boot ROM disabled??
            if self.read(0xff50) > 0 {
                return self.cart[address_usize];
            } else {
                return self.boot_rom[address_usize];
            }
        } else if address < VRAM_START {
            return self.cart[address_usize];
        } else if address >= VRAM_START && address < CARTRIDGE_RAM_START {
            return self.vram[address_usize - VRAM_START as usize];
        } else if address >= IO_REGISTERS_START && address < EMPTY_UNUSABLE_1_START {
            return self.io_regs[address_usize - IO_REGISTERS_START as usize];
        } else if address >= INTERNAL_RAM_START && address < ECHO_INTERNAL_RAM_START {
            return self.ram[address_usize - INTERNAL_RAM_START as usize];
        } else if address >= HIGH_RAM_AREA_START && address < INTERRUPT_ENABLE_REGISTER_START {
            return self.high_ram_area[address_usize - HIGH_RAM_AREA_START as usize];
        } else if address == INTERRUPT_ENABLE_REGISTER_START {
            return self.interrupt_enable_register
                [address_usize - INTERRUPT_ENABLE_REGISTER_START as usize];
        } else {
            panic!("Trying to read invalid/unimplemented memory area");
        };
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let address_usize = address as usize;

        if address <= VRAM_START {
            panic!("Trying to write to invalid address: {:#4x?}", address);
        } else if address >= VRAM_START && address < CARTRIDGE_RAM_START {
            self.vram[address_usize - VRAM_START as usize] = data;
        } else if address >= IO_REGISTERS_START && address < EMPTY_UNUSABLE_1_START {
            self.io_regs[address_usize - IO_REGISTERS_START as usize] = data;
        } else if address >= INTERNAL_RAM_START && address < ECHO_INTERNAL_RAM_START {
            self.ram[address_usize - INTERNAL_RAM_START as usize] = data;
        } else if address >= HIGH_RAM_AREA_START && address < INTERRUPT_ENABLE_REGISTER_START {
            self.high_ram_area[address_usize - HIGH_RAM_AREA_START as usize] = data;
        } else if address == INTERRUPT_ENABLE_REGISTER_START {
            self.interrupt_enable_register
                [address_usize - INTERRUPT_ENABLE_REGISTER_START as usize] = data;
        } else {
            panic!(
                "Trying to write invalid/unimplemented memory area: {:#4x?}",
                address
            );
        };
    }
}
