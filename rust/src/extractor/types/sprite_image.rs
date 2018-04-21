use image::{ImageBuffer, Rgba};

#[derive(Debug, Clone)]
pub struct SpriteImage {
    // I'm pretty sure the first two numbers are all negative offsets,
    // indicating how far from a specific point to draw the image.
    // I still haven't determined what they are relative to, possibly
    // the position of the base object on the screen
    pub offset_h : i16,
    pub offset_v : i16,
    pub width : u16,
    pub height : u16,
    pub image_buf : ImageBuffer<Rgba<u8>, Vec<u8>>,
}
