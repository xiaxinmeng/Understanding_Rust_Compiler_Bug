rust
#![feature(trait_alias)]

mod alpha {
    trait A {}
    pub trait C = A;
}

use alpha::C;
