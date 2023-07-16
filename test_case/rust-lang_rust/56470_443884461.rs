plain
travis_time:end:06053a2a:start=1543870099148110190,finish=1543870154643389042,duration=55495278852
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
[00:55:37] 
[00:55:37] running 120 tests
[00:55:40] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:41] ..ii.i.....iiii.....
[00:55:41] 
[00:55:41]  finished in 3.466
[00:55:41] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:56] 
[00:55:56] running 118 tests
[00:56:19] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:24] ......iii.i.....ii
[00:56:24] 
[00:56:24]  finished in 28.118
[00:56:24] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:16] 
[01:01:16] running 277 tests
[01:02:21] .......................i........F................................................................... 100/277
[01:03:15] ......................F.....F...i........F.....................F.................................... 200/277
[01:03:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:03:58] ...............F..F.....................................FF...................
[01:03:58] 
[01:03:58] ---- [rustdoc] rustdoc/edition-doctest.rs stdout ----
[01:03:58] 
[01:03:58] error: rustdoc failed!
---
[01:03:58]   |
[01:03:58] 4 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/edition-doctest.rs - foo (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:58] 
[01:03:58] 
---
[01:03:58] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:03:58] 
[01:03:58] error: rustdoc failed!
[01:03:58] status: exit code: 101
[01:03:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] running 1 test
[01:03:58] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 13) ... FAILED
---
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-23106.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:58] 
[01:03:58] 
---
[01:03:58] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:03:58] 
[01:03:58] error: rustdoc failed!
[01:03:58] status: exit code: 101
[01:03:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] running 1 test
[01:03:58] test /checkout/src/test/rustdoc/issue-25944.rs - main (line 13) ... FAILED
---
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-30252.rs - foo (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:58] 
[01:03:58] 
---
[01:03:58] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:03:58] 
[01:03:58] error: rustdoc failed!
[01:03:58] status: exit code: 101
[01:03:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] running 5 tests
[01:03:58] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51) ... FAILED
---
[01:03:58]   |
[01:03:58] 3 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84) stdout ----
[01:03:58] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]  --> /checkout/src/test/rustdoc/issue-38129.rs:86:19
[01:03:58]   |
[01:03:58] 4 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]  --> /checkout/src/test/rustdoc/issue-38129.rs:59:19
[01:03:58]   |
[01:03:58] 4 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]  --> /checkout/src/test/rustdoc/issue-38129.rs:26:19
[01:03:58]   |
[01:03:58] 3 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 20) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]  --> /checkout/src/test/rustdoc/issue-38129.rs:20:19
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/issue-38129.rs - simple (line 20)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] 
[01:03:58] failures:
---
[01:03:58] thread '[rustdoc] rustdoc/issue-38129.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:03:58] 
[01:03:58] ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
[01:03:58] 
[01:03:58] error: htmldocck failed!
[01:03:58] status: exit code: 1
[01:03:58] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] ------------------------------------------
[01:03:58] stderr:
[01:03:58] stderr:
[01:03:58] ------------------------------------------
[01:03:58] 24: @matches check failed
[01:03:58]  `XPATH PATTERN` did not match
[01:03:58]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:03:58] Encountered 1 errors
[01:03:58] 
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] 
[01:03:58] thread '[rustdoc] rustdoc/playground-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:03:58] 
[01:03:58] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:03:58] 
[01:03:58] error: htmldocck failed!
[01:03:58] status: exit code: 1
[01:03:58] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] ------------------------------------------
[01:03:58] stderr:
[01:03:58] stderr:
[01:03:58] ------------------------------------------
[01:03:58] 38: @matches check failed
[01:03:58]  `XPATH PATTERN` did not match
[01:03:58]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:03:58] Encountered 1 errors
[01:03:58] 
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] 
[01:03:58] thread '[rustdoc] rustdoc/playground.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:03:58] 
[01:03:58] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:03:58] 
[01:03:58] error: rustdoc failed!
[01:03:58] status: exit code: 101
[01:03:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] running 1 test
[01:03:58] running 1 test
[01:03:58] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) ... FAILED
[01:03:58] failures:
[01:03:58] 
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] 
[01:03:58] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] 
[01:03:58] failures:
[01:03:58] failures:
[01:03:58]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)
[01:03:58] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:58] 
[01:03:58] 
[01:03:58] ------------------------------------------
---
[01:03:58] stdout:
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] running 3 tests
[01:03:58] test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:03:58] test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... FAILED
[01:03:58] test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:03:58] failures:
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:18:19
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) stdout ----
[01:03:58] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:25:19
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] 
[01:03:58] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) stdout ----
[01:03:58] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:03:58]   |
[01:03:58] 2 | fn main() -> impl std::process::Termination {
[01:03:58]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:58]   |
[01:03:58]   |
[01:03:58]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:03:58] 
[01:03:58] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:03:58] 
[01:03:58] failures:
[01:03:58] failures:
[01:03:58]     /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)
[01:03:58]     /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)
[01:03:58]     /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)
[01:03:58] test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:58] 
[01:03:58] 
[01:03:58] ------------------------------------------
---
[01:03:58] test result: FAILED. 266 passed; 9 failed; 2 ignored; 0 measured; 0 filtered out
[01:03:58] 
[01:03:58] 
[01:03:58] 
[01:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:58] 
[01:03:58] 
[01:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:58] Build completed unsuccessfully in 0:18:57
[01:03:58] Build completed unsuccessfully in 0:18:57
[01:03:58] Makefile:58: recipe for target 'check' failed
[01:03:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14a7b07c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 21:53:21 UTC 2018
---
travis_time:end:0188c180:start=1543874004086932460,finish=1543874004152802870,duration=65870410
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a0c4118
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06cbeea8
$ dmesg | grep -i kill
