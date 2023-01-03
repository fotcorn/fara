use std::fs::File;
use std::io::Read;

use xmas_elf::ElfFile;

use crate::machine_state::MachineState;

pub fn elf(filename: &str, _symbol: &str, print_instr: bool) {
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let elf_file = ElfFile::new(&buffer).unwrap();

    let text_section = elf_file
        .find_section_by_name(".text")
        .expect(".text section not found in elf file");

    let text_section = text_section.raw_data(&elf_file);

    let mut ms = MachineState::new();
    ms.memory[0x1000..0x1000 + text_section.len()].copy_from_slice(&text_section);

    ms.run(print_instr);
}
