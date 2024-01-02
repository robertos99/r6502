#[derive(Debug)]
pub struct Rom {
    pub mem: [u8; 65535],
}

impl Rom {
    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}
