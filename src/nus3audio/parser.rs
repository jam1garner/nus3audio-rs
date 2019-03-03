use nom::{le_u32, rest, IResult};
use super::{AudioFile, Nus3audioFile};
use super::internal::{Section, Nmof, Adof, Tnnm};

fn take_section(buf: &[u8], track_count: u32) -> IResult<&[u8], Section> {
    do_parse!(
        buf,
        section: alt!(
            do_parse!(
                tag!(b"NMOF") >>
                length_bytes!(le_u32) >>
                name_offsets: many0!(le_u32) >>
                (Section::Nmof(Nmof { name_offsets }))
            ) | 
            do_parse!(
                tag!(b"ADOF") >>
                length_bytes!(le_u32) >>
                entries: many0!(do_parse!(
                    offset: le_u32 >>
                    size: le_u32 >>
                    ((offset, size))
                )) >>
                (Section::Adof(Adof { entries }))

            ) | 
            do_parse!(
                tag!(b"TNNM") >>
                length_bytes!(le_u32) >>
                track_nums: many0!(le_u32) >>
                (Section::Tnnm(Tnnm { track_nums }))
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
                string: terminated!(rest, char!('\0')) >>
                (std::str::from_utf8(string).unwrap().to_string())
            ).unwrap().1
        })
        .collect()
}

fn get_track_ids(sections: &Vec<Section>) -> Option<Vec<u32>> {
    for section in sections {
        if let Section::Tnnm(tnnm) = section {
            return Some(tnnm.track_nums.clone());
        }
    }
    None
}

fn take_audiindx(buf: &[u8]) -> IResult<&[u8], u32> {
    do_parse!(
        buf,
        tag!(b"AUDIINDX") >>
        length_bytes!(le_u32) >>
        track_count: le_u32 >>
        (track_count)
    )
}

pub fn take_file(input: &[u8]) -> IResult<&[u8], Nus3audioFile> {
    do_parse!(
        input,
        tag!(b"NUS3") >>
        length_bytes!(le_u32) >>
        track_count: take_audiindx >>
        sections: many0!(apply!(take_section, track_count)) >>
        ({
            let adof_entries = get_adof_entries(&sections)
                                .unwrap();
            let files = adof_entries
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
                             data, 
                             name: name.to_string(),
                             id: id
                         }
                    )
                    .collect(),
            }
        })
    )
}
