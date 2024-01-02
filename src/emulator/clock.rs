use super::cpu::Cpu;

pub struct Clock {
    cpu: Cpu,
}

impl Clock {
    pub fn new(cpu: Cpu) -> Clock {
        Self { cpu }
    }

    pub fn start(&mut self) {
        self.cpu.init_sequence();
        loop {
            self.cpu.pulse();
        }
    }
}
