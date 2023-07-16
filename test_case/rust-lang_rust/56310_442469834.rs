plain
travis_time:end:18a6a24a:start=1543411553080475180,finish=1543411554109725659,duration=1029250479
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:06] .................................................................................................... 100/5065
[00:49:09] .................................................................................................... 200/5065
[00:49:12] .............................ii............................................ii...................ii.. 300/5065
[00:49:15] ..............................................................................................iii... 400/5065
[00:49:17] .....iiiiiiii.iii............................iii...........................................i........ 500/5065
[00:49:24] .................................................................................................... 700/5065
[00:49:30] ................................................................................................i... 800/5065
[00:49:34] ........i........................................................................................... 900/5065
[00:49:37] ...............iiiii..................ii.iiii....................................................... 1000/5065
---
[00:50:15] .................................................................................................... 2300/5065
[00:50:20] .................................................................................................... 2400/5065
[00:50:23] .................................................................................................... 2500/5065
[00:50:27] .................................................................................................... 2600/5065
[00:50:30] ........iiiiiiiii................................................................................... 2700/5065
[00:50:36] .................................................................................................... 2900/5065
[00:50:40] .................................................................................................... 3000/5065
[00:50:43] .......................................................................i............................ 3100/5065
[00:50:46] .................................................................................................... 3200/5065
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:24] 
[01:04:24] running 117 tests
[01:04:27] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:04:28] i.i.....iiii.....
[01:04:28] 
[01:04:28]  finished in 3.418
[01:04:28] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:42] 
[01:04:42] running 118 tests
[01:05:05] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:09] ......iii.i.....ii
[01:05:09] 
[01:05:09]  finished in 27.497
[01:05:09] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:42] 
[01:11:42] running 276 tests
[01:12:51] .......................i.......F.................................................................... 100/276
[01:13:47] ......................F.....F...i.........F....................F.................................... 200/276
[01:14:30] ...............F..F.....................................FF..................
[01:14:30] 
[01:14:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:14:30] ---- [rustdoc] rustdoc/edition-doctest.rs stdout ----
[01:14:30] 
---
[01:14:30]   --> /checkout/src/test/rustdoc/edition-doctest.rs:15:29
[01:14:30]    |
[01:14:30] 4  |   fn main() -> Result<(), ()> {
[01:14:30]    |  _____________________________^
[01:14:30] 5  | | use std::num::ParseIntError;
[01:14:30] 6  | |
[01:14:30] 7  | | let result: Result<i32, ParseIntError> = try {
[01:14:30] ...  |
[01:14:30] 19 | | assert!(result.is_err());
[01:14:30]    | |_^ expected enum `std::result::Result`, found ()
[01:14:30]    |
[01:14:30]    = note: expected type `std::result::Result<(), ()>`
[01:14:30]               found type `()`
---
[01:14:30] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:14:30] 
[01:14:30] error: rustdoc failed!
[01:14:30] status: exit code: 101
[01:14:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] running 1 test
[01:14:30] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 13) ... FAILED
---
[01:14:30] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:14:30] 
[01:14:30] error: rustdoc failed!
[01:14:30] status: exit code: 101
[01:14:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] running 1 test
[01:14:30] test /checkout/src/test/rustdoc/issue-25944.rs - main (line 13) ... FAILED
---
[01:14:30]  --> /checkout/src/test/rustdoc/issue-25944.rs:13:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | let a = r#"
[01:14:30] 4 | | foo
[01:14:30] 5 | | bar"#;
[01:14:30] 6 | | let b = "\nfoo\nbar";
[01:14:30] 7 | | assert_eq!(a, b);
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
---
[01:14:30] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:14:30] 
[01:14:30] error: rustdoc failed!
[01:14:30] status: exit code: 101
[01:14:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] running 1 test
[01:14:30] test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 13) ... FAILED
---
[01:14:30]  --> /checkout/src/test/rustdoc/issue-30252.rs:13:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | assert_eq!(cfg!(feature = "bar"), true);
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
---
[01:14:30] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:14:30] 
[01:14:30] error: rustdoc failed!
[01:14:30] status: exit code: 101
[01:14:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] running 5 tests
[01:14:30] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51) ... FAILED
---
[01:14:30]  --> /checkout/src/test/rustdoc/issue-38129.rs:52:29
[01:14:30]   |
[01:14:30] 3 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30] 4 | | assert_eq!(1 + 1, 2);
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
---
[01:14:30]   --> /checkout/src/test/rustdoc/issue-38129.rs:26:29
[01:14:30]    |
[01:14:30] 3  |   fn main() -> Result<(), ()> {
[01:14:30]    |  _____________________________^
[01:14:30] 4  | | macro_rules! recurse {
[01:14:30] 5  | |     (()) => {};
[01:14:30] 6  | |     (() $($rest:tt)*) => { recurse!($($rest)*); }
[01:14:30] ...  |
[01:14:30] 24 | | assert_eq!(1 + 1, 2);
[01:14:30]    | |_^ expected enum `std::result::Result`, found ()
[01:14:30]    |
[01:14:30]    = note: expected type `std::result::Result<(), ()>`
[01:14:30]               found type `()`
---
[01:14:30]   --> /checkout/src/test/rustdoc/issue-38129.rs:86:29
[01:14:30]    |
[01:14:30] 4  |   fn main() -> Result<(), ()> {
[01:14:30]    |  _____________________________^
[01:14:30] 5  | | macro_rules! recurse {
[01:14:30] 6  | |     (()) => {};
[01:14:30] 7  | |     (() $($rest:tt)*) => { recurse!($($rest)*); }
[01:14:30] ...  |
[01:14:30] 25 | | assert_eq!(1 + 1, 2);
[01:14:30]    | |_^ expected enum `std::result::Result`, found ()
[01:14:30]    |
[01:14:30]    = note: expected type `std::result::Result<(), ()>`
[01:14:30]               found type `()`
---
[01:14:30]   --> /checkout/src/test/rustdoc/issue-38129.rs:59:29
[01:14:30]    |
[01:14:30] 4  |   fn main() -> Result<(), ()> {
[01:14:30]    |  _____________________________^
[01:14:30] 5  | | macro_rules! recurse {
[01:14:30] 6  | |     (()) => {};
[01:14:30] 7  | |     (() $($rest:tt)*) => { recurse!($($rest)*); }
[01:14:30] ...  |
[01:14:30] 25 | | assert_eq!(1 + 1, 2);
[01:14:30]    | |_^ expected enum `std::result::Result`, found ()
[01:14:30]    |
[01:14:30]    = note: expected type `std::result::Result<(), ()>`
[01:14:30]               found type `()`
---
[01:14:30]  --> /checkout/src/test/rustdoc/issue-38129.rs:20:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | assert_eq!(1 + 1, 2);
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
---
[01:14:30] thread '[rustdoc] rustdoc/issue-38129.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:30] 
[01:14:30] ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
[01:14:30] 
[01:14:30] error: htmldocck failed!
[01:14:30] status: exit code: 1
[01:14:30] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs"
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] ------------------------------------------
[01:14:30] stderr:
[01:14:30] stderr:
[01:14:30] ------------------------------------------
[01:14:30] 24: @matches check failed
[01:14:30]  `XPATH PATTERN` did not match
[01:14:30]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:14:30] Encountered 1 errors
[01:14:30] 
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] 
[01:14:30] thread '[rustdoc] rustdoc/playground-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:30] 
[01:14:30] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:14:30] 
[01:14:30] error: htmldocck failed!
[01:14:30] status: exit code: 1
[01:14:30] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] ------------------------------------------
[01:14:30] stderr:
[01:14:30] stderr:
[01:14:30] ------------------------------------------
[01:14:30] 38: @matches check failed
[01:14:30]  `XPATH PATTERN` did not match
[01:14:30]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:14:30] Encountered 1 errors
[01:14:30] 
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] 
[01:14:30] thread '[rustdoc] rustdoc/playground.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:30] 
[01:14:30] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:14:30] 
[01:14:30] error: rustdoc failed!
[01:14:30] status: exit code: 101
[01:14:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] running 1 test
[01:14:30] running 1 test
[01:14:30] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) ... FAILED
[01:14:30] failures:
[01:14:30] 
[01:14:30] 
[01:14:30] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) stdout ----
[01:14:30]  --> /checkout/src/test/rustdoc/test_option_check/bar.rs:16:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | println!("foo?");
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
[01:14:30]              found type `()`
[01:14:30] 
[01:14:30] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:14:30] 
[01:14:30] 
[01:14:30] failures:
[01:14:30] failures:
[01:14:30]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)
[01:14:30] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:30] 
[01:14:30] 
[01:14:30] ------------------------------------------
---
[01:14:30] stdout:
[01:14:30] ------------------------------------------
[01:14:30] 
[01:14:30] running 3 tests
[01:14:30] test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:14:30] test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:14:30] test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... FAILED
[01:14:30] failures:
[01:14:30] 
[01:14:30] 
[01:14:30] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) stdout ----
[01:14:30]  --> /checkout/src/test/rustdoc/test_option_check/bar.rs:16:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | println!("foo?");
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
[01:14:30]              found type `()`
[01:14:30] 
[01:14:30] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:14:30] 
[01:14:30] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:14:30] error[E0308]: mismatched types
[01:14:30]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:18:29
[01:14:30]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:18:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | println!("baaaaaar");
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
---
[01:14:30]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:25:29
[01:14:30]   |
[01:14:30] 2 |   fn main() -> Result<(), ()> {
[01:14:30]   |  _____________________________^
[01:14:30] 3 | | println!("fooooo");
[01:14:30]   | |_^ expected enum `std::result::Result`, found ()
[01:14:30]   |
[01:14:30]   = note: expected type `std::result::Result<(), ()>`
[01:14:30]              found type `()`
[01:14:30]              found type `()`
[01:14:30] 
[01:14:30] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:14:30] 
[01:14:30] 
[01:14:30] failures:
[01:14:30]     /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)
[01:14:30]     /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)
[01:14:30]     /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)
[01:14:30] test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:30] 
[01:14:30] 
[01:14:30] ------------------------------------------
---
[01:14:30] test result: FAILED. 265 passed; 9 failed; 2 ignored; 0 measured; 0 filtered out
[01:14:30] 
[01:14:30] 
[01:14:30] 
[01:14:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:30] 
[01:14:30] 
[01:14:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:30] Build completed unsuccessfully in 0:29:17
[01:14:30] Build completed unsuccessfully in 0:29:17
[01:14:30] Makefile:58: recipe for target 'check' failed
[01:14:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1810dc00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 28 14:40:33 UTC 2018
---
travis_time:end:1778c78a:start=1543416037505541923,finish=1543416037511337796,duration=5795873
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03302ca0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/co
