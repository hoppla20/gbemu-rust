use std::{
    fs::File,
    io::{BufReader, Read},
    process::Command,
};

use regex::Regex;
use tracing::{info, warn};

use crate::{emulator::Emulator, tests::setup_logger};

fn test_blargg_cpu_instrs(file_path: &str, doctor_test_num: Option<usize>) {
    setup_logger();

    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mut emu = Emulator::new_from_buffer(&rom, None);
    emu.mmu.graphics.registers.lcd_y = 0x90;

    let mut cycle = 0;
    let re_failed = Regex::new(r"^Failed.*$").unwrap();
    let re_passed = Regex::new(r"^Passed$").unwrap();
    let test_result;
    loop {
        cycle += 1;
        if let Err(err) = emu.step() {
            warn!("Encountered error on cycle {}: {:02X?}", cycle, err);
            test_result = false;
            break;
        }

        if re_failed.is_match(emu.mmu.serial.get_last_buffer()) {
            warn!("Tests failed!");
            test_result = false;
            break;
        }

        if re_passed.is_match(emu.mmu.serial.get_last_buffer()) {
            info!("Tests passed!");
            test_result = true;
            break;
        }
    }

    if let Some(test_num) = doctor_test_num {
        info!("Running gameboy-doctor");
        if cfg!(target_os = "windows") {
            unimplemented!("Executing this test on windows is currently not supported");
        } else {
            let gd_command = Command::new("/usr/bin/env")
                .arg("python3")
                .arg("external/gameboy-doctor/gameboy-doctor")
                .arg("trace.log")
                .arg("cpu_instrs")
                .arg(format!("{}", test_num))
                .output()
                .expect("Could not execute gameboy-doctor");

            if test_result && gd_command.status.success() {
                info!(
                    "Gameboy-doctor:\n{}",
                    String::from_utf8(gd_command.stdout).unwrap()
                );
            } else {
                panic!(
                    "Gameboy-doctor:\n{}",
                    String::from_utf8(gd_command.stdout).unwrap()
                );
            }
        };
    }
}

#[test]
fn test_blargg_cpu_instrs_01() {
    test_blargg_cpu_instrs("test_roms/blargg/cpu_instrs/individual/01-special.gb", None);
}

#[test]
fn test_blargg_cpu_instrs_02() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/02-interrupts.gb",
        None,
    );
}

#[test]
fn test_blargg_cpu_instrs_03() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/03-op sp,hl.gb",
        Some(3),
    );
}
