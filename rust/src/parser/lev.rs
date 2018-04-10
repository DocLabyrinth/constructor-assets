use std::string::String;
use std::io::{Read, SeekFrom, Seek};
use std::fs::File;
use std::path::Path;
use image::{ColorType, Pixel, Rgba};
use buffer::ReadBuffer;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};

use parser::types::AssetError;

pub fn extract_color_palette(file_path_str: &str) -> Result<Vec<Rgba<u8>>, AssetError> {
    let file_path = Path::new(file_path_str);
    let mut file = try!(File::open(&file_path));
    try!(file.seek(SeekFrom::Start(12)));

    for _chunk_idx in 0..6 {
        let mut chunk_id_raw = [0; 4];
        try!(file.read_exact(&mut chunk_id_raw));
        let chunk_id = try!(String::from_utf8(chunk_id_raw.to_vec()));
        let chunk_size = try!(file.read_u32::<BigEndian>());
        let mut body_buf = Vec::with_capacity(chunk_size as usize);
        try!(file.read_buffer(&mut body_buf));

        if chunk_id == "CMAP" {
            return Ok(
                body_buf
                    .chunks(3)
                    .map(|chunks| {
                        let alpha =
                            if chunks[0] == 0u8 && chunks[1] == 0u8 && chunks[2] == 0u8 { 0 } else { 255 };
                        Rgba::from_channels(chunks[0], chunks[1], chunks[2], alpha as u8)
                    })
                    .collect()
            )
        }
    }

    Err(AssetError::Other("CMAP chunk missing"))
}
