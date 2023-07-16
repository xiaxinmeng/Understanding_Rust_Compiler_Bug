
foo.rs:4:5: 4:20 error: the trait `core::marker::Sized` is not implemented for the type `[f32]` [E0277]
foo.rs:4     maybe: Vec<bad>,
             ^~~~~~~~~~~~~~~
foo.rs:4:5: 4:20 help: run `rustc --explain E0277` to see a detailed explanation
foo.rs:4:5: 4:20 note: `[f32]` does not have a constant size known at compile-time
foo.rs:4     maybe: Vec<bad>,
             ^~~~~~~~~~~~~~~
foo.rs:4:5: 4:20 note: required because it appears within the type `bad`
foo.rs:4     maybe: Vec<bad>,
             ^~~~~~~~~~~~~~~
foo.rs:4:5: 4:20 note: required by `collections::vec::Vec`
foo.rs:4     maybe: Vec<bad>,
             ^~~~~~~~~~~~~~~
