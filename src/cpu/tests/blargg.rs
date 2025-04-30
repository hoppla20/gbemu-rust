use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    cpu::Cpu,
    tests::{Mbc0, Mmu},
};

#[test]
#[ignore = "not finished"]
fn test_blargg_cpu_instrs_01() {
    let f = File::open("test_roms/blargg/cpu_instrs/individual/01-special.gb").unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();
    println!("{}", rom[0x0001]);
    println!("{}", rom[0x0100]);

    let mbc = Mbc0::new_from_buffer(&rom, false);
    let mut mmu = Mmu::new(Box::new(mbc), false);
    let mut cpu = Cpu::new(&mmu);

    println!("{}", mmu.read_byte(0x0001));
    println!("{}", mmu.read_byte(0x0100));

    loop {
        if let Err(err) = cpu.step(&mut mmu) {
            panic!("{:?}", err)
        }
    }
}
