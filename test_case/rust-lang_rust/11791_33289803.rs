
fn main() {
    use std::libc::c_void;
    static ICONV_ERROR: *c_void = -1 as *c_void;
    match -1 as *c_void {
        ICONV_ERROR => None,
        val => Some(val),
    };
}
