
$ rustc --crate-type lib wut.rs  
wut.rs:11:38: 11:53 error: mismatched types: expected `Future<(i32, i32, i32)>`, found `Future<(test::TestDesc, test::TestResult, collections::vec::Vec<u8>)>` (expected i32, found struct test::TestDesc)
wut.rs:11     let _: Future<(i32, i32, i32)> = join((0, 0, 0));
                                               ^~~~~~~~~~~~~~~
error: aborting due to previous error
