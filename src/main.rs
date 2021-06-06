mod cpu;
use std::env;
use std::fs;

const TITLE_START: usize = 0x0134;
const TITLE_END: usize = 0x0143;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cart = fs::read(&args[1]).expect("Could not read file");
    let mut cpu = cpu::init_cpu();
    println!("{}", parse_title(&cart));
    cpu.execute(&cart, 7);
}

fn parse_title(rom: &Vec<u8>) -> String {
    let title_slice = &rom[TITLE_START..=TITLE_END];
    let title_string = String::from_utf8(title_slice.to_vec());
    return title_string.expect("Could not parse title");
}
