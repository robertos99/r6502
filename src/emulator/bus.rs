use crate::emulator::display::Display;
use crate::emulator::ram::Ram;
use crate::emulator::rom::Rom;

pub enum Device {
    Ram(Ram),
    Rom(Rom),
    Display(Display),
}

impl Device {
    fn read(&self, addr: u16) -> u8 {
        match self {
            Device::Ram(ram) => ram.read(addr),
            Device::Rom(rom) => rom.read(addr),
            Device::Display(_) => {
                panic!("Can't read from Display!")
            }
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match self {
            Device::Ram(ram) => ram.write(addr, data),
            Device::Rom(rom) => rom.write(addr, data),
            Device::Display(display) => display.write(data),
        }
    }
}

type AddrRange = (u16, u16);

pub struct Bus {
    connected_dev: Vec<(AddrRange, Device)>,
}

impl Bus {
    pub fn new() -> Bus {
        Self {
            connected_dev: Vec::new(),
        }
    }

    pub fn attach(&mut self, dev: Device, addr_range: AddrRange) {
        self.connected_dev.push((addr_range, dev));
    }

    pub fn write_to(&mut self, addr: u16, data: u8) {
        for (addr_range, dev) in self.connected_dev.iter_mut() {
            if addr >= addr_range.0 && addr <= addr_range.1 {
                dev.write(addr, data);
                return;
            }
        }
        panic!("No device reads on that address");
    }

    pub fn read_from(&self, addr: u16) -> u8 {
        for (addr_range, dev) in self.connected_dev.iter() {
            if addr >= addr_range.0 && addr <= addr_range.1 {
                return dev.read(addr);
            }
        }
        panic!("No device writes to that address!");
    }
}
