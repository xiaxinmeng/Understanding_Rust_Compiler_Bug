
src/example_group.rs:41:13: 41:31 error: the trait `core::marker::Sized` is not implemented for the type `for<'r, 'r> core::ops::Fn(&'r std::panicking::PanicInfo<'r>) + Send + Sync` [E0277]
src/example_group.rs:41             panic::set_handler(*orig_panic_handler);
                                    ^~~~~~~~~~~~~~~~~~
src/example_group.rs:41:13: 41:31 help: run `rustc --explain E0277` to see a detailed explanation
src/example_group.rs:41:13: 41:31 note: `for<'r, 'r> core::ops::Fn(&'r std::panicking::PanicInfo<'r>) + Send + Sync` does not have a constant size known at compile-time
src/example_group.rs:41:13: 41:31 note: required by `std::panicking::set_handler`
error: aborting due to previous error
Could not compile `descriptor`.
