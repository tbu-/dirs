use std::error;
use std::result::Result as RustResult;
use std::string::FromUtf16Error;
use std::io;

/// A specialized `Result` type for this library.
pub type Result<T> = RustResult<T, DirsError>;

#[derive(Debug)]
pub enum DirsError {
    HomeMissing,
    ParseError(String),
    IoError(io::Error),
}

impl From<FromUtf16Error> for DirsError {
    fn from(err: FromUtf16Error) -> Self {
        // Rust lacks some detailed error messages on Utf16...
        // So I decided to simply... print an error message and call it a day.
        // <.<
        DirsError::ParseError(String::from("invalid utf-16"))
    }
}