
hello.rs:3:9: 3:10 error: the trait `core::marker::Sized` is not implemented for the type `str` [E0277]
hello.rs:3     let x = y[1..2];
                   ^
hello.rs:3:9: 3:10 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:3:9: 3:10 note: `str` does not have a constant size known at compile-time
hello.rs:3:9: 3:10 note: all local variables must have a statically known size
error: aborting due to previous error
