mod cpu;
mod mem;

use std::env;
use std::fs;

const TITLE_START: u16 = 0x0134;
const TITLE_END: u16 = 0x0143;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cart = fs::read(&args[1]).expect("Could not read file");

    let mut cpu = cpu::init_cpu();
    let mem = mem::init_mem(cart);

    println!("{}", parse_title(&mem));
    let num_instructions =
        u8::from_str_radix(&args[2], 10).expect("Could not parse num_instructions parameter");
    cpu.execute(&mem, num_instructions);
}

fn parse_title(mem: &mem::Mem) -> String {
    let title_vec = mem.read_range(TITLE_START..TITLE_END);
    let title_string = String::from_utf8(title_vec.to_vec());
    return title_string.expect("Could not parse title");
}
