
foo.rs:5:29: 5:41 error: the trait `core::marker::Sync` is not implemented for the type `core::cell::UnsafeCell<u32>` [E0277]
foo.rs:5 pub static FOO: Cell<u32> = Cell::new(3);
                                     ^~~~~~~~~~~~
foo.rs:5:29: 5:41 note: `core::cell::UnsafeCell<u32>` cannot be shared between threads safely
foo.rs:5 pub static FOO: Cell<u32> = Cell::new(3);
                                     ^~~~~~~~~~~~
error: aborting due to previous error
