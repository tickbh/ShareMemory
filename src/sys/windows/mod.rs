
use libc;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::io::{self, Result, Error, ErrorKind};

use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{HANDLE, PAGE_READWRITE};
use winapi::um::memoryapi::{CreateFileMappingW, OpenFileMappingW, FILE_MAP_ALL_ACCESS, MapViewOfFile, UnmapViewOfFile};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::shared::minwindef::DWORD;
use winapi::ctypes::c_void;

pub type MemoryId = HANDLE;
pub const INVALID_MEMORY_ID: MemoryId = INVALID_HANDLE_VALUE;
pub const NULL: HANDLE = 0 as HANDLE;

/// Returns the last error from the Windows socket interface.
fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { GetLastError() as i32 })
}

#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

/// Checks if the signed integer is the Windows constant `SOCKET_ERROR` (-1)
/// and if so, returns the last error from the Windows socket interface. This
/// function must be called before another call to the socket API is made.
pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(last_error())
    } else {
        Ok(t)
    }
}


#[derive(Debug)]
pub struct Memory {
    id: HANDLE,
    first: Option<*mut libc::c_void>,
}

impl Memory {

    pub fn new_create(mut name: String, size: usize, path_name: Option<String>) -> Result<Memory> {
        if name.len() == 0 {
            name = String::from("/tmp");
        }

        unsafe {
            let mut name: Vec<u16> = OsStr::new(&name).encode_wide().chain(once(0)).collect();

            let handle = CreateFileMappingW(INVALID_HANDLE_VALUE, 0 as *mut SECURITY_ATTRIBUTES, PAGE_READWRITE, (size >> 16 >> 16) as DWORD, (size & 0xffffffff) as DWORD, name.as_mut_ptr());
            if handle == NULL {
                return Err(last_error());
            }

            return Ok(Memory {
                id: handle,
                first: None
            })
        }
    }

    pub fn new_open(mut name: String, size: usize, path_name: Option<String>) -> Result<Memory> {
        if name.len() == 0 {
            name = String::from("/tmp");
        }

        unsafe {
            let mut name: Vec<u16> = OsStr::new(&name).encode_wide().chain(once(0)).collect();
            let handle = OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, name.as_mut_ptr());
            if handle == NULL {
                return Err(last_error());
            }

            return Ok(Memory {
                id: handle,
                first: None
            })
        }
    }

    pub fn first_memory(&mut self) -> Option<*mut libc::c_void> {
        if !self.is_vaild() {
            return None;
        }
        
        unsafe {
            match MapViewOfFile(self.id, FILE_MAP_ALL_ACCESS, 0, 0, 0) {
                addr if addr.is_null() => None,
                addr => {
                    self.first = Some(addr as *mut libc::c_void);
                    self.first
                }
            }
        }
    }

    pub fn deattch(&mut self) -> Result<()> {
        self.check_vaild()?;
        if self.first.is_none() {
            return Ok(());
        }
        unsafe {
            if self.first.is_some() {
                cvt(UnmapViewOfFile(self.first.unwrap() as *const c_void))?;
                self.first = None;
            }

            cvt(CloseHandle(self.id))?;
            self.id = INVALID_HANDLE_VALUE;
        }
        return Ok(());
    }

    pub fn destory(&mut self) -> Result<()> {
        self.check_vaild()?;
        self.deattch()?;
        Ok(())
    }

    pub fn is_vaild(&self) -> bool {
        return self.id != INVALID_MEMORY_ID
    }

    pub fn check_vaild(&self) -> Result<bool> {
        if !self.is_vaild() {
            return Err(Error::new(ErrorKind::InvalidData, "no vaild"));
        }
        return Ok(true);
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        let _ = self.deattch();
    }
}