 shell
$ rustc main.rs
main.rs:25:17: 25:40 error: use of undeclared associated type `DerefMut::Target`
main.rs:25    member: *mut <T as DerefMut>::Target,
                           ^~~~~~~~~~~~~~~~~~~~~~~
main.rs:33:41: 33:64 error: use of undeclared associated type `DerefMut::Target`
main.rs:33     fn member<'a>(&'a self) -> &'a *mut <T as DerefMut>::Target {
                                                   ^~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
