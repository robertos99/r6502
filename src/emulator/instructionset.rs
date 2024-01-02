use crate::emulator::cpu::Cpu;

static ZERO_FLAG: u8 = 0b01000000;

pub fn exec_ins(opt_code: u8, cpu: &mut Cpu) {
    match opt_code {
        0x20 => JSR(cpu),
        0x4C => JMP_ABS(cpu),
        0x6C => panic!("JMP_IND not yet implemented!"),
        0x60 => RTS(cpu),
        0x69 => ADC_IMM(cpu),
        0x8D => STA_ABS(cpu),
        0x85 => STA_ZPG(cpu),
        0xA9 => LDA_IMM(cpu),
        0xEA => NOP(cpu),
        _ => panic!("Opt code {opt_code:x} doesn't exist or is not implemented!"),
    }
}

fn JSR(cpu: &mut Cpu) {
    let jump_addr = cpu.bus.read_from(cpu.programm_counter + 1) as u16
        | (cpu.bus.read_from(cpu.programm_counter + 2) as u16) << 8;
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

fn JMP_ABS(cpu: &mut Cpu) {
    let jump_addr = cpu.bus.read_from(cpu.programm_counter + 1) as u16
        | (cpu.bus.read_from(cpu.programm_counter + 2) as u16) << 8;
    cpu.programm_counter = jump_addr;
}

fn RTS(cpu: &mut Cpu) {
    let return_addr: u16 = cpu.bus.read_from(cpu.stack_pointer as u16) as u16
        | (cpu.bus.read_from((cpu.stack_pointer + 1) as u16) as u16) << 8;
    cpu.programm_counter += return_addr;
}

fn ADC_IMM(cpu: &mut Cpu) {
    let data = cpu.bus.read_from(cpu.programm_counter + 1);
    cpu.accumulator = cpu.accumulator + data;
    if cpu.accumulator == 0 {
        cpu.status_flags = cpu.status_flags | ZERO_FLAG;
    }
    cpu.programm_counter += 2;
}

fn STA_ZPG(cpu: &mut Cpu) {
    let addr = cpu.bus.read_from(cpu.programm_counter + 1);
    cpu.bus.write_to(addr as u16, cpu.accumulator);
    cpu.programm_counter += 2;
}

fn STA_ZPGX(cpu: &mut Cpu) {
    let addr: u8 = (cpu.bus.read_from(cpu.programm_counter + 1) as u16 % 256) as u8;
    cpu.bus.write_to(addr as u16, cpu.accumulator);
    cpu.programm_counter += 2;
}

fn STA_ABS(cpu: &mut Cpu) {
    let addr: u16 = cpu.bus.read_from(cpu.programm_counter + 1) as u16
        | (cpu.bus.read_from(cpu.programm_counter + 2) as u16) << 8;
    cpu.bus.write_to(addr, cpu.accumulator);
    cpu.programm_counter += 3;
}

fn LDA_IMM(cpu: &mut Cpu) {
    let data = cpu.bus.read_from(cpu.programm_counter + 1);
    cpu.accumulator = data;
    cpu.programm_counter += 2;
}

fn NOP(cpu: &mut Cpu) {
    cpu.programm_counter += 1;
}

#[cfg(test)]
mod tests {

    #[test]
    fn wrap_around() {
        let result: u8 = (0xFF + 0x80 % 256) as u8;
        assert_eq!(result, 0x07F_u8);
    }
}
