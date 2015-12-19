//! Library offering an API to query operating system specific paths.

#![warn(missing_docs)]

extern crate xdg;

#[cfg(all(unix, target_os = "macos"))]
mod osx;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(windows)]
mod windows;

mod error;

#[cfg(all(unix, target_os = "macos"))]
use osx as platform;
#[cfg(all(unix, not(target_os = "macos")))]
use unix as platform;
#[cfg(windows)]
use windows as platform;

use std::path::Path;
use std::path::PathBuf;

pub use error::Error;
pub use error::Result;

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
    /// On Windows, this is the `AppData\Roaming\Prefix` directory of the
    /// current user, corresponding to `FOLDERID_RoamingAppData`.
    ///
    /// On UNIX systems, this is determined by the XDG Base Directory
    /// specification, and can be set by the environment variable
    /// `XDG_CONFIG_HOME`. It defaults to `~/.config/prefix`.
    ///
    /// On OS X, this is `~/Library/Prefix`.
    pub fn config_home(&self) -> PathBuf {
        self.inner.config_home()
    }

    /// Returns the user-specific directory for cache files.
    ///
    /// On Windows, this is the `AppData\Local\Temp\Prefix` directory of the
    /// current user, obtained through `GetTempPath`.
    ///
    /// On UNIX systems, this is determined by the XDG Base Directory
    /// specification, and can be set by the environment variable
    /// `XDG_CACHE_DIR`. It defaults to `~/.cache/prefix`.
    ///
    /// On OS X, this is `~/Library/Caches/Prefix`.
    pub fn cache_home(&self) -> PathBuf {
        self.inner.cache_home()
    }

    /// Returns the user-specific directory for executables.
    ///
    /// On Windows, this is the `AppData\Local\Programs\Prefix` directory of
    /// the current user, corresponding to `FOLDERID_UserProgramFiles`.
    ///
    /// On UNIX systems, this is determined by the XDG Base Directory
    /// specification, and can be set by the environment variable
    /// `XDG_CACHE_DIR`.
    ///
    /// On OSX, this is `~/Library/Prefix/bin`.
    pub fn bin_home(&self) -> PathBuf {
        self.inner.bin_home()
    }
}
