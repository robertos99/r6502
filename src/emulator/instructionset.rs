use crate::emulator::cpu::Cpu;

static CARRY_FLAG: u8 = 0b10000000;
static ZERO_FLAG: u8 = 0b01000000;
static INTERRUPT_DISABLE_FLAG: u8 = 0b00100000;
static DECIMAL_MODE_FLAG: u8 = 0b00010000;
static BREAK_COMMAND_FLAG: u8 = 0b00001000;
static OVERFLOW_FLAG: u8 = 0b000000100;
static NEGATIVE_FLAG: u8 = 0b00000010;

enum MemMode {
    ACC,
    IMM,
    ZPG,
    ZPGX,
    ZPGY,
    REL,
    ABS,
    ABSX,
    ABSY,
    IND,
    IDXIND,
    INDIDX,
}

macro_rules! not_implemented {
    () => {
        panic!("not yet implemented");
    };
}

pub fn exec_ins(opt_code: u8, cpu: &mut Cpu) {
    match opt_code {
        0x69 => ADC(cpu, MemMode::IMM, 2),
        0x65 => ADC(cpu, MemMode::ZPG, 2),
        0x75 => ADC(cpu, MemMode::ZPGX, 2),
        0x6D => ADC(cpu, MemMode::ABS, 3),
        0x7D => ADC(cpu, MemMode::ABSX, 3),
        0x79 => ADC(cpu, MemMode::ABSY, 3),

        0x29 => AND(cpu, MemMode::IMM, 2),
        0x25 => AND(cpu, MemMode::ZPG, 2),
        0x35 => AND(cpu, MemMode::ZPGX, 2),
        0x2D => AND(cpu, MemMode::ABS, 3),
        0x3D => AND(cpu, MemMode::ABSX, 3),
        0x39 => AND(cpu, MemMode::ABSY, 3),

        0x0A => ASL(cpu, MemMode::ACC, 1),
        0x06 => ASL(cpu, MemMode::ZPG, 2),
        0x16 => ASL(cpu, MemMode::ZPGX, 2),
        0x0E => ASL(cpu, MemMode::ABS, 3),
        0x1E => ASL(cpu, MemMode::ABSX, 3),

        0x90 => BCC(cpu),

        0xB0 => BCS(cpu),

        0xF0 => BEQ(cpu),

        0x24 => BIT(cpu, MemMode::ZPG),
        0x2C => BIT(cpu, MemMode::ABS),

        0x30 => BMI(cpu),

        0xD0 => BNE(cpu),

        0x10 => BPL(cpu),

        0x00 => BRK(cpu),

        0x50 => BVC(cpu),

        0x70 => BVS(cpu),

        0x18 => CLC(cpu),

        0xD8 => CLD(cpu),

        0x58 => CLI(cpu),

        0xB8 => CLV(cpu),

        0xC9 => CMP(cpu, MemMode::IMM, 2),
        0xC5 => CMP(cpu, MemMode::ZPG, 2),
        0xD5 => CMP(cpu, MemMode::ZPGX, 2),
        0xCD => CMP(cpu, MemMode::ABS, 3),
        0xDD => CMP(cpu, MemMode::ABSX, 3),
        0xD9 => CMP(cpu, MemMode::ABSY, 3),

        0xE0 => CPX(cpu, MemMode::IMM, 2),
        0xE4 => CPX(cpu, MemMode::ZPG, 2),
        0xEC => CPX(cpu, MemMode::ABS, 3),

        0xC0 => CPY(cpu, MemMode::IMM, 2),
        0xC4 => CPY(cpu, MemMode::ZPG, 2),
        0xCC => CPY(cpu, MemMode::ABS, 3),

        0xC6 => DEC(cpu, MemMode::ZPG, 2),
        0xD6 => DEC(cpu, MemMode::ZPGX, 2),
        0xCE => DEC(cpu, MemMode::ABS, 3),
        0xDE => DEC(cpu, MemMode::ABSX, 3),

        0xCA => DEX(cpu),

        0x88 => DEY(cpu),

        0x49 => EOR(cpu, MemMode::IMM, 2),
        0x45 => EOR(cpu, MemMode::ZPG, 2),
        0x55 => EOR(cpu, MemMode::ZPGX, 2),
        0x4D => EOR(cpu, MemMode::ABS, 3),
        0x5D => EOR(cpu, MemMode::ABSX, 3),
        0x59 => EOR(cpu, MemMode::ABSY, 3),

        0xE6 => INC(cpu, MemMode::ZPG, 2),
        0xF6 => INC(cpu, MemMode::ZPGX, 2),
        0xEE => INC(cpu, MemMode::ABS, 3),
        0xFE => INC(cpu, MemMode::ABSX, 3),

        0xE8 => INX(cpu),

        0xC8 => INX(cpu),

        0x4C => JMP(cpu, MemMode::ABS),
        0x6C => JMP(cpu, MemMode::IND),

        0x20 => JSR(cpu),

        0xA9 => LDA(cpu, MemMode::IMM, 2),
        0xA5 => LDA(cpu, MemMode::ZPG, 2),
        0xB5 => LDA(cpu, MemMode::ZPGX, 2),
        0xAD => LDA(cpu, MemMode::ABS, 3),
        0xBD => LDA(cpu, MemMode::ABSX, 3),
        0xB9 => LDA(cpu, MemMode::ABSY, 3),

        0xA2 => LDX(cpu, MemMode::IMM, 2),
        0xA6 => LDX(cpu, MemMode::ZPG, 2),
        0xB6 => LDX(cpu, MemMode::ZPGY, 2),
        0xAE => LDX(cpu, MemMode::ABS, 3),
        0xBE => LDX(cpu, MemMode::ABSY, 3),

        0xA0 => LDY(cpu, MemMode::IMM, 2),
        0xA4 => LDY(cpu, MemMode::ZPG, 2),
        0xB4 => LDY(cpu, MemMode::ZPGX, 2),
        0xAC => LDY(cpu, MemMode::ABS, 3),
        0xBC => LDY(cpu, MemMode::ABSX, 3),

        0x4A => LSR(cpu, MemMode::ACC, 1),
        0x46 => LSR(cpu, MemMode::ZPG, 2),
        0x56 => LSR(cpu, MemMode::ZPGX, 2),
        0x4E => LSR(cpu, MemMode::ABS, 3),
        0x5E => LSR(cpu, MemMode::ABSX, 3),

        0xEA => NOP(cpu),

        0x09 => ORA(cpu, MemMode::IMM, 2),
        0x05 => ORA(cpu, MemMode::ZPG, 2),
        0x15 => ORA(cpu, MemMode::ZPGX, 2),
        0x0D => ORA(cpu, MemMode::ABS, 3),
        0x1D => ORA(cpu, MemMode::ABSX, 3),
        0x19 => ORA(cpu, MemMode::ABSY, 3),

        0x48 => PHA(cpu),

        0x08 => PHP(cpu),

        0x68 => PLA(cpu),

        0x28 => PLP(cpu),

        0x2A => ROL(cpu, MemMode::ACC, 1),
        0x26 => ROL(cpu, MemMode::ZPG, 2),
        0x36 => ROL(cpu, MemMode::ZPGX, 2),
        0x2E => ROL(cpu, MemMode::ABS, 3),
        0x3E => ROL(cpu, MemMode::ABSX, 3),

        0x6A => ROR(cpu, MemMode::ACC, 1),
        0x66 => ROR(cpu, MemMode::ZPG, 2),
        0x76 => ROR(cpu, MemMode::ZPGX, 2),
        0x6E => ROR(cpu, MemMode::ABS, 3),
        0x7E => ROR(cpu, MemMode::ABSX, 3),

        0x40 => RTI(cpu),

        0x60 => RTS(cpu),

        0xE9 => SBC(cpu, MemMode::IMM, 2),
        0xE5 => SBC(cpu, MemMode::ZPG, 2),
        0xF5 => SBC(cpu, MemMode::ZPGX, 2),
        0xED => SBC(cpu, MemMode::ABS, 3),
        0xFD => SBC(cpu, MemMode::ABSX, 3),
        0xF9 => SBC(cpu, MemMode::ABSY, 3),

        0x38 => SEC(cpu),

        0xF8 => SED(cpu),

        0x78 => SEI(cpu),

        0x85 => STA(cpu, MemMode::ZPG, 2),
        0x95 => STA(cpu, MemMode::ZPGX, 2),
        0x8D => STA(cpu, MemMode::ABS, 3),
        0x9D => STA(cpu, MemMode::ABSX, 3),
        0x99 => STA(cpu, MemMode::ABSY, 3),

        0x86 => STX(cpu, MemMode::ZPG, 2),
        0x96 => STX(cpu, MemMode::ZPGY, 2),
        0x8E => STX(cpu, MemMode::ABS, 3),

        0x84 => STY(cpu, MemMode::ZPG, 2),
        0x94 => STY(cpu, MemMode::ZPGX, 2),
        0x8C => STY(cpu, MemMode::ABS, 3),

        0xAA => TAX(cpu),

        0xA8 => TAY(cpu),

        0xBA => TSX(cpu),

        0x8A => TXA(cpu),

        0x9A => TXS(cpu),

        0x98 => TYA(cpu),

        _ => panic!("Opt code {opt_code:x} doesn't exist or is not implemented!"),
    }
}

