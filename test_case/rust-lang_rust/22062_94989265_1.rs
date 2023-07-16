 shell
$ rustc main.rs
main.rs:6:16: 6:39 error: use of undeclared associated type `DerefMut::Target`
main.rs:6   member: *mut <T as DerefMut>::Target,
                         ^~~~~~~~~~~~~~~~~~~~~~~
main.rs:13:39: 13:62 error: use of undeclared associated type `DerefMut::Target`
main.rs:13   fn member<'a>(&'a self) -> &'a *mut <T as DerefMut>::Target {
                                                 ^~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
