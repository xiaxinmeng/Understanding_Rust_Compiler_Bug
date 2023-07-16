
#[cfg(rpass1)]
extern crate a;
#[cfg(rpass1)]
extern crate b;

#[cfg(rpass2)]
extern crate b;
#[cfg(rpass2)]
extern crate a;

use a::A;
use b::B;

pub fn main() { 
    A + B;
}
