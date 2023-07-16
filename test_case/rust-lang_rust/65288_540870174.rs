plain
2019-10-11T01:00:38.8477950Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T01:00:38.8646929Z ##[command]git config gc.auto 0
2019-10-11T01:00:38.8694282Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T01:00:38.8757101Z ##[command]git config --get-all http.proxy
2019-10-11T01:00:38.8908257Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65288/merge:refs/remotes/pull/65288/merge
---
2019-10-11T02:03:34.0084282Z .................................................................................................... 1600/9146
2019-10-11T02:03:41.7312750Z .................................................................................................... 1700/9146
2019-10-11T02:03:53.2842608Z .................i...............i.................................................................. 1800/9146
2019-10-11T02:04:00.8070210Z .................................................................................................... 1900/9146
2019-10-11T02:04:16.2573533Z ........iiiii....................................................................................... 2000/9146
2019-10-11T02:04:26.5732585Z .................................................................................................... 2200/9146
2019-10-11T02:04:29.3261106Z .................................................................................................... 2300/9146
2019-10-11T02:04:35.2637499Z .................................................................................................... 2400/9146
2019-10-11T02:04:41.8385518Z .................................................................................................... 2500/9146
---
2019-10-11T02:07:41.1076987Z .................................................................................................... 4700/9146
2019-10-11T02:07:48.6446108Z .i...............i.................................................................................. 4800/9146
2019-10-11T02:08:00.2037311Z .................................................................................................... 4900/9146
2019-10-11T02:08:06.1271155Z .................................................................................................... 5000/9146
2019-10-11T02:08:17.7665446Z ...............................................................................................ii.ii 5100/9146
2019-10-11T02:08:28.6305705Z .................................................................................................... 5300/9146
2019-10-11T02:08:38.7364539Z .................................................................................................... 5400/9146
2019-10-11T02:08:45.6901691Z .............................................................i...................................... 5500/9146
2019-10-11T02:08:53.2280765Z .................................................................................................... 5600/9146
2019-10-11T02:08:53.2280765Z .................................................................................................... 5600/9146
2019-10-11T02:09:00.8912399Z .................................................................................................... 5700/9146
2019-10-11T02:09:11.3979551Z ..........................................................ii...i..ii...........i.................... 5800/9146
2019-10-11T02:09:38.2851965Z .................................................................................................... 6000/9146
2019-10-11T02:09:47.8264589Z .................................................................................................... 6100/9146
2019-10-11T02:09:47.8264589Z .................................................................................................... 6100/9146
2019-10-11T02:09:55.8815292Z ................................................................i..ii............................... 6200/9146
2019-10-11T02:10:25.9743433Z .................................................................................................... 6400/9146
2019-10-11T02:10:28.2228311Z ........................i........................................................................... 6500/9146
2019-10-11T02:10:30.6411422Z .................................................................................................i.. 6600/9146
2019-10-11T02:10:33.4754230Z .................................................................................................... 6700/9146
---
2019-10-11T02:14:55.5007597Z failures:
2019-10-11T02:14:55.5007685Z 
2019-10-11T02:14:55.5008203Z ---- [compile-fail] compile-fail/chalkify/impl_wf.rs stdout ----
2019-10-11T02:14:55.5008255Z 
2019-10-11T02:14:55.5008706Z error: /checkout/src/test/compile-fail/chalkify/impl_wf.rs:27: unexpected error: '27:5: 27:21: the trait bound `f32: Foo` is not satisfied [E0277]'
2019-10-11T02:14:55.5008858Z error: 1 unexpected errors found, 0 expected errors not found
2019-10-11T02:14:55.5008934Z status: exit code: 1
2019-10-11T02:14:55.5008934Z status: exit code: 1
2019-10-11T02:14:55.5009820Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/chalkify/impl_wf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/chalkify/impl_wf" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "chalk" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/chalkify/impl_wf/auxiliary" "-A" "unused"
2019-10-11T02:14:55.5009976Z unexpected errors (from JSON output): [
2019-10-11T02:14:55.5010031Z     Error {
2019-10-11T02:14:55.5010087Z         line_num: 27,
2019-10-11T02:14:55.5010157Z         kind: Some(
2019-10-11T02:14:55.5010529Z         ),
2019-10-11T02:14:55.5010529Z         ),
2019-10-11T02:14:55.5010643Z         msg: "27:5: 27:21: the trait bound `f32: Foo` is not satisfied [E0277]",
2019-10-11T02:14:55.5010752Z ]
2019-10-11T02:14:55.5010947Z 
2019-10-11T02:14:55.5011830Z thread '[compile-fail] compile-fail/chalkify/impl_wf.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-10-11T02:14:55.5011928Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-10-11T02:14:55.5018292Z FAILED. 31 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-11T02:14:55.5018338Z 
2019-10-11T02:14:55.5018675Z 
2019-10-11T02:14:55.5018738Z 
2019-10-11T02:14:55.5027495Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-11T02:14:55.5027859Z 
2019-10-11T02:14:55.5027894Z 
2019-10-11T02:14:55.5043836Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-11T02:14:55.5104682Z Build completed unsuccessfully in 1:07:48
2019-10-11T02:14:55.5104682Z Build completed unsuccessfully in 1:07:48
2019-10-11T02:14:55.5111148Z == clock drift check ==
2019-10-11T02:14:55.5138928Z   local time: Fri Oct 11 02:14:55 UTC 2019
2019-10-11T02:14:55.5520690Z   network time: Fri, 11 Oct 2019 02:14:55 GMT
2019-10-11T02:14:55.5522388Z == end clock drift check ==
2019-10-11T02:14:55.9846185Z ##[error]Bash exited with code '1'.
2019-10-11T02:14:55.9900355Z ##[section]Starting: Checkout
2019-10-11T02:14:55.9902760Z ==============================================================================
2019-10-11T02:14:55.9902825Z Task         : Get sources
2019-10-11T02:14:55.9902898Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
