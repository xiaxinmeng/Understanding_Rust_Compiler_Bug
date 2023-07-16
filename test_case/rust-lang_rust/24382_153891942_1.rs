 rust
#![no_implicit_prelude]

use cast::From as _0;
use convert::From;

// minimal version of `std::convert::From`
mod convert {
    pub trait From<Src> {
        fn from(Src) -> Self;
    }

    // more concrete version of `impl<T> From<T> for T`
    impl From<u16> for u16 {
        fn from(src: u16) -> u16 {
            src
        }
    }
}

mod cast {
    pub trait From<Src> {
        type Output;

        fn from(Src) -> Self::Output;
    }

    // integer promotion
    impl From<u8> for u16 {
        type Output = u16;

        fn from(src: u8) -> u16 {
            src as u16
        }
    }
}

fn main() {
    // This error is wrong, because only `cast::From` can be "selected". Or in other words
    // `<u16 as convert::From<u16>>::from(0u8)` is not a valid candidate (see function signature)
    let x = u16::from(0u8);
    //~^ error: multiple applicable methods in scope
    //~| note: candidate #1 is defined in an impl of the trait `convert::From` for the type `u16`
    //~| note: candidate #2 is defined in an impl of the trait `cast::From` for the type `u16`
}
