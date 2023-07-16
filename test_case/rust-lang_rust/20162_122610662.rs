
> rustc --version
rustc 1.3.0-nightly (e4e93196e 2015-07-14)
> rustc foo.rs
foo.rs:6:7: 6:13 error: the trait `core::cmp::Ord` is not implemented for the type `X` [E0277]
foo.rs:6     b.sort();
               ^~~~~~
foo.rs:6:7: 6:13 help: run `rustc --explain E0277` to see a detailed explanation
