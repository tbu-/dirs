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
        // This is a helper function that is called only when the `XDG`
        // variables are not found.
        let make_path_fallback = |fallback_path, prefix| {
            let home = env::home_dir().ok_or(DirsError::HomeMissing)?;
            let mut path = home;
            
            // Checks if the home path is empty.
            if path.as_os_str() == "" { 
                return Err(DirsError::HomeMissing); 
            }
            
            // Push some `path` into it if it's not.
            path.push(fallback_path);
            path.push(prefix);
            Ok(path)
        };

        let make_path = |var, fallback_path, prefix| 
            if let Some(xdg) = env::var_os(var) {
                // Convert the OsString($XDG, or `var`) into a PathBuf and
                // return.
                let mut path: PathBuf = convert::From::from(xdg);

                // Checks if the path is empty.
                if path.as_os_str() == "" { 
                    return Err(DirsError::HomeMissing); 
                }

                path.push(prefix);
                Ok(path)
            } else {
                make_path_fallback(fallback_path, prefix)
            };

        // Since there are no XDG variables for ~./local/bin, fallback.
        let bin = make_path_fallback(".local/bin", prefix.as_ref());
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