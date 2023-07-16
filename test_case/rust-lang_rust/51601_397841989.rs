plain
[01:05:39] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:39]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:05:46] error[E0271]: type mismatch resolving `<std::iter::StepBy<std::ops::Range<i32>> as std::iter::Iterator>::Item == isize`
[01:05:46]      |
[01:05:46]      |
[01:05:46] 1602 |     assert_eq!((0..20).step_by(5).collect::<Vec<isize>>(), [0, 5, 10, 15]);
[01:05:46]      |                                   ^^^^^^^ expected i32, found isize
[01:05:46]      = note: expected type `i32`
[01:05:46]                 found type `isize`
[01:05:46] 
[01:05:46] 
[01:05:46] error[E0271]: type mismatch resolving `<std::iter::StepBy<std::ops::Range<i32>> as std::iter::Iterator>::Item == u8`
[01:05:46]      |
[01:05:46]      |
[01:05:46] 1605 |     assert_eq!((200..255).step_by(50).collect::<Vec<u8>>(), [200, 250]);
[01:05:46]      |                                       ^^^^^^^ expected i32, found u8
[01:05:46]      = note: expected type `i32`
[01:05:46]                 found type `u8`
[01:05:46] 
[01:05:46] 
[01:05:46] error[E0271]: type mismatch resolving `<std::iter::StepBy<std::ops::Range<i32>> as std::iter::Iterator>::Item == isize`
[01:05:46]      |
[01:05:46]      |
[01:05:46] 1606 |     assert_eq!((200..-5).step_by(1).collect::<Vec<isize>>(), []);
[01:05:46]      |                                     ^^^^^^^ expected i32, found isize
[01:05:46]      = note: expected type `i32`
[01:05:46]                 found type `isize`
[01:05:46] 
[01:05:46] 
[01:05:46] error[E0271]: type mismatch resolving `<std::iter::StepBy<std::ops::Range<i32>> as std::iter::Iterator>::Item == isize`
[01:05:46]      |
[01:05:46]      |
[01:05:46] 1607 |     assert_eq!((200..200).step_by(1).collect::<Vec<isize>>(), []);
[01:05:46]      |                                      ^^^^^^^ expected i32, found isize
[01:05:46]      = note: expected type `i32`
[01:05:46]                 found type `isize`
[01:05:46] 
[01:05:51] error: aborting due to 4 previous errors
[01:05:51] error: aborting due to 4 previous errors
[01:05:51] 
[01:05:51] For more information about this error, try `rustc --explain E0271`.
[01:05:51] error: Could not compile `core`.
[01:05:51] 
[01:05:51] To learn more, run the command again with --verbose.
[01:05:51] 
[01:05:51] 
[01:05:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:05:51] 
[01:05:51] 
[01:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:51] Build completed unsuccessfully in 0:25:25
[01:05:51] Build completed unsuccessfully in 0:25:25
[01:05:51] make: *** [check] Error 1
[01:05:51] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13d40a90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
