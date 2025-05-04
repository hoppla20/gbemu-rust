use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    process::Command,
};
use tracing::{info, warn};

use crate::{emulator::Emulator, tests::setup_logger};

const TRACES_DIR: &str = "traces";

fn trace_file_path(test_num: usize) -> PathBuf {
    Path::new(TRACES_DIR).join(format!("cpu_instrs_{:02}.log", test_num))
}

fn test_blargg_cpu_instrs(rom_file_path: &str, test_num: usize, min_traces_option: Option<usize>) {
    let traces = trace_file_path(test_num);
    let _guard = setup_logger(&traces);

    let f = File::open(rom_file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mut emu = Emulator::new_from_buffer(&rom, None, None);
    emu.mmu.graphics.registers.lcd_y = 0x90;

    let re_failed = Regex::new(r"^Failed").unwrap();
    let re_passed = Regex::new(r"^Passed").unwrap();
    let mut test_passed = false;
    let mut cycle = 0;
    loop {
        if let Err(err) = emu.step() {
            warn!("Encountered error on cycle {}: {:02X?}", cycle, err);
            test_passed = false;
            break;
        }

        if re_failed.is_match(emu.mmu.serial.get_last_buffer()) {
            warn!("Tests failed!");
            test_passed = false;
            break;
        }

        if !test_passed && re_passed.is_match(emu.mmu.serial.get_last_buffer()) {
            info!("Tests passed after {} traces!", emu.instruction_counter());
            test_passed = true;
        }

        cycle += 0;
        if test_passed {
            if let Some(min_traces) = min_traces_option {
                if emu.instruction_counter() >= min_traces {
                    info!("Ran {} number of traces", emu.instruction_counter());
                    break;
                }
            }
        }
    }

    info!("Running gameboy-doctor");
    if cfg!(target_os = "windows") {
        unimplemented!("Executing this test on windows is currently not supported");
    } else {
        let gd_command = Command::new("/usr/bin/env")
            .arg("python3")
            .arg("external/gameboy-doctor/gameboy-doctor")
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
fn test_blargg_cpu_instrs_01() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/01-special.gb",
        1,
        Some(1257587),
    );
}

#[test]
fn test_blargg_cpu_instrs_02() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/02-interrupts.gb",
        2,
        Some(161057),
    );
}

#[test]
fn test_blargg_cpu_instrs_03() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/03-op sp,hl.gb",
        3,
        Some(1066160),
    );
}

#[test]
fn test_blargg_cpu_instrs_04() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/04-op r,imm.gb",
        4,
        Some(1260504),
    );
}

#[test]
fn test_blargg_cpu_instrs_05() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/05-op rp.gb",
        5,
        Some(1761126),
    );
}

#[test]
fn test_blargg_cpu_instrs_06() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb",
        6,
        Some(241011),
    );
}

#[test]
fn test_blargg_cpu_instrs_07() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
        7,
        Some(587415),
    );
}

#[test]
fn test_blargg_cpu_instrs_08() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/08-misc instrs.gb",
        8,
        Some(221630),
    );
}

#[test]
fn test_blargg_cpu_instrs_09() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/09-op r,r.gb",
        9,
        Some(4418120),
    );
}

#[test]
fn test_blargg_cpu_instrs_10() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/10-bit ops.gb",
        10,
        Some(6712461),
    );
}

#[test]
fn test_blargg_cpu_instrs_11() {
    test_blargg_cpu_instrs(
        "test_roms/blargg/cpu_instrs/individual/11-op a,(hl).gb",
        11,
        Some(7427500),
    );
}
