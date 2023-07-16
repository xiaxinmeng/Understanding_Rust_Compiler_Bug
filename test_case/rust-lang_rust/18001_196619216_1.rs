
18001.rs:7:29: 7:41 error: the trait `core::marker::Sync` is not implemented for the type `core::cell::Cell<u32>` [E0277]
18001.rs:7 pub static FOO: Cell<u32> = Cell::new(3);
                                       ^~~~~~~~~~~~
18001.rs:7:29: 7:41 help: run `rustc --explain E0277` to see a detailed explanation
18001.rs:7:29: 7:41 note: `core::cell::Cell<u32>` cannot be shared between threads safely
18001.rs:7:29: 7:41 note: shared static variables must have a type that implements `Sync`
error: aborting due to previous error
