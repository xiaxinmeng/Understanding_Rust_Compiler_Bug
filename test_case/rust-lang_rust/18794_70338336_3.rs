
$ rustc ice.rs
ice.rs:6:24: 6:28 error: type mismatch: the type `fn(_) {foo}` implements the trait `core::ops::Fn(_)`, but the trait `for<'r> core::ops::Fn(&'r i32)` is required (expected concrete lifetime, found bound lifetime parameter )
ice.rs:6     let x: &Fn(&i32) = &foo;
                                ^~~~
ice.rs:6:24: 6:28 note: required for the cast to the object type `for<'r> core::ops::Fn(&'r i32)`
ice.rs:6     let x: &Fn(&i32) = &foo;
                                ^~~~
error: aborting due to previous error
