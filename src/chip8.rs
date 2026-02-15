use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicBool, Ordering};

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

#[derive(Debug, FromPrimitive)]
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

static REGISTER_DUMP_FILE_INITIALIZED: AtomicBool = AtomicBool::new(false);

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

    pub fn load_instructions(
        &mut self,
        path_to_rom: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rom_file = File::open(path_to_rom)?;

        let file_reader = BufReader::new(rom_file);

        let mut binary_data = Vec::new();

        for line in file_reader.lines() {
            let line = line?;

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

        self.dump_memory_state()
    }

    pub fn dump_memory_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut memory_map_file = File::create("memory.map")?;

        for (address, &instruction) in self.memory.iter().enumerate() {
            writeln!(memory_map_file, "0x{:04X} : 0x{:02X}", address, instruction)?;
        }

        Ok(())
    }

    pub fn dump_register_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let first_access = !REGISTER_DUMP_FILE_INITIALIZED.swap(true, Ordering::SeqCst);

        let mut register_dump_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(first_access)
            .append(!first_access)
            .open("general_registers.dump")?;

        writeln!(register_dump_file, "******************************")?;

        for (index, &value) in self.general_registers.iter().enumerate() {
            writeln!(
                register_dump_file,
                "{:?} : {}",
                GeneralRegisters::from_usize(index).unwrap(),
                value
            )?;
        }

        writeln!(register_dump_file, "******************************")?;

        Ok(())
    }
}
