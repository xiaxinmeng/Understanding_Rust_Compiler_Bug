rust
extern crate libc;

fn main() {
    unsafe {
        let ptr = libc::malloc(80);
        libc::free(ptr);
        libc::free(ptr);
    }
}
