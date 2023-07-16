rust
#![feature(in_band_lifetimes)]
#![warn(single_use_lifetimes)]

trait Foo {}

struct Bar<'a>(&'a i32);

impl Foo for Bar<'a> {}
