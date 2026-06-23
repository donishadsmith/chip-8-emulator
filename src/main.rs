/*Great references:
 -https://github.com/starrhorne/chip8-rust/
 -https://github.com/tendstofortytwo/chip8-rust/
*/

mod components;
mod fontset;
mod utils;
mod vm;

use crate::{components::cartridge::Cartridge, utils::get_key, vm::VirtualMachine};
use macroquad::prelude::*;

#[macroquad::main("Chip-8")]
async fn main() -> Result<(), std::io::Error> {
    let cartridge = Cartridge::load()?;
    let mut vm = VirtualMachine::boot(&cartridge.variant);
    vm.controller(&cartridge);

    loop {
        if let Some(key) = get_key() {
            if key == KeyCode::Escape {
                break;
            }
        }

        vm.process();

        next_frame().await;
    }

    //println!("N bytes {}", cartridge.n_bytes);
    //println!("Bytes: {:02X?}", &cartridge.buffer);

    Ok(())
}
