rust
#![feature(associated_consts)]

macro_rules! m { () => { 0 } }

struct S;
impl S {
    const C: i32 = m!(); // Replacing `m!()` with `0` compiles
}
