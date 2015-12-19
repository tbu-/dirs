use std::env;
use std::path::Path;
use std::path::PathBuf;

use Result;
use error;

pub struct Directories {
    bin_home: PathBuf,
    cache_home: PathBuf,
    config_home: PathBuf,
}

impl Directories {
    pub fn with_prefix(_prefix_lowercased: &Path, prefix_capitalized: &Path)
        -> Result<Directories>
    {
        let mut path = try!(env::home_dir().ok_or(error::missing_home()));
        path.push("Library");

        let mut cache_home = path.clone();
        cache_home.push("Caches");
        cache_home.push(prefix_capitalized);

        let mut config_home = path;
        config_home.push(prefix_capitalized);

        let mut bin_home = config_home.clone();
        bin_home.push("bin");

        Ok(Directories {
            bin_home: bin_home,
            cache_home: cache_home,
            config_home: config_home,
        })
    }
    pub fn config_home(&self) -> PathBuf {
        self.config_home.clone()
    }
    pub fn cache_home(&self) -> PathBuf {
        self.cache_home.clone()
    }
    pub fn bin_home(&self) -> PathBuf {
        self.bin_home.clone()
    }
}
