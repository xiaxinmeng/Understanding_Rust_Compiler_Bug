plain
travis_time:end:1d215a03:start=1543922247991577263,finish=1543922325097464129,duration=77105886866
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
[00:55:57] 
[00:55:57] running 120 tests
[00:55:59] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:56:00] ..ii.i.....iiii.....
[00:56:00] 
[00:56:00]  finished in 3.465
[00:56:00] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:15] 
[00:56:15] running 118 tests
[00:56:38] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:42] ......iii.i.....ii
[00:56:42] 
[00:56:42]  finished in 27.849
[00:56:42] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:38] 
[01:01:38] running 279 tests
[01:02:45] .......................i.......F.................................................................... 100/279
[01:03:40] ......................F.....F...i.........F....................F.................................... 200/279
[01:04:25] test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 13) ... FAILED
[01:04:25] test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 32) ... ok
[01:04:25] 
[01:04:25] failures:
---
[01:04:25] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:04:25] 
[01:04:25] error: rustdoc failed!
[01:04:25] status: exit code: 101
[01:04:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:04:25] ------------------------------------------
[01:04:25] 
[01:04:25] running 1 test
[01:04:25] test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 13) ... FAILED
---
[01:04:25] 
[01:04:25] ------------------------------------------
[01:04:25] stderr:
[01:04:25] ------------------------------------------
[01:04:25] 24: @matches check failed
[01:04:25]  `XPATH PATTERN` did not match
[01:04:25]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:04:25] Encountered 1 errors
[01:04:25] 
[01:04:25] ------------------------------------------
[01:04:25] 
[01:04:25] 
[01:04:25] thread '[rustdoc] rustdoc/playground-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:04:25] 
[01:04:25] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:04:25] 
[01:04:25] error: htmldocck failed!
[01:04:25] status: exit code: 1
[01:04:25] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:04:25] ------------------------------------------
[01:04:25] 
[01:04:25] ------------------------------------------
[01:04:25] stderr:
[01:04:25] stderr:
[01:04:25] ------------------------------------------
[01:04:25] 38: @matches check failed
[01:04:25]  `XPATH PATTERN` did not match
[01:04:25]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:04:ng 1 test
[01:04:ng 1 test
[01:04:25] test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) ... FAILED
[01:04:25] failures:
[01:04:25] 
[01:04:25] 
[01:04:25] ---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16) stdout ----
[01:04:25] error[E0433]: failed to resolve: could not find `process` in `core`
[01:04:25]   |
[01:04:25] 2 | fn main() -> impl core::process::Termination {
[01:04:25]   |                         ^^^^^^^ could not find `process` in `core`
[01:04:25] 
[01:04:25] 
[01:04:25] thread '/checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:04:25] 
[01:04:25] 
[01:04:25] failures:
[01:04:25] failures:
[01:04:25]     /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 16)
[01:04:25] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:25] 
[01:04:25] 
[01:04:25] ------------------------------------------
