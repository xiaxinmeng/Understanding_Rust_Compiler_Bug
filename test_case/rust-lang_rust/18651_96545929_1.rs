
$ rustc --version
rustc 1.1.0-nightly (da623844a 2015-04-25) (built 2015-04-26)
$ rustc test_18651.rs 
test_18651.rs:5:32: 5:39 error: type `collections::vec::Vec<MyStruct>` does not implement any method in scope named `clone`
test_18651.rs:5     let v1: Vec<MyStruct> = v1.clone();
                                               ^~~~~~~
error: aborting due to previous error
