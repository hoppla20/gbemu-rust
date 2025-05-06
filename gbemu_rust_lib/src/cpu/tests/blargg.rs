use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tracing::info;
use tracing::warn;

use crate::{
    emulator::Emulator,
    tests::{setup_default_logger, setup_gameboy_doctor_logger},
};

const TRACES_DIR: &str = "traces";

fn trace_file_path(test_num: usize) -> PathBuf {
    Path::new(TRACES_DIR).join(format!("cpu_instrs_{:02}.log", test_num))
}

fn test_blargg_cpu_instrs(rom_file_path: &str, num_steps: usize) -> bool {
    let f = File::open(rom_file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mut emu = Emulator::new_from_buffer(rom, None, None).unwrap();
    emu.mmu.graphics.registers.lcd_y = 0x90;

    let re_failed = Regex::new(r"^Failed").unwrap();
    let re_passed = Regex::new(r"^Passed").unwrap();
    let mut test_passed = false;
    let mut steps = 0;
    loop {
        if let Err(err) = emu.step() {
            warn!("Encountered error on cycle {}: {:02X?}", steps, err);
            test_passed = false;
            break;
        }

        if re_failed.is_match(emu.mmu.io.serial.get_last_buffer()) {
            warn!("Tests failed!");
            test_passed = false;
            break;
        }

        if !test_passed && re_passed.is_match(emu.mmu.io.serial.get_last_buffer()) {
            info!("Tests passed after {} steps!", steps);
            test_passed = true;
        }

        steps += 1;

        if steps >= num_steps {
            info!("Ran {} number of steps", steps,);
            break;
        }
    }

    test_passed
}

fn test_blargg_with_gameboy_doctor(rom_file_path: &str, test_num: usize, num_steps: usize) {
    let traces = trace_file_path(test_num);
    let _guard = setup_gameboy_doctor_logger(&traces);

    let test_passed = test_blargg_cpu_instrs(rom_file_path, num_steps);

    info!("Running gameboy-doctor");
    if cfg!(target_os = "windows") {
        unimplemented!("Executing this test on windows is currently not supported");
    } else {
        let gd_command = Command::new("/usr/bin/env")
            .arg("python3")
            .arg("../external/gameboy_doctor/gameboy-doctor")
            .arg(&traces)
            .arg("cpu_instrs")
            .arg(format!("{}", test_num))
            .output()
            .expect("Could not execute gameboy-doctor");

        if test_passed && gd_command.status.success() {
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

#[test]
fn test_cpu_instrs_01() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/01-special.gb",
        1,
        2357225,
    );
}

#[test]
fn test_cpu_instrs_02() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/02-interrupts.gb",
        2,
        388512,
    );
}

#[test]
fn test_cpu_instrs_03() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/03-op sp,hl.gb",
        3,
        2377729,
    );
}

#[test]
fn test_cpu_instrs_04() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/04-op r,imm.gb",
        4,
        2773231,
    );
}

#[test]
fn test_cpu_instrs_05() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/05-op rp.gb",
        5,
        3777275,
    );
}

#[test]
fn test_cpu_instrs_06() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb",
        6,
        545758,
    );
}

#[test]
fn test_cpu_instrs_07() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
        7,
        1572742,
    );
}

#[test]
fn test_cpu_instrs_08() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/08-misc instrs.gb",
        8,
        508675,
    );
}

#[test]
fn test_cpu_instrs_09() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/09-op r,r.gb",
        9,
        9352104,
    );
}

#[test]
fn test_cpu_instrs_10() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/10-bit ops.gb",
        10,
        14231718,
    );
}

#[test]
fn test_cpu_instrs_11() {
    test_blargg_with_gameboy_doctor(
        "../external/test_roms/blargg/cpu_instrs/individual/11-op a,(hl).gb",
        11,
        18065918,
    );
}

#[test]
#[ignore = "manual only"]
fn test_cpu_instrs_full() {
    let _guard = setup_default_logger();

    test_blargg_cpu_instrs(
        "../external/test_roms/blargg/cpu_instrs/cpu_instrs.gb",
        54729535,
    );
}
