use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rom>", args[0]);
        return;
    }

    let rom_path = &args[1];
    let rom_data = fs::read(rom_path)
        .expect("Faled to read the ROM file.");

    println!("Successfully loaded ROM: {}", rom_path);
    println!("ROM size: {} bytes", rom_data.len());
}
