use crate::chip8::Chip8;

const BIT_MASK_3: u16 = 0xF000;
const BIT_MASK_2: u16 = 0x0F00;
const BIT_MASK_1: u16 = 0x00F0;
const BIT_MASK_0: u16 = 0x000F;

pub fn opcode_family_0x6000(emulated_chip: &mut Chip8, instruction: u16) {
    let target_register = (instruction & BIT_MASK_2) >> 8;
    let target_value = instruction & (BIT_MASK_1 | BIT_MASK_0);

    emulated_chip.set_register_to_value(target_register as usize, target_value as u8);

    emulated_chip.dump_register_state();
} 