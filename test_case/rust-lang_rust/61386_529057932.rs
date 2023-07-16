rust
pub struct FileAttr {
    #[cfg(not(target_os = "linux"))]
    stat: stat64,
    #[cfg(target_os = "linux")]
    stat: statx,
}

[…]

// I don’t think I can do anything here, because I can only return one type
#[cfg(target_os = "linux")]
impl AsInner<statx> for FileAttr {
    fn as_inner(&self) -> &statx { &self.stat }
}

impl MetadataExt for Metadata {
    #[allow(deprecated)]
    fn as_raw_stat(&self) -> &raw::stat {
        unsafe {
            // this doesn’t work, and I’m not sure I can make it work
            // because I need to convert the struct from `statx` to `raw::stat`
            // but I can’t return a ref to something that I’ll create in the fn…
            &*(self.as_inner().as_inner() as *const libc::statx
                                          as *const raw::stat)
        }
    }
    […]
}
