 shell
$ rustc main.rs
main.rs:11:25: 11:88 error: non-scalar cast: `Self` as `&core::ops::Index<usize, Output=<Self as core::ops::Index<usize>>::Output>`
main.rs:11         let indexer = &(*self as &Index<usize, Output = <Self as Index<usize>>::Output>);
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
