use nom::{le_u32, IResult};
use super::{AudioFile, Nus3audioFile};
use super::internal::{Section, Nmof, Adof, Tnid};

fn take_section(buf: &[u8]) -> IResult<&[u8], Section> {
    do_parse!(
        buf,
        section: alt!(
            do_parse!(
                tag!(b"NMOF") >>
                size: le_u32 >>
                name_offsets: count!(le_u32, size as usize / 4) >>
                (Section::Nmof(Nmof { name_offsets }))
            ) | 
            do_parse!(
                tag!(b"ADOF") >>
                size: le_u32 >>
                entries: count!(do_parse!(
                    offset: le_u32 >>
                    size: le_u32 >>
                    ((offset, size))
                ), size as usize / 8) >>
                (Section::Adof(Adof { entries }))

            ) | 
            do_parse!(
                tag!(b"TNID") >>
                size: le_u32 >>
                track_nums: count!(le_u32, size as usize / 4) >>
                (Section::Tnid(Tnid { track_nums }))
            ) | 
            do_parse!(
                tag!(b"TNNM") >>
                length_bytes!(le_u32) >>
                (Section::Tnnm)
            ) | 
            do_parse!(
                tag!("JUNK") >>
                length_bytes!(le_u32) >>
                (Section::Junk)
            ) |
            do_parse!(
                tag!("PACK") >>
                length_bytes!(le_u32) >>
                (Section::Pack)
            )
        ) >>
        (section)
    )
}

fn get_adof_entries(sections: &Vec<Section>) -> Option<Vec<(u32, u32)>> {
    for section in sections {
        if let Section::Adof(adof) = section {
            return Some(adof.entries.clone());
        }
    }
    None
}

fn get_nmof_entries(sections: &Vec<Section>) -> Option<Vec<u32>> {
    for section in sections {
        if let Section::Nmof(nmof) = section {
            return Some(nmof.name_offsets.clone());
        }
    }
    None
}

fn get_names(name_offsets: Vec<u32>, input: &[u8]) -> Vec<String> {
    name_offsets
        .iter()
        .map(|offset| {
            do_parse!(&input[*offset as usize..],
                string: take_until!("\0") >>
                (std::str::from_utf8(string).unwrap().to_string())
            ).unwrap().1
        })
        .collect()
}

fn get_track_ids(sections: &Vec<Section>) -> Option<Vec<u32>> {
    for section in sections {
        if let Section::Tnid(tnid) = section {
            return Some(tnid.track_nums.clone());
        }
    }
    None
}

fn take_audiindx(buf: &[u8]) -> IResult<&[u8], u32> {
    do_parse!(
        buf,
        tag!(b"AUDIINDX") >>
        size: le_u32 >>
        track_count: le_u32 >>
        take!(size - 4) >>
        (track_count)
    )
}

pub fn take_file(input: &[u8]) -> IResult<&[u8], Nus3audioFile> {
    do_parse!(
        input,
        tag!(b"NUS3") >>
        bytes: length_bytes!(le_u32) >> 
        ({
            let sections = do_parse!(
                bytes,
                _track_count: take_audiindx >>
                secs: many0!(complete!(take_section)) >>
                (secs)
            ).unwrap().1;
            let adof_entries = get_adof_entries(&sections)
                                .unwrap();
            let files =
                adof_entries
                    .iter()
                    .map(|(offset,size)| (*offset as usize, *size as usize))
                    .map(|(offset, size)| &input[offset..offset+size]);
            let names = get_names(get_nmof_entries(&sections).unwrap(), input);
            let ids = get_track_ids(&sections).unwrap();

            Nus3audioFile {
                files :
                    izip!(files, names, ids)
                    .map(|(data, name, id)|
                         AudioFile {
                             data: Vec::from(data),
                             name: name.to_string(),
                             id: id
                         }
                    )
                    .collect(),
            }
        })
    )
}
