mod helpers;

use std::path::Path;

use helpers::TRACES_DIR;
use helpers::setup_gameboy_doctor_logger;
use helpers::test_blargg_cpu_instrs;

#[test]
fn test_cpu_instrs_full() {
    let trace_path = Path::new(TRACES_DIR).join("cpu_instrs_full.log");
    let _guard = setup_gameboy_doctor_logger(&trace_path);

    test_blargg_cpu_instrs(
        "../external/test_roms/blargg/cpu_instrs/cpu_instrs.gb",
        54729535,
    );
}
