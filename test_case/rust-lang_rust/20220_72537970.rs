
$ rustc foo.rs
foo.rs:4:34: 4:62 error: illegal recursive type; insert an enum or struct in the cycle, if this is desired [E0246]
foo.rs:4     type IntoIter: Iterator<Item=<Self as IntoIterator>::Item>;
                                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
