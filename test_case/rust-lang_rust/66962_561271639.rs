rust
#![allow(incomplete_features)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(const_if_match)]

use std::fmt::{self, Debug, Formatter};

struct CFG<const OPT_A: char, const OPT_B: bool> {}

impl<const OPT_A: char, const OPT_B: bool> CFG<OPT_A, OPT_B> {
    const LEN: usize = match OPT_A {
        'A' => 64,
        'B' => 128,
        'C' => 256,
        'D' => 512,
        _ => 512,
    };
    const IS_TRUE: bool = OPT_B;
}

struct CFGBuilder<const A: char, const B: bool> {}

type Options = (usize, bool);

impl<const A: char, const B: bool> CFGBuilder<A, B> {
    const RESULT: Options = (CFG::<A, B>::LEN, CFG::<A, B>::IS_TRUE);
}

const DEFAULT_CFG: Options = CFGBuilder::<'A', true>::RESULT;
const ALT_CFG: Options = CFGBuilder::<'B', false>::RESULT;

struct B1 {
    arr: [u8; DEFAULT_CFG.0],
}

impl B1 {
    const CAP: usize = DEFAULT_CFG.0;
    const IS_TRUE: bool = DEFAULT_CFG.1;
    const fn new(val: u8) -> Self {
        Self {
            arr: [val; Self::CAP],
        }
    }
}

impl Debug for B1 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "B1 IS_TRUE: {}\n{:?}", Self::IS_TRUE, unsafe {
            // Not actually unsafe, because it can't go out of bounds.
            self.arr.get_unchecked(..)
        })
    }
}

struct B2 {
    arr: [u8; ALT_CFG.0],
}

impl B2 {
    const CAP: usize = ALT_CFG.0;
    const IS_TRUE: bool = ALT_CFG.1;
    const fn new(val: u8) -> Self {
        Self {
            arr: [val; Self::CAP],
        }
    }
}

impl Debug for B2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "B2 IS_TRUE: {}\n{:?}", Self::IS_TRUE, unsafe {
            // Not actually unsafe, because it can't go out of bounds.
            self.arr.get_unchecked(..)
        })
    }
}

const ONE: B1 = B1::new(1);
const TWO: B2 = B2::new(2);

fn main() {
    println!("{:?}", ONE);
    println!("{:?}", TWO);
}
