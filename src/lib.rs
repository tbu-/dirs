//! Library offering an API to query operating system specific paths.

#![warn(missing_docs)]

extern crate xdg;

#[cfg(all(unix, target_os = "macos"))]
mod osx;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(all(unix, target_os = "macos"))]
use osx as platform;
#[cfg(all(unix, not(target_os = "macos")))]
use unix as platform;
#[cfg(windows)]
use windows as platform;

use std::error;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;


/// Error that is returned when the operation system's interfaces cannot be
/// queried for the path information.
#[derive(Debug)]
pub struct Error {
    _unused: (),
}

impl Error {
    fn new() -> Error {
        Error {
            _unused: (),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "error getting configuration directories"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

/// A specialized `Result` type for this library.
pub type Result<T> = std::result::Result<T, Error>;

/// The main type of this library. Create one via `Directories::with_prefix`
/// and use it to query operation system specific paths, such as configuration
/// directories.
pub struct Directories {
    inner: platform::Directories,
}

impl Directories {
    /// Creates a `Directories` object that can be queried for application
    /// specific paths for things like configuration files and cache locations.
    pub fn with_prefix<P, Q>(prefix_lowercased: P, prefix_capitalized: Q)
        -> Result<Directories>
        where P: AsRef<Path>, Q: AsRef<Path>
    {
        fn with_prefix_(prefix_lowercased: &Path, prefix_capitalized: &Path)
            -> Result<Directories>
        {
            Ok(Directories {
                inner: try!(platform::Directories::with_prefix(prefix_lowercased,
                                                               prefix_capitalized)),
            })
        }
        with_prefix_(prefix_lowercased.as_ref(), prefix_capitalized.as_ref())
    }

    /// Returns the user-specific directory for configuration files.
    ///
    /// On Windows, this is the `AppData\Roaming` directory of the current
    /// user.
    ///
    /// On UNIX systems, this is determined by the XDG Base Directory
    /// specification, and can be set by the environment variable
    /// `XDG_CONFIG_HOME`.
    pub fn config_home(&self) -> PathBuf {
        self.inner.config_home()
    }

    /// Returns additional directories for configuration files, ordered by
    /// preference.
    ///
    /// On UNIX systems, this is determined by the XDG Base Directory
    /// specification, and can be set by the environment variable
    /// `XDG_CONFIG_DIRS`.
    pub fn config_dirs(&self) -> Vec<PathBuf> {
        self.inner.config_dirs()
    }

    /// Returns the user-specific directory for cache files.
    ///
    /// On Windows, this is the `AppData\Local` directory of the current user.
    ///
    /// On UNIX systems, this is determined by the XDG Base Directory
    /// specification, and can be set by the environment variable
    /// `XDG_CACHE_DIR`.
    pub fn cache_home(&self) -> PathBuf {
        self.inner.cache_home()
    }
}
