#![allow(dead_code)]

use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tracing::Event;
use tracing::Subscriber;
use tracing::dispatcher;
use tracing::info;
use tracing::warn;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::FilterFn;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;

use gbemu_rust_lib::prelude::*;

pub const TRACES_DIR: &str = "traces";

pub fn trace_file_path(test_num: usize) -> PathBuf {
    Path::new(TRACES_DIR).join(format!("cpu_instrs_{:02}.log", test_num))
}

struct DoctorEventFormatter;

impl<S, N> FormatEvent<S, N> for DoctorEventFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

pub fn logging_layer_stdout<S>() -> Box<dyn Layer<S> + Send + Sync>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env())
        .boxed()
}

pub fn logging_layer_gameboy_doctor<S>(path: &PathBuf) -> Box<dyn Layer<S> + Send + Sync>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.try_exists().unwrap() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
    }

    let trace_log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    tracing_subscriber::fmt::layer()
        .with_writer(trace_log_file)
        .without_time()
        .with_level(false)
        .with_target(false)
        .event_format(DoctorEventFormatter)
        .with_filter(FilterFn::new(|metadata| metadata.name() == "cpu::state"))
        .boxed()
}

pub fn setup_default_logger() -> dispatcher::DefaultGuard {
    let registry = tracing_subscriber::registry().with(logging_layer_stdout());

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}

pub fn setup_gameboy_doctor_logger(path: &PathBuf) -> dispatcher::DefaultGuard {
    let registry = tracing_subscriber::registry()
        .with(logging_layer_stdout())
        .with(logging_layer_gameboy_doctor(path));

    let guard = tracing::subscriber::set_default(registry);

    info!("Logger initialized");

    guard
}

pub fn test_blargg_cpu_instrs(rom_file_path: &str, num_steps: usize) -> bool {
    let f = File::open(rom_file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut rom = Vec::new();
    reader.read_to_end(&mut rom).unwrap();

    let mut emu = Emulator::new_from_buffer(rom, false, None, None).unwrap();
    emu.system.graphics.registers.set_lcd_ly(0x90);

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

        if re_failed.is_match(emu.system.io.serial.get_last_buffer()) {
            warn!("Tests failed!");
            test_passed = false;
            break;
        }

        if !test_passed && re_passed.is_match(emu.system.io.serial.get_last_buffer()) {
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

pub fn test_blargg_with_gameboy_doctor(rom_file_path: &str, test_num: usize, num_steps: usize) {
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
