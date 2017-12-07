use std::env;
use std::path::Path;
use std::path::PathBuf;

use error::{ Result, DirsError };

pub struct Directories {
    bin_home: PathBuf,
    cache_home: PathBuf,
    config_home: PathBuf,
    data_home: PathBuf,
}

impl Directories {
    pub fn with_prefix(prefix: &Path) -> Result<Directories>
    {
        // I think it is recommended to use OSX's API to find the path, but I
        // don't know much about the OSX environment to confirm this.
        let mut path = env::home_dir().ok_or(DirsError::HomeMissing)?;
        path.push("Library");

        let mut cache_home = path.clone();
        cache_home.push("Caches");
        cache_home.push(prefix);

        let mut config_home = path;
        config_home.push(prefix);

        let mut bin_home = config_home.clone();
        bin_home.push("bin");

        let mut data_home = path.clone();
        path.push("Application Support");
        path.push(prefix);

        Ok(Directories {
            bin_home: bin_home,
            cache_home: cache_home,
            config_home: config_home,
            data_home: data_home,
        })
    }

    pub fn config_home(&self) -> &Path {
        &self.config_home
    }
    pub fn cache_home(&self) -> &Path {
        &self.cache_home
    }
    pub fn bin_home(&self) -> &Path {
        &self.bin_home
    }
    pub fn data_home(&self) -> &Path {
        &self.data_home
    }
}
