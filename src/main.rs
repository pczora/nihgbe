extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::env;
use std::fs;
use crate::mem::Mem;
use crate::cpu::CPU;

mod cpu;
mod mem;

const TITLE_START: u16 = 0x0134;
const TITLE_END: u16 = 0x0143;

fn main() {
    let args: Vec<String> = env::args().collect();
    let boot_rom = fs::read(&args[1]).expect("Could not read boot ROM  file");
    let cart = fs::read(&args[2]).expect("Could not read cartridge file");

    let mut cpu = cpu::init_cpu();
    let mut mem = mem::init_mem(boot_rom, cart);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("nihgbe", 160, 140)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        cpu.update(&mut mem);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}



fn parse_title(mem: &mem::Mem) -> String {
    let title_vec = mem.read_range(TITLE_START..TITLE_END);
    let title_string = String::from_utf8(title_vec);
    return title_string.expect("Could not parse title");
}
