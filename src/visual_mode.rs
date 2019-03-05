use super::nus3audio::{Nus3audioFile, AudioFile};
use std::{io, fs};
use std::io::prelude::*;

fn display_nus3(nus3: &Nus3audioFile) {
    println!("\n\nContents: \n---------");
    for (i, file) in nus3.files.iter().enumerate() {
        println!("[{:2}] id: {:2} size: {:6} name: '{}'", i, file.id, file.data.len(), file.name);
    }
}

fn flush() {
    io::stdout().flush().ok().expect("Could not flush stdout");
}

const ENTRY_HELP: &str = "
Help
----
i - edit id
n - edit name
d - replace data with another file's contents
q - quit editing this entry
? - display this menu
";

fn edit_entry(audio_file: &mut AudioFile) {
    let reader = io::stdin();
    loop {
        print!("[i/n/d/q/?] ");
        flush();
        let mut command = String::new();
        reader.read_line(&mut command).unwrap();
        let command = command.trim();
        match &command.trim()[..] {
            "i" => {
                print!("New ID: ");
                flush();
                let mut id_text = String::new();
                reader.read_line(&mut id_text).unwrap();
                let id_text = id_text.trim();
                match id_text.parse::<u32>() {
                    Ok(id) => {
                        audio_file.id = id;
                    }
                    _ => {
                        println!("Not a number.");
                    }
                }
            }
            "n" => {
                print!("New name: ");
                flush();
                let mut name = String::new();
                reader.read_line(&mut name).unwrap();
                let name = name.trim();
                audio_file.name = String::from(name);
            }
            "d" => {
                print!("Path of file to replace with: ");
                flush();
                let mut path = String::new();
                reader.read_line(&mut path).unwrap();
                let path = path.trim();
                let mut data: Vec<u8> = vec![];
                match fs::File::open(path) {
                    Ok(mut f) => {
                        match f.read_to_end(&mut data) {
                            Ok(_) => {
                                audio_file.data = data;
                            }
                            _ => {
                                "Failed to read from file";
                            }
                        }
                    }
                    _ => {
                        println!("Failed to open file");
                    }
                }
            }
            "?" => {
                println!("{}", ENTRY_HELP);
            }
            "q" => {
                break;
            }
            _ => {

            }
        }
    }
}

const HELP_TEXT: &str = "
Help
----
w - write to file
e - edit entry
d - delete entry
a - add entry
r - renumber ids to match index
q - quit program
? - show this menu
";

pub fn main(nus3_file: &mut Nus3audioFile) {
    let reader = io::stdin();
    loop {
        display_nus3(nus3_file);
        print!("[w/e/d/a/r/q/?] ");
        flush();
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let input = input.trim();
        match &input[..] {
            "w" => {
                // Write
                print!("Write to: ");
                flush();
                let mut path = String::new();
                reader.read_line(&mut path).unwrap();
                let path = path.trim();
                let mut file_bytes: Vec<u8> = Vec::with_capacity(nus3_file.calc_size());
                nus3_file.write(&mut file_bytes);
                fs::File::create(path)
                    .expect("Failed to open writing file")
                    .write_all(&file_bytes[..])
                    .expect("Failed to write bytes to file");
            }
            "e" => {
                // Edit
                print!("Entry to edit (number): ");
                flush();
                let mut index_text = String::new();
                reader.read_line(&mut index_text).unwrap();
                let index_text = index_text.trim();
                let num_files = nus3_file.files.len();
                match index_text.parse::<usize>() {
                    Ok(index) => {
                        if index < num_files {
                            edit_entry(&mut nus3_file.files[index]);
                        }
                        else {
                            println!("Index out of range.");
                        }
                        
                    }
                    _ => {
                        println!("Not a number.");
                    }
                }
            }
            "a" => {
                // Add
                print!("Insert at (leave blank for end): ");
                flush();
                let mut index_text = String::new();
                reader.read_line(&mut index_text).unwrap();
                let index_text = index_text.trim();
                let num_files = nus3_file.files.len();
                let index = index_text.parse::<usize>().unwrap_or(num_files);
                if index <= num_files {
                    nus3_file.files.insert(index, AudioFile::from_id(index as u32));
                }
                else {
                    println!("Index out of range.");
                }
            }
            "d" => {
                // Delete
                print!("Delete index (number): ");
                flush();
                let mut index_text = String::new();
                reader.read_line(&mut index_text).unwrap();
                let index_text = index_text.trim();
                match index_text.parse::<usize>() {
                    Ok(index) => {
                        if index < nus3_file.files.len() {
                            nus3_file.files.remove(index);
                        }
                        else {
                            println!("Index out of range.");
                        }
                    }
                    _ => {
                        println!("Not a number.");
                    }
                }
            }
            "r" => {
                // Renumber
                for (index, file) in nus3_file.files.iter_mut().enumerate() {
                    file.id = index as u32;
                }
            }
            "?" => {
                println!("{}", HELP_TEXT);
            }
            "q" => {
                break;
            }
            _ => {
                println!("Unrecognized command. Use '?' for help.");
            }
        }
    }
}
