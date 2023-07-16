rust
extern "C" {
    fn variadic(arg: u8, ...);
}

fn main() {
    unsafe {
        let f = "file";
        variadic(0, f, f, f);
    }
}
