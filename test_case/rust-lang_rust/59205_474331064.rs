plain
travis_time:end:189e1534:start=1552990217777264156,finish=1552990220079867817,duration=2302603661
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:35:13] 
[01:35:13] running 120 tests
[01:35:42] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:35:47] .i......iii.i.....ii
[01:35:47] 
[01:35:47]  finished in 34.005
[01:35:47] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:39:37] 
[01:39:37] running 300 tests
[01:40:45] ............F..............i...F....F.F............................................................. 100/300
[01:41:42] .............F................F...F.F...i........F.....................F.....F...............F...... 200/300
[01:42:40] F..................................F.......................................FF....................... 300/300
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- [rustdoc] rustdoc/comment-in-doctest.rs stdout ----
[01:42:40] 
---
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/comment-in-doctest.rs -  (line 10) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/doctest-manual-crate-name.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name" "/checkout/src/test/rustdoc/doctest-manual-crate-name.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/doctest-manual-crate-name.rs -  (line 3) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 22) ... ok
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/edition-flag.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "-Z" "unstable-options" "--edition=2018"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/edition-flag.rs - main (line 6) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/edition-flag.rs - main (line 6) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/edition-flag.rs - main (line 6) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/issue-18199.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199" "/checkout/src/test/rustdoc/issue-18199.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/issue-23744.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744" "/checkout/src/test/rustdoc/issue-23744.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 2 tests
[01:42:40] test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-23744.rs - foo (line 5) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-23744.rs - foo (line 5)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-23744.rs - foo (line 9)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:42:40] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 5 tests
[01:42:40] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-38129.rs - simple (line 10) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - non_feature_attr (line 15)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 10) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/issue-38129.rs - simple (line 10)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:42:40] ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/issue-48377.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377" "/checkout/src/test/rustdoc/issue-48377.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/issue-48377.rs -  (line 5) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/issue-48377.rs -  (line 5) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-48377.rs -  (line 5) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/issue-54478-demo-allocator.rs -  (line 19) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
---
[01:42:40] ---- [rustdoc] rustdoc/process-termination.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 3 tests
[01:42:40] test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:42:40] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:42:40] 
[01:42:40] error: rustdoc failed!
[01:42:40] status: exit code: 101
[01:42:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 1 test
[01:42:40] running 1 test
[01:42:40] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6) ... FAILED
[01:42:40] failures:
[01:42:40] 
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] failures:
[01:42:40] failures:
[01:42:40]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6)
[01:42:40] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:40] 
[01:42:40] 
[01:42:40] ------------------------------------------
---
[01:42:40] stdout:
[01:42:40] ------------------------------------------
[01:42:40] 
[01:42:40] running 3 tests
[01:42:40] test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... FAILED
[01:42:40] test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... FAILED
[01:42:40] failures:
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) stdout ----
[01:42:40] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] thread '/checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) stdout ----
[01:42:40] error: internal compiler error: src/librustc/ty/steal.rs:36: attempted to read from stolen value
[01:42:40] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:42:40] error: aborting due to previous error
[01:42:40] 
[01:42:40] 
[01:42:40] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] 
[01:42:40] failures:
[01:42:40] failures:
[01:42:40]     /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)
[01:42:40]     /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15)
[01:42:40]     /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8)
[01:42:40] test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:40] 
[01:42:40] 
[01:42:40] ------------------------------------------
---
[01:42:40] test result: FAILED. 282 passed; 16 failed; 2 ignored; 0 measured; 0 filtered out
[01:42:40] 
[01:42:40] 
[01:42:40] 
[01:42:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:40] 
[01:42:40] 
[01:42:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:40] Build completed unsuccessfully in 0:19:53
[01:42:40] Build completed unsuccessfully in 0:19:53
[01:42:40] Makefile:48: recipe for target 'check' failed
[01:42:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0266397a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 11:53:12 UTC 2019
---
travis_time:end:0df7ce2c:start=1552996394496092543,finish=1552996394554654454,duration=58561911
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02a42e99
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28646673
$ dmesg | grep -i kill
