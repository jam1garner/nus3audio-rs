use nom::{le_u32, IResult};
use super::Nus3audioFile;

pub fn take_file(input: &[u8]) -> IResult<&[u8], Nus3audioFile> {
    do_parse!(input,
        tag!(b"NUS3") >>
        size: le_u32 >>
        ({println!("{}", size); Nus3audioFile::new()})
    )
}
