use crate::emulator::bus::Bus;
use crate::emulator::instructionset;

#[allow(non_snake_case)]
pub struct StatusFlag {
    pub CARRY_FLAG: bool,
    pub ZERO_FLAG: bool,
    pub INTERRUPT_DISABLE_FLAG: bool,
    pub DECIMAL_MODE_FLAG: bool,
    pub BREAK_COMMAND_FLAG: bool,
    pub OVERFLOW_FLAG: bool,
    pub NEGATIVE_FLAG: bool,
}

pub struct Cpu {
    pub programm_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub status_flags: StatusFlag,

    pub bus: Bus,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Self {
            programm_counter: 0,
            // stack growing downwards
            // register is 8 bit however the stack goes from 0x01FF to 0x0100
            stack_pointer: 0xFF,
            accumulator: 0,
            x: 0,
            y: 0,
            status_flags: StatusFlag {
                CARRY_FLAG: false,
                ZERO_FLAG: false,
                INTERRUPT_DISABLE_FLAG: false,
                DECIMAL_MODE_FLAG: false,
                BREAK_COMMAND_FLAG: false,
                OVERFLOW_FLAG: false,
                NEGATIVE_FLAG: false,
            },

            bus,
        }
    }

    pub fn init_sequence(&mut self) {
        let programm_start_adr: u16 =
            self.bus.read_from(0xFFFC) as u16 | (self.bus.read_from(0xFFFD) as u16) << 8;
        self.programm_counter = programm_start_adr;
    }

    pub fn pulse(&mut self) {
        self.exec_cycle();
    }

    fn exec_cycle(&mut self) {
        let opt_code = self.bus.read_from(self.programm_counter);
        instructionset::exec_ins(opt_code, self);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitshift_u8_to_u16() {
        let little: u8 = 0b00000011;
        let big: u8 = 0b00000001;
        let hexa: u16 = (little as u16) | (big as u16) << 8;
        println!("{hexa:b}");
        assert_eq!(0b0000000100000011, hexa);
    }
}
