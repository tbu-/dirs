use std::result::Result as RustResult;
use std::io;

/// A specialized `Result` type for this library.
pub type Result<T> = RustResult<T, DirsError>;

pub enum DirsError {
    VariableMissing(String),
    IoError(io::Error),
}
