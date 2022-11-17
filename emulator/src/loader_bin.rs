use std::io::prelude::*;
use std::fs::File;

use crate::{machine_state, cpu, decoder};

pub fn dump(filename: &str) {
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let mut ms = machine_state::MachineState::new();

    ms.memory[0x1000..0x1000 + buffer.len()].copy_from_slice(&buffer);

    loop {
        let (instr, offset) = decoder::decode(&ms);
        ms.pc += offset;
        cpu::execute(&mut ms, &instr);
        if ms.halt {
            break;
        }
    }
}
