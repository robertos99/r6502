pub struct Ram {
    pub mem: [u8; 32767],
}

impl Ram {
    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}
