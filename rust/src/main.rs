extern crate clap;
use clap::{Arg, App, SubCommand};

#[macro_use] extern crate nom;
#[macro_use] extern crate maplit;
#[macro_use(quick_error)] extern crate quick_error;
extern crate byteorder;
extern crate buffer;
extern crate image;

mod extractor;

fn main() {
    let matches = App::new("Constructor Assets Extractor")
                      .version("0.1")
                      .about("Extracts assets from Constructor, a classic strategy game from 1997")
                      .subcommand(SubCommand::with_name("spr")
                          .about("handles .SPR sprite files")
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
                            .help("path to the target .SPR file")
                          )
                          .arg(Arg::with_name("level-file")
                            .short("p")
                            .long("palette-file")
                            .required(true)
                            .takes_value(true)
                            .help("path to a level .DAT file (to extract the color palette)")
                          )
                          .arg(Arg::with_name("output-dir")
                            .short("d")
                            .long("output-dir")
                            // .required(true)
                            .takes_value(true)
                            .help("path to a directory to extract the files into")
                          )
                      )
                      .subcommand(SubCommand::with_name("fil")
                          .about("handles raw .FIL files")
                          .arg(Arg::with_name("action")
                            .index(1)
                            .required(true)
                            .possible_values(&["inspect", "extract"])
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


    if matches.is_present("spr") {
        extractor::spr::cli::handle_cli_args(&matches)
    } else if matches.is_present("fil") {
        extractor::fil::handle_cli_args(&matches)
    }
}
