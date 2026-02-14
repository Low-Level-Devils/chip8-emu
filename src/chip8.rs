use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Chip8 {
    general_registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    index_register: u16,
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
}

enum GeneralRegisters {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

const PROGRAM_START_ADDRESS: u16 = 0x200;
const DELAY_TIMER_FREQUENCY_HZ: u8 = 60;
const MAX_MEMORY_ADDRESS: u16 = 0xFFF;

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            general_registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            index_register: 0x00,
            program_counter: PROGRAM_START_ADDRESS,
            stack_pointer: 0x00,
            delay_timer: 0x00,
            sound_timer: 0x00,
        }
    }

    pub fn load_instructions(&mut self, path_to_rom: &str) {
        let mut rom_file = File::open(path_to_rom).expect("Failed to find input file");

        let file_reader = BufReader::new(rom_file);

        let mut binary_data = Vec::new();

        for line in file_reader.lines() {
            let line = line.expect("Failed to read line");

            let parsed_instruction = line.split('$').next().unwrap_or("").trim();

            if parsed_instruction.is_empty() {
                continue;
            }

            let hex_string = parsed_instruction.replace("0x", "");

            if let Ok(opcode) = u16::from_str_radix(&hex_string, 16) {
                binary_data.push((opcode >> 8) as u8);
                binary_data.push((opcode & 0xFF) as u8);
            }
        }

        for (instrcution_number, &byte) in binary_data.iter().enumerate() {
            let address = PROGRAM_START_ADDRESS as usize + instrcution_number;

            if address <= MAX_MEMORY_ADDRESS as usize {
                self.memory[address] = byte;
            } else {
                println!("CRITICAL: ROM size too big");
                break;
            }
        }
    }

    pub fn dump_memory_state(&self) {
        for (address, &instruction) in self.memory.iter().enumerate() {
            println!("0x{:04X} : 0x{:02X}", address, instruction);
        }
    }
}
