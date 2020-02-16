extern crate itertools;
extern crate clap;
extern crate crc;
extern crate nus3audio;
mod visual_mode;

use clap::Clap;
use std::io::prelude::*;
use nus3audio::{Nus3audioFile, AudioFile};
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::num::ParseIntError;

fn extract_name<P: AsRef<Path>>(nus3: &Nus3audioFile, folder: P) {
    fs::create_dir_all(folder.as_ref()).expect("Failed to create extract directory");
    for file in nus3.files.iter() {
        let folder = folder.as_ref().to_str().unwrap();
        let path: PathBuf = [folder, &file.filename()[..]].iter().collect();
        fs::write(path, &file.data).expect("Failed to write bytes to extract file");
        println!("{}", file.filename());
    }

}

fn extract_id<P: AsRef<Path>>(nus3: &Nus3audioFile, folder: P) {
    let folder = folder.as_ref();
    fs::create_dir_all(folder).expect("Failed to create extract directory");
    for file in nus3.files.iter() {

        let content_type = match Path::new(&file.filename()).extension() {
            None => "",
            Some(os_str) => {
                match os_str.to_str() {
                    Some("lopus") => ".lopus",
                    Some("idsp") => ".idsp",
                    _ => panic!("Invalid"),
                }
            }
        };

        let file_export = format!("{}{}", &file.id.to_string(), content_type);
        
        let path: PathBuf = [folder, file_export.as_ref()].iter().collect();
        
        fs::write(path, &file.data).expect("Failed to write bytes to extract file");

        println!("{}", file_export);
    }
}

#[derive(Clap)]
#[clap(version, author="jam1garner", author="BenHall-7")]
struct Args {
    #[clap(long, short="n", help="Creates a new nus3audio file instead of reading one in")]
    new: bool,

    #[clap(long, short="r", value_names=&["INDEX", "NEWFILE"])]
    replace: Vec<String>,

    #[clap(long, short="A", value_names=&["NAME", "NEWFILE"])]
    append: Vec<String>,

    #[clap(long, short="w", help="Write to FILE after performing all other operations")]
    write: Vec<PathBuf>,

    #[clap(long, short="e", help="Extract nus3audio contents with their filenames to FOLDER")]
    extract_name: Vec<PathBuf>,

    #[clap(long, short="i", help="Extract nus3audio contents with their ids to FOLDER", value_name="FOLDER")]
    extract_id: Vec<PathBuf>,

    #[clap(long, short="R", help="Rebuild nus3audio contents with filenames from a FOLDER", value_name="FOLDER")]
    rebuild_name: Option<PathBuf>,

    #[clap(long, help="Rebuild nus3audio contents with ids from a FOLDER", value_name="FOLDER")]
    rebuild_id: Option<PathBuf>,

    #[clap(long, short, help="Delete file at INDEX in nus3audio file", value_name="INDEX")]
    delete: Vec<usize>,

    #[clap(long, short, help="Prints the contents of the nus3audio file")]
    print: bool,

    #[clap(long, short, help="Prints the contents of the nus3audio file as json", conflicts_with="print")]
    json: bool,

    #[clap(long, short, help="Edit in visual mode", conflicts_with="print", conflicts_with="json")]
    visual: bool,

    #[clap(help="nus3audio file to open", conflicts_with="new", required_unless="new")]
    file: Option<PathBuf>,

    #[clap(long, short, help="Generates a corresponding .tonelabel file", requires="file")]
    tonelabel: bool,
}

fn show_help() -> ! {
    Args::parse_from(vec!["-h"]);
    unreachable!()
}

type IndexFilePairs = Vec<(usize, PathBuf)>;
type NameFilePairs = Vec<(String, PathBuf)>;

const REPLACE_ERROR: &str = "Replace Usage: nus3audio --replace [INDEX] [NEW FILE]";
const APPEND_ERROR: &str = "Append Usage: nus3audio --append [NAME] [NEW FILE]";

