use clap;

use std::fs::{File, create_dir_all};
use std::path::Path;
use image;

use extractor::lev::extract_color_palette;

use super::parser::parse_sprite_file;
use super::sheet::make_sprite_sheet;
use super::indexes::men;

use extractor::types::{SpriteBuffer,AssetError};

pub fn handle_cli_args(matches: &clap::ArgMatches) {
    let spr_matches = matches.subcommand_matches("spr").unwrap();
    let action = spr_matches.value_of("action").unwrap();

    let file_path_str = spr_matches.value_of("file").unwrap();
    let palette_path_str = spr_matches.value_of("level-file").unwrap();

    let color_palette = extract_color_palette(palette_path_str).unwrap();

    let sprites = match parse_sprite_file(file_path_str, &color_palette) {
        Ok(spr) => spr,
        Err(err) => {
            println!("Failed to parse sprite file: {}", err);
            return;
        }
    };

    if action == "inspect" {
        for (sprite_idx, sprite) in sprites.iter().enumerate() {
            println!(
                "index: {}, width: {:?}, height: {:?}, offset_h?: {}, offsetV?: {}",
                sprite_idx,
                sprite.width,
                sprite.height,
                sprite.offset_h,
                sprite.offset_v
            );
        }
    } else if action == "extract" {
        let output_path = match spr_matches.value_of("output-dir") {
            Some(path) => Path::new(path),
            None => {
                println!("An output directory is required to extract files");
                return;
            }
        };
        if !output_path.exists() || !output_path.is_dir() {
            println!("{:?} does not exist or is not a directory", output_path.to_str());
            return;
        }

        for (character_id, anims) in &men::get_index() {
            let character_path = output_path.join(character_id);
            create_dir_all(character_path.clone()).expect(
                &format!("Failed to create directory for sprite: {}", character_id)
            );
            for (anim_id, variations) in anims.iter() {
                for (variation_id, frame_idxs) in variations.iter() {
                    let target_path = character_path.join(format!("{}_{}", anim_id, variation_id));
                    let frames = frame_idxs
                        .iter()
                        .map(|frame_idx| sprites[*frame_idx as usize].clone())
                        .collect();
                    let variation_sheet = match make_sprite_sheet(frames) {
                        Ok(sheet) => sheet,
                        Err(err) => {
                            println!(
                                "Failed to create sprite sheet for: {}/{}_{} - {}",
                                character_id,
                                anim_id,
                                variation_id,
                                err
                            );
                            continue;
                        }
                    };

                    match target_path.to_str() {
                        Some(path) => { save_image(path, variation_sheet); },
                        None => {
                            println!("Invalid output path: {:?}", target_path);
                            return;
                        }
                    }
                }
            }
        }

        // let splatter_anim = &sprites[1499..1508];
        // let sheet_buf = make_sprite_sheet(splatter_anim.to_vec()).expect("Failed to create sprite sheet");
        // let target_path = output_path.join("sprite_sheet.png");
        // // let mut output_file = File::create(target_path).expect("Failed to create sprite file");
        // // image::ImageRgba8(sheet_buf.clone()).save(&mut output_file, image::PNG).unwrap();
        // if let Some(target_path_str) = target_path.to_str() {
        //     save_image(target_path_str, sheet_buf);
        // }
    }
}

fn save_image(file_path_str: & str, image: SpriteBuffer) -> Result<(), AssetError> {
    let mut output_file = try!(File::create(Path::new(file_path_str)));
    // match image::ImageRgba8(sheet_buf.clone()) {
    //     Ok(im) => {
    //         im.save(&mut output_file, image::PNG)
    //         Ok(true)
    //     },
    //     Err(err) => AssetError::Other(err)
    // };
    try!(image::ImageRgba8(image).save(&mut output_file, image::PNG));
    Ok(())
}
