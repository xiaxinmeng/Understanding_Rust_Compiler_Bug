
pub struct ReadDir {
    dirp: *mut libc::DIR,
    root: Rc<PathBuf>,
}

pub struct DirEntry {
    buf: Vec<u8>,
    dirent: *mut libc::dirent_t,
    root: Rc<PathBuf>,
}
