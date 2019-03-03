mod parser;
mod internal;

#[derive(Debug)]
pub struct Nus3audioFile<'a> {
    files: Vec<AudioFile<'a>>
}

#[derive(Debug)]
pub struct AudioFile<'a> {
    id: u32,
    name: String,
    data: &'a [u8]
}

impl<'a> Nus3audioFile<'a> {
    pub fn new() -> Self {
        Nus3audioFile { files: vec![] }
    }
    
    pub fn from_bytes(data: &[u8]) -> Option<Nus3audioFile> {
        Some(parser::take_file(
            &data[..]
        ).expect("Failed to parse file").1)
    }
}

