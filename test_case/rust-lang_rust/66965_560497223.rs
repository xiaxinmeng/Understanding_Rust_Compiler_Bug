plain
2019-12-02T16:13:43.0652812Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T16:13:43.0862443Z ##[command]git config gc.auto 0
2019-12-02T16:13:43.0952407Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T16:13:43.1016162Z ##[command]git config --get-all http.proxy
2019-12-02T16:13:43.1196820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66965/merge:refs/remotes/pull/66965/merge
---
2019-12-02T17:18:09.9003820Z .................................................................................................... 1600/9320
2019-12-02T17:18:14.9463597Z .................................................................................................... 1700/9320
2019-12-02T17:18:28.4737190Z ........................................i........................................................... 1800/9320
2019-12-02T17:18:36.8340931Z .................................................................................................... 1900/9320
2019-12-02T17:18:51.4424428Z .........................iiiii...................................................................... 2000/9320
2019-12-02T17:19:02.4320071Z .................................................................................................... 2200/9320
2019-12-02T17:19:05.1711110Z .................................................................................................... 2300/9320
2019-12-02T17:19:10.0564741Z .................................................................................................... 2400/9320
2019-12-02T17:19:33.0514034Z .................................................................................................... 2500/9320
---
2019-12-02T17:22:36.6024757Z ...........................i...............i........................................................ 4800/9320
2019-12-02T17:22:48.1109344Z .................................................................................................... 4900/9320
2019-12-02T17:22:54.6646831Z .................................................................................................... 5000/9320
2019-12-02T17:23:04.1436389Z .................................................................................................... 5100/9320
2019-12-02T17:23:11.5332233Z .................................ii.ii...........i.................................................. 5200/9320
2019-12-02T17:23:21.9102729Z .................................................................................................... 5400/9320
2019-12-02T17:23:32.4626893Z .................................................................................................... 5500/9320
2019-12-02T17:23:40.2967441Z ...............i.................................................................................... 5600/9320
2019-12-02T17:23:47.0314681Z .................................................................................................... 5700/9320
2019-12-02T17:23:47.0314681Z .................................................................................................... 5700/9320
2019-12-02T17:23:59.3270291Z .................................................................................................... 5800/9320
2019-12-02T17:24:12.0941082Z .ii...i..ii...........i............................................................................. 5900/9320
2019-12-02T17:24:31.9759285Z .................................................................................................... 6100/9320
2019-12-02T17:24:40.1882784Z .................................................................................................... 6200/9320
2019-12-02T17:24:40.1882784Z .................................................................................................... 6200/9320
2019-12-02T17:24:59.9969526Z ........................i..ii....................................................................... 6300/9320
2019-12-02T17:25:22.6954066Z ...............................................................................................i.... 6500/9320
2019-12-02T17:25:25.3837483Z .................................................................................................... 6600/9320
2019-12-02T17:25:28.0926673Z ......................................................................................i............. 6700/9320
2019-12-02T17:25:31.6326617Z .................................................................................................... 6800/9320
---
2019-12-02T17:30:54.4161964Z ---- [ui] ui/enum/union-in-enum.rs stdout ----
2019-12-02T17:30:54.4162331Z 
2019-12-02T17:30:54.4162483Z error: ui test compiled successfully!
2019-12-02T17:30:54.4162531Z status: exit code: 0
2019-12-02T17:30:54.4163448Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum/union-in-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/union-in-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/union-in-enum/auxiliary" "-A" "unused"
2019-12-02T17:30:54.4164098Z ------------------------------------------
2019-12-02T17:30:54.4164137Z 
2019-12-02T17:30:54.4164701Z ------------------------------------------
2019-12-02T17:30:54.4165232Z stderr:
2019-12-02T17:30:54.4165232Z stderr:
2019-12-02T17:30:54.4165679Z ------------------------------------------
2019-12-02T17:30:54.4165738Z warning: variant `union` should have an upper camel case name
2019-12-02T17:30:54.4166017Z   --> /checkout/src/test/ui/enum/union-in-enum.rs:1:10
2019-12-02T17:30:54.4166271Z    |
2019-12-02T17:30:54.4166356Z LL | enum A { union } //~ WARNING
2019-12-02T17:30:54.4166474Z    |          ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `Union`
2019-12-02T17:30:54.4166624Z    = note: `#[warn(non_camel_case_types)]` on by default
2019-12-02T17:30:54.4166688Z 
2019-12-02T17:30:54.4166762Z warning: variant `union` should have an upper camel case name
2019-12-02T17:30:54.4167099Z   --> /checkout/src/test/ui/enum/union-in-enum.rs:2:10
2019-12-02T17:30:54.4167099Z   --> /checkout/src/test/ui/enum/union-in-enum.rs:2:10
2019-12-02T17:30:54.4167168Z    |
2019-12-02T17:30:54.4167397Z LL | enum B { union {} } //~ WARNING
2019-12-02T17:30:54.4167516Z    |          ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `Union`
2019-12-02T17:30:54.4167605Z warning: variant `union` should have an upper camel case name
2019-12-02T17:30:54.4167939Z   --> /checkout/src/test/ui/enum/union-in-enum.rs:3:10
2019-12-02T17:30:54.4168194Z    |
2019-12-02T17:30:54.4168194Z    |
2019-12-02T17:30:54.4168395Z LL | enum C { union() } //~ WARNING
2019-12-02T17:30:54.4168718Z    |          ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `Union`
2019-12-02T17:30:54.4169001Z 
2019-12-02T17:30:54.4169493Z ------------------------------------------
2019-12-02T17:30:54.4169730Z 
2019-12-02T17:30:54.4169757Z 
---
2019-12-02T17:30:54.4226244Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-02T17:30:54.4227256Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-02T17:30:54.4243056Z 
2019-12-02T17:30:54.4243347Z 
2019-12-02T17:30:54.4245927Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-02T17:30:54.4246570Z 
2019-12-02T17:30:54.4246708Z 
2019-12-02T17:30:54.4252505Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-02T17:30:54.4252789Z Build completed unsuccessfully in 1:10:44
2019-12-02T17:30:54.4252789Z Build completed unsuccessfully in 1:10:44
2019-12-02T17:30:54.4319364Z == clock drift check ==
2019-12-02T17:30:54.4335747Z   local time: Mon Dec  2 17:30:54 UTC 2019
2019-12-02T17:30:54.7205457Z   network time: Mon, 02 Dec 2019 17:30:54 GMT
2019-12-02T17:30:54.7209983Z == end clock drift check ==
2019-12-02T17:30:55.4722963Z 
2019-12-02T17:30:55.4873821Z ##[error]Bash exited with code '1'.
2019-12-02T17:30:55.4909253Z ##[section]Starting: Checkout
2019-12-02T17:30:55.4911184Z ==============================================================================
2019-12-02T17:30:55.4911255Z Task         : Get sources
2019-12-02T17:30:55.4911303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
