use std::io;
use std::string;
use std::str;
use image;

quick_error! {
    #[derive(Debug)]
    pub enum AssetError {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Utf8(err: str::Utf8Error) {
            from()
            from(err: string::FromUtf8Error) -> (err.utf8_error())
        }
        Image(err: image::ImageError) {
            from()
            description("image error")
            display("Image error: {}", err)
            cause(err)
        }
        Other(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }
    }
}
