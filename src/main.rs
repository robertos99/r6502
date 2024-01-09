use r6502::emulator::{
    bus::{Bus, Device},
    clock::Clock,
    cpu::Cpu,
    display::Display,
    rom::Rom,
};

fn main() {
    println!("Hello, world!");

    let mut prog_rom = Rom { mem: [0; 65535] };
    let programm = read_bytes_from_file("./asm/a.out");
    for (i, v) in programm.iter().enumerate() {
        prog_rom.mem[i] = *v;
    }

    let display = Display::new();

    let mut bus = Bus::new();
    bus.attach(Device::Rom(prog_rom), (0x400, 0xFFFF));
    bus.attach(Device::Display(display), (0x200, 0x200));
    let cpu = Cpu::new(bus);
    let mut clock = Clock::new(cpu);
    clock.start();
}

fn read_bytes_from_file(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn bithshift() {
        let num: u16 = 0x00 as u16 | (0x30 as u16) << 8;
        println!("{num:x}");
    }
}
