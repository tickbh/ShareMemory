
extern crate libc;
extern crate winapi;
use std::io::Result;


mod sys;
pub struct ShareMemory(sys::Memory);
/// 提供进程间的共享内存模块
/// 其中unix使用ftok, shmget, shmat, shmdt, shmctl来实现
/// 其实windows使用CreateFileMappingW, OpenFileMappingW, MapViewOfFile, UnmapViewOfFile来实现
/// # Examples
///
/// ```no_run
/// extern crate share_memory;
/// extern crate time;
/// use share_memory::ShareMemory;
/// type ARRAY = [i32; 80];
/// fn main () {
///     let size : usize = std::mem::size_of::<ARRAY>();
///     let mut share = ShareMemory::new_create(String::from("."), size * 2, None).unwrap();
///     if let Some(addr) = share.first_memory().ok().unwrap() {
///         let mut data: &mut ARRAY = unsafe {
///             std::mem::transmute(addr)
///         };
///         data[0] += 1;
///         println!("{:?}", data[0]);
///     }
///     loop {
///     }
/// }
/// ```
impl ShareMemory {
    /// 打开一个已存在共享内存, 打开失败则返回相应的错误
    pub fn new_open(name: String, size: usize, path_name: Option<String>) -> Result<ShareMemory> {
        Ok(ShareMemory(sys::Memory::new_open(name, size, path_name)?))
    }

    /// 创建一个已存在共享内存, 创建失败会尝试打开已存在的共享内存, 如果依然失败则返回相应的错误
    pub fn new_create(name: String, size: usize, path_name: Option<String>) -> Result<ShareMemory> {
        Ok(ShareMemory(sys::Memory::new_create(name, size, path_name)?))
    }

    /// 返回共享内存的首地址
    pub fn first_memory(&mut self) -> Result<Option<*mut libc::c_void>> {
        self.0.first_memory()
    }

    /// 返回共享内存的距首地址的偏移地址
    pub fn offset_memory(&mut self, offset: usize) -> Result<Option<*mut libc::c_void>> {
        self.0.offset_memory(offset)
    }
    
    /// 取消掉当前对共享内存的引用状态
    pub fn deattch(&mut self) -> Result<()> {
        self.0.deattch()
    }
    
    /// 删除当前的共享内存块, 其中windows只是取消掉引用状态, 当引用状态为0的时候系统将会自动删除
    pub fn destory(&mut self) -> Result<()> {
        self.0.destory()
    }
}