fn get_value(cpu: &Cpu, mem_mode: MemMode) -> (u8, u16, bool) {
    // returns (val, addr, is_acc)
    // val is the deref of addr
    match mem_mode {
        MemMode::ACC => (cpu.accumulator, 0, true),
        MemMode::IMM => (cpu.bus.read_from(cpu.programm_counter + 1), 0, false),
        MemMode::ZPG => {
            let zpg_addr = cpu.bus.read_from(cpu.programm_counter + 1);
            (cpu.bus.read_from(zpg_addr as u16), zpg_addr as u16, false)
        }
        MemMode::ZPGX => {
            let zpg_x_addr = cpu.bus.read_from(cpu.programm_counter + 1) + cpu.x;
            // potential wrap around
            let zpg_x_addr_w = zpg_x_addr as u16 % 256;
            (cpu.bus.read_from(zpg_x_addr_w as u16), zpg_x_addr_w, false)
        }
        MemMode::ZPGY => {
            let zpg_y_addr = cpu.bus.read_from(cpu.programm_counter + 1) + cpu.y;
            // potential wrap around
            let zpg_y_addr_w = zpg_y_addr as u16 % 256;
            (cpu.bus.read_from(zpg_y_addr_w as u16), zpg_y_addr_w, false)
        }
        MemMode::REL => {
            not_implemented!();
        }
        MemMode::ABS => {
            let abs_addr = cpu.bus.read_from(cpu.programm_counter + 1) as u16
                | (cpu.bus.read_from(cpu.programm_counter + 2) as u16) << 8;
            (cpu.bus.read_from(abs_addr), abs_addr, false)
        }
        MemMode::ABSX => {
            let abs_addr = cpu.bus.read_from(cpu.programm_counter + 1) as u16
                | (cpu.bus.read_from(cpu.programm_counter + 2) as u16) << 8;
            let abs_addr_x = abs_addr + cpu.x as u16;
            (cpu.bus.read_from(abs_addr_x), abs_addr_x, false)
        }
        MemMode::ABSY => {
            let abs_addr = cpu.bus.read_from(cpu.programm_counter + 1) as u16
                | (cpu.bus.read_from(cpu.programm_counter + 2) as u16) << 8;
            let abs_addr_y = abs_addr + cpu.y as u16;
            (cpu.bus.read_from(abs_addr_y), abs_addr_y, false)
        }
        MemMode::IND => {
            not_implemented!();
        }
        MemMode::IDXIND => {
            not_implemented!();
        }
        MemMode::INDIDX => {
            not_implemented!();
        }
    }
}

