
$ rustc sample.rs
sample.rs:6:43: 6:47 error: not a sendable value
sample.rs:6          io::print(fmt!("%s says: '%d'\n", name, num))
                                                       ^~~~
note: in expansion of #fmt
sample.rs:6:19: 6:54 note: expansion site
error: aborting due to previous error
