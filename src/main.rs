mod cpu;
mod mem;

use std::env;
use std::fs;

const TITLE_START: u16 = 0x0134;
const TITLE_END: u16 = 0x0143;

fn main() {
    let args: Vec<String> = env::args().collect();
    let boot_rom = fs::read(&args[1]).expect("Could not read boot ROM  file");
    let cart = fs::read(&args[2]).expect("Could not read cartridge file");

    let mut cpu = cpu::init_cpu();
    let mut mem = mem::init_mem(boot_rom, cart);

    println!("{}", parse_title(&mem));
    let num_instructions =
        u16::from_str_radix(&args[3], 10).expect("Could not parse num_instructions parameter");
    cpu.execute(&mut mem, num_instructions);
}

fn parse_title(mem: &mem::Mem) -> String {
    let title_vec = mem.read_range(TITLE_START..TITLE_END);
    let title_string = String::from_utf8(title_vec);
    return title_string.expect("Could not parse title");
}
