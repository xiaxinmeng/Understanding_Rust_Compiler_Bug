 Rust
pub trait Routing<I> {}

pub trait ToRouting {
    type Input;
    type Routing = Routing<Self::Input>;
}

fn main() {}
