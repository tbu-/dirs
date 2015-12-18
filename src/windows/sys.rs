extern crate ole32;
extern crate shell32;
extern crate uuid;
extern crate winapi;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::ptr;
use std::slice;

#[derive(Clone, Copy, Debug)]
pub enum KnownFolder {
    RoamingAppData,
    UserProgramFiles,
    Temp,
}

pub struct String {
    value: winapi::PWSTR,
    len: usize,
}
pub struct Error(winapi::HRESULT);

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
    -> Result<String, Error>
{
    let mut result: winapi::PWSTR = ptr::null_mut();
    let error;
    unsafe {
        error = shell32::SHGetKnownFolderPath(rfid, flags, token, &mut result);
    }
    if error != winapi::S_OK {
        return Err(Error(error));
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

pub fn known_folder_path(known_folder: KnownFolder) -> Result<PathBuf, ()> {
    translate(known_folder).map(|id| {
        SHGetKnownFolderPath(id, 0, ptr::null_mut())
            .map(|s| PathBuf::from(s.to_os_string()))
            .map_err(|_| ())
    }).unwrap_or_else(|| {
        // KnownFolder::Temp
        Ok(env::temp_dir())
    })
}
