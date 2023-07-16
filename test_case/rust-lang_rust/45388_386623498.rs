rust
// `#![feature(crate_visibility_modifier)]` is not required
#![feature(crate_in_paths)] // <-- On the other hand, this is required

struct Z;

mod m {
    pub struct S(crate::Z);
    
    pub fn get_s() -> S { S(::Z) }
}

fn main() {
    m::get_s().0; // ERROR field `0` of struct `m::S` is private
}
