extern crate clap;
use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::path::Path;

// #[macro_use]
extern crate nom;
extern crate byteorder;

mod parser;

fn main() {
    let matches = App::new("Constructor Assets Extractor")
                      .version("0.1")
                      .about("Extracts assets from Constructor, a classic strategy game from 1997")
                      .subcommand(SubCommand::with_name("fil")
                                  .about("handles raw .FIL files")
                                  .arg(Arg::with_name("action")
                                    .index(1)
                                    .required(true)
                                    .help("inspect, extract")
                                  )
                                  .arg(Arg::with_name("file")
                                    .short("f")
                                    .long("file")
                                    .required(true)
                                    .takes_value(true)
                                    .help("path to the target .FIL file")
                                  )
                                  .arg(Arg::with_name("output-dir")
                                    .short("d")
                                    .long("output-dir")
                                    // .required(true)
                                    .takes_value(true)
                                    .help("path to a directory to extract the files into")
                                  )
                      )
                      .get_matches();

    if matches.is_present("fil") {
        let fil_matches =  matches.subcommand_matches("fil").unwrap();
        let action = fil_matches.value_of("action").unwrap();
        let file_path = Path::new(fil_matches.value_of("file").unwrap());

        let mut file = match File::open(&file_path) {
             // The `description` method of `io::Error` returns a string that
             // describes the error
             Err(why) => panic!("couldn't open input file {}", why),
             Ok(file) => file,
         };

         let fil_index = parser::fil::load_index(&mut file).unwrap();


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

             if !output_path.exists() && output_path.is_dir() {
                 print!("{:?} does not exist or is not a directory", output_path.to_str());
             }

             match parser::fil::extract_files(&mut file, &fil_index, output_path) {
                 Ok(num) => print!("Extracted this many files: {:?}", num),
                 Err(err) => panic!("Failed to extract one or more files: {}", err)
             };
         }
    }

}
