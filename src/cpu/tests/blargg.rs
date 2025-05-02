use std::{
    fs::File,
    io::{BufReader, Read},
    process::Command,
};

use log::{error, info, warn};

use crate::{
    cpu::Cpu,
    tests::{Mbc0, Mmu, setup_logger},
};

#[test]
#[ignore = "manual only"]
fn test_blargg_cpu_instrs_01() {
    let _guards = setup_logger();

    let f = File::open("test_roms/blargg/cpu_instrs/individual/01-special.gb").unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mbc = Mbc0::new_from_buffer(&rom, false);
    let mut mmu = Mmu::new(Box::new(mbc), false);
    let mut cpu = Cpu::new_dmg(&mmu);

    let mut cycle = 0;
    loop {
        cycle += 1;
        if let Err(err) = cpu.step(&mut mmu) {
            warn!("Encountered error on cycle {}: {:?}", cycle, err);
            break;
        }
    }

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
            info!("{}", String::from_utf8(gd_command.stdout).unwrap());
        } else {
            error!(
                "Gameboy-doctor failed:\n{}",
                String::from_utf8(gd_command.stdout).unwrap()
            );
            panic!("Gameboy-doctor failed");
        }
    };
}
