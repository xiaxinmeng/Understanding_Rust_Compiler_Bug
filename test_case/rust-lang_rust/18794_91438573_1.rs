
$ rustc test.rs
test.rs:8:24: 8:28 error: type mismatch: the type `fn(_) {foo}` implements the trait `core::ops::Fn<(_,)>`, but the trait `for<'r> core::ops::Fn<(&'r i32,)>` is required (expected concrete lifetime, found bound lifetime parameter ) [E0281]
test.rs:8     let x: &Fn(&i32) = &foo;
                                 ^~~~
test.rs:8:24: 8:28 note: required for the cast to the object type `for<'r> core::ops::Fn(&'r i32)`
test.rs:8     let x: &Fn(&i32) = &foo;
                                 ^~~~
test.rs:8:24: 8:28 error: type mismatch resolving `for<'r> <fn(_) {foo} as core::ops::FnOnce<(&'r i32,)>>::Output == ()`:
 expected bound lifetime parameter ,
    found concrete lifetime [E0271]
test.rs:8     let x: &Fn(&i32) = &foo;
                                 ^~~~
test.rs:8:24: 8:28 note: required for the cast to the object type `for<'r> core::ops::Fn(&'r i32)`
test.rs:8     let x: &Fn(&i32) = &foo;
                                 ^~~~
error: aborting due to 2 previous errors
