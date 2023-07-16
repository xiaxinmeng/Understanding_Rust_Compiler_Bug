plain
travis_time:end:01de1745:start=1543567390310570939,finish=1543567392722327107,duration=2411756168
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:13] .................................................................................................... 100/5065
[00:53:16] .................................................................................................... 200/5065
[00:53:19] .............................ii............................................ii...................ii.. 300/5065
[00:53:22] ..............................................................................................iii... 400/5065
[00:53:25] .....iiiiiiii.iii............................iii...........................................i........ 500/5065
[00:53:32] .................................................................................................... 700/5065
[00:53:38] ................................................................................................i... 800/5065
[00:53:42] ........i........................................................................................... 900/5065
[00:53:45] ...............iiiii..................ii.iiii....................................................... 1000/5065
---
[00:54:28] .................................................................................................... 2300/5065
[00:54:32] .................................................................................................... 2400/5065
[00:54:36] .................................................................................................... 2500/5065
[00:54:40] .................................................................................................... 2600/5065
[00:54:44] ........iiiiiiiii................................................................................... 2700/5065
[00:54:50] .................................................................................................... 2900/5065
[00:54:55] .................................................................................................... 3000/5065
[00:54:59] ......................................................................i............................. 3100/5065
[00:55:02] .................................................................................................... 3200/5065
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:47] 
[01:09:47] running 119 tests
[01:09:50] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[01:09:51] .ii.i.....iiii.....
[01:09:51] 
[01:09:51]  finished in 4.037
[01:09:51] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:07] 
[01:10:07] running 118 tests
[01:10:34] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:10:38] ......iii.i.....ii
[01:10:38] 
[01:10:38]  finished in 31.550
[01:10:38] travis_fold:end:test_debuginfo

---
[01:48:47]         line_num: 1,
[01:48:47]         kind: Some(
[01:48:47]             Error
[01:48:47]         ),
[01:48:47]         msg: "1:1: 1:1: Missing code example in this documentation [missing_doc_code_examples]"
[01:48:47]     Error {
[01:48:47]         line_num: 13,
[01:48:47]         kind: Some(
[01:48:47]             Error
[01:48:47]             Error
[01:48:47]         ),
[01:48:47]         msg: "13:1: 13:15: Missing code example in this documentation [missing_doc_code_examples]"
[01:48:47]     Error {
[01:48:47]         line_num: 16,
[01:48:47]         kind: Some(
[01:48:47]             Error
[01:48:47]             Error
[01:48:47]         ),
[01:48:47]         msg: "16:1: 16:33: Missing code example in this documentation [missing_doc_code_examples]"
[01:48:47]     Error {
[01:48:47]         line_num: 18,
[01:48:47]         kind: Some(
[01:48:47]             Error
[01:48:47]             Error
[01:48:47]         ),
[01:48:47]         msg: "18:5: 18:48: Missing code example in this documentation [missing_doc_code_examples]"
[01:48:47] ]
[01:48:47] 
[01:48:47] thread '[ui] rustdoc-ui/doc-without-codeblock.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[01:48:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:48:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:48:47] 
[01:48:47] ---- [ui] rustdoc-ui/intra-link-span-ice-55723.rs stdout ----
[01:48:47] 
[01:48:47] error: /checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs:21: unexpected error: '21:10: 21:11: `[i]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
[01:48:47] error: 1 unexpected errors found, 0 expected errors not found
[01:48:47] status: exit code: 1
[01:48:47] status: exit code: 1
[01:48:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/auxiliary"
[01:48:47]     Error {
[01:48:47]         line_num: 21,
[01:48:47]         kind: Some(
[01:48:47]             Error
[01:48:47]             Error
[01:48:47]         ),
[01:48:47]         msg: "21:10: 21:11: `[i]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]"
[01:48:47] ]
[01:48:47] 
[01:48:47] thread '[ui] rustdoc-ui/intra-link-span-ice-55723.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[01:48:47] 
[01:48:47] 
[01:48:47] ---- [ui] rustdoc-ui/private-item-doc-test.rs stdout ----
[01:48:47] 
[01:48:47] error: /checkout/src/test/rustdoc-ui/private-item-doc-test.rs:14: unexpected error: '14:5: 18:12: Documentation test in private item [private_doc_tests]'
[01:48:47] error: 1 unexpected errors found, 0 expected errors not found
[01:48:47] status: exit code: 1
[01:48:47] status: exit code: 1
[01:48:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/private-item-doc-test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test/auxiliary"
[01:48:47]     Error {
[01:48:47]         line_num: 14,
[01:48:47]         kind: Some(
[01:48:47]             Error
[01:48:47]             Error
[01:48:47]         ),
[01:48:47]         msg: "14:5: 18:12: Documentation test in private item [private_doc_tests]"
[01:48:47] ]
[01:48:47] 
[01:48:47] thread '[ui] rustdoc-ui/private-item-doc-test.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[01:48:47] 
---
[01:48:47] test result: FAILED. 7 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:48:47] 
[01:48:47] 
[01:48:47] 
[01:48:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:48:47] 
[01:48:47] 
[01:48:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:47] Build completed unsuccessfully in 0:59:43
[01:48:47] Build completed unsuccessfully in 0:59:43
[01:48:47] Makefile:58: recipe for target 'check' failed
[01:48:47] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c4914fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 10:32:08 UTC 2018
