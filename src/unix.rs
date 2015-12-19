extern crate xdg;

use std::env;
use std::path::Path;
use std::path::PathBuf;

use Result;
use error;

pub struct Directories {
    xdg: xdg::BaseDirectories,
    bin_home: PathBuf,
}

impl Directories {
    pub fn with_prefix(prefix_lowercased: &Path, _prefix_capitalized: &Path)
        -> Result<Directories>
    {
        let home = try!(env::home_dir().ok_or(error::missing_home()));
        let mut bin_home = home;
        bin_home.push(".local");
        bin_home.push("bin");
        let xdg = try!(xdg::BaseDirectories::with_prefix(prefix_lowercased)
                       .map_err(|e| error::from_error(e)));
        Ok(Directories {
            xdg: xdg,
            bin_home: bin_home,
        })
    }
    pub fn config_home(&self) -> PathBuf {
        self.xdg.get_config_home()
    }
    pub fn cache_home(&self) -> PathBuf {
        self.xdg.get_cache_home()
    }
    pub fn bin_home(&self) -> PathBuf {
        self.bin_home.clone()
    }
}
