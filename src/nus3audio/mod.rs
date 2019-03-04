mod parser;
mod internal;

#[derive(Debug)]
pub struct Nus3audioFile<'a> {
    pub files: Vec<AudioFile<'a>>
}

#[derive(Debug)]
pub struct AudioFile<'a> {
    pub id: u32,
    pub name: String,
    pub data: &'a [u8]
}

impl<'a> Nus3audioFile<'a> {
    pub fn new() -> Self {
        Nus3audioFile { files: vec![] }
    }
    
    pub fn from_bytes(data: &[u8]) -> Nus3audioFile {
        parser::take_file(
            &data[..]
        ).expect("Failed to parse file").1
    }
}

