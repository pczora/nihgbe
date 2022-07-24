extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::fs;
use std::time::Duration;

use crate::debug::dump_mem;

mod cpu;
mod debug;
mod mem;
mod ppu;
mod registers;

const TITLE_START: u16 = 0x0134;
const TITLE_END: u16 = 0x0143;
const CPU_FREQUENCY_HZ: i32 = 4_194_304;

fn main() {
    let args: Vec<String> = env::args().collect();
    let boot_rom = fs::read(&args[1]).expect("Could not read boot ROM  file");
    let cart = fs::read(&args[2]).expect("Could not read cartridge file");

    let mut cpu = cpu::init_cpu();
    let mut ppu = ppu::init_ppu();
    let mut mem = mem::init_mem(boot_rom, cart);

    let title = parse_title(&mem);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(&title, 160, 144)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut paused = false;
    let mut breakpoint = 0x0235;
    let mut breakpoint_hit = false;
    println!("Running: {}", parse_title(&mem));
    'running: loop {
        let mut cycles_left: i32 = CPU_FREQUENCY_HZ / 60;

        let pc = cpu.get_16bit_register(&registers::Registers::PC);
        if !paused {
            println!("{:#06x}", pc);
            if pc == breakpoint && breakpoint_hit == false {
                println!("Hit breakpoint: {:#06x}", breakpoint);
                paused = true;
                breakpoint_hit = true;
            } else {
                while cycles_left > 0 {
                    let (new_cpu, cycles) = cpu.execute(&mut mem);
                    cycles_left -= cycles as i32;
                    cpu = new_cpu;
                    ppu = ppu.update(cycles, &mut mem);
                }
                breakpoint_hit = false;
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::F5),
                    ..
                } => {
                    paused = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F6),
                    ..
                } => {
                    breakpoint_hit = false;
                    paused = false;
                    breakpoint = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F12),
                    ..
                } => {
                    println!("{}", cpu);
                    println!("{:#06x}", mem.read(pc));
                    println!("{:#06x}", mem.read(pc + 1));
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn parse_title(mem: &mem::Mem) -> String {
    let title_vec = mem.read_range(TITLE_START..TITLE_END);
    let title_string = String::from_utf8(title_vec).expect("Could not parse title");

    // Game titles are padded with NUL bytes; we need to remove them
    return String::from(title_string.trim_end_matches(char::from('\0')));
}
