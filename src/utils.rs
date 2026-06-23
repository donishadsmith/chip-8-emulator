use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed};

pub fn get_key() -> Option<KeyCode> {
    let mut key_press = get_keys_pressed().iter().next().cloned();
    if key_press.is_none() {
        key_press = get_keys_down().iter().next().cloned();
    }

    key_press
}

fn chip8_to_keycode(key: u8) -> Option<KeyCode> {
    match key {
        0x1 => Some(KeyCode::Key1),
        0x2 => Some(KeyCode::Key2),
        0x3 => Some(KeyCode::Key3),
        0xC => Some(KeyCode::Key4),
        0x4 => Some(KeyCode::Q),
        0x5 => Some(KeyCode::W),
        0x6 => Some(KeyCode::E),
        0xD => Some(KeyCode::R),
        0x7 => Some(KeyCode::A),
        0x8 => Some(KeyCode::S),
        0x9 => Some(KeyCode::D),
        0xE => Some(KeyCode::F),
        0xA => Some(KeyCode::Z),
        0x0 => Some(KeyCode::X),
        0xB => Some(KeyCode::C),
        0xF => Some(KeyCode::V),
        _ => None,
    }
}

pub fn is_chip8_key_down(chip8_key: u8) -> bool {
    match chip8_to_keycode(chip8_key) {
        Some(code) => get_keys_down().contains(&code),
        None => false,
    }
}

fn map_key(key: Option<KeyCode>) -> Option<u16> {
    match key {
        Some(KeyCode::Key1) => Some(0x1),
        Some(KeyCode::Key2) => Some(0x2),
        Some(KeyCode::Key3) => Some(0x3),
        Some(KeyCode::Key4) => Some(0xC),
        Some(KeyCode::Q) => Some(0x4),
        Some(KeyCode::W) => Some(0x5),
        Some(KeyCode::E) => Some(0x6),
        Some(KeyCode::R) => Some(0xD),
        Some(KeyCode::A) => Some(0x7),
        Some(KeyCode::S) => Some(0x8),
        Some(KeyCode::D) => Some(0x9),
        Some(KeyCode::F) => Some(0xE),
        Some(KeyCode::Z) => Some(0xA),
        Some(KeyCode::X) => Some(0x0),
        Some(KeyCode::C) => Some(0xB),
        Some(KeyCode::V) => Some(0xF),
        _ => None,
    }
}

pub fn key_event_pressed_only() -> Option<u16> {
    map_key(get_keys_pressed().iter().next().cloned())
}
