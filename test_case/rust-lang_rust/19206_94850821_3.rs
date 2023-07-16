
$ rustc src/bin/main.rs
src/bin/main.rs:11:33: 11:40 error: the type of this value must be known in this context
src/bin/main.rs:11         f: Arc::new(Box::new(|_|{} as _))
                                                   ^~~~~~~
error: aborting due to previous error
