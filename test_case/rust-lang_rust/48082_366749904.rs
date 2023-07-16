
[01:34:13] failures:
[01:34:13] 
[01:34:13] ---- [pretty] run-pass/hygiene/issue-47311.rs stdout ----
[01:34:13] 	
[01:34:13] error: pretty-printed source does not typecheck
[01:34:13] status: exit code: 101
[01:34:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hygiene/issue-47311.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hygiene/issue-47311.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:34:13] stdout:
[01:34:13] ------------------------------------------
[01:34:13] 
[01:34:13] ------------------------------------------
[01:34:13] stderr:
[01:34:13] ------------------------------------------
[01:34:13] error[E0433]: failed to resolve. Maybe a missing `extern crate m;`?
[01:34:13]   --> <anon>:21:14
[01:34:13]    |
[01:34:13] 21 |     fn f() { ::m!(S , x); }
[01:34:13]    |              ^^^ Maybe a missing `extern crate m;`?
[01:34:13] 
[01:34:13] error: aborting due to previous error
[01:34:13] 
[01:34:13] 
[01:34:13] ------------------------------------------
[01:34:13] 
[01:34:13] thread '[pretty] run-pass/hygiene/issue-47311.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9
[01:34:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:34:13] 
[01:34:13] 
[01:34:13] failures:
[01:34:13]     [pretty] run-pass/hygiene/issue-47311.rs

