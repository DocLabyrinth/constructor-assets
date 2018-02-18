use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use std::result::{Result};
use std::io;
use std::io::{Error, Read, SeekFrom, Seek};
use std::fs::File;
use std::path::Path;

use parser::types::{FilFileIndex, FilIndexItem};
use clap;


const ENTRIES_DECRYPTION_VALUE : u32 = 0x3BD7A59A;

pub fn handle_cli_args(matches: &clap::ArgMatches) {
    let fil_matches =  matches.subcommand_matches("fil").unwrap();
    let action = fil_matches.value_of("action").unwrap();
    let file_path = Path::new(fil_matches.value_of("file").unwrap());

    let mut file = match File::open(&file_path) {
         // The `description` method of `io::Error` returns a string that
         // describes the error
         Ok(file) => file,
         Err(err) => {
             println!("Couldn't open input file {}", err);
             return;
         },
     };

     let fil_index = match load_index(&mut file) {
         Ok(file) => file,
         Err(err) => {
             println!("Failed to load the file index: {}", err);
             return;
         }
     };

     if action == "inspect" {
         println!("{:?} contains this many entries: {:?}", file_path, fil_index.num_entries);

         for file_idx in 0..fil_index.num_entries - 1 {
             let current_filename = &fil_index.entries[file_idx as usize].filename;
             let current_offset = &fil_index.entries[file_idx as usize].offset;

             let next_offset = &fil_index.entries[file_idx as usize + 1].offset;
             let file_size = next_offset - current_offset;

             println!("Filename: {:?}, Size: {:?}kb", current_filename, file_size / 1024);
         }
     }
     else if action == "extract" {
         let output_path = Path::new(fil_matches.value_of("output-dir").unwrap());

         if !output_path.exists() || !output_path.is_dir() {
             println!("{:?} does not exist or is not a directory", output_path.to_str());
             return;
         }

         match extract_files(&mut file, &fil_index, output_path) {
             Ok(num) => print!("Extracted this many files: {:?}", num),
             Err(err) => {
                 println!("Failed to extract one or more files: {}", err);
                 return;
             }
         };
     }
}

pub fn parse_index_entry(entry_buf: &mut [u8], offset: u64) -> FilIndexItem {
    // This is the code which undoes the obfuscation on the
    // indexes of the files. It's really horrific but it works, somehow...
    // I tried  to stick to the same integer types but it
    // kept getting errors where it went below 0 on an unsigned
    // integer type or went over the allowed size for the type.
    // There's almost certainly a better way of doing this, but
    // I couldn't figure it out :(

    for byte_idx in 0usize..17usize {
        let mut current_byte : i32;
        let bump_value = byte_idx as u64 + (offset * 17);

        current_byte = entry_buf[byte_idx] as i32;
        current_byte -= 39i32;
        current_byte ^= 0xA5i32;
        current_byte -= 27i32 + bump_value as i32;
        entry_buf[byte_idx] = current_byte as u8;
    }

    let mut filename_raw = entry_buf[0..12].to_vec();
    filename_raw.retain(|c| *c as char != '\0');
    let filename = match String::from_utf8(filename_raw) {
        Ok(name) => name,
        Err(_) => "parsing failed for this entry".to_owned(),
    };

    FilIndexItem {
        filename: filename,
        offset: u64::from(LittleEndian::read_u32(&entry_buf[13..]))
    }
}

pub fn load_index(file: &mut File) -> Result<FilFileIndex, Error> {
    let raw_num_entries = match file.read_u32::<LittleEndian>() {
        Ok(num) => num ^ ENTRIES_DECRYPTION_VALUE,
        Err(e) => return Err(e)
    };

    let mut fil_index = FilFileIndex {
        num_entries: u64::from(raw_num_entries),
        entries: Vec::with_capacity(raw_num_entries as usize),
    };

    let mut index_buf = [0; 17];

    for entry_idx in 0..(fil_index.num_entries) {
        let index_entry = match file.read(&mut index_buf) {
            Err(err) => return Err(err),
            Ok(_) => parse_index_entry(&mut index_buf, entry_idx),
        };
        fil_index.entries.push(index_entry);
    }

    Ok(fil_index)
}

pub fn extract_files(file: &mut File, file_index: &FilFileIndex, output_dir: &Path) -> Result<u64, Error> {
    for file_idx in 0..file_index.num_entries {
        let current_filename = &file_index.entries[file_idx as usize].filename;
        let current_offset = &file_index.entries[file_idx as usize].offset;

        if current_filename == "" {
            break;
        }

        let target_path = output_dir.join(current_filename);
        let mut output_file = try!(File::create(target_path));

        let next_offset = &file_index.entries[file_idx as usize + 1].offset;
        let bytes_to_read = next_offset - current_offset;

        try!(file.seek(SeekFrom::Start(*current_offset)));
        try!(io::copy(&mut file.take(bytes_to_read), &mut output_file));
    }

    Ok(file_index.num_entries)
}
