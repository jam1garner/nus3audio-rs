pub enum Section {
    Audiindx(Audiindx),
    Nmof(Nmof),
    Adof(Adof),
    Tnnm(Tnnm),
    Junk,
    Pack,
}

pub struct Audiindx {
    pub size: u32,
    pub track_count: u32,
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
