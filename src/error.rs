use std::error::Error as ErrorTrait;
use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::result::Result as RustResult;
use std::string::FromUtf16Error;
use std::io;

/// A specialized `Result` type for this library.
pub type Result<T> = RustResult<T, DirsError>;

/// Error that is returned when the operation system's interfaces cannot be
/// queried for the path information.
#[derive(Debug)]
pub enum DirsError {
    /// This error occurs when the $HOME variable is not set.
    HomeMissing,

    /// This error occurs when the user don't have required permission for
    /// the file/directory.
    IoError(io::Error), 

    /// This error occurs when the Unicode string is invalid and cannot be
    /// parsed.
    ParseError(String),

    /// This error occurs when there are platform-specific errors such as
    /// Windows API related errors.
    PlatformError(String),
}

impl From<FromUtf16Error> for DirsError {
    fn from(_: FromUtf16Error) -> Self {
        // Rust lacks some detailed error messages on Utf16...
        // So I decided to simply... print an error message and call it a day.
        // <.<
        DirsError::ParseError(String::from("invalid utf-16"))
    }
}

use self::DirsError::*;
impl ErrorTrait for DirsError {
    fn description(&self) -> &str {
        match self {
            &HomeMissing => "unable to obtain the home directory",
            &IoError(ref err) => err.description(),
            &ParseError(ref why) => why,
            &PlatformError(ref why) => why,
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        match self {
            &IoError(ref why) => Some(why),
            _ => None
        }
    }
}

impl Display for DirsError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &HomeMissing => write!(f, "{}", self.description()),
            &IoError(ref err) => err.fmt(f),
            &ParseError(ref why) => write!(f, "{}", why),
            &PlatformError(ref why) => write!(f, "{}", why),
        }
    }
}