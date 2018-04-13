use image::{ImageBuffer, Rgba};

#[derive(Debug, Clone)]
pub struct SpriteImage {
    // I suspect that chunk1 and chunk2 indicate sequences or related sprites
    // but I've not been able to confirm this yet
    pub chunk1 : u16,
    pub chunk2 : u16,
    pub width : u16,
    pub height : u16,
    pub image_buf : ImageBuffer<Rgba<u8>, Vec<u8>>,
}
