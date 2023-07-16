rust
pub fn f(_a: Option<String>) -> Option<u32> {
    loop {
        f(None);
        ()
    }
}

fn main() {
    f(None);
}
