
foo.rs:2:14: 2:15 error: cannot capture variable of type `T`, which does not fulfill `'static`, in a bounded closure
foo.rs:2       || copy a
                       ^
foo.rs:2:14: 2:15 note: this closure's environment must satisfy `'static`
foo.rs:2       || copy a
                       ^
error: aborting due to previous error
