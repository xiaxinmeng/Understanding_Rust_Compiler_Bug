 rust
issue-21166.rs:4:1: 4:16 warning: type could implement `Copy`; consider adding `impl Copy`, #[warn(missing_copy_implementations)] on by default
issue-21166.rs:4 pub struct Foo;
                 ^~~~~~~~~~~~~~~
issue-21166.rs:3:1: 3:18 warning: unused attribute, #[warn(unused_attributes)] on by default
issue-21166.rs:3 #[deriving(Copy)]
                 ^~~~~~~~~~~~~~~~~
