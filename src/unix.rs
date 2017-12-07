use std::convert;
use std::env;
use std::path::{ Path, PathBuf };

use error::{ Result, DirsError };

pub struct Directories {
    bin_home: PathBuf,
    cache_home: PathBuf,
    config_home: PathBuf,
    data_home: PathBuf,
}

impl Directories {
    pub fn with_prefix<P>(prefix: P, _: P) -> Result<Directories>
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
                let mut path = home;
                
                // Checks if the home path is empty.
                if path.as_os_str() == "" { 
                    return Err(DirsError::HomeMissing); 
                }
                
                // Push some `path` into it if it's not.
                path.push(fallback_path);
                path.push(prefix);
                Ok(path)

            } else {

                // At this point, we just give up and return an Err.
                Err(DirsError::HomeMissing)

            };
        
        // The "=" is not allowed in variable names, so it is guaranteed to be
        // empty, and therefore, force `make_path` to fallback.
        let bin = make_path("=", ".local/bin", prefix.as_ref());
        let cache = make_path("XDG_CACHE_HOME", ".cache", prefix.as_ref());
        let config = make_path("XDG_CONFIG_HOME", ".config", prefix.as_ref());
        let data = make_path("XDG_DATA_HOME", ".local/share", prefix.as_ref());

        Ok(Directories {
            bin_home: bin?,
            cache_home: cache?,
            config_home: config?,
            data_home: data?,
        })
    }

    pub fn bin_home(&self) -> &Path {
        &self.bin_home
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