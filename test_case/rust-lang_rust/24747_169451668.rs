
rust-24747.rs:15:5: 15:20 error: the trait `Trait<bar::Type>` is not implemented for the type `u8` [E0277]
rust-24747.rs:15     foo::<u8, Type>();
                     ^~~~~~~~~~~~~~~
rust-24747.rs:15:5: 15:20 help: run `rustc --explain E0277` to see a detailed explanation
rust-24747.rs:15:5: 15:20 note: required by `foo`
error: aborting due to previous error
