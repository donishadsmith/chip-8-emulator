use macroquad::prelude::*;

use crate::{components::ram::RAM, utils::key_event};

pub const STARTING_MEMORY_ADDRESS: usize = 0x000;
pub const STARTING_ROM_ADDRESS: u16 = 0x200;

pub struct ProgramCounter {
    pub address: u16, // memory address of subsequent instruction, goes from 0x000 to 0xFFF (..4095).
                      // Start is 0x200 = 2 * 16^2 + 0 * 16^1 + 0 * 16^0 = 512
}

impl ProgramCounter {
    pub fn start() -> Self {
        Self {
            address: STARTING_ROM_ADDRESS,
        }
    }

    pub fn increment(&mut self) {
        self.address += 2 as u16;
    }

    pub fn jump(&mut self, address: u16) {
        self.address = address;
    }

    pub fn skip(&mut self, step: u16) {
        self.address += 2 * step;
    }

    // Remember to jump back to return address
    pub fn call(&mut self, address: u16) -> u16 {
        let return_address = self.address;
        self.address = address;

        return_address
    }
}

// Won't have a separate ALU;
pub struct ControlUnit {
    instruction_register: Option<u16>, // Current instruction
    program_counter: ProgramCounter,
    stack_pointer: Option<usize>, // point to top of stack
}

impl ControlUnit {
    pub fn start() -> Self {
        Self {
            instruction_register: None,
            program_counter: ProgramCounter::start(),
            stack_pointer: None,
        }
    }

    pub fn push(&mut self, address: u16, ram: &mut RAM) {
        let return_address = self.program_counter.call(address);

        let index = match self.stack_pointer {
            Some(mut index) => {
                index = index + 1;
                ram.stack[index] = return_address;
                index
            }
            None => {
                let index = 0;
                ram.stack[index] = return_address;
                index
            }
        };

        self.stack_pointer = Some(index);
    }

    pub fn pop(&mut self, ram: &RAM) {
        let return_address = ram.stack[self.stack_pointer.unwrap()];
        self.program_counter.jump(return_address);

        // Should never be None by the time pop is called
        let index = match self.stack_pointer {
            Some(0) => None,
            _ => Some(self.stack_pointer.unwrap() - 1),
        };

        self.stack_pointer = index;
    }

    pub fn cycle(
        &mut self,
        variant: &str,
        ram: &mut RAM,
        index_register: &mut usize,
        registers: &mut [u8; 16],
        delay_timer: &mut u8,
        sound_timer: &mut u8,
    ) {
        self.fetch(ram);
        let nibbles = self.decode(ram);
        self.execute(
            nibbles,
            variant,
            ram,
            index_register,
            registers,
            delay_timer,
            sound_timer,
        );
    }

    pub fn fetch(&mut self, ram: &RAM) {
        let pc = self.program_counter.address as usize;
        let opcode = (ram.code_segment[pc] as u16) << 8 | (ram.code_segment[pc + 1] as u16);
        self.instruction_register = Some(opcode);

        self.program_counter.increment();
    }

    pub fn decode(&self, ram: &RAM) -> Option<[u8; 4]> {
        if let Some(opcode) = self.instruction_register {
            Some(self.separate_opcode(opcode))
        } else {
            None
        }
    }

