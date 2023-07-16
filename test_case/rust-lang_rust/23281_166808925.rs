
foo.rs:4:9: 4:50 error: the trait `core::marker::Sized` is not implemented for the type `core::ops::Fn() + 'static` [E0277]
foo.rs:4         pub fn function(funs: Vec<Fn() -> ()>) {}
                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
foo.rs:4:9: 4:50 help: run `rustc --explain E0277` to see a detailed explanation
foo.rs:4:9: 4:50 note: `core::ops::Fn() + 'static` does not have a constant size known at compile-time
foo.rs:4:9: 4:50 note: required by `collections::vec::Vec`
error: aborting due to previous error
