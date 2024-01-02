use r6502::emulator::{
    bus::{Bus, Device},
    clock::Clock,
    cpu::Cpu,
    display::Display,
    rom::Rom,
};

fn main() {
    let mut prog_rom = Rom { mem: [0; 65535] };
    let programm = read_bytes_from_file("./asm/a.out");
    let programm_start_addr = 0x0;
    for (i, v) in programm.iter().enumerate() {
        prog_rom.mem[programm_start_addr + i] = *v;
    }

    let mut boot_rom = Rom { mem: [0; 65535] };
    // programm start addr
    boot_rom.mem[0xFFFC] = 0x0;
    boot_rom.mem[0xFFFD] = 0x0;

    let display = Display::new();

    let mut bus = Bus::new();
    bus.attach(Device::Rom(boot_rom), (0xFFFC, 0xFFFD));
    bus.attach(Device::Rom(prog_rom), (0x0, 0xFF));
    bus.attach(Device::Display(display), (0x200, 0x200));
    let cpu = Cpu::new(bus);
    let mut clock = Clock::new(cpu);
    clock.start();
}

fn read_bytes_from_file(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap()
}
