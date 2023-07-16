 Rust
pub fn from_pointer() where for<'s> &'s mut u8: Clone {
    let v = &mut 0;
    v.clone();
}

fn main() { }
