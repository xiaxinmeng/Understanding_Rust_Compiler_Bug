plain
[00:57:01] ........................i...........................................................................
[00:57:06] ...............................................................................................i....
[00:57:10] ....................................................................................................
[00:57:17] ....................................................................................................
[00:57:23] .............................................................................F......................
[00:57:36] ....................................................................................................
[00:57:45] ....................................................................................................
[00:57:50] i..................................................................................................i
[00:57:57] ..ii................................................................................................
---
[00:58:41] error: /checkout/src/test/compile-fail/issue-32829.rs:11: expected error not found: could not evaluate static initializer
[00:58:41] 
[00:58:41] error: 0 unexpected errors found, 1 expected errors not found
[00:58:41] status: exit code: 101
[00:58:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-32829.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32829/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32829/auxiliary" "-A" "unused"
[00:58:41]     Error {
[00:58:41]         line_num: 11,
[00:58:41]         kind: Some(
[00:58:41]             Error
[00:58:41]             Error
[00:58:41]         ),
[00:58:41]         msg: "could not evaluate static initializer"
[00:58:41] ]
[00:58:41] 
[00:58:41] 
[00:58:41] thread '[compile-fail] compile-fail/issue-32829.rs' panicked at 'explicit panic', tools/cndroid-cross-path" "" "--color" "always"
[00:58:41] 
[00:58:41] 
[00:58:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:41] Build completed unsuccessfully in 0:14:55
[00:58:41] Build completed unsuccessfully in 0:14:55
[00:58:41] Makefile:58: recipe for target 'check' failed
[00:58:41] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:22a8c3ec
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
