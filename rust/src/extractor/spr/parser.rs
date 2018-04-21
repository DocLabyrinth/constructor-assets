use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Error, SeekFrom, Seek};
use buffer::ReadBuffer;

use image;
use image::{ImageBuffer};
use extractor::types::SpriteImage;
use nom::{IResult,le_u16,le_i16,le_u8};
use byteorder::{LittleEndian, ReadBytesExt};

pub fn parse_sprite_file<'a>(file_path_str: &'a str, palette: &'a Vec<image::Rgba<u8>>) -> Result<Vec<SpriteImage>, Error> {
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

     let mut sprite_vec = Vec::<SpriteImage>::with_capacity(num_entries as usize);

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

        match parse_sprite(&mut read_vec, &palette) {
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

 fn flatten_pixel_vector(raw_pixels: Vec<Vec<u8>>) -> Vec<u8> {
     raw_pixels
        .iter()
        .fold(Vec::<u8>::new(), |mut acc, bytes| {
            acc.extend_from_slice(bytes);
            acc
        })
 }

 fn parse_sprite<'a>(buffer: &'a [u8], palette: &Vec<image::Rgba<u8>>) -> IResult<&'a [u8], SpriteImage> {
    // TODO: this is throwing a warning about an unused variable called e
    // this most likely comes from using map!() instead of map_res!()
    do_parse!(
        buffer,
        offset_h: le_i16 >>
        offset_v: le_i16 >>
        width: le_u16 >>
        height: le_u16 >>
        raw_pixels: map!(many0!(
            switch!(
                peek!(le_u8),
                0 => map!(take!(2), |bytes| vec![0; bytes[1] as usize]) |
                _ => map!(take!(1), |bytes| bytes.to_vec())
             )
        ), flatten_pixel_vector) >>
        ({
            let mut image_buf = ImageBuffer::new(width as u32, height as u32);
            let mut pixel_iter = raw_pixels.iter();

            for (_x, _y, pixel) in image_buf.enumerate_pixels_mut() {
                match pixel_iter.next() {
                    Some(color_code) => {
                        *pixel = palette[*color_code as usize].clone()
                    }
                    _ => { continue; }
                }
            }

            SpriteImage { offset_h, offset_v, width, height, image_buf }
        })
    )
}
