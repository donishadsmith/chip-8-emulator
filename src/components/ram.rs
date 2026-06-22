use crate::fontset::FONTSET;

pub struct RAM {
    pub code_segment: [u8; 4096],
    pub stack: [u16; 16],
}

impl RAM {
    pub fn start() -> Self {
        let mut code_segment = [0u8; 4096];
        for index in 0..FONTSET.len() {
            code_segment[index] = FONTSET[index];
        }

        let stack = [0u16; 16];

        Self {
            code_segment,
            stack,
        }
    }
}
