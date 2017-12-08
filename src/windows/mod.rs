use std::path::Path;
use std::path::PathBuf;

use error::Result;
use self::sys::KnownFolder;

mod sys;

pub struct Directories {
    bin_home: PathBuf,
    cache_home: PathBuf,
    config_home: PathBuf,
    data_home: PathBuf,
}

impl Directories {
    pub fn with_prefix(_prefix_lowercased: &Path, prefix_capitalized: &Path)
        -> Result<Directories>
    {
        let get = |k| -> Result<PathBuf> {
            let mut buf = sys::get_known_folder_path(k)?;
            buf.push(prefix_capitalized);
            Ok(buf)
        };

        let cache_home = get(KnownFolder::Temp)?;
        let config_home = get(KnownFolder::RoamingAppData)?;
        let bin_home = get(KnownFolder::UserProgramFiles)?;
        let data_home = get(KnownFolder::LocalAppData)?;

        Ok(Directories {
            cache_home: cache_home,
            config_home: config_home,
            bin_home: bin_home,
            data_home: data_home,
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
