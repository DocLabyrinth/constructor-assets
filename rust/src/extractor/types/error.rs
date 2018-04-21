use std::io;
use std::string;
use std::str;

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
        Other(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }
    }
}
