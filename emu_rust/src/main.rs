extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io::Read;

mod instruction_set;
mod machine_state;

fn main() {
    let matches = App::new("emulator")
        .arg(Arg::with_name("file").required(true))
        .get_matches();
    let filename = matches.value_of("file").unwrap();

    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let mut ms = machine_state::MachineState::new();

    ms.memory[0x1000..0x1000 + buffer.len()].copy_from_slice(&buffer);
}
