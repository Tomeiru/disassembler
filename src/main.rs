mod destination;
mod disp;
mod header;
mod instruction;
mod modifier;
mod register;
mod register_memory;
mod word;

use std::{env, fs, process, usize};

// TODO: overhaul errors
// TODO: think about how to handle size checks in instructions
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Problem parsing arguments: one file argument should be provided, received {}",
            args.len() - 1
        );
        std::process::exit(1);
    }
    let data: Vec<u8> = match fs::read(&args[1]) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    let header = match header::Header::init(&data) {
        Ok(header) => header,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    let text_start = header.get_text_position();
    let text = &data[text_start..text_start + header.get_text_size() as usize].to_vec();

    let categories = instruction::create_categories();
    let mut i = 0;
    while i < text.len() {
        let mut treated_bytes = 1;
        let mut found = false;
        for category in &categories {
            for instruction in &category.instructions {
                if !(instruction.recognize)(&text[i..]) {
                    continue;
                }
                let (formatted_arguments, used_bytes) =
                    (instruction.disassemble)(&text[i..]).unwrap();
                treated_bytes = used_bytes.len();
                let formatted_instructions: Vec<String> = used_bytes
                    .into_iter()
                    .map(|instruction| format!("{:02x}", instruction))
                    .collect();
                println!(
                    "{:04x}: {:<13} {} {}",
                    i,
                    formatted_instructions.join(""),
                    category.mnemonic,
                    formatted_arguments.join(", ")
                );
                found = true;
                break;
            }
            if found {
                break;
            }
        }
        i += treated_bytes;
    }
}
