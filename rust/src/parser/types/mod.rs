#[derive(Debug)]
pub struct FilIndexItem {
    pub filename : String,
    pub offset : u64,
}

pub struct FilFileIndex {
    pub num_entries : u64,
    pub entries : Vec<FilIndexItem>,
}

mod error;
mod sprite_image;

pub use self::sprite_image::SpriteImage;
pub use self::error::AssetError;
