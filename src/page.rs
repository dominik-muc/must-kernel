#[repr(align(4096))]
pub struct PageTable {
    pub entries: [Entry; 512],
}

impl PageTable {
    pub fn map(self: &mut Self) {}
}

#[derive(Clone, Copy)]
pub struct Entry(usize);

impl Entry {
    pub fn new(paddr: usize) -> Self {
        Self(paddr)
    }
    pub fn flags(self, flags: impl Iterator<Item = Flag>) -> Self {
        Self(flags.fold(self.0, |acc, f| acc | f as usize))
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Flag {
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Accessed = 1 << 6,
    Dirty = 1 << 7,
}
