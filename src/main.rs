mod chip8;

use std::env;

use crate::chip8::Chip8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("INCORRECT USE!! Usage: ");
        println!("cargo run <path_to_rom_file>");

        return;
    }

    let mut emulated_chip8 = Chip8::new();

    let _ = emulated_chip8.load_instructions(&args[1]);
}
