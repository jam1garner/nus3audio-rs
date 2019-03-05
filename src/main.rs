#![feature(trace_macros)]
#[macro_use] extern crate itertools;
#[macro_use] extern crate nom;
extern crate clap;
mod visual_mode;
mod nus3audio;

use clap::{Arg, App};
use std::io::prelude::*;
use itertools::sorted;
use nus3audio::Nus3audioFile;
use std::path::PathBuf;
use std::fs;

fn extract(nus3: &Nus3audioFile, folder: &str) {
    fs::create_dir_all(folder).expect("Failed to create extract directory");
    for file in nus3.files.iter() {
        let path: PathBuf = [folder, &file.filename()[..]].iter().collect();
        fs::File::create(path)
            .expect("Failed to open extract file")
            .write_all(&file.data[..])
            .expect("Failed to write bytes to extract file");
    }
}

fn main() {
    let args = 
        App::new("nus3audio")
        .version("1.0")
        .about("Tool for working with nus3audio archive files")
        .author("jam1garner")
        .arg(Arg::with_name("new")
                .help("Creates a new nus3audio file instead of reading one it")
                .short("n")
                .long("new"))
        .arg(Arg::with_name("replace")
                .help("Replaces a file at INDEX with NEWFILE")
                .short("r")
                .long("replace")
                .multiple(true)
                .value_names(&["INDEX", "NEWFILE"]))
        .arg(Arg::with_name("write")
                .help("Write to FILE after performing all other operations")
                .short("w")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .long("write"))
        .arg(Arg::with_name("extract")
                .help("Extract nus3audio contents to FOLDER")
                .short("e")
                .long("extract")
                .value_name("FOLDER")
                .multiple(true)
                .takes_value(true))
        .arg(Arg::with_name("delete")
                .help("Delete file at INDEX in nus3audio file")
                .short("d")
                .long("delete")
                .value_name("INDEX")
                .multiple(true)
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
                .required_unless("new")
                .conflicts_with("new"))
        .get_matches();

    let mut nus3_file =
        if let Some(file_name) = args.value_of("file") {
            let mut data = Vec::new();
            std::fs::File::open(file_name).expect("Failed to open file")
                .read_to_end(&mut data).expect("Failed to read data");
            nus3audio::Nus3audioFile::from_bytes(&data[..])
        }
        else {
            nus3audio::Nus3audioFile::new()
        };
    

    if let Some(replace_values) = args.values_of("replace") {
        let replace_values = replace_values.collect::<Vec<_>>();
        let pairs = replace_values
                .chunks(2)
                .map(|x| (x[0].parse::<usize>()
                              .expect("Provided replace index not a number"),
                          x[1]));
        for (index, filename) in pairs {
            let mut new_file = Vec::new();
            fs::File::open(filename)
                .expect("Failed to open replacement file")
                .read_to_end(&mut new_file)
                .expect("Failed to read from replacement file");
            nus3_file.files[index].data = new_file;
        }
    }

    if let Some(delete_indices) = args.values_of("delete") {
        let indices = 
            sorted(delete_indices.map(|i|
                    i.parse::<usize>()
                     .expect("Deleted index not valid u32"))
                  ).rev();
        for i in indices {
            nus3_file.files.remove(i);
        }
    }

    if let Some(export_folders) = args.values_of("extract") {
        for folder in export_folders {
            extract(&nus3_file, folder);
        }
    }

    if args.is_present("print") {
        for audio_file in nus3_file.files.iter() {
            println!("name: {}, id: {}, filesize: {}", audio_file.name, audio_file.id, audio_file.data.iter().len())
        }
    }

    if args.is_present("visual") {
        visual_mode::main(&mut nus3_file);        
    }

    if let Some(write_files) = args.values_of("write") {
        let mut file_bytes: Vec<u8> = Vec::with_capacity(nus3_file.calc_size());
        nus3_file.write(&mut file_bytes);
        for path in write_files {
            fs::File::create(path)
                .expect("Failed to open writing file")
                .write_all(&file_bytes[..])
                .expect("Failed to write bytes to file");
        }
    }
}
