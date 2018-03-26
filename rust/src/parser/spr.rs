use clap;
use byteorder::{LittleEndian, ReadBytesExt};

use std::collections::HashMap;
use buffer::ReadBuffer;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Error,Seek,SeekFrom};
use nom::{IResult,le_u16,le_u8};

use parser::types::SpriteImage;
use image::{GenericImage, ImageBuffer, RGB};

pub fn handle_cli_args(matches: &clap::ArgMatches) {
    let spr_matches = matches.subcommand_matches("spr").unwrap();
    let action = spr_matches.value_of("action").unwrap();

    let file_path_str = spr_matches.value_of("file").unwrap();

    let sprites = match parse_sprite_file(file_path_str) {
        Ok(spr) => spr,
        Err(err) => {
            println!("Failed to parse sprite file: {}", err);
            return;
        }
    };

    let mut index : HashMap<String,Vec<SpriteImage>> = HashMap::new();
    for sprite in sprites {
        // println!("{},{}  w:{}, h: {}", sprite.chunk1, sprite.chunk2, sprite.width, sprite.height);
        let sprite_key = format!("{},{}", sprite.chunk1, sprite.chunk2);
        index.entry(sprite_key).or_insert(Vec::new()).push(sprite);
    }

    if action == "inspect" {
        for (sprite_key, sprites) in index.iter() {
            println!("sprite: {:?}, frames: {}", sprite_key, sprites.len());
        }
    } else if action == "extract" {

    }
}

fn load_color_palette(file_path_str: &str) -> Result<Vec<u8>, Error> {
    let mut palette = Vec::with_capacity(256);
    let file_path = Path::new(file_path_str);
    let mut file = try!(File::open(&file_path));
    file.read_buffer(&mut palette);
    Ok(palette.iter().map(|color| RGB(*color)).collect())
}

fn parse_sprite_file(file_path_str: &str) -> Result<Vec<SpriteImage>, Error> {
    let file_metadata = try!(fs::metadata(file_path_str));

    let file_size = file_metadata.len();
    let file_path = Path::new(file_path_str);

    let mut file = try!(File::open(&file_path));

    let num_entries = try!(file.read_u32::<LittleEndian>());
    let mut offsets = vec![0; num_entries as usize];

    offsets.push(0);

    for _ in 0..num_entries {
         let offset = try!(file.read_u32::<LittleEndian>());
         // an offset of 0 means no sprite for that entry
         // (why are these even present in the file??)
         if offset > 0 {
             offsets.push(offset)
         }
     }

     let mut sprite_vec = Vec::with_capacity(num_entries as usize);

     for (offset_idx, offset) in offsets.iter().enumerate() {
        let entry_length = if offset_idx < offsets.len() - 1 {
            (offsets[offset_idx+1] - offset) as u64
        } else {
            file_size
        };

        if entry_length < 1 {
            continue;
        }

        match file.seek(SeekFrom::Start(*offset as u64)) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to seek to offset in sprite file: {}", err);
                continue;
            }
        }

        let mut read_vec: Vec<u8> = Vec::with_capacity(entry_length as usize);

        match file.read_buffer(&mut read_vec) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to read from file: {}", err);
                continue;
            }
        };

        match parse_sprite(&mut read_vec, entry_length - 8) {
            IResult::Done(_, spr) => sprite_vec.push(spr),
            IResult::Incomplete(_) => {
                // we just read the amount of bytes specified by the
                // file index so something must be wrong
                println!("Failed to read sprite, missing bytes");
                continue;
            },
            IResult::Error(err) => {
                println!("Failed to read sprite, parsing error: {}", err);
                continue;
            }
        };
     }

     println!("Read this many sprites: {}", sprite_vec.len());

     sprite_vec.shrink_to_fit();
     Ok(sprite_vec)
 }

 fn parse_sprite(buffer: &[u8], pixel_length: u64) -> IResult<&[u8], SpriteImage> {
    do_parse!(
        buffer,
        chunk1: le_u16 >>
        chunk2: le_u16 >>
        width: le_u16 >>
        height: le_u16 >>
        // pixels: take!(pixel_length) >>
        pixels: many0!(
            switch!(
                peek!(le_u8),
                0 => map!(take!(2), |bytes| vec![0; bytes[1] as usize]) |
                _ => map!(take!(1), |bytes| bytes.to_vec())
             )
        ) >>
        (SpriteImage{
            chunk1: chunk1,
            chunk2: chunk2,
            width: width,
            height: height,
            pixels: pixels.iter().fold(Vec::<u8>::new(), |mut acc, pixels| {
                acc.extend_from_slice(&pixels);
                acc
            })
        })
    )
}