fn ADC(cpu: &mut Cpu, mem_mode: MemMode, bytes: u8) {
    let (to_add, _, _) = get_value(cpu, mem_mode);

    cpu.accumulator = cpu.accumulator + to_add;
    cpu.accumulator = cpu.accumulator + to_add;
    if cpu.accumulator == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += bytes as u16;
}

fn AND(cpu: &mut Cpu, mem_mode: MemMode, bytes: u8) {
    let (to_and, _, _) = get_value(cpu, mem_mode);

    cpu.accumulator = cpu.accumulator & to_and;
    if cpu.accumulator == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += bytes as u16;
}

fn ASL(cpu: &mut Cpu, mem_mode: MemMode, bytes: u8) {
    let (val, addr, is_acc) = get_value(cpu, mem_mode);
    let val = (val as u16) << 1;
    let overflows = val & 0b00010000 > 0;
    if is_acc {
        cpu.accumulator = val as u8;
    } else {
        cpu.bus.write_to(addr, val as u8);
    }
    if overflows {
        cpu.status_flags |= CARRY_FLAG;
    }
    cpu.programm_counter += bytes as u16;
}

fn BCC(cpu: &mut Cpu) {
    let is_carry_clear = (cpu.status_flags & CARRY_FLAG) == 0;
    if is_carry_clear {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BCS(cpu: &mut Cpu) {
    let is_carry_set = (cpu.status_flags & CARRY_FLAG) != 0;
    if is_carry_set {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BEQ(cpu: &mut Cpu) {
    let is_zero_set = (cpu.status_flags & ZERO_FLAG) != 0;
    if is_zero_set {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BIT(cpu: &mut Cpu, mem_mode: MemMode) {
    let (val, _, _) = get_value(cpu, mem_mode);

    if val & 0b11111111 == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
}

fn BMI(cpu: &mut Cpu) {
    let is_negative_set = (cpu.status_flags & NEGATIVE_FLAG) != 0;
    if is_negative_set {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BNE(cpu: &mut Cpu) {
    let is_zero_clear = (cpu.status_flags & ZERO_FLAG) == 0;
    if is_zero_clear {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BPL(cpu: &mut Cpu) {
    let is_negative_clear = (cpu.status_flags & NEGATIVE_FLAG) == 0;
    if is_negative_clear {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BRK(cpu: &mut Cpu) {
    not_implemented!();
}

fn BVC(cpu: &mut Cpu) {
    let is_overflow_clear = (cpu.status_flags & OVERFLOW_FLAG) == 0;
    if is_overflow_clear {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn BVS(cpu: &mut Cpu) {
    let is_overflow_set = (cpu.status_flags & ZERO_FLAG) != 0;
    if is_overflow_set {
        let rel = cpu.bus.read_from(cpu.programm_counter + 1);
        cpu.programm_counter = cpu.programm_counter + rel as u16;
    } else {
        cpu.programm_counter += 2;
    }
}

fn CLC(cpu: &mut Cpu) {
    cpu.status_flags |= !CARRY_FLAG;
    cpu.programm_counter += 1;
}

fn CLD(cpu: &mut Cpu) {
    cpu.status_flags |= !DECIMAL_MODE_FLAG;
    cpu.programm_counter += 1;
}

fn CLI(cpu: &mut Cpu) {
    cpu.status_flags |= !INTERRUPT_DISABLE_FLAG;
    cpu.programm_counter += 1;
}

fn CLV(cpu: &mut Cpu) {
    cpu.status_flags |= !OVERFLOW_FLAG;
    cpu.programm_counter += 1;
}

fn CMP(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);

    let is_a_equal_m = cpu.accumulator == val;
    if is_a_equal_m {
        cpu.status_flags |= ZERO_FLAG;
    }

    let is_a_bigger_or_equal = cpu.accumulator >= val;
    if is_a_bigger_or_equal {
        cpu.status_flags |= OVERFLOW_FLAG;
    }

    cpu.programm_counter += bytes;
}

fn CPX(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);

    let is_x_equal_m = cpu.x == val;
    if is_x_equal_m {
        cpu.status_flags |= ZERO_FLAG;
    }

    let is_x_bigger_or_equal = cpu.x >= val;
    if is_x_bigger_or_equal {
        cpu.status_flags |= OVERFLOW_FLAG;
    }

    cpu.programm_counter += bytes;
}

fn CPY(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);

    let is_y_equal_m = cpu.y == val;
    if is_y_equal_m {
        cpu.status_flags |= ZERO_FLAG;
    }

    let is_y_bigger_or_equal = cpu.y >= val;
    if is_y_bigger_or_equal {
        cpu.status_flags |= OVERFLOW_FLAG;
    }

    cpu.programm_counter += bytes;
}

fn DEC(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, addr, _) = get_value(cpu, mem_mode);

    let val_dec = val - 1;
    if val_dec == 0 {
        cpu.status_flags |= ZERO_FLAG
    }
    cpu.bus.write_to(addr, val_dec);
    cpu.programm_counter += bytes;
}

fn DEX(cpu: &mut Cpu) {
    cpu.x -= 1;

    if cpu.x == 0 {
        cpu.status_flags |= ZERO_FLAG
    }
    cpu.programm_counter += 1;
}

fn DEY(cpu: &mut Cpu) {
    cpu.y -= 1;

    if cpu.y == 0 {
        cpu.status_flags |= ZERO_FLAG
    }
    cpu.programm_counter += 1;
}

fn EOR(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);
    cpu.accumulator = cpu.accumulator ^ val;

    if cpu.accumulator == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }

    cpu.programm_counter += bytes;
}

fn INC(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, addr, _) = get_value(cpu, mem_mode);
    let val_inc = val + 1;

    if val_inc == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }

    cpu.bus.write_to(addr, val_inc);

    cpu.programm_counter += bytes;
}

fn INX(cpu: &mut Cpu) {
    cpu.x += 1;
    if cpu.x == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn INY(cpu: &mut Cpu) {
    cpu.y += 1;
    if cpu.y == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn JMP(cpu: &mut Cpu, mem_mode: MemMode) {
    let (_, addr, _) = get_value(cpu, mem_mode);
    cpu.programm_counter = addr;
}

fn JSR(cpu: &mut Cpu) {
    let (_, jump_addr, _) = get_value(cpu, MemMode::ABS);
    // save return adresse to stack
    // little endian
    cpu.stack_pointer -= 1;
    cpu.bus.write_to(
        (cpu.stack_pointer) as u16,
        (cpu.programm_counter + 4 >> 8) as u8,
    );
    cpu.stack_pointer -= 1;
    cpu.bus
        .write_to((cpu.stack_pointer) as u16, (cpu.programm_counter + 3) as u8);
    cpu.programm_counter = jump_addr;
}

fn LDA(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);
    cpu.accumulator = val;

    if val == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += bytes;
}

fn LDX(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);
    cpu.x = val;

    if val == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += bytes;
}

fn LDY(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);
    cpu.y = val;

    if val == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += bytes;
}

fn LSR(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, addr, is_acc) = get_value(cpu, mem_mode);
    let is_carry = val & 0b00000001 != 0;
    if is_carry {
        cpu.status_flags |= CARRY_FLAG;
    }
    let shifted_val = val >> 1;
    if shifted_val == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }

    if is_acc {
        cpu.accumulator = shifted_val;
    } else {
        cpu.bus.write_to(addr, shifted_val);
    }

    cpu.programm_counter += bytes;
}

fn NOP(cpu: &mut Cpu) {
    cpu.programm_counter += 1;
}

fn ORA(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);
    cpu.accumulator = cpu.accumulator | val;
    if val == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += bytes;
}

fn PHA(cpu: &mut Cpu) {
    cpu.stack_pointer -= 1;
    cpu.bus.write_to(cpu.stack_pointer as u16, cpu.accumulator);

    cpu.programm_counter += 1;
}

fn PHP(cpu: &mut Cpu) {
    cpu.stack_pointer -= 1;
    cpu.bus.write_to(cpu.stack_pointer as u16, cpu.status_flags);

    cpu.programm_counter += 1;
}

fn PLA(cpu: &mut Cpu) {
    cpu.accumulator = cpu.bus.read_from(cpu.stack_pointer as u16);
    cpu.stack_pointer += 1;
    cpu.programm_counter += 1;
}

fn PLP(cpu: &mut Cpu) {
    cpu.status_flags = cpu.bus.read_from(cpu.stack_pointer as u16);
    cpu.stack_pointer += 1;
    cpu.programm_counter += 1;
}

fn ROL(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, addr, is_acc) = get_value(cpu, mem_mode);
    let is_seven_set = val & 0b10000000 != 0;
    let is_carry_set = cpu.status_flags & CARRY_FLAG != 0;
    if is_seven_set {
        cpu.status_flags |= CARRY_FLAG;
    }
    let mut val_shifted = val << 1;
    if is_carry_set {
        val_shifted += 1;
    }

    if is_acc {
        cpu.accumulator = val_shifted;
    } else {
        cpu.bus.write_to(addr, val_shifted);
    }

    cpu.programm_counter += bytes;
}

fn ROR(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, addr, is_acc) = get_value(cpu, mem_mode);
    let is_zero_bit_set = val & 0b00000001 != 0;
    let is_carry_set = cpu.status_flags & CARRY_FLAG != 0;
    if is_zero_bit_set {
        cpu.status_flags |= CARRY_FLAG;
    }
    let mut val_shifted = val >> 1;
    if is_carry_set {
        val_shifted |= 0b10000000;
    }

    if is_acc {
        cpu.accumulator = val_shifted;
    } else {
        cpu.bus.write_to(addr, val_shifted);
    }

    cpu.programm_counter += bytes;
}

