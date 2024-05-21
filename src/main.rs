use std::{env, fs, process};

// TODO: transcript in rust the a.out header structure described in /usr/local/core/minix2/usr/include/a.out.h
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
    let text_size_begin: usize = 8;
    let text_size_end: usize = 12;
    let text_size = i32::from_le_bytes(
        <[u8; 4]>::try_from(&data[text_size_begin..text_size_end])
            .expect("should not fail with test_assets/asem/a.out"),
    );
    let text_begin: usize = 32;
    let text = &data[text_begin..text_begin + text_size as usize];
    print!(
        "0000: bb0000        mov bx, 0000
0003: cd20          int 20
0005: bb1000        mov bx, 0010
0008: cd20          int 20
000a: 0000          add [bx+si], al
000c: 0000          add [bx+si], al
000e: 0000          add [bx+si], al"
    );
}
