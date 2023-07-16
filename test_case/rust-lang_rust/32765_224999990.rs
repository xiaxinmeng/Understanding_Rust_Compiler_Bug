 rust
pub trait A {}

pub enum C {
    A(A),
}

fn main() {
    let _: Option<Box<C>> = None;
}
