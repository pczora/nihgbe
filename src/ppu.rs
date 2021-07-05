use super::mem;

#[derive(Copy, Clone)]
pub struct PPU {
    scanline_counter: i16,
}

const ADDR_LY: u16 = 0xff44;
const ADDR_LSTAT: u16 = 0xff41;

impl PPU {
    pub fn update(&self, cycles: u8, mem: &mut mem::Mem) -> PPU {
        if self.display_disabled(mem) {
            return *self;
        }
        self.update_status(mem);
        if self.scanline_counter <= 0 {
            mem.write(ADDR_LY, mem.read(ADDR_LY) + 1);
            let current_line = mem.read(ADDR_LY);
            if current_line == 144 {
                //TODO: VBlank interrupt
            } else if current_line > 153 {
                // end of vblank period
                mem.write(ADDR_LY, 0);
            } else if current_line < 144 {
                //TODO: Draw scanline
            }
            return init_ppu();
        }

        return PPU {
            scanline_counter: self.scanline_counter - cycles as i16,
        };
    }

    fn display_disabled(&self, mem: &mem::Mem) -> bool {
        return if mem.read(0xff40) & 0b10000000 != 0 {
            false
        } else {
            true
        };
    }

    fn update_status(&self, mem: &mut mem::Mem) {
        // TODO: Set coincidence flag
        match self.scanline_counter {
            0..=80 => {
                mem.set_bit(ADDR_LSTAT, 1);
                mem.reset_bit(ADDR_LSTAT, 0);
                //TODO: Interrupt
            }
            81..=248 => {
                mem.set_bit(ADDR_LSTAT, 1);
                mem.set_bit(ADDR_LSTAT, 0);
                //TODO: Interrupt

            }
            249..=456 => {
                mem.reset_bit(ADDR_LSTAT, 1);
                mem.reset_bit(ADDR_LSTAT, 0);
                //TODO: Interrupt

            }
            _ => {
                mem.reset_bit(ADDR_LSTAT, 1);
                mem.set_bit(ADDR_LSTAT, 0);
                //TODO: Interrupt
            }
        }
    }

}

pub fn init_ppu() -> PPU {
    PPU {
        scanline_counter: 456,
    }
}
