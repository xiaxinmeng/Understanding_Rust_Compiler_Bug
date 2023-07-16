plain
[00:47:30] ....................................................................................................
[00:47:34] ....................................................................................................
[00:47:36] ..........i.........................................................................................
[00:47:39] ....................................................................................................
[00:47:41] ...........................................................iiiiiiiii................................
[00:47:47] ....................................................................................................
[00:47:50] ....................................................................................................
[00:47:53] .......................................i............................................................
[00:47:56] .........................................................................................i.i..ii....
---
[00:53:58] ---- [run-pass] run-pass/issue-48006.rs stdout ----
[00:53:58] 
[00:53:58] error: compilation failed!
[00:53:58] status: exit code: 1
[00:53:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-48006.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48006/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48006/auxiliary"
[00:53:58] ------------------------------------------
[00:53:58] 
[00:53:58] ------------------------------------------
[00:53:58] stderr:
[00:53:58] stderr:
[00:53:58] ------------------------------------------
[00:53:58] error[E0432]: unresolved import `std::ops::Step`
[00:53:58]   --> /checkout/src/test/run-pass/issue-48006.rs:13:5
[00:53:58]    |
[00:53:58] 13 | use std::ops::Step;
[00:53:58]    |     ^^^^^^^^^^^^^^ no `Step` in `ops`
[00:53:58] error: aborting due to previous error
[00:53:58] 
[00:53:58] For more information about this error, try `rustc --explain E0432`.
[00:53:58] 
---
[00:53:58] 
[00:53:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:58] 
[00:53:58] 
[00:53:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_65cod2e
129804 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
129128 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
129124 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
126284 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
