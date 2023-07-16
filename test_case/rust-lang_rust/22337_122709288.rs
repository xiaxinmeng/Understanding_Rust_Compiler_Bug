
> rustc --version
rustc 1.3.0-nightly (e4e93196e 2015-07-14)
> rustc foo.rs 
foo.rs:8:22: 8:23 error: the trait `core::fmt::Debug` is not implemented for the type `Y` [E0277]
foo.rs:8     println!("{:?}", x);
