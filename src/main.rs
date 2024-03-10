use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

use tiff::header::parse_tiff_header;
use tiff::ifd::parse_ifd;

mod tiff;

fn read_file_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn read_tiff(file_path: &str) {
    let bytes = match read_file_bytes(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            exit(1);
        }
    };

    let header = match parse_tiff_header(&bytes) {
        Ok(header) => header,
        Err(e) => {
            eprintln!("Invalid tiff: {:#?}", e);
            exit(1);
        }
    };

    let ifd = match parse_ifd(
        &bytes[header.ifd_offset as usize..],
        header.ifd_offset,
        header.byte_order,
    ) {
        Ok(ifd) => ifd,
        Err(e) => {
            eprintln!("Invalid tiff: {:#?}", e);
            exit(1);
        }
    };
    println!(
        "File is a valid tiff
Byte order is: {}
First IFD is at offset: {:#010x}
First IFD contains {} fields
Next IFD is at offset: {:#010x}",
        header.byte_order, header.ifd_offset, ifd.n_fields, ifd.next_ifd_offset
    );

    for (i, &ref field) in ifd.fields.iter().enumerate() {
        println!(
            "Field {}:
    Tag: {}
    Type: {:#?}
    Count: {}",
            i, field.tag, field.ftype, field.count
        );
        match field.value {
            Some(value) => println!("    Value: {}", value),
            None => (),
        }

        match field.offset {
            Some(offset) => println!("    Offset: {}", offset),
            None => (),
        }
    }
}

fn help() {
    println!(
        "Usage:    
main <tif_file>"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let file_path = &args[1];
            read_tiff(file_path);
        }
        _ => {
            eprintln!("Expected exactly 1 argument");
            help();
            exit(1);
        }
    }
}
