mod header;
use crate::header::Header;
use std::{env, fs, process, usize};

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
    let header = match Header::init(&data) {
        Ok(header) => header,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    let text_start = header.get_text_position();
    let text = &data[text_start..text_start + header.get_text_size() as usize];
    dbg!(text);
}
