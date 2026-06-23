use crate::{
    components::{
        cartridge::Cartridge,
        cpu::{CPU, STARTING_ROM_ADDRESS},
        ram::RAM,
    },
    fontset::FONTSET,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct VirtualMachine<'a> {
    pub cpu: CPU,
    pub ram: RAM,
    pub screen: [bool; WIDTH * HEIGHT],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub variant: &'a str,
}

impl<'a> VirtualMachine<'a> {
    pub fn boot(variant: &'a str) -> Self {
        Self {
            cpu: CPU::start(),
            ram: RAM::start(),
            screen: [false; WIDTH * HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            variant: variant,
        }
    }

    pub fn controller(&mut self, cartridge: &Cartridge) {
        for index in 0..FONTSET.len() {
            self.ram.code_segment[index] = FONTSET[index];
        }

        for (index, byte) in cartridge.buffer.iter().enumerate() {
            let address = STARTING_ROM_ADDRESS + index as u16;
            if address >= 4096 {
                break;
            }

            self.ram.code_segment[address as usize] = *byte;
        }
    }

    pub fn process(&mut self) {
        self.cpu.control_unit.cycle(
            self.variant,
            &mut self.ram,
            &mut self.cpu.index_register,
            &mut self.cpu.registers,
            &mut self.delay_timer,
            &mut self.sound_timer,
        );
    }
}
