mod helpers;

use helpers::test_blargg_with_gameboy_doctor;

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
