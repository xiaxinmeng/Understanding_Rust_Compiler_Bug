plain
2019-09-30T20:48:13.1977367Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T20:48:14.1642136Z ##[command]git config gc.auto 0
2019-09-30T20:48:14.1645218Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T20:48:14.1648215Z ##[command]git config --get-all http.proxy
2019-09-30T20:48:14.1651104Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64700/merge:refs/remotes/pull/64700/merge
---
2019-09-30T21:54:37.0750373Z .................................................................................................... 1500/9077
2019-09-30T21:54:43.9753911Z .................................................................................................... 1600/9077
2019-09-30T21:54:53.7944288Z .................................................................................................i.. 1700/9077
2019-09-30T21:55:02.5813493Z .............i...................................................................................... 1800/9077
2019-09-30T21:55:09.7887440Z ........................................................................................iiiii....... 1900/9077
2019-09-30T21:55:32.7742937Z .................................................................................................... 2100/9077
2019-09-30T21:55:35.3356418Z .................................................................................................... 2200/9077
2019-09-30T21:55:38.0874245Z .................................................................................................... 2300/9077
2019-09-30T21:55:44.8238406Z .................................................................................................... 2400/9077
---
2019-09-30T21:58:52.6512977Z ............................................................................i...............i....... 4700/9077
2019-09-30T21:59:01.6563396Z .................................................................................................... 4800/9077
2019-09-30T21:59:11.9192338Z .................................................................................................... 4900/9077
2019-09-30T21:59:18.4460901Z .................................................................................................... 5000/9077
2019-09-30T21:59:30.2184748Z ................................................................ii.ii............................... 5100/9077
2019-09-30T21:59:40.8971865Z .................................................................................................... 5300/9077
2019-09-30T21:59:51.3014647Z .................................................................................................... 5400/9077
2019-09-30T21:59:59.3058074Z ..............................i..................................................................... 5500/9077
2019-09-30T22:00:05.9373301Z .................................................................................................... 5600/9077
2019-09-30T22:00:05.9373301Z .................................................................................................... 5600/9077
2019-09-30T22:00:18.9701638Z .................................................................................................... 5700/9077
2019-09-30T22:00:31.1003002Z .........................ii...i..ii...........i..................................................... 5800/9077
2019-09-30T22:00:54.5741035Z .................................................................................................... 6000/9077
2019-09-30T22:01:01.4112124Z .................................................................................................... 6100/9077
2019-09-30T22:01:01.4112124Z .................................................................................................... 6100/9077
2019-09-30T22:01:16.5155757Z ............................i..ii................................................................... 6200/9077
2019-09-30T22:01:37.0946696Z .......................................................................................i............ 6400/9077
2019-09-30T22:01:39.4653573Z .................................................................................................... 6500/9077
2019-09-30T22:01:41.9813253Z ...........................................................i........................................ 6600/9077
2019-09-30T22:01:45.2198153Z .................................................................................................... 6700/9077
---
2019-09-30T22:06:00.7055289Z failures:
2019-09-30T22:06:00.7103648Z 
2019-09-30T22:06:00.7104343Z ---- [ui] ui/const-generics/const-expression-parameter.rs stdout ----
2019-09-30T22:06:00.7104412Z 
2019-09-30T22:06:00.7105128Z error: /checkout/src/test/ui/const-generics/const-expression-parameter.rs:64: unexpected error: '64:26: 64:28: expected one of `!`, `(`, `,`, `>`, `?`, `for`, lifetime, or path, found `42`'
2019-09-30T22:06:00.7105181Z 
2019-09-30T22:06:00.7105567Z error: /checkout/src/test/ui/const-generics/const-expression-parameter.rs:68: unexpected error: '68:16: 68:18: expected one of `!`, `(`, `,`, `>`, `?`, `for`, lifetime, or path, found `42`'
2019-09-30T22:06:00.7105691Z error: 2 unexpected errors found, 0 expected errors not found
2019-09-30T22:06:00.7105757Z status: exit code: 1
2019-09-30T22:06:00.7105757Z status: exit code: 1
2019-09-30T22:06:00.7106526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-expression-parameter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-expression-parameter" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-expression-parameter/auxiliary" "-A" "unused"
2019-09-30T22:06:00.7106651Z unexpected errors (from JSON output): [
2019-09-30T22:06:00.7106697Z     Error {
2019-09-30T22:06:00.7106743Z         line_num: 64,
2019-09-30T22:06:00.7106804Z         kind: Some(
2019-09-30T22:06:00.7106895Z         ),
2019-09-30T22:06:00.7106895Z         ),
2019-09-30T22:06:00.7106963Z         msg: "64:26: 64:28: expected one of `!`, `(`, `,`, `>`, `?`, `for`, lifetime, or path, found `42`",
2019-09-30T22:06:00.7107054Z     Error {
2019-09-30T22:06:00.7107113Z         line_num: 68,
2019-09-30T22:06:00.7107155Z         kind: Some(
2019-09-30T22:06:00.7107198Z             Error,
2019-09-30T22:06:00.7107198Z             Error,
2019-09-30T22:06:00.7107238Z         ),
2019-09-30T22:06:00.7107306Z         msg: "68:16: 68:18: expected one of `!`, `(`, `,`, `>`, `?`, `for`, lifetime, or path, found `42`",
2019-09-30T22:06:00.7107392Z ]
2019-09-30T22:06:00.7107436Z 
2019-09-30T22:06:00.7107749Z thread '[ui] ui/const-generics/const-expression-parameter.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-09-30T22:06:00.7107822Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-09-30T22:06:00.7108516Z 
2019-09-30T22:06:00.7154341Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-30T22:06:00.7168503Z 
2019-09-30T22:06:00.7168611Z 
2019-09-30T22:06:00.7180345Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-30T22:06:00.7182155Z 
2019-09-30T22:06:00.7182365Z 
2019-09-30T22:06:00.7190602Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-30T22:06:00.7191073Z Build completed unsuccessfully in 1:10:02
2019-09-30T22:06:00.7191073Z Build completed unsuccessfully in 1:10:02
2019-09-30T22:06:00.7260078Z == clock drift check ==
2019-09-30T22:06:00.7284522Z   local time: Mon Sep 30 22:06:00 UTC 2019
2019-09-30T22:06:00.8815118Z   network time: Mon, 30 Sep 2019 22:06:00 GMT
2019-09-30T22:06:00.8818852Z == end clock drift check ==
2019-09-30T22:06:02.0782558Z ##[error]Bash exited with code '1'.
2019-09-30T22:06:02.0831580Z ##[section]Starting: Checkout
2019-09-30T22:06:02.0833834Z ==============================================================================
2019-09-30T22:06:02.0833907Z Task         : Get sources
2019-09-30T22:06:02.0833953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
