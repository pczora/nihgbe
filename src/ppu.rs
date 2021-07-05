use super::mem;

#[derive(Copy, Clone)]
pub struct PPU {
    scanline_counter: i16,
}

const ADDR_LY: u16 = 0xff44;

impl PPU {
    pub fn update(&self, cycles: u8, mem: &mut mem::Mem) -> PPU {
        if self.display_disabled(mem) {
            return *self;
        }

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
}

pub fn init_ppu() -> PPU {
    PPU {
        scanline_counter: 456,
    }
}