fn RTI(cpu: &mut Cpu) {
    let status_flags = cpu.bus.read_from(cpu.stack_pointer as u16);
    let return_addr: u16 = cpu.bus.read_from((cpu.stack_pointer + 1) as u16) as u16
        | (cpu.bus.read_from((cpu.stack_pointer + 2) as u16) as u16) << 8;

    cpu.stack_pointer += 3;

    cpu.status_flags = status_flags;
    cpu.programm_counter = return_addr;
}

fn RTS(cpu: &mut Cpu) {
    let return_addr: u16 = cpu.bus.read_from(cpu.stack_pointer as u16) as u16
        | (cpu.bus.read_from((cpu.stack_pointer + 1) as u16) as u16) << 8;

    cpu.stack_pointer += 2;

    cpu.programm_counter = return_addr;
}

fn SBC(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (val, _, _) = get_value(cpu, mem_mode);
    cpu.accumulator -= val;

    if cpu.accumulator == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }

    cpu.programm_counter += bytes;
}

fn SEC(cpu: &mut Cpu) {
    cpu.status_flags |= CARRY_FLAG;
    cpu.programm_counter += 1;
}

fn SED(cpu: &mut Cpu) {
    cpu.status_flags |= DECIMAL_MODE_FLAG;
    cpu.programm_counter += 1;
}

