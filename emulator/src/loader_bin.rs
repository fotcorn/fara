use std::fs::File;
use std::io::prelude::*;

use crate::machine_state;

pub fn dump(filename: &str) {
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let mut ms = machine_state::MachineState::new();

    ms.memory[0x1000..0x1000 + buffer.len()].copy_from_slice(&buffer);

    ms.run();
}
