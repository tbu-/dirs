use std::error;
use std::fmt;
use std::result;

use self::ErrorKind::*;

/// Error that is returned when the operation system's interfaces cannot be
/// queried for the path information.
pub struct Error {
    kind: ErrorKind,
}

// Some variants might not be used depending on the operating system.
#[allow(dead_code)]
#[derive(Debug)]
enum ErrorKind {
    MissingHomeVariable,
    PlatformError(Box<error::Error+Send+Sync>),
}

// These can't be static methods on `Error` as we'd export them to users of
// this library otherwise. TODO: Maybe we could just do that.
#[allow(dead_code)]
pub fn missing_home() -> Error {
    Error {
        kind: MissingHomeVariable,
    }
}
#[allow(dead_code)]
pub fn from_error<T: error::Error+Send+Sync+'static>(cause: T) -> Error {
    fn impl_(cause: Box<error::Error+Send+Sync>) -> Error {
        Error {
            kind: PlatformError(cause),
        }
    }
    impl_(Box::new(cause))
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            MissingHomeVariable => "$HOME must be set",
            PlatformError(..) => "could not get configuration directories",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            PlatformError(ref e) => Some(&**e),
            MissingHomeVariable => None,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            MissingHomeVariable => write!(f, "{}", error::Error::description(self)),
            PlatformError(ref e) =>
                write!(f, "could not get configuration directories: {}", e),
        }
    }
}

/// A specialized `Result` type for this library.
pub type Result<T> = result::Result<T, Error>;
