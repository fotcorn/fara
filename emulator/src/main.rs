extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate num_derive;

mod cpu;
mod cpu_utils;
mod decoder;
mod instruction;
mod instruction_set;
mod machine_state;
mod loader_bin;
mod loader_elf;

fn main() {
    let matches = App::new("emulator")
        .arg(Arg::with_name("file").required(true))
        .arg(Arg::with_name("symbol")
        .help("symbol to execute in elf file")
        .long("symbol")
        .short("s")
        .takes_value(true))
    .arg(Arg::with_name("loader")
        .help("binary loader type")
        .long("loader")
        .short("l")
        .takes_value(true)
        .possible_values(&["elf", "bin"]))
        .get_matches();
    let filename = matches.value_of("file").unwrap();
    let symbol = matches.value_of("symbol").unwrap_or("main");
    let loader = matches.value_of("loader").unwrap_or("elf");

    match loader {
        "elf" => {
            loader_elf::elf(filename, symbol);
        }
        "bin" => {
            loader_bin::dump(filename);
        }
        _ => unreachable!("Values already validated by clap"),
    }
}
