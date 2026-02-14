mod chip8;

use std::env;

use crate::chip8::Chip8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("INCORRECT USE!! Usage: ");
        print!("cargo run <path_to_rom_file>");

        return;
    }

    let mut emulated_chip8 = Chip8::new();

    emulated_chip8.load_instructions(&args[1]);

    emulated_chip8.dump_memory_state();
}
