plain
[00:43:20]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:43:21] warning: `[String::push_str]` cannot be resolved, ignoring it...
[00:43:21]     --> liballoc/string.rs:1930:52
[00:43:21]      |
[00:43:21] 1930 | /// This has the same behavior as the [`push_str`][String::push_str] method.
[00:43:21]      |
[00:43:21]      = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:43:21]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:21] 
[00:43:21] 
[00:43:21] warning: `[String::push_str]` cannot be resolved, ignoring it...
[00:43:21]     --> liballoc/string.rs:1930:52
[00:43:21]      |
[00:43:21] 1930 | /// This has the same behavior as the [`push_str`][String::push_str] method.
[00:43:21]      |
[00:43:21]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:43:21] 
[00:43:21] warning: `[Weak]` cannot be resolved, ignoring it...
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:05] 
[01:05:05] running 261 tests
[01:06:16] .......................i..............................................................F.F...........
[01:07:53] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:07:53] failures:
[01:07:53] 
[01:07:53] ---- [rustdoc] rustdoc/intra-link-extern-crate.rs stdout ----
[01:07:53] 
[01:07:53] 
[01:07:53] error: rustdoc failed!
[01:07:53] status: exit code: 1
[01:07:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-extern-crate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-extern-crate" "/checkout/src/test/rustdoc/intra-link-extern-crate.rs"
[01:07:53] ------------------------------------------
[01:07:53] 
[01:07:53] ------------------------------------------
[01:07:53] stderr:
---
[01:07:53] ---- [rustdoc] rustdoc/intra-link-private.rs stdout ----
[01:07:53] 
[01:07:53] error: rustdoc failed!
[01:07:53] status: exit code: 1
[01:07:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-private" "/checkout/src/test/rustdoc/intra-link-private.rs"
[01:07:53] ------------------------------------------
[01:07:53] 
[01:07:53] ------------------------------------------
[01:07:53] stderr:
---
[01:07:53]     --> /checkout/src/test/rustdoc/intra-link-private.rs:15:9
[01:07:53]      |
[01:07:53] 15   | #![deny(intra_doc_link_resolution_failure)]
[01:07:53]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:07:53]      = help: to escape `[`ons " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:53] 
[01:07:53] 
[01:07:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:53] Build completed unsuccessfully in 0:23:38
[01:07:53] Build completed unsuccessfully in 0:23:38
[01:07:53] Makefile:58: recipe for target 'check' failed
[01:07:53] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:013e968a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
