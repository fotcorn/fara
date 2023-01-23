use std::fs::File;
use std::io::Read;

use zero::read_str;

use xmas_elf::symbol_table::Entry;
use xmas_elf::{program, sections, ElfFile};

use crate::machine_state::MachineState;

pub fn elf(filename: &str, symbol_name: &str, print_instructions: bool) {
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file.");

    let elf_file = ElfFile::new(&buffer).unwrap();

    // get the virtual address of the main function
    let main_symbol_address = get_main_symbol_address(&elf_file, symbol_name);

    let mut machine_state = MachineState::new();
    load_program_image(&elf_file, &buffer, &mut machine_state);
    machine_state.pc = main_symbol_address as i64;
    machine_state.run(print_instructions);
}

fn load_program_image(elf_file: &ElfFile, buffer: &[u8], machine_state: &mut MachineState) {
    for sect in elf_file.program_iter() {
        let t = sect.get_type().unwrap();
        match t {
            program::Type::Load => {
                let from = sect.offset() as usize;
                let to = (sect.offset() + sect.file_size()) as usize;
                machine_state.write_memory(sect.virtual_addr() as usize, &buffer[from..to]);
            }
            _ => (),
        }
    }
}

fn get_main_symbol_address(elf_file: &ElfFile, symbol_name: &str) -> u64 {
    let symbol_string_table = elf_file
        .find_section_by_name(".strtab")
        .expect("strtab (String table) section not found, is this a stripped binary?");
    let symbol_string_table = symbol_string_table.raw_data(&elf_file);

    let symbol_table = elf_file
        .find_section_by_name(".symtab")
        .expect("symtab (Symbol table) section not found");
    if let sections::SectionData::SymbolTable64(data) = symbol_table.get_data(&elf_file).unwrap() {
        let symbol = data
            .iter()
            .find(|&symbol| read_str(&symbol_string_table[symbol.name() as usize..]) == symbol_name)
            .expect("symbol not found");
        symbol.value()
    } else {
        unreachable!();
    }
}
