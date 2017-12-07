use std::convert;
use std::env;
use std::path::{ Path, PathBuf };

use error::{ Result, DirsError };

pub struct Directories {
    cache_home: PathBuf,
    config_home: PathBuf,
    data_home: PathBuf,
}

impl Directories {
    pub fn with_prefix<P>(prefix: P) -> Result<Directories>
        where P: AsRef<Path>
    {
        let make_path = |var, fallback_path, prefix| 
            if let Some(xdg) = env::var_os(var) {

                // Convert the OsString($XDG, or `var`) into a PathBuf and
                // return.
                let mut path: PathBuf = convert::From::from(xdg);
                path.push(prefix);
                Ok(path)

            } else if let Some(home) = env::home_dir() {
                
                // Tries to get the HOME path, and push some `path` into it.
                let mut path = home;
                path.push(fallback_path);
                path.push(prefix);
                Ok(path)

            } else {

                // At this point, we just give up and return an Err.
                Err(DirsError::VariableMissing(String::from("$HOME is not set.")))

            };
        
        let cache = make_path("$XDG_CACHE_HOME", ".cache", prefix.as_ref());
        let config = make_path("$XDG_CONFIG_HOME", ".config", prefix.as_ref());
        let data = make_path("$XDG_DATA_HOME", ".local/share", prefix.as_ref());

        Ok(Directories {
            cache_home: cache?,
            config_home: config?,
            data_home: data?,
        })
    }

    pub fn cache_home(&self) -> &Path {
        &self.cache_home
    }

    pub fn config_home(&self) -> &Path {
        &self.config_home
    }

    pub fn data_home(&self) -> &Path {
        &self.data_home
    }
}