rust
extern crate libloading;

fn main() {
    unsafe {
        let library = libloading::Library::new("libc.so.6").unwrap();
        let symbol: Option<libloading::Symbol<extern fn(i32) -> !>> =
            library.get("exit".as_bytes()).ok();
        symbol.map(|s| *s).unwrap()(0);
    }
}
