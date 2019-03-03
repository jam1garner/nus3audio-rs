extern crate clap;
#[macro_use]
extern crate nom;
mod nus3audio;

use clap::{Arg, App};
use std::io::prelude::*;

fn main() {
    let matches = 
        App::new("nus3audio")
        .version("1.0")
        .about("Tool for working with nus3audio files")
        .author("jam1garner")
        .arg(Arg::with_name("replace")
                .help("Replaces a file at INDEX with NEWFILE")
                .short("r")
                .long("replace")
                .value_names(&["INDEX", "NEWFILE"]))
        .arg(Arg::with_name("write")
                .help("Write to FILE after performing all other operations")
                .short("w")
                .value_name("FILE")
                .takes_value(true)
                .long("write"))
        .arg(Arg::with_name("extract")
                .help("Extract nus3audio contents to FOLDER")
                .short("o")
                .long("output")
                .value_name("FOLDER")
                .takes_value(true))
        .arg(Arg::with_name("delete")
                .help("Delete file at INDEX in nus3audio file")
                .short("d")
                .long("delete")
                .value_name("INDEX")
                .takes_value(true))
        .arg(Arg::with_name("print")
                .help("Prints the contents of the nus3audio file")
                .short("p")
                .long("print"))
        .arg(Arg::with_name("visual")
                .help("Edit in visual mode")
                .short("v")
                .long("visual"))
        .arg(Arg::with_name("file")
                .help("nus3audio file to open")
                .required(true))
        .get_matches();

    let file_name = matches.value_of("file").unwrap();
    let mut data = Vec::new();
    std::fs::File::open(file_name).expect("Failed to open file")
        .read_to_end(&mut data).expect("Failed to read data");

    println!("{:?}", nus3audio::Nus3audioFile::from_bytes(&data[..]));
}
