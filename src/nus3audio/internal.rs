#[derive(Debug)]
pub enum Section {
    Nmof(Nmof),
    Adof(Adof),
    Tnid(Tnid),
    Tnnm,
    Junk,
    Pack,
}

#[derive(Debug)]
pub struct Nmof {
    pub name_offsets: Vec<u32>,
}

#[derive(Debug)]
pub struct Adof {
    pub entries: Vec<(u32, u32)>,
}

#[derive(Debug)]
pub struct Tnid {
    pub track_nums: Vec<u32>,
}
