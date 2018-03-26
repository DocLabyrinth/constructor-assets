use image::{GenericImage, ImageBuffer, Pixel, Pixels, RGB};

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
//
// impl Clone for SpriteImage {
//     fn clone(&self) -> SpriteImage { *self }
// }

// impl GenericImage for SpriteImage {
//     /// The pixel type.
//     type Pixel: Pixel;
//
//     /// The width and height of this image.
//     fn dimensions(&self) -> (u32, u32) {
//         (self.width as u32, self.height as u32)
//     }
//
//     /// The bounding rectangle of this image.
//     fn bounds(&self) -> (u32, u32, u32, u32) {
//         (0, 0, self.width as u32, self.height as u32)
//     }
//
//     /// Return the pixel located at (x, y)
//     fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
//         Pixel::new(self.pixels[(y * x) + y])
//     }
//
//     fn get_mut_pixel(&mut Self, u32, u32) -> &mut <Self as GenericImage>::Pixel {}
//
//     /// Put a pixel at location (x, y)
//     fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
//         self.pixels[(y * x) + y] = pixel
//     }
//
//     /// Return an Iterator over the pixels of this image.
//     /// The iterator yields the coordinates of each pixel
//     /// along with their value
//     fn pixels(&self) -> Pixels<Self> {
//         self.pixels.map(|pixel| Pixel::new(pixel))
//     }
// }
