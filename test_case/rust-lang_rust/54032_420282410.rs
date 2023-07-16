plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:31] 
[01:01:31] running 97 tests
[01:03:19] ........................................F.........................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:04:57] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:04:57] failures:
[01:04:57] 
[01:04:57] ---- [run-pass] run-pass-fulldeps/newtype_index.rs stdout ----
[01:04:57] 
[01:04:57] 
[01:04:57] error: compilation failed!
[01:04:57] status: exit code: 1
[01:04:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/newtype_index.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/auxiliary"
[01:04:57] ------------------------------------------
[01:04:57] 
[01:04:57] ------------------------------------------
[01:04:57] stderr:
[01:04:57] stderr:
[01:04:57] ------------------------------------------
[01:04:57] error[E0658]: const fn is unstable (see issue #53555)
[01:04:57]  --> /checkout/src/test/run-pass-fulldeps/newtype_index.rs:8:1
[01:04:57]   |
[01:04:57] 8 | newtype_index!(struct MyIdx { MAX = 0xFFFF_FFFA });
[01:04:57]   |
[01:04:57]   |
[01:04:57]   = help: add #![feature(min_const_fn)] to the crate attributes to enable
[01:04:57] 
[01:04:57] error[E0658]: const fn is unstable (see issue #53555)
[01:04:57]  --> /checkout/src/test/run-pass-fulldeps/newtype_index.rs:8:1
[01:04:57]   |
[01:04:57]   |
[01:04:57] 8 | newtype_index!(struct MyIdx { MAX = 0xFFFF_FFFA });
[01:04:57]   |
[01:04:57]   |
[01:04:57]   = help: add #![feature(min_const_fn)] to the crate attributes to enable
[01:04:57] 
[01:04:57] error: aborting due to 2 previous errors
[01:04:57] 
[01:04:57] For more information about this error, try `rustc --explain E0658`.
---
travis_time:end:0e26a3ca:start=1536673992131257186,finish=1536673992262861230,duration=131604044
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:037de8c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15380d3a
$ dmesg | grep -i kill
