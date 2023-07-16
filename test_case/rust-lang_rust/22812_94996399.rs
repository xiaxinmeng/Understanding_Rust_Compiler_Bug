 shell
$ rustc main.rs
main.rs:24:37: 24:72 error: use of undeclared associated type `std::ops::DerefMut::Target`
main.rs:24     fn deref_mut(&mut self) -> &mut <Baz as std::ops::DerefMut>::Target {
                                               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
