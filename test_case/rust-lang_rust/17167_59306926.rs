
extern crate libc;
fn main() {
    unsafe {
        static FOO: *const i8 = "foo\0" as *const str as *const i8;
        libc::puts(FOO);
    }
}
