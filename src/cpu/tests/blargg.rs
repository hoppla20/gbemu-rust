use std::{
    fs::File,
    io::{BufReader, Read},
    process::Command,
};

use log::{info, warn};
use regex::Regex;

use crate::{
    cpu::Cpu,
    serial::LogSerial,
    tests::{Mbc0, Mmu, setup_logger},
};

fn test_blargg(file_path: &str) {
    setup_logger();

    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mbc = Mbc0::new_from_buffer(&rom, false);
    let serial = LogSerial::default();
    let mut mmu = Mmu::new(Box::new(mbc), Box::new(serial));
    mmu.graphics.registers.lcd_y = 0x90;
    let mut cpu = Cpu::new(&mmu);

    let mut cycle = 0;
    let re_failed = Regex::new(r"^Failed .*$").unwrap();
    let re_passed = Regex::new(r"^Passed$").unwrap();
    loop {
        cycle += 1;
        if let Err(err) = cpu.step(&mut mmu) {
            warn!("Encountered error on cycle {}: {:02X?}", cycle, err);
            break;
        }

        assert!(!re_failed.is_match(mmu.serial.get_last_buffer()));

        if re_passed.is_match(mmu.serial.get_last_buffer()) {
            info!("Tests passed!");
            break;
        }
    }
}

#[test]
fn test_blargg_cpu_instrs_01() {
    test_blargg("test_roms/blargg/cpu_instrs/individual/01-special.gb");

    info!("Running gameboy-doctor");
    if cfg!(target_os = "windows") {
        unimplemented!("Executing this test on windows is currently not supported");
    } else {
        let gd_command = Command::new("/usr/bin/env")
            .arg("python3")
            .arg("external/gameboy-doctor/gameboy-doctor")
            .arg("trace.log")
            .arg("cpu_instrs")
            .arg("1")
            .output()
            .expect("Could not execute gameboy-doctor");

        if gd_command.status.success() {
            info!(
                "Gameboy-doctor passed:\n{}",
                String::from_utf8(gd_command.stdout).unwrap()
            );
        } else {
            panic!(
                "Gameboy-doctor failed:\n{}",
                String::from_utf8(gd_command.stdout).unwrap()
            );
        }
    };
}

#[test]
fn test_blargg_cpu_instrs_02() {
    test_blargg("test_roms/blargg/cpu_instrs/individual/02-interrupts.gb");
}
