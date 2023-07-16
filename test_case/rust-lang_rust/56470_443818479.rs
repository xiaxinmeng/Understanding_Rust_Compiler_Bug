plain
travis_time:end:00100170:start=1543858007986963562,finish=1543858090536406074,duration=82549442512
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
[00:58:03] 
[00:58:03] running 120 tests
[00:58:06] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:58:06] ..ii.i.....iiii.....
[00:58:06] 
[00:58:06]  finished in 3.577
[00:58:06] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:22] 
[00:58:22] running 118 tests
[00:58:47] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:58:51] ......iii.i.....ii
[00:58:51] 
[00:58:51]  finished in 29.202
[00:58:51] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:59] 
[01:03:59] running 277 tests
[01:05:08] .......................i.......F.................................................................... 100/277
[01:06:06] ......................F.....F...i........F.....................F.................................... 200/277
[01:06:51] test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 13) ... FAILED
[01:06:51] test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 32) ... ok
[01:06:51] failures:
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 13) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/edition-doctest.rs:15:19
[01:06:51]   |
[01:06:51] 4 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/edition-doctest.rs - foo (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] 
[01:06:51] 
---
[01:06:51] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:06:51] 
[01:06:51] error: rustdoc failed!
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] running 1 test
[01:06:51] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 13) ... FAILED
---
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-23106.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] 
[01:06:51] 
---
[01:06:51] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:06:51] 
[01:06:51] error: rustdoc failed!
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] running 1 test
[01:06:51] test /checkout/src/test/rustdoc/issue-25944.rs - main (line 13) ... FAILED
---
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-25944.rs - main (line 13)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] 
[01:06:51] 
---
[01:06:51] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:06:51] 
[01:06:51] error: rustdoc failed!
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] running 1 test
[01:06:51] running 1 test
[01:06:51] testinux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] running 5 tests
[01:06:51] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51) ... FAILED
---
[01:06:51]   |
[01:06:51] 3 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 51)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25) stdout ----
[01:06:51] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/issue-38129.rs:26:19
[01:06:51]   |
[01:06:51] 3 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/issue-38129.rs:59:19
[01:06:51]   |
[01:06:51] 4 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 57)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/issue-38129.rs:86:19
[01:06:51]   |
[01:06:51] 4 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 84)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 20) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/issue-38129.rs:20:19
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/issue-38129.rs - simple (line 20)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] 
[01:06:51] 
[01:06:51] failures:
---
[01:06:51] thread '[rustdoc] rustdoc/issue-38129.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:06:51] 
[01:06:51] ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
[01:06:51] 
[01:06:51] error: htmldocck failed!
[01:06:51] status: exit code: 1
[01:06:51] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] 24: @matches check failed
[01:06:51]  `XPATH PATTERN` did not match
[01:06:51]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:06:51] Encountered 1 errors
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] thread '[rustdoc] rustdoc/playground-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:06:51] 
[01:06:51] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:06:51] 
[01:06:51] error: htmldocck failed!
[01:06:51] status: exit code: 1
[01:06:51] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] 38: @matches check failed
[01:06:51]  `XPATH PATTERN` did not match
[01:06:51]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:06:51] Encountered 1 errors
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] thread '[rustdoc] rustdoc/playground.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:06:51] 
[01:06:51] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:06:51] 
[01:06:51] error: rustdoc failed!
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:06:51] stdout:
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] running 1 test
[01:06:51] running 1 test
[01:06:51] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) ... FAILED
[01:06:51] failures:
[01:06:51] 
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] 
[01:06:51] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] 
[01:06:51] 
[01:06:51] failures:
[01:06:51] failures:
[01:06:51]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)
[01:06:51] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:51] 
[01:06:51] 
[01:06:51] ------------------------------------------
---
[01:06:51] stdout:
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] running 3 tests
[01:06:51] test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) ... FAILED
[01:06:51] test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) ... FAILED
[01:06:51] test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) ... FAILED
[01:06:51] failures:
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:25:19
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:51] 
[01:06:51] 
[01:06:51] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] 
[01:06:51] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18) stdout ----
[01:06:51] error[E0658]: use of unstable library feature 'termination_trait_lib' (see issue #43301)
[01:06:51]  --> /checkout/src/test/rustdoc/test_option_check/test.rs:18:19
[01:06:51]   |
[01:06:51]   |
[01:06:51] 2 | fn main() -> impl std::process::Termination {
[01:06:51]   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:51]   |
[01:06:51]   = help: add #![feature(termination_trait_lib)] to the crate attributes to enable
[01:06:51] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:51] 
[01:06:51] 
[01:06:51] failures:
[01:06:51] failures:
[01:06:51]     /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 16)
[01:06:51]     /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 25)
[01:06:51]     /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 18)
[01:06:51] test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:51] 
[01:06:51] 
[01:06:51] ------------------------------------------
---
[01:06:51] test result: FAILED. 266 passed; 9 failed; 2 ignored; 0 measured; 0 filtered out
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:51] 
[01:06:51] 
[01:06:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:52] Build completed unsuccessfully in 0:20:03
[01:06:52] Build completed unsuccessfully in 0:20:03
[01:06:52] Makefile:58: recipe for target 'check' failed
[01:06:52] make: *** [check] Error 1
55944 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
55940 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
55732 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass/proc-macro
53856 ./.git/modules/src/tools
---
travis_time:end:06c230a0:start=1543862115113998655,finish=1543862115120106039,duration=6107384
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ee008b1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11883c00
travis_time:start:11883c00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0809bef0
$ dmesg | grep -i kill
