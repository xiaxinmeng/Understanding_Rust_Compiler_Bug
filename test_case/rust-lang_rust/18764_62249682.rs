 rust
#![feature(associated_types)]
pub trait Transfer<S> {
    type R;
    fn transfer(&mut self) -> R;
}

fn main(){}
