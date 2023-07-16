plain
2019-11-16T23:59:54.8145285Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T23:59:54.8314728Z ##[command]git config gc.auto 0
2019-11-16T23:59:54.8377221Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T23:59:54.8410077Z ##[command]git config --get-all http.proxy
2019-11-16T23:59:54.8553503Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66120/merge:refs/remotes/pull/66120/merge
---
2019-11-17T00:54:46.5093087Z .................................................................................................... 1500/9250
2019-11-17T00:54:52.7723959Z ....................................................................F............................... 1600/9250
2019-11-17T00:55:00.8353957Z .................................................................................................... 1700/9250
2019-11-17T00:55:09.6483634Z .........i.......................................................................................... 1800/9250
2019-11-17T00:55:15.8808113Z .............................................................................................iiiii.. 1900/9250
2019-11-17T00:55:36.0232616Z .................................................................................................... 2100/9250
2019-11-17T00:55:38.1814112Z .................................................................................................... 2200/9250
2019-11-17T00:55:40.5087381Z .................................................................................................... 2300/9250
2019-11-17T00:55:46.4314452Z .................................................................................................... 2400/9250
---
2019-11-17T00:59:21.0748803Z .................................................................................................... 5400/9250
2019-11-17T00:59:31.3364875Z ...............................................................................i.................... 5500/9250
2019-11-17T00:59:39.0984016Z .................................................................................................... 5600/9250
2019-11-17T00:59:45.4823974Z .................................................................................................... 5700/9250
2019-11-17T00:59:55.7728216Z .................................................................ii...i..ii...........i............. 5800/9250
2019-11-17T01:00:17.4276489Z .................................................................................................... 6000/9250
2019-11-17T01:00:25.4112260Z .................................................................................................... 6100/9250
2019-11-17T01:00:25.4112260Z .................................................................................................... 6100/9250
2019-11-17T01:00:30.5496346Z ....................................................................................i..ii........... 6200/9250
2019-11-17T01:00:57.7480154Z .................................................................................................... 6400/9250
2019-11-17T01:01:02.4726458Z ....................................................i............................................... 6500/9250
2019-11-17T01:01:04.6945326Z .................................................................................................... 6600/9250
2019-11-17T01:01:07.1510730Z ........................................i........................................................... 6700/9250
---
2019-11-17T01:05:31.0624519Z ......................................................i............................................. 9200/9250
2019-11-17T01:05:40.8602862Z ..................................................
2019-11-17T01:05:40.8603074Z failures:
2019-11-17T01:05:40.8634540Z 
2019-11-17T01:05:40.8635863Z ---- [ui] ui/consts/const_in_pattern/warn_corner_cases.rs stdout ----
2019-11-17T01:05:40.8636090Z 
2019-11-17T01:05:40.8636683Z error: /checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs:26: unexpected warning: '26:47: 26:52: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-17T01:05:40.8636860Z 
2019-11-17T01:05:40.8637962Z error: /checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs:31: unexpected warning: '31:47: 31:51: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-17T01:05:40.8638285Z 
2019-11-17T01:05:40.8638935Z error: /checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs:36: unexpected warning: '36:47: 36:58: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-17T01:05:40.8639251Z error: 3 unexpected errors found, 0 expected errors not found
2019-11-17T01:05:40.8639397Z status: exit code: 0
2019-11-17T01:05:40.8639397Z status: exit code: 0
2019-11-17T01:05:40.8640350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/warn_corner_cases/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/warn_corner_cases/auxiliary"
2019-11-17T01:05:40.8640620Z unexpected errors (from JSON output): [
2019-11-17T01:05:40.8640907Z         line_num: 26,
2019-11-17T01:05:40.8641026Z         kind: Some(
2019-11-17T01:05:40.8641143Z             Warning,
2019-11-17T01:05:40.8641290Z         ),
2019-11-17T01:05:40.8641290Z         ),
2019-11-17T01:05:40.8641422Z         msg: "26:47: 26:52: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-17T01:05:40.8641685Z     Error {
2019-11-17T01:05:40.8641800Z         line_num: 31,
2019-11-17T01:05:40.8641939Z         kind: Some(
2019-11-17T01:05:40.8642060Z             Warning,
2019-11-17T01:05:40.8642060Z             Warning,
2019-11-17T01:05:40.8642173Z         ),
2019-11-17T01:05:40.8642320Z         msg: "31:47: 31:51: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-17T01:05:40.8642560Z     Error {
2019-11-17T01:05:40.8642691Z         line_num: 36,
2019-11-17T01:05:40.8642805Z         kind: Some(
2019-11-17T01:05:40.8643066Z             Warning,
2019-11-17T01:05:40.8643066Z             Warning,
2019-11-17T01:05:40.8643206Z         ),
2019-11-17T01:05:40.8643334Z         msg: "36:47: 36:58: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-17T01:05:40.8643589Z ]
2019-11-17T01:05:40.8643688Z 
2019-11-17T01:05:40.8643688Z 
2019-11-17T01:05:40.8644116Z thread '[ui] ui/consts/const_in_pattern/warn_corner_cases.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-17T01:05:40.8644433Z 
2019-11-17T01:05:40.8644533Z 
2019-11-17T01:05:40.8644665Z failures:
2019-11-17T01:05:40.8644665Z failures:
2019-11-17T01:05:40.8644787Z     [ui] ui/consts/const_in_pattern/warn_corner_cases.rs
2019-11-17T01:05:40.8645468Z test result: FAILED. 9206 passed; 1 failed; 43 ignored; 0 measured; 0 filtered out
2019-11-17T01:05:40.8645611Z 
2019-11-17T01:05:40.8664369Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-17T01:05:40.8678938Z 
2019-11-17T01:05:40.8678938Z 
2019-11-17T01:05:40.8679258Z 
2019-11-17T01:05:40.8681126Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-17T01:05:40.8682956Z 
2019-11-17T01:05:40.8683148Z 
2019-11-17T01:05:40.8685584Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-17T01:05:40.8685916Z Build completed unsuccessfully in 0:59:46
2019-11-17T01:05:40.8685916Z Build completed unsuccessfully in 0:59:46
2019-11-17T01:05:40.8737092Z == clock drift check ==
2019-11-17T01:05:41.8598418Z   local time: Sun Nov 17 01:05:40 UTC 2019
2019-11-17T01:05:41.8607928Z   network time: Sun, 17 Nov 2019 01:05:41 GMT
2019-11-17T01:05:41.8607991Z == end clock drift check ==
2019-11-17T01:05:43.2500944Z 
2019-11-17T01:05:43.2607695Z ##[error]Bash exited with code '1'.
2019-11-17T01:05:43.2654782Z ##[section]Starting: Checkout
2019-11-17T01:05:43.2656343Z ==============================================================================
2019-11-17T01:05:43.2656390Z Task         : Get sources
2019-11-17T01:05:43.2656431Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