fn get_replace_append(args: &Args) -> Result<(IndexFilePairs, NameFilePairs), ParseIntError> {
    dbg!(&args.append);
    let replace_len = args.replace.len();
    let append_len = args.append.len();
    if replace_len % 2 != 0 {
        eprintln!("{}", REPLACE_ERROR);
        show_help()
    } else if append_len % 2 != 0 {
        eprintln!("{}", APPEND_ERROR);
        show_help()
    }
    Ok((
        args.replace
            .chunks_exact(2)
            .map(|chunk|{
                if let &[ref index, ref file] = chunk {
                    Ok((index.parse()?, file.into()))
                } else {
                    unsafe { std::hint::unreachable_unchecked() }
                }
            })
            .collect::<Result<_, _>>()?,
        args.append
            .chunks_exact(2)
            .map(|chunk|{
                if let &[ref name, ref file] = chunk {
                    Ok((name.clone(), file.into()))
                } else {
                    unsafe { std::hint::unreachable_unchecked() }
                }
            })
            .collect::<Result<_, _>>()?,
    ))
}

fn main() {
    let args = Args::parse();
    let (replace, append) = get_replace_append(&args)
        .unwrap_or_else(|e| {
            show_help();
        });

    let mut nus3_file =
        if let &Some(ref file_name) = &args.file {
            nus3audio::Nus3audioFile::open(file_name).expect("Failed to read file")
        }
        else {
            nus3audio::Nus3audioFile::new()
        };

    if let Some(rebuild_folder) = args.rebuild_name {
        for file in std::fs::read_dir(rebuild_folder).expect("failed to open rebuild folder") {
            let file = file.unwrap();
            let path = file.path();
            if path.is_dir() {
                continue
            }
            let filename = path.file_stem().unwrap().to_str().unwrap();
            if let Some(file) = nus3_file.files.iter_mut().find(|file| file.name == filename) {
                file.data = fs::read(path).expect("failed to read file");
            } else {
                println!("File '{}' not found in nus3audio file", filename);
            }
        }
    }


    if let Some(rebuild_folder) = args.rebuild_id {
        for file in std::fs::read_dir(rebuild_folder).expect("failed to open rebuild folder") {
            let file = file.unwrap();
            let path = file.path();
            if path.is_dir() {
                continue
            }
            let filename = path.file_stem().unwrap().to_str().unwrap();
            if let Some(file) = nus3_file.files.iter_mut().find(|file| file.id.to_string() == filename) {
                file.data = fs::read(path).expect("failed to read file");
            } else {
                println!("File '{}' not found in nus3audio file", filename);
            }
        }
    }

    for (index, filename) in replace {
        nus3_file.files[index].data = fs::read(filename).expect("Failed to open replacement file");
    }
    
    for i in args.delete {
        nus3_file.files.remove(i);
    }

    for (name, path) in append {
        nus3_file.files.push(AudioFile {
            id: nus3_file.files.len() as u32,
            name,
            data: fs::read(path).expect("Failed to read append data")
        });
    }
    
    for folder in args.extract_name {
        extract_name(&nus3_file, folder);
    }

    for folder in args.extract_id {
        extract_id(&nus3_file, folder);
    }

    if args.print {
        for audio_file in nus3_file.files.iter() {
            println!("name: {}, id: {}, filesize: {}", audio_file.name, audio_file.id, audio_file.data.iter().len())
        }
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&nus3_file).unwrap());
    }

    if args.visual {
        visual_mode::main(&mut nus3_file);        
    }

    if args.tonelabel {
        let mut file_bytes: Vec<u8> = Vec::with_capacity(nus3_file.calc_tonelabel_size());
        nus3_file.write_tonelabel(&mut file_bytes);
        let mut name = args.file.unwrap().clone();
        name.set_extension("tonelabel");
        fs::File::create(name)
            .expect("Failed to open tonelabel writing file")
            .write_all(&file_bytes[..])
            .expect("Failed to write bytes to tonelabel file")
    }

    if !args.write.is_empty() {
        let mut file_bytes: Vec<u8> = Vec::with_capacity(nus3_file.calc_size());
        nus3_file.write(&mut file_bytes);
        for path in args.write {
            fs::write(path, &file_bytes).expect("Failed to write bytes to file");
        }
    }
}
