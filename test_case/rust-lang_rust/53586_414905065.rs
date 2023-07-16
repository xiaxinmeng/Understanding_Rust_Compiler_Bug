plain
[00:48:07] ....................................................................................................
[00:48:11] ....................................................................................................
[00:48:13] ........i...........................................................................................
[00:48:16] ....................................................................................................
[00:48:19] .........................................................iiiiiiiii..................................
[00:48:24] ....................................................................................................
[00:48:28] ....................................................................................................
[00:48:30] ......................................i.............................................................
[00:48:33] ........................................................................................i.i..ii.....
---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:53] 
[01:18:53] running 6 tests
ld/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:18:53] ------------------------------------------
[01:18:53] 
[01:18:53] running 2 tests
[01:18:53] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:18:53] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:18:53] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) ... FAILED
[01:18:53] failures:
[01:18:53] 
[01:18:53] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:18:53] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:18:53] error[E0425]: cannot find value `no` in this scope
[01:18:53]   |
[01:18:53] 3 | no
[01:18:53]   | ^^ not found in this scope
[01:18:53] 
[01:18:53] 
[01:18:53] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:333:13
[01:18:53] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:18:53] 
[01:18:53] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:18:53] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)' panicked at 'test executable failed:
[01:18:53] 
[01:18:53] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:18:53] 
[01:18:53] ', src/librustdoc/test.rs:368:17
[01:18:53] 
[01:18:53] 
[01:18:53] 
[01:18:53] failures:
[01:18:53]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)
[01:18:53]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)
[01:18:53] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:53] 
[01:18:53] 
[01:18:53] ------------------------------------------
