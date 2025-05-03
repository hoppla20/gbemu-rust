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

fn test_blargg_cpu_instrs(rom_file_path: &str, test_num: usize, max_traces_option: Option<usize>) {
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
            info!("Tests passed after {} traces!", emu.trace_counter());
            test_passed = true;
        }

        cycle += 0;
        if let Some(max_traces) = max_traces_option {
            if emu.trace_counter() >= max_traces {
                info!("Ran {} number of traces", emu.trace_counter());
                break;
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
        None,
    );
}
