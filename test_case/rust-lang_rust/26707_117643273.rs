 Rust
#![feature(zero_one)]

use std::ops::{Not,Add,Sub,Shl,BitAnd,BitOr};
use std::num::{Zero,One};

fn set_bits<T, I = T>(start: I, end: I, val: T) -> T
    where T : Zero + One + Eq + Not<Output=T> + Sub<Output=T> + Shl<I, Output=T> 
                   + BitAnd<Output=T> + BitOr + Copy,
          I : One + Add<Output=I> + Sub<Output=I> + Copy
{
    let len = end - start + I::one();
    let mask = (T::one() << len) - T::one();
    if !mask & val != T::zero() {
        panic!("Stored value is larger then field");
    }
    val << start
}
