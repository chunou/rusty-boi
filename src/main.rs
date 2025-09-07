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

struct Cpu {
    // 8-bit registers
    a: u8, f: u8, b: u8, c: u8,
    d: u8, e: u8, h: u8, l: u8,
    // 16-bit registers
    pc: u16, sp: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0x01, f: 0xB0, b: 0x00, c: 0x13,
            d: 0x00, e: 0xD8, h: 0x01, l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn fetch_instruction(&mut self, mmu: &Mmu) -> u8 {
        let instruction = mmu.read_byte(self.pc);
        self.pc += 1;
        instruction
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

    let mut cpu: Cpu = Cpu::new();
    let mmu = Mmu::new(rom_data);
    let entry_point_instr = cpu.fetch_instruction(&mmu);

    println!("Successfully loaded ROM: {}", rom_path);
    println!("ROM size: {} bytes", mmu.rom.len());
    println!("First instruction at entry point (0x0100): 0x{:02X}", entry_point_instr);
    println!("PC is now at: 0x{:04X}", cpu.pc);
}
