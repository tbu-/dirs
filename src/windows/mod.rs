use std::path::Path;
use std::path::PathBuf;

use Error;
use Result;
use self::sys::KnownFolder;

mod sys;

pub struct Directories {
    cache_home: PathBuf,
    config_home: PathBuf,
    bin_home: PathBuf,
}

impl Directories {
    pub fn with_prefix(_prefix_lowercased: &Path, prefix_capitalized: &Path)
        -> Result<Directories>
    {
        let get = |k| {
            let mut buf = try!(sys::known_folder_path(k).map_err(|()| Error::new()));
            buf.push(prefix_capitalized);
            Ok(buf)
        };
        let cache_home = try!(get(KnownFolder::Temp));
        let config_home = try!(get(KnownFolder::RoamingAppData));
        let bin_home = try!(get(KnownFolder::UserProgramFiles));
        Ok(Directories {
            cache_home: cache_home,
            config_home: config_home,
            bin_home: bin_home,
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
