const RAM_START: u16 = 0xff80;
const INTERRUPT_ENABLE_REGISTER_START: u16 = 0xffff;
const VRAM_START: u16 = 0x8000;

pub struct Mem {
    cart: Vec<u8>,
    ram: Vec<u8>,
}

pub fn init_mem(cart: Vec<u8>) -> Mem {
    let ram_size: usize = INTERRUPT_ENABLE_REGISTER_START as usize - RAM_START as usize;
    Mem {
        cart: cart,
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
        } else if address > RAM_START && address < INTERRUPT_ENABLE_REGISTER_START {
            return self.ram[RAM_START as usize + address_usize];
        } else {
            panic!("Trying to read invalid/unimplemented memory area");
        };

        return 0;
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let address_usize = address as usize;

        if address < VRAM_START {
            panic!("Trying to write to Cart ROM");
        } else if address > RAM_START && address < INTERRUPT_ENABLE_REGISTER_START {
            self.ram[RAM_START as usize + address_usize] = data;
        } else {
            panic!("Trying to write invalid/unimplemented memory area");
        };
    }
}
