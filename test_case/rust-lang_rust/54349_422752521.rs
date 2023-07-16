plain
[00:52:25] ....................................................................................................
[00:52:27] .....................................................i..............................................
[00:52:30] ....................................................................................................
[00:52:33] ....................................................................................................
[00:52:36] .iiiiiiiii..........................................................................................
[00:52:42] ....................................................................................................
[00:52:45] ...................................................................................i................
[00:52:47] ....................................................................................................
[00:52:50] ......................................i.i..ii.......................................................
---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:27:06] 
[01:27:06] running 7 tests
ng: `[BarD]` cannot be resolved, ignoring it...
[01:27:07] -    |
[01:27:07] -    |
[01:27:07] - 48 | #[doc = "Foo/nbar [BarD] bar/nbaz"]
[01:27:07] -    |
[01:27:07] -    = note: the link appears in this line:
[01:27:07] -            
[01:27:07] -            
[01:27:07] -            bar [BarD] bar
[01:27:07] -                 ^^^^
[01:27:07] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:27:07] - 
[01:27:07] - warning: `[BarF]` cannot be resolved, ignoring it...
[01:27:07] -    |
[01:27:07] -    |
[01:27:07] - 53 |         #[doc = $f]
[01:27:07] - ...
[01:27:07] - ...
[01:27:07] - 57 | f!("Foo/nbar [BarF] bar/nbaz");
[01:27:07] -    | ------------------------------- in this macro invocation
[01:27:07] -    = note: the link appears in this line:
[01:27:07] -            
[01:27:07] -            
[01:27:07] -            bar [BarF] bar
[01:27:07] -                 ^^^^
[01:27:07] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:27:07] - 
[01:27:07] 
[01:27:07] 
[01:27:07] 
[01:27:07] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr`: No such file or directory (os error 2)
[01:27:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:07] 
[01:27:07] 
[01:27:07] failures:
[01:27:07] failures:
[01:27:07]     [ui] rustdoc-ui/intra-links-warning.rs
[01:27:07] 
[01:27:07] test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:27:07] 
[01:27:07] 
[01:27:07] 
[01:27:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:27:07] 
[01:27:07] 
[01:27:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:07] Bu
