use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed};

pub fn get_key() -> Option<KeyCode> {
    let mut key_press = get_keys_pressed().iter().next().cloned();
    if key_press.is_none() {
        key_press = get_keys_down().iter().next().cloned();
    }

    key_press
}

pub fn key_event() -> Option<u16> {
    match get_key() {
        Some(KeyCode::Kp1) => Some(0x1),
        Some(KeyCode::Kp2) => Some(0x2),
        Some(KeyCode::Kp3) => Some(0x3),
        Some(KeyCode::Kp4) => Some(0xC),
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
