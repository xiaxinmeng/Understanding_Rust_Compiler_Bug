plain
[00:50:33] .................................................................................................... 3300/4314
[00:50:36] .................................................................................................... 3400/4314
[00:50:39] .................................................................................................... 3500/4314
[00:50:43] .................................i.................................................................. 3600/4314
[00:50:47] .............................................................F...................................... 3700/4314
[00:50:54] ...................................................................................................i 3900/4314
[00:50:57] .................................................................................................... 4000/4314
[00:51:01] .................................................................................................... 4100/4314
[00:51:03] .................................................................................................... 4200/4314
[00:51:03] .................................................................................................... 4200/4314
[00:51:06] 
[00:51:06] error: /checkout/src/test/ui/self/suggest-self.rs:27: expected error not found: cannot find value `this` in this scope
[00:51:06] error: 1 unexpected errors found, 1 expected errors not found
[00:51:06] status: exit code: 1
[00:51:06] status: exit code: 1
[00:51:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/suggest-self.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self/auxiliary" "-A" "unused"
[00:51:06]     Error {
[00:51:06]         line_num: 27,
[00:51:06]         kind: Some(
[00:51:06]             Error
[00:51:06]             Error
[00:51:06]         ),
[00:51:06]         msg: "27:9: 27:11: cannot find value `my` in this scope [E0425]"
[00:51:06] ]
[00:51:06] 
[00:51:06] not found errors (from test file): [
[00:51:06]     Error {
[00:51:06]     Error {
[00:51:06]         line_num: 27,
[00:51:06]         kind: Some(
[00:51:06]             Error
[00:51:06]         ),
[00:51:06]         msg: "cannot find value `this` in this scope"
[00:51:06] ]
[00:51:06] 
[00:51:06] thread '[ui] ui/self/suggest-self.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1341:13
[00:51:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
