
% ./x86_64-apple-darwin/stage1/bin/rustc /tmp/r.rs
/tmp/r.rs:7:22: 7:26 error: use of undeclared type name `Self`
/tmp/r.rs:7 impl<T> Bar<T> where Self: Foo {}
                                 ^~~~
error: aborting due to previous error
