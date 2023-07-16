plain
2019-09-21T14:11:38.1235527Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T14:11:38.9593473Z ##[command]git config gc.auto 0
2019-09-21T14:11:38.9597725Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T14:11:38.9601751Z ##[command]git config --get-all http.proxy
2019-09-21T14:11:38.9603961Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64635/merge:refs/remotes/pull/64635/merge
---
2019-09-21T15:09:03.5951897Z .................................................................................................... 1500/9032
2019-09-21T15:09:09.3389633Z .............................................................F...................................... 1600/9032
2019-09-21T15:09:20.9689737Z ........................................................................i...............i........... 1700/9032
2019-09-21T15:09:27.6197146Z .................................................................................................... 1800/9032
2019-09-21T15:09:36.1118312Z ...............................................................iiiii................................ 1900/9032
2019-09-21T15:09:54.9926271Z .................................................................................................... 2100/9032
2019-09-21T15:09:57.3656017Z .................................................................................................... 2200/9032
2019-09-21T15:10:00.3858530Z .................................................................................................... 2300/9032
2019-09-21T15:10:08.3514309Z .................................................................................................... 2400/9032
---
2019-09-21T15:12:55.6050531Z ...................................................i...............i................................ 4700/9032
2019-09-21T15:13:04.2341730Z .................................................................................................... 4800/9032
2019-09-21T15:13:12.0352699Z .................................................................................................... 4900/9032
2019-09-21T15:13:20.3891387Z .................................................................................................... 5000/9032
2019-09-21T15:13:28.2671405Z ...................................ii.ii............................................................ 5100/9032
2019-09-21T15:13:37.4095273Z .................................................................................................... 5300/9032
2019-09-21T15:13:47.1746715Z ...................................................................................................i 5400/9032
2019-09-21T15:13:54.8393295Z .................................................................................................... 5500/9032
2019-09-21T15:13:59.3840236Z .................................................................................................... 5600/9032
2019-09-21T15:13:59.3840236Z .................................................................................................... 5600/9032
2019-09-21T15:14:09.7613901Z ..............................................................................................ii...i 5700/9032
2019-09-21T15:14:22.8438903Z ..ii............i................................................................................... 5800/9032
2019-09-21T15:14:42.5655448Z .................................................................................................... 6000/9032
2019-09-21T15:14:48.5637702Z ................................................................................................i..i 6100/9032
2019-09-21T15:15:01.9800903Z i................................................................................................... 6200/9032
2019-09-21T15:15:12.0695107Z .................................................................................................... 6300/9032
---
2019-09-21T15:19:15.6734594Z 
2019-09-21T15:19:15.6735129Z ---- [ui] ui/consts/issue-56164.rs stdout ----
2019-09-21T15:19:15.6735475Z diff of stderr:
2019-09-21T15:19:15.6735529Z 
2019-09-21T15:19:15.6735571Z 4 LL | const fn foo() { (||{})() }
2019-09-21T15:19:15.6735672Z 6 
2019-09-21T15:19:15.6736178Z - error: function pointers are not allowed in const fn
2019-09-21T15:19:15.6736251Z + error: function pointers in `const fn` are unstable
2019-09-21T15:19:15.6736454Z 8   --> $DIR/issue-56164.rs:8:5
2019-09-21T15:19:15.6736454Z 8   --> $DIR/issue-56164.rs:8:5
2019-09-21T15:19:15.6736491Z 9    |
2019-09-21T15:19:15.6736542Z 10 LL |     input()
2019-09-21T15:19:15.6736565Z 
2019-09-21T15:19:15.6736585Z 
2019-09-21T15:19:15.6736621Z The actual stderr differed from the expected stderr.
2019-09-21T15:19:15.6736871Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164/issue-56164.stderr
2019-09-21T15:19:15.6737078Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T15:19:15.6737287Z To only update this specific test, also pass `--test-args consts/issue-56164.rs`
2019-09-21T15:19:15.6737367Z error: 1 errors occurred comparing output.
2019-09-21T15:19:15.6737402Z status: exit code: 1
2019-09-21T15:19:15.6737402Z status: exit code: 1
2019-09-21T15:19:15.6737984Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-56164.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164/auxiliary" "-A" "unused"
2019-09-21T15:19:15.6738262Z ------------------------------------------
2019-09-21T15:19:15.6738290Z 
2019-09-21T15:19:15.6738462Z ------------------------------------------
2019-09-21T15:19:15.6738499Z stderr:
2019-09-21T15:19:15.6738499Z stderr:
2019-09-21T15:19:15.6738736Z ------------------------------------------
2019-09-21T15:19:15.6738781Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-21T15:19:15.6738987Z   --> /checkout/src/test/ui/consts/issue-56164.rs:3:18
2019-09-21T15:19:15.6739239Z    |
2019-09-21T15:19:15.6739273Z LL | const fn foo() { (||{})() }
2019-09-21T15:19:15.6739348Z 
2019-09-21T15:19:15.6739385Z error: function pointers in `const fn` are unstable
2019-09-21T15:19:15.6739791Z   --> /checkout/src/test/ui/consts/issue-56164.rs:8:5
2019-09-21T15:19:15.6739847Z    |
---
2019-09-21T15:19:15.6775593Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-21T15:19:15.6776045Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T15:19:15.6791775Z 
2019-09-21T15:19:15.6792469Z 
2019-09-21T15:19:15.6795456Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T15:19:15.6796010Z 
2019-09-21T15:19:15.6796154Z 
2019-09-21T15:19:15.6799093Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-21T15:19:15.6799322Z Build completed unsuccessfully in 1:00:57
2019-09-21T15:19:15.6799322Z Build completed unsuccessfully in 1:00:57
2019-09-21T15:19:15.6848923Z == clock drift check ==
2019-09-21T15:19:15.6886592Z   local time: Sat Sep 21 15:19:15 UTC 2019
2019-09-21T15:19:15.8895256Z   network time: Sat, 21 Sep 2019 15:19:15 GMT
2019-09-21T15:19:15.8896918Z == end clock drift check ==
2019-09-21T15:19:16.8024619Z ##[error]Bash exited with code '1'.
2019-09-21T15:19:16.8057654Z ##[section]Starting: Checkout
2019-09-21T15:19:16.8059229Z ==============================================================================
2019-09-21T15:19:16.8059273Z Task         : Get sources
2019-09-21T15:19:16.8059325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
