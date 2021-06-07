const INTERNAL_RAM_START: u16 = 0xc000;
const ECHO_INTERNAL_RAM_START: u16 = 0xe000;
const INTERRUPT_ENABLE_REGISTER_START: u16 = 0xffff;
const VRAM_START: u16 = 0x8000;

pub struct Mem {
    cart: Vec<u8>,
    ram: Vec<u8>,
}

pub fn init_mem(cart: Vec<u8>) -> Mem {
    let ram_size: usize = INTERRUPT_ENABLE_REGISTER_START as usize - INTERNAL_RAM_START as usize;
    Mem {
        cart,
        ram: vec![0; ram_size],
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

        if address < VRAM_START {
            return self.cart[address_usize];
        } else if address > INTERNAL_RAM_START && address < ECHO_INTERNAL_RAM_START {
            return self.ram[INTERNAL_RAM_START as usize + address_usize];
        } else {
            panic!("Trying to read invalid/unimplemented memory area");
        };
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let address_usize = address as usize;

        if address < VRAM_START {
            panic!("Trying to write to Cart ROM");
        } else if address > INTERNAL_RAM_START && address < ECHO_INTERNAL_RAM_START {
            self.ram[address_usize - INTERNAL_RAM_START as usize] = data;
        } else {
            panic!("Trying to write invalid/unimplemented memory area: {:#4x?}", address);
        };
    }
}
