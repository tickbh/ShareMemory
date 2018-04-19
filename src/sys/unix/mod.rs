use libc;

use std::io::{self, Result, Error, ErrorKind};
pub type MemoryId = libc::c_int;
pub const INVALID_MEMORY_ID: MemoryId = -1;

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

pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

#[derive(Debug)]
pub struct Memory {
    id: MemoryId,
    first: Option<*mut libc::c_void>,
}

impl Memory {
    pub fn new_create(mut name: String, size: usize, path_name: Option<String>) -> Result<Memory> {
        if name.len() == 0 {
            name = String::from("/tmp");
        }

        unsafe {
            let key = cvt(libc::ftok(name.as_bytes().as_ptr() as *mut i8, 0))?;
            match cvt(libc::shmget(key, size, 0o0666 | libc::IPC_CREAT | libc::IPC_EXCL)) {
                Ok(id) => {
                    return Ok(Memory {
                        id: id,
                        first: None,
                    })
                }
                Err(_) => {
                    let id = cvt(libc::shmget(key, size, 0o0666))?;
                    return Ok(Memory {
                        id: id,
                        first: None,
                    })
                }
            }
        }
    }

    pub fn new_open(mut name: String, size: usize, path_name: Option<String>) -> Result<Memory> {
        if name.len() == 0 {
            name = String::from("/tmp");
        }

        unsafe {
            let key = cvt(libc::ftok(name.as_bytes().as_ptr() as *mut i8, 0))?;
            match cvt(libc::shmget(key, size, 0o0666 | libc::IPC_CREAT | libc::IPC_EXCL)) {
                Ok(id) => {
                    return Ok(Memory {
                        id: id,
                        first: None,
                    })
                }
                Err(_) => {
                    let id = cvt(libc::shmget(key, size, 0o0666))?;
                    return Ok(Memory {
                        id: id,
                        first: None,
                    })
                }
            }
        }
    }

    pub fn first_memory(&mut self) -> Option<*mut libc::c_void> {
        if !self.is_vaild() {
            return None;
        }
        if self.first.is_some() {
            return self.first;
        }
        unsafe {
            match libc::shmat(self.id, ::std::ptr::null_mut(), 0) {
                addr if addr.is_null() => None,
                addr => {
                    self.first = Some(addr);
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
                println!("shmdt !!!!!!!!!!!!{:?}", self.first);
                cvt(libc::shmdt(self.first.unwrap()))?;
                self.first = None;
            }
        }
        return Ok(());
    }

    pub fn destory(&mut self) -> Result<()> {
        self.check_vaild()?;
        self.deattch()?;
        unsafe {
            cvt(libc::shmctl(self.id, libc::IPC_RMID, ::std::ptr::null_mut()))?;
        }
        self.id = INVALID_MEMORY_ID;
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