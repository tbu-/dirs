extern crate xdg;

use std::env;
use std::path::Path;
use std::path::PathBuf;

use Error;
use Result;

pub struct Directories {
    xdg: xdg::BaseDirectories,
}

impl Directories {
    pub fn with_prefix(prefix_lowercased: &Path, _prefix_capitalized: &Path)
        -> Result<Directories>
    {
        // FIXME: Classic TOCTOU.
        let _ = try!(env::home_dir().ok_or(Error::new()));
        Ok(Directories {
            xdg: xdg::BaseDirectories::with_prefix(prefix_lowercased)
        })
    }
    pub fn config_home(&self) -> PathBuf {
        self.xdg.get_config_home()
    }
    pub fn config_dirs(&self) -> Vec<PathBuf> {
        self.xdg.get_config_dirs()
    }
    pub fn cache_home(&self) -> PathBuf {
        self.xdg.get_cache_home()
    }
}
