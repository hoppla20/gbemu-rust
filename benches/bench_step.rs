use std::{
    fs::File,
    io::{BufReader, Read},
};

use criterion::{Criterion, criterion_group, criterion_main};
use gbemu_rust::prelude::*;

const ROM_FILE_PATH: &str = "test_roms/blargg/cpu_instrs/cpu_instrs.gb";

pub fn bench_blargg_cpu_instrs_full(c: &mut Criterion) {
    let f = File::open(ROM_FILE_PATH).unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mut emu = Emulator::new_from_buffer(rom, None, None).unwrap();
    emu.mmu.graphics.registers.lcd_y = 0x90;

    c.bench_function("blargg_cpu_instrs", |b| b.iter(|| emu.step()));
}

criterion_group!(benches, bench_blargg_cpu_instrs_full);
criterion_main!(benches);
