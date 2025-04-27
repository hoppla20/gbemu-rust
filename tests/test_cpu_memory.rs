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
        0x09, // 0x000C: ADD HL, BC
        0xCB, // 0x000D: PREFIX
        0x84, // 0x000E: RES 0, H
        0x2B, // 0x000F: DEC HL
        0x34, // 0x0010: INC (HL)
        0xCB, // 0x0011: PREFIX
        0x36, // 0x0011: SWAP (HL)
        0x35, // 0x0013: DEC (HL)
        0xCB, // 0x0014: PREFIX
        0x66, // 0x0015: BIT 4, (HL)
        0xCB, // 0x0016: PREFIX
        0xE6, // 0x0017: SET 4, (HL)
        0xCB, // 0x0018: PREFIX
        0xA6, // 0x0019: RES 4, (HL)
    ];

    mbc.memory.rom[0..instructions.len()].copy_from_slice(&(instructions));

    assert!(cpu.step(&mut mbc).unwrap()); // first instruction: NOP

    assert!(cpu.step(&mut mbc).unwrap()); // ADD A, B
    assert_eq!(cpu.registers.a, 0b11);

    assert!(cpu.step(&mut mbc).unwrap()); // SUB A, B
    assert_eq!(cpu.registers.a, 0b10);

    cpu.registers.set_flag_carry(true);
    assert!(cpu.step(&mut mbc).unwrap()); // ADC A, B
    assert_eq!(cpu.registers.a, 0b100);

    cpu.registers.set_flag_carry(true);
    assert!(cpu.step(&mut mbc).unwrap()); // SBC A, B
    assert_eq!(cpu.registers.a, 0b10);

    assert!(cpu.step(&mut mbc).unwrap()); // OR A, B
    assert_eq!(cpu.registers.a, 0b11);

    assert!(cpu.step(&mut mbc).unwrap()); // SUB A, B
    assert_eq!(cpu.registers.a, 0b10);

    assert!(cpu.step(&mut mbc).unwrap()); // AND A, B
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.get_flag_zero());

    assert!(cpu.step(&mut mbc).unwrap()); // INC A
    assert_eq!(cpu.registers.a, 0x01);

    assert!(cpu.step(&mut mbc).unwrap()); // DEC A
    assert_eq!(cpu.registers.a, 0x00);

    assert!(!cpu.step(&mut mbc).unwrap()); // INC HL
    assert!(cpu.step(&mut mbc).unwrap()); // INC HL
    assert_eq!(cpu.registers.get_hl(), 0x0001);

    assert!(!cpu.step(&mut mbc).unwrap()); // ADD A, (HL)
    assert_eq!(cpu.registers.z, 0x80);
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.step(&mut mbc).unwrap()); // ADD A, (HL)
    assert_eq!(cpu.registers.a, 0x80);

    assert!(!cpu.step(&mut mbc).unwrap()); // ADD HL, BC
    assert!(cpu.step(&mut mbc).unwrap()); // ADD HL, BC
    assert_eq!(cpu.registers.get_hl(), 0x0101);

    assert!(cpu.step(&mut mbc).unwrap()); // PREFIX
    assert!(cpu.step(&mut mbc).unwrap()); // RES 0, H
    assert_eq!(cpu.registers.get_hl(), 0x0001);

    assert!(!cpu.step(&mut mbc).unwrap()); // DEC HL
    assert!(cpu.step(&mut mbc).unwrap()); // DEC HL
    assert_eq!(cpu.registers.get_hl(), 0x0000);

    cpu.registers.set_hl(0xFF80);
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x00);

    assert!(!cpu.step(&mut mbc).unwrap()); // INC (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // INC (HL)
    assert!(cpu.step(&mut mbc).unwrap()); // INC (HL)
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x01);

    assert!(cpu.step(&mut mbc).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // SWAP (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // SWAP (HL)
    assert!(cpu.step(&mut mbc).unwrap()); // SWAP (HL)
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x10);

    assert!(!cpu.step(&mut mbc).unwrap()); // DEC (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // DEC (HL)
    assert!(cpu.step(&mut mbc).unwrap()); // DEC (HL)
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x0F);

    assert!(cpu.step(&mut mbc).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // BIT 4, (HL)
    assert!(cpu.step(&mut mbc).unwrap()); // BIT 4, (HL)
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x0F);
    assert!(cpu.registers.get_flag_zero());

    assert!(cpu.step(&mut mbc).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // SET 4, (HL)
    assert!(cpu.step(&mut mbc).unwrap()); // SET 4, (HL)
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x1F);

    assert!(cpu.step(&mut mbc).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mbc).unwrap()); // RES 4, (HL)
    assert!(cpu.step(&mut mbc).unwrap()); // RES 4, (HL)
    assert_eq!(mbc.read_byte(cpu.registers.get_hl()), 0x0F);
}
