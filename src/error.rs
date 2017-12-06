/// A specialized `Result` type for this library.
pub type Result<T> = std::result::Result<T, DirsError>;

pub enum DirsError {
    VariableMissing(String),
    IoError(io::Error),
}
