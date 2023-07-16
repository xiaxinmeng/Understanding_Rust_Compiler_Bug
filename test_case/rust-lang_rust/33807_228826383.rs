
foo.rs:4:21: 4:26 error: the trait `core::marker::Reflect` is not implemented for the type `T` [E0277]
foo.rs:4     let value_any = value as &Any;
                             ^~~~~
foo.rs:4:21: 4:26 help: run `rustc --explain E0277` to see a detailed explanation
foo.rs:4:21: 4:26 note: `T` does not implement `Any`; ensure all type parameters are bounded by `Any`
foo.rs:4:21: 4:26 note: required for the cast to the object type `core::any::Any + 'static`
error: aborting due to previous error
