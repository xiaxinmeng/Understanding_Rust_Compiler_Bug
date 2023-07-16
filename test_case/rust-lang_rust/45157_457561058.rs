Rust
#![allow(unused)]

#[derive(Clone, Copy, Default)]
struct S {
    a: u8,
    b: u8,
}
#[derive(Clone, Copy, Default)]
struct Z {
    c: u8,
    d: u8,
}

union U {
    s: S,
    z: Z,
}



fn main() { unsafe {
    let mut u = U { s: Default::default() };
    
    let mref = &mut u.s.a;
    let err = &u.z.c; // This line compiles, but it certainly shouldn't 
    drop(mref);
}}
