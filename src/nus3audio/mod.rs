mod parser;
mod writer;
mod internal;

#[derive(Debug)]
pub struct Nus3audioFile {
    pub files: Vec<AudioFile>
}

#[derive(Debug)]
pub struct AudioFile {
    pub id: u32,
    pub name: String,
    pub data: Vec<u8>
}

impl Nus3audioFile {
    pub fn new() -> Self {
        Nus3audioFile { files: vec![] }
    }
    
    pub fn from_bytes(data: &[u8]) -> Nus3audioFile {
        parser::take_file(
            &data[..]
        ).expect("Failed to parse file").1
    }
}

impl AudioFile {
    pub fn from_id(id: u32) -> Self {
        AudioFile { data: vec![], name: String::new(), id }
    }
    
    pub fn filename(&self) -> String {
        self.name.clone() + 
            if self.data.len() >= 4 {
                match &self.data[..4] {
                    b"OPUS" => ".lopus",
                    b"IDSP" => ".idsp",
                    _ => ".bin",
                }
            } else {
                ".bin"
            }
    }
}
