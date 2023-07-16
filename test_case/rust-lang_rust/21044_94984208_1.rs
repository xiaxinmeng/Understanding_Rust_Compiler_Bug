 shell
$ rustc main.rs
main.rs:3:10: 3:14 error: the trait `core::clone::Clone` is not implemented for the type `Foo` [E0277]
main.rs:3 #[derive(Copy)]
                   ^~~~
main.rs:3:10: 3:14 note: in expansion of #[derive_Copy]
main.rs:3:10: 3:14 note: expansion site
error: aborting due to previous error
