rust
#[cfg(windows)]
fn windows_absolute_path(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    use std::ffi::OsString;
    use std::io::Error;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use std::ptr::null_mut;
    #[link(name = "kernel32")]
    extern "system" {
        fn GetFullPathNameW(
            lpFileName: *const u16,
            nBufferLength: u32,
            lpBuffer: *mut u16,
            lpFilePart: *mut *const u16,
        ) -> u32;
    }

    unsafe {
        // encode the path as UTF-16
        let path: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
        let mut buffer = Vec::new();
        // Loop until either success or failure.
        loop {
            // Try to get the absolute path
            let len = GetFullPathNameW(
                path.as_ptr(),
                buffer.len().try_into().unwrap(),
                buffer.as_mut_ptr(),
                null_mut(),
            );
            match len as usize {
                // Failure
                0 => return Err(Error::last_os_error()),
                // Buffer is too small, resize.
                len if len > buffer.len() => buffer.resize(len, 0),
                // Success!
                len => {
                    buffer.truncate(len);
                    return Ok(OsString::from_wide(&buffer).into());
                }
            }
        }
    }
}
