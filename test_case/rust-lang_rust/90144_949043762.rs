rust
fn main() {
    unsafe {
        let mut buffer = vec![0_u16; 260];
        let mut len: u32 = buffer.len() as _;
        let result = GetUserProfileDirectoryW(-4, buffer.as_mut_ptr(), &mut len);
        if result != 0 {
            println!("{}", String::from_utf16_lossy(&buffer[..len as _]));
        } else {
            println!("{}", std::io::Error::last_os_error());
        }
    }
}
extern "system" {
    fn GetUserProfileDirectoryW(hToken: isize, lpProfileDir: *mut u16, lpcchSize: *mut u32) -> i32;
}
