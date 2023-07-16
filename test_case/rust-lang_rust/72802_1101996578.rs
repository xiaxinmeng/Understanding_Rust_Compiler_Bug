Rust
        let lock = std::io::stdin().lock();

        #[cfg(any(target_family="unix", target_family="wasi"))]
        let mut seekable_stdin = unsafe {
            use std::os::unix::io::{AsRawFd, FromRawFd};
            std::fs::File::from_raw_fd(lock.as_raw_fd())
        };

        #[cfg(target_family="windows")]
        let mut seekable_stdin = unsafe {
            use std::os::windows::io::{AsRawHandle, FromRawHandle};
            std::fs::File::from_raw_handle(lock.as_raw_handle())
        };
