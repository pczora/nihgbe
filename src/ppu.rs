use super::mem;
use std::fmt::Formatter;

struct Tile {
    data: Vec<u8>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02x} {:02x}\n\
        {:02x} {:02x}\n\
        {:02x} {:02x}\n\
        {:02x} {:02x}\n\
        {:02x} {:02x}\n\
        {:02x} {:02x}\n\
        {:02x} {:02x}\n\
        {:02x} {:02x}",
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
            self.data[7],
            self.data[8],
            self.data[9],
            self.data[10],
            self.data[11],
            self.data[12],
            self.data[13],
            self.data[14],
            self.data[15]
        )
    }
}

impl Tile {
    fn get_pixel_value(&self, pixel: u8) -> u8 {
        let row = pixel / 8;
        let pixel_in_row = pixel % 8;
        let msbit = if self.data[row as usize + 1] & (1 << (7 - pixel_in_row)) > 0 {
            1
        } else {
            0
        };
        let lsbit = if self.data[row as usize] & (1 << (7 - pixel_in_row)) > 0 {
            1
        } else {
            0
        };
        (msbit << 1) | lsbit
    }
}

fn init_tile(data: Vec<u8>) -> Tile {
    Tile { data }
}

#[derive(Copy, Clone)]
pub struct PPU {
    scanline_counter: i16,
}

const ADDR_LY: u16 = 0xff44;
const ADDR_LSTAT: u16 = 0xff41;
const ADDR_SCY: u16 = 0xff42;
const ADDR_SCX: u16 = 0xff43;

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
                // Adressing mode? For now, only consider 0x8000 (LCDC.4=1)
                let vram_addr: u16 = 0x8000;
                let ty = mem.read(ADDR_SCY).wrapping_add(current_line) / 8;
                for pixel in 0..=160 {
                    let tx = (mem.read(ADDR_SCX) + pixel) / 8;
                    let tile_offset_addr = 0x9800 + ty as u16 * 32 + tx as u16;
                    let tile_offset = mem.read(tile_offset_addr);
                    // TODO: Get tile (somewhat done), get pixel in tile, draw
                    // Also: probably do not get Tile for each and every pixel
                    let tile_address = vram_addr.wrapping_add(tile_offset as u16);
                    let tile_data = mem.read_bytes(vram_addr.wrapping_add(tile_offset as u16), 16);
                    let tile = init_tile(tile_data);
                    //if tile_address == 0x8010 {
                    //print!("{}", tile);
                    //}
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_pixel() {
        let tile_data = vec![
            0b10101010u8,
            0b01010101u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let tile = init_tile(tile_data);
        let pixel_data = tile.get_pixel_value(0);
        assert_eq!(pixel_data, 1);
    }
}
