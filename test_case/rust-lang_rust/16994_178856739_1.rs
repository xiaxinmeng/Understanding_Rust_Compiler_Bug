 text
hello.rs:4:28: 4:30 error: the type of this value must be known in this context
hello.rs:4     cb(|(k, &(ref v, b))| (*k, v.clone(), b));
                                      ^~
hello.rs:4:34: 4:39 error: the type of this value must be known in this context
hello.rs:4     cb(|(k, &(ref v, b))| (*k, v.clone(), b));
                                            ^~~~~
hello.rs:4:8: 4:45 error: mismatched types:
 expected `Box<core::ops::Fn((&i32, &(collections::vec::Vec<&'static i32>, bool))) -> _ + 'static>`,
    found `[closure@hello.rs:4:8: 4:45]`
(expected box,
    found closure) [E0308]
hello.rs:4     cb(|(k, &(ref v, b))| (*k, v.clone(), b));
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:4:8: 4:45 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to 3 previous errors
