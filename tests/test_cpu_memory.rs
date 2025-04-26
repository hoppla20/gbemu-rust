use gbemu_rust::{
    cpu::{Cpu, registers::Registers},
    memory::mbcs::{Mbc, MbcRomOnly},
};

fn new_test_cpu(mbc: &impl Mbc) -> Cpu {
    let mut result = Cpu::new_dmg(mbc, false);
    result.registers = Registers::default();

    result
}

#[test]
fn test_arithmetics_simple() {
    let mut mbc = MbcRomOnly::new(0, 0, false);
    let mut cpu = new_test_cpu(&mbc);

    cpu.registers.a = 0b10;
    cpu.registers.b = 0b01;

    let instructions = [
        0x00, // 0x0000: NOP
        0x80, // 0x0001: ADD A, B
        0x90, // 0x0002: SUB A, B
        0x88, // 0x0003: ADC A, B
        0x98, // 0x0004: SBC A, B
        0xB0, // 0x0005: OR A, B
        0x90, // 0x0006: SUB A, B
        0xA0, // 0x0007: AND A, B
        0x3C, // 0x0008: INC A
        0x3D, // 0x0009: DEC A
        0x23, // 0x000A: INC HL
        0x86, // 0x000B: ADD A, (HL)
    ];

    mbc.memory.rom[0..instructions.len()].copy_from_slice(&(instructions));

    cpu.step(&mbc).unwrap(); // first instruction: NOP

    cpu.step(&mbc).unwrap(); // ADD A, B
    assert_eq!(cpu.registers.a, 0b11);

    cpu.step(&mbc).unwrap(); // SUB A, B
    assert_eq!(cpu.registers.a, 0b10);

    cpu.registers.set_flag_carry(true);
    cpu.step(&mbc).unwrap(); // ADC A, B
    assert_eq!(cpu.registers.a, 0b100);

    cpu.registers.set_flag_carry(true);
    cpu.step(&mbc).unwrap(); // SBC A, B
    assert_eq!(cpu.registers.a, 0b10);

    cpu.step(&mbc).unwrap(); // OR A, B
    assert_eq!(cpu.registers.a, 0b11);

    cpu.step(&mbc).unwrap(); // SUB A, B
    assert_eq!(cpu.registers.a, 0b10);

    cpu.step(&mbc).unwrap(); // AND A, B
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.get_flag_zero());

    cpu.step(&mbc).unwrap(); // INC A
    assert_eq!(cpu.registers.a, 0x01);

    cpu.step(&mbc).unwrap(); // DEC A
    assert_eq!(cpu.registers.a, 0x00);

    cpu.step(&mbc).unwrap(); // INC HL
    cpu.step(&mbc).unwrap(); // INC HL
    assert_eq!(cpu.registers.get_hl(), 0x0001);

    assert_eq!(cpu.registers.get_hl(), 0x0001);
    assert_eq!(mbc.read_byte(0x0001), 0x80);
    cpu.step(&mbc).unwrap(); // ADD A, (HL)
    assert_eq!(cpu.registers.z, 0x80);
    assert_eq!(cpu.registers.a, 0x00);
    cpu.step(&mbc).unwrap(); // ADD A, (HL)
    assert_eq!(cpu.registers.a, 0x80);
}
