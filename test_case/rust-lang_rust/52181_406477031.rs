plain
[01:15:41] failures:
[01:15:41] 
[01:15:41] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:15:41] 
[01:15:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:15:41] status: exit code: 101
[01:15:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:15:41] ------------------------------------------
[01:15:41] 
[01:15:41] running 2 tests
[01:15:41] running 2 tests
[01:15:41] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:15:41] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) ... FAILED
[01:15:41] failures:
[01:15:41] 
[01:15:41] 
[01:15:41] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:15:41] error[E0425]: cannot find value `no` in this scope
[01:15:41]  --> /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:27:1
[01:15:41] 3 | no
[01:15:41]   | ^^ not found in this scope
[01:15:41] 
[01:15:41] 
[01:15:41] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:15:41] 
[01:15:41] 
[01:15:41] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:15:41] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)' panicked at 'test executable failed:
[01:15:41] 
[01:15:41] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:15:41] 
[01:15:41] ', librustdoc/test.rs:367:17
[01:15:41] 
[01:15:41] 
[01:15:41] 
[01:15:41] failures:
[01:15:41]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)
[01:15:41]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)
[01:15:41] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:15:41] 
[01:15:41] 
[01:15:41] ------------------------------------------
---
[01:15:41] 
[01:15:41] failures:
[01:15:41]     [ui] rustdoc-ui/failed-doctest-output.rs
[01:15:41] 
[01:15:41] test result: FAILED.ecipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04d9a876
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
