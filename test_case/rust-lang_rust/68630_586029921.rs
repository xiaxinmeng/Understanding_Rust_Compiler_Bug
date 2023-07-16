Rust
// (╯°□°）╯︵ ┻┻

/*
error: constant expression depends on a generic parameter
   […snip…]
   = note: this may fail depending on what value the parameter takes
*/

#![feature(const_generics)]
#![allow(incomplete_features)]



const fn gud_gaemu() -> usize {
    0xFF7_600D
}



// Example showing a configurable type that takes
// const generics via a struct.
#[derive(Eq, PartialEq)]
struct Magic {
    pub fairy_dust: usize,
}

struct X<const N: Magic> {
    arr: [u32; N.fairy_dust],
}



// Example showing a configurable type that takes
// const generics via a trait.
trait Nightmare {
    const SCAWWY: usize;
}

impl Nightmare for () {
    const SCAWWY: usize = gud_gaemu();
}

struct Y<T: Nightmare> {
    arr: [u32; <T as Nightmare>::SCAWWY],
    _but_i_am_using_t_indirectly: std::marker::PhantomData<T>,
}



// Example showing a configurable type that takes
// bare const generics.
//
// This is the only one that works!
struct Z<const N: usize> {
    arr: [u32; N],
}



// Example showing a configurable type that takes
// const generics via a tuple.
struct W<const N: (usize,)> {
    arr: [u32; N.0],
}



fn main() {
    const M: Magic = Magic { fairy_dust: gud_gaemu() };
    println!("{}B", std::mem::size_of::<X<{           M  }>>());
    println!("{}B", std::mem::size_of::<Y<           ()   >>());
    println!("{}B", std::mem::size_of::<Z<{ gud_gaemu()  }>>());
    println!("{}B", std::mem::size_of::<W<{(gud_gaemu(),)}>>());
}
