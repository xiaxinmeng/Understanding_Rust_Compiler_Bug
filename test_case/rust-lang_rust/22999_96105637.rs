
~/Downloads $ rustc test.rs
test.rs:8:16: 8:26 error: type `A` does not implement any method in scope named `iter_mut`
test.rs:8     for p in a.iter_mut() {
                         ^~~~~~~~~~
test.rs:8:16: 8:26 help: methods from traits can only be called if the trait is implemented and in scope; the following trait defines a method `iter_mut`, perhaps you need to implement it:
test.rs:8:16: 8:26 help: candidate #1: `core::slice::SliceExt`
test.rs:15:32: 15:42 error: the trait `core::marker::Sized` is not implemented for the type `[_]` [E0277]
test.rs:15     let a: [_; 100] = unsafe { init_array(|| 0u8) };
                                          ^~~~~~~~~~
test.rs:15:32: 15:42 note: `[_]` does not have a constant size known at compile-time
test.rs:15     let a: [_; 100] = unsafe { init_array(|| 0u8) };
                                          ^~~~~~~~~~
test.rs:15:32: 15:50 error: mismatched types:
 expected `[_; 100]`,
    found `[u8]`
(expected array of 100 elements,
    found slice) [E0308]
test.rs:15     let a: [_; 100] = unsafe { init_array(|| 0u8) };
                                          ^~~~~~~~~~~~~~~~~~
error: aborting due to 3 previous errors

~/Downloads $ rustc --version
rustc 1.0.0-beta.2 (e9080ec39 2015-04-16) (built 2015-04-16)
