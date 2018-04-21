use clap;

use std::fs::File;
use std::path::Path;
use image;

use extractor::lev::extract_color_palette;

use extractor::spr::parser::parse_sprite_file;
use extractor::spr::sheet::make_sprite_sheet;
use extractor::types::sprite_image::SpriteImage;

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

        // for (sprite_idx, sprite) in sprites.iter().enumerate() {
        //     let filename = format!("{}.png", sprite_idx);
        //     let target_path = output_path.join(filename);
        //     let mut output_file = File::create(target_path).expect("Failed to create sprite file");
        //     image::ImageRgba8(sprite.image_buf.clone()).save(&mut output_file, image::PNG).unwrap();
        // }

        let worker_frames = vec![0,2,4,6];
        // let splatter_anim = &sprites[1499..1508];
        let splatter_anim : Vec<SpriteImage> = worker_frames
            .iter()
            .map(|frame| sprites[*frame as usize].clone())
            .collect();
        let sheet_buf = make_sprite_sheet(splatter_anim.to_vec()).expect("Failed to create sprite sheet");
        let target_path = output_path.join("sprite_sheet.png");
        let mut output_file = File::create(target_path).expect("Failed to create sprite file");
        image::ImageRgba8(sheet_buf.clone()).save(&mut output_file, image::PNG).unwrap();
    }
}
