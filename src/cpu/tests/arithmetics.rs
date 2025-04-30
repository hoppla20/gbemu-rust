use crate::{
    cpu::Cpu,
    prelude::{Mbc0, Mmu},
};

#[test]
fn test_arithmetics_simple() {
    let instructions = vec![
        0x80, // ADD A, B
        0x90, // SUB A, B
        0x88, // ADC A, B
        0x98, // SBC A, B
        0xB0, // OR A, B
        0x90, // SUB A, B
        0xA0, // AND A, B
        0x3C, // INC A
        0x3D, // DEC A
        0x23, // INC HL
        0x2B, // DEC HL
        0x86, // ADD A, (HL)
        0x09, // ADD HL, BC
        0xCB, // PREFIX
        0x84, // RES 0, H
        0x34, // INC (HL)
        0xCB, // PREFIX
        0x36, // SWAP (HL)
        0x35, // DEC (HL)
        0xCB, // PREFIX
        0x66, // BIT 4, (HL)
        0xCB, // PREFIX
        0xE6, // SET 4, (HL)
        0xCB, // PREFIX
        0xA6, // RES 4, (HL)
    ];

    let mbc = Mbc0::new_from_buffer(&instructions, false);
    let mut mmu = Mmu::new(Box::new(mbc), false);
    let mut cpu = Cpu::new(&mmu);

    cpu.registers.a = 0b10;
    cpu.registers.b = 0b01;

    assert!(cpu.step(&mut mmu).unwrap()); // ADD A, B
    assert_eq!(cpu.registers.a, 0b11);

    assert!(cpu.step(&mut mmu).unwrap()); // SUB A, B
    assert_eq!(cpu.registers.a, 0b10);

    cpu.registers.set_flag_carry(true);
    assert!(cpu.step(&mut mmu).unwrap()); // ADC A, B
    assert_eq!(cpu.registers.a, 0b100);

    cpu.registers.set_flag_carry(true);
    assert!(cpu.step(&mut mmu).unwrap()); // SBC A, B
    assert_eq!(cpu.registers.a, 0b10);

    assert!(cpu.step(&mut mmu).unwrap()); // OR A, B
    assert_eq!(cpu.registers.a, 0b11);

    assert!(cpu.step(&mut mmu).unwrap()); // SUB A, B
    assert_eq!(cpu.registers.a, 0b10);

    assert!(cpu.step(&mut mmu).unwrap()); // AND A, B
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.get_flag_zero());

    assert!(cpu.step(&mut mmu).unwrap()); // INC A
    assert_eq!(cpu.registers.a, 0x01);

    assert!(cpu.step(&mut mmu).unwrap()); // DEC A
    assert_eq!(cpu.registers.a, 0x00);

    assert!(!cpu.step(&mut mmu).unwrap()); // INC HL
    assert!(cpu.step(&mut mmu).unwrap()); // INC HL
    assert_eq!(cpu.registers.get_hl(), 0x0001);

    assert!(!cpu.step(&mut mmu).unwrap()); // DEC HL
    assert!(cpu.step(&mut mmu).unwrap()); // DEC HL
    assert_eq!(cpu.registers.get_hl(), 0x0000);

    assert!(!cpu.step(&mut mmu).unwrap()); // ADD A, (HL)
    assert_eq!(cpu.registers.z, 0x80);
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.step(&mut mmu).unwrap()); // ADD A, (HL)
    assert_eq!(cpu.registers.a, 0x80);

    assert!(!cpu.step(&mut mmu).unwrap()); // ADD HL, BC
    assert!(cpu.step(&mut mmu).unwrap()); // ADD HL, BC
    assert_eq!(cpu.registers.get_hl(), 0x0100);

    assert!(cpu.step(&mut mmu).unwrap()); // PREFIX
    assert!(cpu.step(&mut mmu).unwrap()); // RES 0, H
    assert_eq!(cpu.registers.get_hl(), 0x0000);

    cpu.registers.set_hl(0xFF80);
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x00);

    assert!(!cpu.step(&mut mmu).unwrap()); // INC (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // INC (HL)
    assert!(cpu.step(&mut mmu).unwrap()); // INC (HL)
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x01);

    assert!(cpu.step(&mut mmu).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // SWAP (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // SWAP (HL)
    assert!(cpu.step(&mut mmu).unwrap()); // SWAP (HL)
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x10);

    assert!(!cpu.step(&mut mmu).unwrap()); // DEC (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // DEC (HL)
    assert!(cpu.step(&mut mmu).unwrap()); // DEC (HL)
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x0F);

    assert!(cpu.step(&mut mmu).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // BIT 4, (HL)
    assert!(cpu.step(&mut mmu).unwrap()); // BIT 4, (HL)
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x0F);
    assert!(cpu.registers.get_flag_zero());

    assert!(cpu.step(&mut mmu).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // SET 4, (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // SET 4, (HL)
    assert!(cpu.step(&mut mmu).unwrap()); // SET 4, (HL)
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x1F);

    assert!(cpu.step(&mut mmu).unwrap()); // PREFIX (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // RES 4, (HL)
    assert!(!cpu.step(&mut mmu).unwrap()); // RES 4, (HL)
    assert!(cpu.step(&mut mmu).unwrap()); // RES 4, (HL)
    assert_eq!(mmu.read_byte(cpu.registers.get_hl()), 0x0F);
}
