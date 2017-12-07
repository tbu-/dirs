extern crate ole32;
extern crate shell32;
extern crate winapi;

use std::env;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use self::winapi as api;

use error::{ DirsError, Result };

pub enum KnownFolder {
    RoamingAppData,
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
            let hresult = shell32::SHGetKnownFolderPath(
                &rfid as api::REFKNOWNFOLDERID,
                0u32,
                0 as api::HANDLE,
                &mut result
            );

            if hresult != api::S_OK {
                return Err(DirsError::PlatformError(
                    format!("SHGetKnownFolderPath returned {}", hresult)
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
            ole32::CoTaskMemFree(
                result as api::LPVOID
            );

            Ok(path)
        },
    }
}

/*
pub struct String {
    value: winapi::PWSTR,
    len: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SHGetKnownFolderPathError(winapi::HRESULT);

impl std::error::Error for SHGetKnownFolderPathError {
    fn description(&self) -> &str {
        "SHGetKnownFolderPath returned an error"
    }
}

impl fmt::Display for SHGetKnownFolderPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let SHGetKnownFolderPathError(code) = *self;
        write!(f, "{} ({:#010x})", std::error::Error::description(self), code)
    }
}

impl String {
    unsafe fn new(value: winapi::PWSTR) -> String {
        unsafe fn len(s: winapi::PWSTR) -> usize {
            let mut result = 0;
            let mut cur = s;
            while *cur != 0 {
                result += 1;
                cur = cur.offset(1);
            }
            result
        }
        String {
            value: value,
            len: len(value),
        }
    }
    fn as_slice(&self) -> &[u16] {
        unsafe {
            slice::from_raw_parts(self.value, self.len)
        }
    }
    fn to_os_string(&self) -> OsString {
        use std::os::windows::ffi::OsStringExt;
        OsStringExt::from_wide(self.as_slice())
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            ole32::CoTaskMemFree(self.value as *mut _);
        }
    }
}

#[allow(non_snake_case)]
fn SHGetKnownFolderPath(rfid: &winapi::KNOWNFOLDERID,
                        flags: winapi::DWORD,
                        token: winapi::HANDLE)
    -> Result<String, SHGetKnownFolderPathError>
{
    let mut result: winapi::PWSTR = ptr::null_mut();
    let error;
    unsafe {
        error = shell32::SHGetKnownFolderPath(rfid, flags, token, &mut result);
    }
    if error != winapi::S_OK {
        return Err(SHGetKnownFolderPathError(error));
    }
    Ok(unsafe { String::new(result) })
}

fn translate(known_folder: KnownFolder) -> Option<&'static winapi::KNOWNFOLDERID> {
    Some(match known_folder {
        KnownFolder::RoamingAppData => &uuid::FOLDERID_RoamingAppData,
        KnownFolder::UserProgramFiles => &uuid::FOLDERID_UserProgramFiles,
        KnownFolder::Temp => return None,
    })
}

pub fn known_folder_path(known_folder: KnownFolder) -> Result<PathBuf, Error> {
    translate(known_folder).map(|id| {
        SHGetKnownFolderPath(id, 0, ptr::null_mut())
            .map(|s| PathBuf::from(s.to_os_string()))
            .map_err(|e| error::from_error(e))
    }).unwrap_or_else(|| {
        // KnownFolder::Temp
        Ok(env::temp_dir())
    })
}
*/