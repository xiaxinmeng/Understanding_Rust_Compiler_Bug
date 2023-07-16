
hello.rs:14:5: 14:12 error: the trait `core::marker::Sync` is not implemented for the type `*mut u8` [E0277]
hello.rs:14     is_send(arc);
                ^~~~~~~
hello.rs:14:5: 14:12 help: run `rustc --explain E0277` to see a detailed explanation
hello.rs:14:5: 14:12 note: `*mut u8` cannot be shared between threads safely
hello.rs:14:5: 14:12 note: required because it appears within the type `Inner`
hello.rs:14:5: 14:12 note: required by `is_send`
