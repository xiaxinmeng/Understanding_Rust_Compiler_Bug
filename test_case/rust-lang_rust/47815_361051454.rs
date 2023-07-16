rust
#![feature(unboxed_closures)]

fn f(_: u8) -> u8 { 0 }

fn main() {
    let x: &Fn(u8) -> _ = &f;
    
    // Current desugaring, `_` is an inference variable
    // OK
    let x: &Fn<(u8,), Output = _> = &f;
    
    // Proposed desugaring for signatures
    // ERROR: the value of the associated type `Output` must be specified
    let x: &Fn<(u8,)> = &f;
}
