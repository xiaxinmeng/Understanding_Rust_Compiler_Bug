rust
use std::marker::PhantomData;

pub enum Empty {}

fn main() {
    let _x: PhantomData<(Empty, [usize])>;
}
