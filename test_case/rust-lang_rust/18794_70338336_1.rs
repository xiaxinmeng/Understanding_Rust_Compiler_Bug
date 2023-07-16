
$ rustc ice.rs
ice.rs:6:24: 6:27 error: the trait `Foo` is not implemented for the type `i32`
ice.rs:6     let x: &Fn(i32) = &foo;
                                ^~~
error: aborting due to previous error
