rust
extern "C" {
    fn variadic(arg: u8, ...);
}

fn main() {
    unsafe {
        let f = b"file\0";
        variadic(0, f.as_ptr(), f.as_ptr(), f.as_ptr()); // does not ICE
        variadic(0, f, f, f); // will ICE
    }
}
