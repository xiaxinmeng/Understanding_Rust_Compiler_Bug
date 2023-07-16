 Rust
#![feature(zero_one)]

use std::ops::{Not,Add,Sub,Shl,BitAnd,BitOr};
use std::num::{Zero,One};

pub trait Bits<I> : Zero + One + Eq + Not<Output=Self> + BitAnd<Output=Self>
                     + BitOr<Output=Self> + Shl<I, Output=Self> + Copy {}
impl<T, I> Bits<I> for T where
    T: Zero + One + Eq + Not<Output=T> + BitAnd<Output=T>
            + BitOr<Output=T> + Shl<I, Output=T> + Copy {}

pub fn set_bits<T, I = T>(start: I, end: I, val: T) -> T
    where T : Bits<I> + Sub<Output=T>,
          I : One + Add<Output=I> + Sub<Output=I> + Copy
{
    let len = end - start + I::one();
    let mask = (T::one() << len) - T::one();
    if !mask & val != T::zero() {
        panic!("Stored value is larger then field");
    }
    val << start
}

fn main() {
    println!("{:x}", set_bits(4,12,0x1c));
}
