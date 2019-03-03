pub enum Section {
    Nmof(Nmof),
    Adof(Adof),
    Tnnm(Tnnm),
    Junk,
    Pack,
}

pub struct Nmof {
    pub name_offsets: Vec<u32>,
}

pub struct Adof {
    pub entries: Vec<(u32, u32)>,
}

pub struct Tnnm {
    pub track_nums: Vec<u32>,
}
