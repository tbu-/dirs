use std::env;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows::api;

use error::{ DirsError, Result };

pub enum KnownFolder {
    RoamingAppData,
    LocalAppData,
    UserProgramFiles,
    ProgramData,
    Temp,
}

impl Into<api::GUID> for KnownFolder {
    fn into(self) -> api::GUID {
        use self::KnownFolder::*;
        match self {
            RoamingAppData => api::GUID {
                Data1: 1052149211,
                Data2: 26105,
                Data3: 19702,
                Data4: [160, 58, 227, 239, 101, 114, 159, 61],
            },

            LocalAppData => api::GUID {
                Data1: 4055050117,
                Data2: 28602,
                Data3: 20431,
                Data4: [157, 85, 123, 142, 127, 21, 112, 145],
            },

            UserProgramFiles => api::GUID {
                Data1: 1557638882,
                Data2: 8729,
                Data3: 19047,
                Data4: [184, 93, 108, 156, 225, 86, 96, 203],
            },

            ProgramData => api::GUID {
                Data1: 1655397762,
                Data2: 64961,
                Data3: 19907,
                Data4: [169, 221, 7, 13, 29, 73, 93, 151],
            },

            // We leave Temp as all-zeroes, and it won't change any behaviours
            // even if value is changed anyways, as Temp is a special case in
            // get_known_folder_path().
            Temp => api::GUID {
                Data1: 0,
                Data2: 0,
                Data3: 0,
                Data4: [0, 0, 0, 0, 0, 0, 0, 0],
            },
        }
    }
}

pub fn get_known_folder_path(folder: KnownFolder) -> Result<PathBuf> {
    match folder {
        KnownFolder::Temp => Ok(env::temp_dir()),
        other => unsafe {
            // Converts `folder` into KNOWNFOLDERID, which is just an alias
            // of `GUID`.
            let rfid: api::KNOWNFOLDERID = Into::into(other);
            
            // The magical 0 means NULL here.
            let mut result = 0 as api::PWSTR;

            // Gets the path.
            let hresult = api::shlobj::SHGetKnownFolderPath(
                &rfid as api::REFKNOWNFOLDERID,
                0u32,
                0 as api::HANDLE,
                &mut result
            );

            if hresult != api::winerror::S_OK {
                return Err(DirsError::PlatformError(
                    format!("SHGetKnownFolderPath returned 0x{:x}", hresult)
                ))
            }

            // Read the string into a Vec<u16>.
            // The capacity 130 is chosen because it is the half of MAX_PATH.
            // (As a note: `const MAX_PATH: usize = 260` )
            let mut path = Vec::with_capacity(130); {
            let mut current = result; 
                while *current != 0 {
                    path.push(*current);
                    current = current.offset(1);
                }
            };
            
            // Convert the u16 to an OsString, and then to a PathBuf.
            let path = OsString::from_wide(path.as_slice());
            let path: PathBuf = From::from(path);

            // Drop the WSTR after all of this, and finally return the path.
            api::combaseapi::CoTaskMemFree(
                result as api::LPVOID
            );

            Ok(path)
        },
    }
}
