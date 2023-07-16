`rust
#![feature(negative_impls)]
struct NonDrop;
impl !Drop for NonDrop {}
