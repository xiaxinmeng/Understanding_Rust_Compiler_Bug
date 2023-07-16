plain
Resolving deltas: 100% (612068/612068), completed with 4858 local objects.
---
[00:00:41] configure: rust.quiet-tests     := True
---
[00:38:41] ..........................................................................i.........................
[00:38:47] .................i......................................F...........................................
---
[00:39:20] ...............................................................................................i....
[00:39:27] .....................................................................i..............................
---
[00:39:48] ---- [ui] ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs stdout ----
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:13: unexpected error: '13:12: 13:19: unexpected `for_you` after identifier'
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:21: unexpected error: '21:15: 21:24: unexpected `the_worst` after identifier'
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:30: unexpected error: '30:9: 30:16: unexpected `println` after identifier'
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:42: unexpected error: '42:27: 42:50: unexpected `be_smothered_out_before` after identifier'
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:13: expected error not found: unexpected identifier after identifier
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:21: expected error not found: unexpected identifier after identifier
[00:39:48]
[00:39:48] error: /checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs:30: expected error not found: unexpected identifier after identifier
[]         line_num: 30,
[00:39:48]         kind: Some(
[00:39:48]             Error
[00:39:48]         ),
[00:39:48]         msg: "30:9: 30:16: unexpected `println` after identifier"
---
[00:39:48]         msg: "42:27: 42:50: unexpected `be_smothered_out_before` after identifier"
---
[00:39:48]         msg: "unexpected identifier after identifier"
---
[00:39:48]         msg: "unexpected identifier after identifier"
---
[00:39:48]         msg: "unexpected identifier after identifier"
---
[00:39:48]         msg: "unexpected identifier after identifier"
[00:39:48]     }
[00:39:48] ]
[00:39:48]
[00:39:48] thread '[ui] ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1253:13
[00:39:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:48]
[00:39:48]
[00:39:48] failures:
[00:39:48]     [ui] ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs
---
[00:39:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "alwa3696680 .
