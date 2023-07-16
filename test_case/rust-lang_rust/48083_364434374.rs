
[01:40:50] failures:
[01:40:50] 
[01:40:50] ---- [pretty] run-pass/hygiene/issue-47312.rs stdout ----
[01:40:50] 	
[01:40:50] error: pretty-printed source does not typecheck
[01:40:50] status: exit code: 101
[01:40:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hygiene/issue-47312.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/hygiene/issue-47312.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:40:50] stdout:
[01:40:50] ------------------------------------------
[01:40:50] 
[01:40:50] ------------------------------------------
[01:40:50] stderr:
[01:40:50] ------------------------------------------
[01:40:50] error[E0433]: failed to resolve. Could not find `m` in `foo`
[01:40:50]   --> <anon>:20:28
[01:40:50]    |
[01:40:50] 20 |     fn f() { let s = S(0); ::foo::m!(s , 0); }
[01:40:50]    |                            ^^^^^^^^ Could not find `m` in `foo`
[01:40:50] 
[01:40:50] error: aborting due to previous error
[01:40:50] 
[01:40:50] 
[01:40:50] ------------------------------------------
[01:40:50] 
[01:40:50] thread '[pretty] run-pass/hygiene/issue-47312.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:40:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:40:50] 
[01:40:50] 
[01:40:50] failures:
[01:40:50]     [pretty] run-pass/hygiene/issue-47312.rs
