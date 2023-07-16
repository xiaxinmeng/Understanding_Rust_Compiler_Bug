rust
#![forbid(unsafe_code)]
#![feature(allow_internal_unsafe)]

#[allow_internal_unsafe]
macro_rules! evil {
    ($e:expr) => {
        unsafe {
            $e
        }
    }
}

fn main() {
    println!("Hello, world! {}", evil!(*(0 as *const u8)));
}
