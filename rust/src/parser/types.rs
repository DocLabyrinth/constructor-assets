#[derive(Debug)]
pub struct FilIndexItem {
    pub filename : String,
    pub offset : u64,
}

pub struct FilFileIndex {
    pub num_entries : u64,
    pub entries : Vec<FilIndexItem>,
}
