use std::env;
use std::fs;

struct Mmu {
    rom: Vec<u8>,
}

impl Mmu {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self { rom: rom_data }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rom>", args[0]);
        return;
    }

    let rom_path = &args[1];
    let rom_data = fs::read(rom_path)
        .expect("Faled to read the ROM file.");

    let mmu = Mmu::new(rom_data);
    let entry_point_instr = mmu.read_byte(0x1000);

    println!("Successfully loaded ROM: {}", rom_path);
    println!("ROM size: {} bytes", mmu.rom.len());
    println!("First instruction at entry point (0x1000): 0x{:02X}", entry_point_instr);
}
