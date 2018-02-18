#[derive(Debug)]
pub struct FilIndexItem {
    pub filename : String,
    pub offset : u64,
}

pub struct FilFileIndex {
    pub num_entries : u64,
    pub entries : Vec<FilIndexItem>,
}

#[derive(Debug)]
pub struct SpriteImage {
    // I suspect that chunk1 and chunk2 indicate sequences or related sprites
    // but I've not been able to confirm this yet
    pub chunk1 : u16,
    pub chunk2 : u16,
    pub width : u16,
    pub height : u16,
    pub pixels : Vec<u8>,
}
