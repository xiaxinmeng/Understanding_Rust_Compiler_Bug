rust
extern "C" {
    pub static g_cubeb_log_callback: unsafe extern "C" fn(*const u8, ...);
}

fn main() {
    unsafe {
        let f = "file";
        let op = "Adding";
        let dev = "sink";
        let cstr = b"foo\0";
        g_cubeb_log_callback(cstr.as_ptr(), f, op, dev);
    }
}
