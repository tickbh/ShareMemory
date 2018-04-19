
extern crate libc;
extern crate winapi;
use std::io::Result;

mod sys;
pub use sys::{MemoryId, INVALID_MEMORY_ID};

pub struct ShareMemory(sys::Memory);

impl ShareMemory {
    pub fn new_open(name: String, size: usize, path_name: Option<String>) -> Result<ShareMemory> {
        Ok(ShareMemory(sys::Memory::new_open(name, size, path_name)?))
    }

    pub fn new_create(name: String, size: usize, path_name: Option<String>) -> Result<ShareMemory> {
        Ok(ShareMemory(sys::Memory::new_create(name, size, path_name)?))
    }

    pub fn first_memory(&mut self) -> Option<*mut libc::c_void> {
        self.0.first_memory()
    }
    
    pub fn deattch(&mut self) -> Result<()> {
        self.0.deattch()
    }
    
    pub fn destory(&mut self) -> Result<()> {
        self.0.destory()
    }
}