rust
// Handle is just `#[repr(transparent)]` over `()`
// HandlePtr<T> is just `#[repr(transparent)] *mut T` that communicates that it lives outside of the user-space usable address space (it lives in the thread-local handle address space)
use super::handle::{Handle,HandlePtr};

// This is just a type alias of c_long
use super::result::SysResult;

pub struct FileHandle(Handle); 

extern "C"{
   pub fn OpenFile(st: *const FileOpenOptions, hdl: *mut HandlePtr<FileHandle>) -> SysResult;
   pub fn CloseFile(hdl: HandlePtr<FileHandle>) -> SysResult;
}