    // https://chip8.gulrak.net/ - The classic CHIP-8 for the COSMAC VIP by Joseph Weisbecker, 1977
    pub fn execute(
        &mut self,
        nibbles: Option<[u8; 4]>,
        variant: &str,
        ram: &mut RAM,
        index_register: &mut usize,
        registers: &mut [u8; 16],
        delay_timer: &mut u8,
        sound_timer: &mut u8,
    ) {
        if let Some(nibbles) = nibbles {
            let key = key_event();
            let x = nibbles[1] as usize;
            let y = nibbles[2] as usize;
            let nn = self.combine_bits(&nibbles[2..]) as u8;
            let nnn = self.combine_bits(&nibbles[1..]) as u8;

            match nibbles {
                [0x0, 0x0, 0xE, 0x0] => self.op_0x00e0(),
                [0x0, 0x0, 0xE, 0xE] => {}
                [0x0, _, _, _] => {}
                [0x1, _, _, _] => {}
                [0x2, _, _, _] => {}
                [0x3, _, _, _] | [0x4, _, _, _] => {
                    if nibbles[0] == 0x3 {
                        self.op_0x3xnn(registers, x, nn);
                    } else {
                        self.op_0x3xnn(registers, x, nn);
                    }
                }
                [0x5, _, _, 0x0] => {}
                [0x6, _, _, _] | [0x7, _, _, _] => {
                    if nibbles[0] == 0x6 {
                        self.op_0x6xnn(registers, x, nn);
                    } else {
                        self.op_0x7xnn(registers, x, nn)
                    }
                }
                [0x8, _, _, 0x0] => {}
                [0x8, _, _, 0x1] => {}
                [0x8, _, _, 0x2] => {}
                [0x8, _, _, 0x3] => {}
                [0x8, _, _, 0x4] => {}
                [0x8, _, _, 0x5] => {}
                [0x8, _, _, 0x6] => {}
                [0x8, _, _, 0x7] => {}
                [0x8, _, _, 0xE] => {}
                [0x9, _, _, 0x0] => {}
                [0xA, _, _, _] => {}
                [0xB, _, _, _] => {
                    if variant == "CHIP-8" {
                        self.op_0xbnnn(registers, nnn);
                    } else {
                        self.op_0xbxnn(registers, x, nn);
                    }
                }
                [0xC, _, _, _] => {}
                [0xD, _, _, _] => {}
                [0xE, _, 0x9, 0xE] => {}
                [0xE, _, 0xA, 0x1] => {}
                [0xF, _, 0x0, 0x7] => {}
                [0xF, _, 0x0, 0xA] => {}
                [0xF, _, 0x1, 0x5] => {}
                [0xF, _, 0x1, 0x8] => {}
                [0xF, _, 0x1, 0xE] => {}
                [0xF, _, 0x2, 0x9] => {}
                [0xF, _, 0x3, 0x3] => {}
                [0xF, _, 0x5, 0x5] => {}
                [0xF, _, 0x6, 0x5] => {}
                _ => {}
            }
        }
    }

    fn separate_opcode(&self, opcode: u16) -> [u8; 4] {
        [
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        ]
    }

    fn combine_bits(&self, slice: &[u8]) -> u16 {
        slice.iter().copied().reduce(|a, b| a | b).unwrap() as u16
    }

    fn op_0x00e0(&self) {
        clear_background(BLACK);
    }

    fn op_0x1nnn(&mut self, address: u16) {
        self.program_counter.jump(address);
    }

    fn op_0x2nnn(&mut self, address: u16, ram: &mut RAM) {
        self.push(address, ram);
    }

    fn op_0x3xnn(&mut self, registers: &[u8; 16], x: usize, nn: u8) {
        if registers[x] == nn {
            self.program_counter.skip(1);
        }
    }

    fn op_0x4xnn(&mut self, registers: &[u8; 16], x: usize, nn: u8) {
        if registers[x] != nn {
            self.program_counter.skip(1);
        }
    }

    fn op_0x5xy0(&mut self, x: usize, y: usize, registers: &[u8; 16]) {
        if registers[x] == registers[y] {
            self.program_counter.skip(1);
        }
    }

    fn op_0x6xnn(&mut self, registers: &mut [u8; 16], x: usize, nn: u8) {
        registers[x] = nn;
    }

    fn op_0x7xnn(&mut self, registers: &mut [u8; 16], x: usize, nn: u8) {
        registers[x] += nn;
    }

    fn op_0xbnnn(&mut self, registers: &mut [u8; 16], nnn: u8) {
        self.program_counter.jump(registers[0] as u16 + nnn as u16);
    }

    fn op_0xbxnn(&mut self, registers: &mut [u8; 16], x: usize, nn: u8) {
        self.program_counter.jump(registers[x] as u16 + nn as u16);
    }

    fn op_0xfx07(&mut self, registers: &mut [u8; 16], x: usize, delay_timer: &u8) {
        registers[x] = *delay_timer;
    }

    fn op_0xfx15(&mut self, registers: &mut [u8; 16], x: usize, delay_timer: &mut u8) {
        *delay_timer = registers[x];
    }

    fn op_0xfx18(&mut self, registers: &mut [u8; 16], x: usize, sound_timer: &mut u8) {
        *sound_timer = registers[x];
    }
}

pub struct CPU {
    pub registers: [u8; 16], // Each register holds 1 byte, u8 -> (2^8) - 1 = 0 to 255, registers are V0 to VF/ 0 -> 15
    pub index_register: usize, // Points to data in RAM
    pub control_unit: ControlUnit,
}

impl CPU {
    pub fn start() -> Self {
        Self {
            registers: [0; 16],
            index_register: STARTING_MEMORY_ADDRESS,
            control_unit: ControlUnit::start(),
        }
    }
}
