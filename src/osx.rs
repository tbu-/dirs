use std::env;
use std::path::Path;
use std::path::PathBuf;

use Error;
use Result;

pub struct Directories {
    cache_home: PathBuf,
    config_home: PathBuf,
}

impl Directories {
    pub fn with_prefix(_prefix_lowercased: &Path, prefix_capitalized: &Path)
        -> Result<Directories>
    {
        let mut path = try!(env::home_dir().ok_or(Error::new()));
        path.push("Library");

        let mut cache_home = path.clone();
        cache_home.push("Caches");
        cache_home.push(prefix_capitalized);

        let mut config_home = path;
        config_home.push(prefix_capitalized);

        Ok(Directories {
            cache_home: cache_home,
            config_home: config_home,
        })
    }
    pub fn config_home(&self) -> PathBuf {
        self.config_home.clone()
    }
    pub fn config_dirs(&self) -> Vec<PathBuf> {
        vec![]
    }
    pub fn cache_home(&self) -> PathBuf {
        self.cache_home.clone()
    }
}
