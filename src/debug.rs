use super::mem;
use std::fs::File;
use chrono;
use std::io::{Write};
use super::cpu;

pub fn dump_mem(mem: &mem::Mem) {
    // TODO: The dump is a few bytes larger than it should be - needs to be debugged - could
    // be an issue with the memory implementation
    let time = chrono::offset::Local::now();
    let mut file = File::create(format!("mem_dump_{}.bin", time)).expect("Unable to create vram dump file");
    let dump = mem.dump();
    for byte in dump {
        file.write_all(&[byte]).expect("Failed to write byte to file");
    }
}
