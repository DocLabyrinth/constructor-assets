use extractor::types::error::AssetError;
use extractor::types::SpriteImage;
use image::{ImageBuffer,GenericImage};
use extractor::types::SpriteBuffer;

pub fn make_sprite_sheet(anim_sprites: Vec<SpriteImage>) -> Result<SpriteBuffer, AssetError> {
    let largest_width : u32 = match anim_sprites
        .iter()
        .max_by_key(|sprite| sprite.width) {
            None => {
                return Err(AssetError::Other("Sprite iterator passed to make_sprite_sheet was empty"))
            }
            Some(sprite) => sprite.width as u32
        };

    let largest_height : u32 = match anim_sprites
        .iter()
        .max_by_key(|sprite| sprite.height) {
            None => {
                return Err(AssetError::Other("Sprite iterator passed to make_sprite_sheet was empty"))
            }
            Some(sprite) => sprite.height as u32
        };


    // println!("largest sprite - h:{}, w:{}", largest_width, largest_height);

    let mut sheet_buf = ImageBuffer::new(
        largest_width * anim_sprites.len() as u32,
        largest_height
    );

    // println!("This many sprites to go in the sheet: {}", anim_sprites.len());
    for (anim_idx, anim_sprite) in anim_sprites.iter().enumerate() {
        let base_x = anim_idx as u32 * largest_width;
        let center_x = base_x + (largest_width / 2);

        sheet_buf.copy_from(
            &anim_sprite.image_buf,
            center_x - anim_sprite.offset_h.abs() as u32,
            largest_height - anim_sprite.height as u32
        );
    }

    return Ok(sheet_buf);
}
