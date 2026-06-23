/*Great references:
 -https://github.com/ablakey/chip8/blob/master/src/chip8.rs
 -https://github.com/starrhorne/chip8-rust/
 -https://github.com/tendstofortytwo/chip8-rust/
*/

mod components;
mod fontset;
mod utils;
mod vm;

use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::{components::cartridge::Cartridge, utils::get_key, vm::VirtualMachine};

// https://github.com/not-fl3/macroquad/issues/749
#[macroquad::main("Chip-8")]
async fn main() -> Result<(), std::io::Error> {
    let fps = 60.0;
    let frame_dur = Duration::from_secs_f64(1.0 / fps);
    let mut next_tick = Instant::now();

    let cartridge = Cartridge::load()?;
    let mut vm = VirtualMachine::boot(&cartridge.variant);
    vm.controller(&cartridge);

    loop {
        clear_background(BLACK);

        if let Some(key) = get_key()
            && key == KeyCode::Escape
        {
            break;
        }

        vm.process();

        vm.update_timers();

        vm.display.draw();

        next_tick += frame_dur;
        let now = Instant::now();
        if next_tick > now {
            spin_sleep::sleep(next_tick - now);
        } else {
            next_tick = now;
        }

        next_frame().await;
    }

    Ok(())
}
