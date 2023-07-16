plain
travis_time:end:1ff2a8d5:start=1543876727250016245,finish=1543876786163502264,duration=58913486019
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
[00:55:33] 
[00:55:33] running 120 tests
[00:55:36] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:36] ..ii.i.....iiii.....
[00:55:36] 
[00:55:36]  finished in 3.569
[00:55:36] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:51] 
[00:55:51] running 118 tests
[00:56:15] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:19] ......iii.i.....ii
[00:56:19] 
[00:56:19]  finished in 27.731
[00:56:19] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:17] 
[01:01:17] running 277 tests
[01:02:24] .......................i........F................................................................... 100/277
[01:03:19] ......................F.....F...i........F.....................F.................................... 200/277
[01:04:02] ...............F..F.....................................FF...................
[01:04:02] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:04:02] 
[01:04:02] ---- [rustdoc] rustdoc/edition-doctest.rs stdout ----
[01:04:02] 
---
[01:04:02] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:04:02] 
[01:04:02] error: rustdoc failed!
[01:04:02] status: exit code: 101
[01:04:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] running 1 test
[01:04:02] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 13) ... FAILED
---
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-23106.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:02] 
[01:04:02] 
---
[01:04:02] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:04:02] 
[01:04:02] error: rustdoc failed!
[01:04:02] status: exit code: 101
[01:04:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] running 1 test
[01:04:02] test /checkout/src/test/rustdoc/issue-25944.rs - main (line 13) ... FAILED
---
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-25944.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:02] 
[01:04:02] 
---
[01:04:02] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:04:02] 
[01:04:02] error: rustdoc failed!
[01:04:02] status: exit code: 101
[01:04:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] running 1 test
[01:04:02] test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 13) ... FAILED
---
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-30252.rs - foo (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:02] 
[01:04:02] 
---
[01:04:02] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:04:02] 
[01:04:02] error: rustdoc failed!
[01:04:02] status: exit code: 101
[01:04:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] running 5 tests
[01:04:02] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51) ... FAILED
---
[01:04:02]   |
[01:04:02] 3 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25) stdout ----
[01:04:02] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]  --> /checkout/src/test/rustdoc/issue-38129.rs:26:19
[01:04:02]   |
[01:04:02] 3 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]  --> /checkout/src/test/rustdoc/issue-38129.rs:86:19
[01:04:02]   |
[01:04:02] 4 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]  --> /checkout/src/test/rustdoc/issue-38129.rs:59:19
[01:04:02]   |
[01:04:02] 4 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 20) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]  --> /checkout/src/test/rustdoc/issue-38129.rs:20:19
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/issue-38129.rs - simple (line 20)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] 
[01:04:02] 
[01:04:02] failures:
[01:04:02] failures:
[01:04:02]     /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57)
[01:04:02]     /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84)
[01:04:02]     /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51)
[01:04:02]     /checkout/src/test/rurg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:04:02] 
[01:04:02] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:04:02] 
[01:04:02] error: htmldocck failed!
[01:04:02] status: exit code: 1
[01:04:02] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] ------------------------------------------
[01:04:02] stderr:
[01:04:02] stderr:
[01:04:02] ------------------------------------------
[01:04:02] 38: @matches check failed
[01:04:02]  `XPATH PATTERN` did not match
[01:04:02]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:04:02] Encountered 1 errors
[01:04:02] 
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] 
[01:04:02] thread '[rustdoc] rustdoc/playground.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:04:02] 
[01:04:02] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:04:02] 
[01:04:02] error: rustdoc failed!
[01:04:02] status: exit code: 101
[01:04:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] running 1 test
[01:04:02] running 1 test
[01:04:02] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) ... FAILED
[01:04:02] failures:
[01:04:02] 
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] 
[01:04:02] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] 
[01:04:02] 
[01:04:02] failures:
[01:04:02] failures:
[01:04:02]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)
[01:04:02] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:02] 
[01:04:02] 
[01:04:02] ------------------------------------------
---
[01:04:02] stdout:
[01:04:02] ------------------------------------------
[01:04:02] 
[01:04:02] running 3 tests
[01:04:02] test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... FAILED
[01:04:02] test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:04:02] test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:04:02] failures:
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:25:19
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:02] 
[01:04:02] 
[01:04:02] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] 
[01:04:02] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:02] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:04:02] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:04:02]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:18:19
[01:04:02]   |
[01:04:02]   |
[01:04:02] 2 | fn main() -> impl std::process::Termination {
[01:04:02]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:02]   |
[01:04:02]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:04:02] 
[01:04:02] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)' pan" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:02] 
[01:04:02] 
[01:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:02] Build completed unsuccessfully in 0:19:17
[01:04:02] Build completed unsuccessfully in 0:19:17
[01:04:02] make: *** [check] Error 1
[01:04:02] Makefile:58: recipe for target 'check' failed
125036 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125032 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
123696 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
122804 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