fn SEI(cpu: &mut Cpu) {
    cpu.status_flags |= INTERRUPT_DISABLE_FLAG;
    cpu.programm_counter += 1;
}

fn STA(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (_, addr, _) = get_value(cpu, mem_mode);

    cpu.bus.write_to(addr, cpu.accumulator);

    cpu.programm_counter += bytes;
}

fn STX(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (_, addr, _) = get_value(cpu, mem_mode);

    cpu.bus.write_to(addr, cpu.x);

    cpu.programm_counter += bytes;
}

fn STY(cpu: &mut Cpu, mem_mode: MemMode, bytes: u16) {
    let (_, addr, _) = get_value(cpu, mem_mode);

    cpu.bus.write_to(addr, cpu.y);

    cpu.programm_counter += bytes;
}

fn TAX(cpu: &mut Cpu) {
    cpu.x = cpu.accumulator;
    if cpu.x == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn TAY(cpu: &mut Cpu) {
    cpu.y = cpu.accumulator;
    if cpu.y == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn TSX(cpu: &mut Cpu) {
    cpu.x = cpu.stack_pointer;
    if cpu.x == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn TXA(cpu: &mut Cpu) {
    cpu.accumulator = cpu.x;
    if cpu.x == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn TXS(cpu: &mut Cpu) {
    cpu.stack_pointer = cpu.x;
    if cpu.x == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

fn TYA(cpu: &mut Cpu) {
    cpu.accumulator = cpu.y;
    if cpu.y == 0 {
        cpu.status_flags |= ZERO_FLAG;
    }
    cpu.programm_counter += 1;
}

#[cfg(test)]
mod tests {

    #[test]
    fn wrap_around() {
        let result: u8 = (0xFF + 0x80 % 256) as u8;
        assert_eq!(result, 0x07F_u8);
    }

    #[test]
    fn overflow_cast() {
        let ueight: u8 = 0b01111111;
        let res = ((ueight as u16) << 1) as u8;

        assert_eq!(0b11111110, res);
    }
}
