 rust
struct IoHandle<T> {
    handle: Mutex<T>,
}

impl IoHandle<T> {
    pub unsafe fn new(handle: T) {
        IoHandle{ handle: Mutex::new(handle) }
    }

    // Use select! with the file descriptor
    pub fn event(&self) -> Receiver { … }
}

impl Deref<Mutex<Fd>> for IoHandle<Fd> {
    fn deref(&self) -> &Mutex<Fd> {
        &self.handle
    }
}

impl Drop for IoHandle<Fd> {
    fn drop(&mut self) {
        let _ = unsafe { libc::close(*self.lock()) };
    }
}

impl Clone for IoHandle<Fd> {
    fn clone(&self) -> IoHandle<Fd> {
        let fd = unsafe { libc::fcntl(*self.lock(), libc::F_DUPFD_CLOEXEC) };
        if fd == -1 {
            panic!(last_error());
        }
        unsafe { IoHandle::new(fd) }
    }
}

trait AsIoHandle<T, W> {
    fn as_iohandle(&self) -> &IoHandle<T>;
    fn from_iohandle(IoHandle<T>) -> W;
}

impl File {
    fn path(&'a self) -> Option<&'a Path> { … }
    fn open_at(&self, path: &Path) -> IoResult<File> { … }
}

impl AsIoHandle for File {
    #[cfg(windows)]
    fn from_iohandle(&self, IoHandle<Handle>) -> IoResult<File> { … }
    #[cfg(unix)]
    fn from_iohandle(&self, IoHandle<Fd>) -> IoResult<File> { … }

    #[cfg(windows)]
    fn as_iohandle(&self) -> &IoHandle<Handle> { … }
    #[cfg(unix)]
    fn as_iohandle(&self) -> &IoHandle<Fd> { … }
}
