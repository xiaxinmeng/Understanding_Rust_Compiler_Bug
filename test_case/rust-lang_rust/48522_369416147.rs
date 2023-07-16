
[01:46:36] failures:
[01:46:36] 
[01:46:36] ---- [pretty] run-pass/issue-48508.rs stdout ----
[01:46:36] 	
[01:46:36] error: pretty-printed source does not typecheck
[01:46:36] status: exit code: 101
[01:46:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48508.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-48508.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g"
[01:46:36] stdout:
[01:46:36] ------------------------------------------
[01:46:36] 
[01:46:36] ------------------------------------------
[01:46:36] stderr:
[01:46:36] ------------------------------------------
[01:46:36] error[E0425]: cannot find function `other` in module `other_file`
[01:46:36]   --> <anon>:25:25
[01:46:36]    |
[01:46:36] 25 | fn main() { other_file::other(); }
[01:46:36]    |                         ^^^^^ not found in `other_file`
[01:46:36] 
[01:46:36] error: aborting due to previous error
[01:46:36] 
[01:46:36] If you want more information on this error, try using "rustc --explain E0425"
[01:46:36] 
[01:46:36] ------------------------------------------
[01:46:36] 
[01:46:36] thread '[pretty] run-pass/issue-48508.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2893:9
[01:46:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:46:36] 
[01:46:36] 
[01:46:36] failures:
[01:46:36]     [pretty] run-pass/issue-48508.rs
[01:46:36] 
[01:46:36] test result: [31mFAILED(B[m. 2879 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out
[01:46:36] 
[01:46:36] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
