plain
travis_time:end:175096c8:start=1544772634138926040,finish=1544772687645523881,duration=53506597841
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:15] 
[00:55:15] running 121 tests
[00:55:18] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:55:18] i..ii.i.....iiii.....
[00:55:18] 
[00:55:18]  finished in 3.479
[00:55:18] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:33] 
[00:55:33] running 119 tests
[00:55:56] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[00:55:59] i......iii.i.....ii
[00:55:59] 
[00:55:59]  finished in 27.003
[00:55:59] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:47] 
[01:00:47] running 282 tests
[01:01:53] .......................i........F................................................................... 100/282
[01:02:48] .......................F.....F...i.........F....................F................................... 200/282
[01:03:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:03:34] .................F..F....F...................................F.F..................
[01:03:34] 
[01:03:34] ---- [rustdoc] rustdoc/edition-doctest.rs stdout ----
[01:03:34] 
[01:03:34] error: rustdoc failed!
---
[01:03:34]   |
[01:03:34] 4 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/edition-doctest.rs - foo (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:34] 
[01:03:34] 
---
[01:03:34] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:03:34] 
[01:03:34] error: rustdoc failed!
[01:03:34] status: exit code: 101
[01:03:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] running 1 test
[01:03:34] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 13) ... FAILED
---
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-23106.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACEtdoc/issue-25944.rs:13:19
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-25944.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:34] 
[01:03:34] 
---
[01:03:34] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:03:34] 
[01:03:34] error: rustdoc failed!
[01:03:34] status: exit code: 101
[01:03:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] running 1 test
[01:03:34] test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 13) ... FAILED
---
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-30252.rs - foo (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:34] 
[01:03:34] 
---
[01:03:34] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:03:34] 
[01:03:34] error: rustdoc failed!
[01:03:34] status: exit code: 101
[01:03:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] running 5 tests
[01:03:34] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51) ... FAILED
---
[01:03:34]   |
[01:03:34] 3 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57) stdout ----
[01:03:34] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/issue-38129.rs:59:19
[01:03:34]   |
[01:03:34] 4 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/issue-38129.rs:86:19
[01:03:34]   |
[01:03:34] 4 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/issue-38129.rs:26:19
[01:03:34]   |
[01:03:34] 3 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 20) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/issue-38129.rs:20:19
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/issue-38129.rs - simple (line 20)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] 
[01:03:34] failures:
---
[01:03:34] thread '[rustdoc] rustdoc/issue-38129.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:03:34] 
[01:03:34] ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
[01:03:34] 
[01:03:34] error: htmldocck failed!
[01:03:34] status: exit code: 1
[01:03:34] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] ------------------------------------------
[01:03:34] stderr:
[01:03:34] stderr:
[01:03:34] ------------------------------------------
[01:03:34] 24: @matches check failed
[01:03:34]  `XPATH PATTERN` did not match
[01:03:34]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:03:34] Encountered 1 errors
[01:03:34] 
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] 
[01:03:34] thread '[rustdoc] rustdoc/playground-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:03:34] 
[01:03:34] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:03:34] 
[01:03:34] error: htmldocck failed!
[01:03:34] status: exit code: 1
[01:03:34] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] ------------------------------------------
[01:03:34] stderr:
[01:03:34] stderr:
[01:03:34] ------------------------------------------
[01:03:34] 38: @matches check failed
[01:03:34]  `XPATH PATTERN` did not match
[01:03:34]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:03:34] Encountered 1 errors
[01:03:34] 
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] 
[01:03:34] thread '[rustdoc] rustdoc/playground.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:03:34] 
[01:03:34] ---- [rustdoc] rustdoc/process-termination.rs stdout ----
[01:03:34] 
[01:03:34] error: rustdoc failed!
[01:03:34] status: exit code: 101
[01:03:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] running 4 tests
[01:03:34] test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 36) ... ok
---
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 29)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 17) stdout ----
[01:03:34] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 17) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/process-termination.rs:17:19
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 23) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/process-termination.rs:23:19
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 23)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] 
[01:03:34] failures:
---
[01:03:34] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:03:34] 
[01:03:34] error: rustdoc failed!
[01:03:34] status: exit code: 101
[01:03:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] running 1 test
[01:03:34] running 1 test
[01:03:34] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) ... FAILED
[01:03:34] failures:
[01:03:34] 
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] 
[01:03:34] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] 
[01:03:34] 
[01:03:34] failures:
[01:03:34] failures:
[01:03:34]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)
[01:03:34] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:34] 
[01:03:34] 
[01:03:34] ------------------------------------------
---
[01:03:34] stdout:
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] running 3 tests
[01:03:34] test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:03:34] test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... FAILED
[01:03:34] test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:03:34] failures:
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:34]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:18:19
[01:03:34]   |
[01:03:34] 2 | fn main() -> impl std::process::Termination {
[01:03:34]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:34]   |
[01:03:34]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:34] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:03:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:34] 
[01:03:34] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) stdout ----
---
[01:03:34] test result: FAILED. 270 passed; 10 failed; 2 ignored; 0 measured; 0 filtered out
[01:03:34] 
[01:03:34] 
[01:03:34] 
[01:03:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:34] 
[01:03:34] 
[01:03:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:34] Build completed unsuccessfully in 0:18:51
[01:03:34] Build completed unsuccessfully in 0:18:51
[01:03:34] Makefile:58: recipe for target 'check' failed
[01:03:34] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04e75990
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 14 08:35:11 UTC 2018
---
travis_time:end:1fbeb3e0:start=1544776513087048223,finish=1544776513093851151,duration=6802928
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:101b6918
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.16896.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 16938]
[New LWP 16937]
[New LWP 16935]
[New LWP 16936]
[New LWP 16939]
[New LWP 16896]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007facc6e0d8ba in ?? ()
#0  0x00007facc6e0d8ba in ?? ()
#1  0x0000000000000000 in ?? ()
travis_time:end:101b6918:start=1544776513102321467,finish=1544776513962487681,duration=860166214
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08399b14
travis_time:start:08399b14
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27b87142
$ dmesg | grep -i kill
