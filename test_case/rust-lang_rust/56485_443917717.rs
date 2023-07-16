rust
#![feature(trait_alias)]

mod alpha {
    pub trait A { fn foo(); }
    pub trait B { fn foo(); }
    pub trait C = A + B;
}

use alpha::C;
