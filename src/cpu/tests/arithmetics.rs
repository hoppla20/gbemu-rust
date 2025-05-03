use crate::{cpu::Cpu, emulator::Emulator};

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

    let mut emu = Emulator::new_from_buffer(&instructions, None);
    emu.cpu = Cpu::new_zeroed(&mut emu.mmu);

    emu.cpu.registers.a = 0b10;
    emu.cpu.registers.b = 0b01;

    emu.step().unwrap(); // ADD A, B
    assert_eq!(emu.cpu.registers.a, 0b11);

    emu.step().unwrap(); // SUB A, B
    assert_eq!(emu.cpu.registers.a, 0b10);

    emu.cpu.registers.set_flag_carry(true);
    emu.step().unwrap(); // ADC A, B
    assert_eq!(emu.cpu.registers.a, 0b100);

    emu.cpu.registers.set_flag_carry(true);
    emu.step().unwrap(); // SBC A, B
    assert_eq!(emu.cpu.registers.a, 0b10);

    emu.step().unwrap(); // OR A, B
    assert_eq!(emu.cpu.registers.a, 0b11);

    emu.step().unwrap(); // SUB A, B
    assert_eq!(emu.cpu.registers.a, 0b10);

    emu.step().unwrap(); // AND A, B
    assert_eq!(emu.cpu.registers.a, 0x00);
    emu.cpu.registers.get_flag_zero();

    emu.step().unwrap(); // INC A
    assert_eq!(emu.cpu.registers.a, 0x01);

    emu.step().unwrap(); // DEC A
    assert_eq!(emu.cpu.registers.a, 0x00);

    emu.step().unwrap(); // INC HL
    emu.step().unwrap(); // INC HL
    assert_eq!(emu.cpu.registers.get_hl(), 0x0001);

    emu.step().unwrap(); // DEC HL
    emu.step().unwrap(); // DEC HL
    assert_eq!(emu.cpu.registers.get_hl(), 0x0000);

    emu.step().unwrap(); // ADD A, (HL
    assert_eq!(emu.cpu.registers.z, 0x80);
    assert_eq!(emu.cpu.registers.a, 0x00);
    emu.step().unwrap(); // ADD A, (HL
    assert_eq!(emu.cpu.registers.a, 0x80);

    emu.step().unwrap(); // ADD HL, BC
    emu.step().unwrap(); // ADD HL, BC
    assert_eq!(emu.cpu.registers.get_hl(), 0x0100);

    emu.step().unwrap(); // PREFIX
    emu.step().unwrap(); // RES 0, H
    assert_eq!(emu.cpu.registers.get_hl(), 0x0000);

    emu.cpu.registers.set_hl(0xFF80);
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x00);

    emu.step().unwrap(); // INC (HL
    emu.step().unwrap(); // INC (HL
    emu.step().unwrap(); // INC (HL
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x01);

    emu.step().unwrap(); // PREFIX (HL
    emu.step().unwrap(); // SWAP (HL
    emu.step().unwrap(); // SWAP (HL
    emu.step().unwrap(); // SWAP (HL
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x10);

    emu.step().unwrap(); // DEC (HL
    emu.step().unwrap(); // DEC (HL
    emu.step().unwrap(); // DEC (HL
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x0F);

    emu.step().unwrap(); // PREFIX (HL
    emu.step().unwrap(); // BIT 4, (HL
    emu.step().unwrap(); // BIT 4, (HL
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x0F);
    emu.cpu.registers.get_flag_zero();

    emu.step().unwrap(); // PREFIX (HL
    emu.step().unwrap(); // SET 4, (HL
    emu.step().unwrap(); // SET 4, (HL
    emu.step().unwrap(); // SET 4, (HL
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x1F);

    emu.step().unwrap(); // PREFIX (HL
    emu.step().unwrap(); // RES 4, (HL
    emu.step().unwrap(); // RES 4, (HL
    emu.step().unwrap(); // RES 4, (HL
    assert_eq!(emu.mmu.read_byte(emu.cpu.registers.get_hl()), 0x0F);
}
