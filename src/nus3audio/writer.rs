use super::Nus3audioFile;
use byteorder::{LittleEndian, WriteBytesExt};
use std::mem::size_of;
use std::collections::HashMap;
use crc::crc32;

fn get_padding_amount(offset: usize) -> usize {
    ((0x18 - (offset as isize % 0x10)) % 0x10) as usize
}

impl Nus3audioFile {
    pub fn calc_size(&self) -> usize {

        let nus3_size = "NUS3".len() + size_of::<u32>();
        let audi_size = "AUDIINDX".len() + (size_of::<u32>() * 2);
        let tnid_size = "TNID".len() + size_of::<u32>()
                        + (size_of::<u32>() * self.files.len());
        let nmof_size = tnid_size;
        let adof_size = "ADOF".len() + size_of::<u32>()
                        + (size_of::<u32>() * self.files.len() * 2);
        
        let string_section_start = nus3_size + audi_size + tnid_size + nmof_size
                                    + adof_size + "TNNM".len() + size_of::<u32>();

        let mut string_section_size = 0u32;
        for file in self.files.iter() {
            string_section_size += file.name.len() as u32 + 1;
        }

        let junk_pad = get_padding_amount(string_section_start + string_section_size as usize
                                          + "JUNK".len() + size_of::<u32>());
        let junk_size = "JUNK".len() + size_of::<u32>() + junk_pad;

        let pack_section_start = string_section_start + string_section_size as usize
                                    + junk_size + "PACK".len() + size_of::<u32>();

        let mut pack_section_size = 0u32;
        for file in self.files.iter() {
            pack_section_size += ((file.data.len() + 0xF) / 0x10) as u32 * 0x10;
        }
        
        pack_section_start + pack_section_size as usize
    }
    
    pub fn write(&self, f: &mut Vec<u8>) {
        macro_rules! write {
            ($e:expr) => {
                WriteImpl::write($e, f);
            }
        }
        
        // Offset calculation time
        let mut string_offsets: Vec<u32> = vec![];
        let mut file_offsets: Vec<(u32,u32)> = vec![];
        
        let nus3_size = "NUS3".len() + size_of::<u32>();
        let audi_size = "AUDIINDX".len() + (size_of::<u32>() * 2);
        let tnid_size = "TNID".len() + size_of::<u32>()
                        + (size_of::<u32>() * self.files.len());
        let nmof_size = tnid_size;
        let adof_size = "ADOF".len() + size_of::<u32>()
                        + (size_of::<u32>() * self.files.len() * 2);
        
        let string_section_start = nus3_size + audi_size + tnid_size + nmof_size
                                    + adof_size + "TNNM".len() + size_of::<u32>();

        let mut string_section_size = 0u32;
        for file in self.files.iter() {
            string_offsets.push(string_section_start as u32 + string_section_size);
            string_section_size += file.name.len() as u32 + 1;
        }

        let junk_pad = get_padding_amount(string_section_start + string_section_size as usize
                                          + "JUNK".len() + size_of::<u32>());
        let junk_size = "JUNK".len() + size_of::<u32>() + junk_pad;

        let pack_section_start = string_section_start + string_section_size as usize
                                    + junk_size + "PACK".len() + size_of::<u32>();

        let mut pack_section_size = 0u32;
        let mut existing_files: HashMap<u32, (u32, u32)> = HashMap::new();
        let mut files_to_pack = vec![];
        for file in self.files.iter() {
            let hash = crc32::checksum_ieee(&file.data);
            
            let offset_pair = 
                match existing_files.get(&hash) {
                    Some(pair) => *pair,
                    None => {
                        let pair = (pack_section_start as u32 + pack_section_size,
                                 file.data.len() as u32);
                        existing_files.insert(hash, pair);
                        files_to_pack.push(&file.data[..]);
                        pack_section_size += ((file.data.len() + 0xF) / 0x10) as u32 * 0x10;
                        
                        pair
                    }
                };
            file_offsets.push(offset_pair);
        }
        
        let filesize = pack_section_start as u32 + pack_section_size;

        // Actually write to file
        write!("NUS3");
        write!(filesize - nus3_size as u32);
        write!("AUDIINDX");
        write!(4u32); // Size of audiindx section
        write!(self.files.len() as u32); // Number of files
        write!("TNID");
        write!(self.files.len() as u32 * 4);
        write!(
            self.files
                .iter()
                .map(|a| a.id as u32)
                .collect::<Vec<u32>>()
        );
        write!("NMOF");
        write!(self.files.len() as u32 * 4);
        write!(string_offsets);
        write!("ADOF");
        write!(self.files.len() as u32 * 8);
        write!(file_offsets);
        write!("TNNM");
        write!(string_section_size);
        for file in self.files.iter() {
            write!(&file.name[..]);
            write!(0u8);
        }
        write!("JUNK");
        write!(junk_pad as u32);
        write!(vec![0u8; junk_pad]);
        write!("PACK");
        write!(pack_section_size);
        for file in files_to_pack.iter() {
            write!(&file[..]);
            write!(vec![0u8; (0x10 - (file.len() % 0x10)) % 0x10 ]);
        }
    }
}

// WriteImpl trait for ezpz clean file writing
trait WriteImpl {
    fn write(self, f: &mut Vec<u8>);
}

impl WriteImpl for u32 {
    fn write(self, f: &mut Vec<u8>) {
        f.write_u32::<LittleEndian>(self).unwrap();
    }
}

impl WriteImpl for u8 {
    fn write(self, f: &mut Vec<u8>) {
        f.push(self);
    }
}

impl WriteImpl for &[u8] {
    fn write(self, f: &mut Vec<u8>) {
        f.extend_from_slice(self);
    }
}

impl WriteImpl for &str {
    fn write(self, f: &mut Vec<u8>) {
        f.extend_from_slice(self.as_bytes());
    }
}

impl<T> WriteImpl for Vec<T> where T: WriteImpl + Copy, {
    fn write(self, f: &mut Vec<u8>) {
        for i in self {
            WriteImpl::write(i, f);
        }
    }
}

impl<T> WriteImpl for &mut Iterator<Item=T> where T: WriteImpl, {
    fn write(self, f: &mut Vec<u8>) {
        loop {
            match self.next() {
                Some(b) => WriteImpl::write(b, f),
                None => break
            }
        }
    }
}

impl<T> WriteImpl for std::slice::Iter<'_, T> where T: WriteImpl + Clone, {
    fn write(self, f: &mut Vec<u8>) {
        for i in self {
            WriteImpl::write(i.clone(), f);
        }        
    }
}

impl<T, T2> WriteImpl for (T, T2)
    where T: WriteImpl, T2: WriteImpl {
    fn write(self, f: &mut Vec<u8>) {
        WriteImpl::write(self.0, f);
        WriteImpl::write(self.1, f);
    }
}
