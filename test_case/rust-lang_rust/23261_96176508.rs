 Rust
pub struct MyPath {
    inner: [u8]
}

pub fn bar(t: &MyPath) {
    match *t {
        MyPath { ref inner } => {}
    }
}

fn main() {}
