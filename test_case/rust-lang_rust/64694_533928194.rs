plain
2019-09-22T22:16:10.8634016Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T22:16:10.8850634Z ##[command]git config gc.auto 0
2019-09-22T22:16:10.8927794Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T22:16:10.8970735Z ##[command]git config --get-all http.proxy
2019-09-22T22:16:10.9099979Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64694/merge:refs/remotes/pull/64694/merge
---
2019-09-22T23:14:52.4579324Z .................................................................................................... 1500/9035
2019-09-22T23:14:58.5196481Z .................................................................................................... 1600/9035
2019-09-22T23:15:10.7578333Z ........................................................................i...............i........... 1700/9035
2019-09-22T23:15:17.5120198Z .................................................................................................... 1800/9035
2019-09-22T23:15:26.2271165Z ...............................................................iiiii................................ 1900/9035
2019-09-22T23:15:45.1323602Z .................................................................................................... 2100/9035
2019-09-22T23:15:47.5813478Z .................................................................................................... 2200/9035
2019-09-22T23:15:50.7635092Z .................................................................................................... 2300/9035
2019-09-22T23:15:59.1821935Z .................................................................................................... 2400/9035
---
2019-09-22T23:19:01.4048625Z ....................................................i..............i................................ 4700/9035
2019-09-22T23:19:10.9149173Z .................................................................................................... 4800/9035
2019-09-22T23:19:19.6422375Z .................................................................................................... 4900/9035
2019-09-22T23:19:27.4545698Z .................................................................................................... 5000/9035
2019-09-22T23:19:37.4332695Z ......................................ii.ii......................................................... 5100/9035
2019-09-22T23:19:47.5999189Z .................................................................................................... 5300/9035
2019-09-22T23:19:58.4947882Z .................................................................................................... 5400/9035
2019-09-22T23:20:06.3421862Z ...i................................................................................................ 5500/9035
2019-09-22T23:20:11.8861398Z .................................................................................................... 5600/9035
2019-09-22T23:20:11.8861398Z .................................................................................................... 5600/9035
2019-09-22T23:20:23.7942093Z ..................................................................................................ii 5700/9035
2019-09-22T23:20:37.7232023Z ...i..ii...........i................................................................................ 5800/9035
2019-09-22T23:21:00.1582859Z .................................................................................................... 6000/9035
2019-09-22T23:21:06.7565211Z .................................................................................................... 6100/9035
2019-09-22T23:21:06.7565211Z .................................................................................................... 6100/9035
2019-09-22T23:21:21.0696863Z i..ii............................................................................................... 6200/9035
2019-09-22T23:21:40.0807025Z ...........................................................i........................................ 6400/9035
2019-09-22T23:21:42.2848888Z .................................................................................................... 6500/9035
2019-09-22T23:21:44.8455687Z ...............................i.................................................................... 6600/9035
2019-09-22T23:21:49.0698388Z .................................................................................................... 6700/9035
---
2019-09-22T23:26:21.6417478Z  finished in 5.425
2019-09-22T23:26:21.6609180Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T23:26:21.8466813Z 
2019-09-22T23:26:21.8467263Z running 150 tests
2019-09-22T23:26:25.2723164Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-22T23:26:27.2691593Z ..iiii..............i.........iii.i.......ii......
2019-09-22T23:26:27.2692175Z 
2019-09-22T23:26:27.2692330Z  finished in 5.608
2019-09-22T23:26:27.2887877Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T23:26:27.4540221Z 
---
2019-09-22T23:26:29.5595587Z  finished in 2.270
2019-09-22T23:26:29.5800721Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T23:26:29.7377909Z 
2019-09-22T23:26:29.7378692Z running 9 tests
2019-09-22T23:26:29.7381003Z iiiiiiiii
2019-09-22T23:26:29.7381850Z 
2019-09-22T23:26:29.7386325Z  finished in 0.158
2019-09-22T23:26:29.7598963Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T23:26:29.9400709Z 
---
2019-09-22T23:26:48.3844662Z  finished in 18.624
2019-09-22T23:26:48.4083700Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T23:26:48.5924008Z 
2019-09-22T23:26:48.5924349Z running 123 tests
2019-09-22T23:27:12.7292345Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-22T23:27:17.3164855Z i.i.i......iii.i.....ii
2019-09-22T23:27:17.3167603Z 
2019-09-22T23:27:17.3169544Z  finished in 28.908
2019-09-22T23:27:17.3218354Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T23:27:17.3218678Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-22T23:28:17.6109151Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-09-22T23:28:17.6109488Z 
2019-09-22T23:28:17.6109615Z error: ui test compiled successfully!
2019-09-22T23:28:17.6109736Z status: exit code: 0
2019-09-22T23:28:17.6110874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-attr-register-deny.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-A" "unused"
2019-09-22T23:28:17.6112044Z ------------------------------------------
2019-09-22T23:28:17.6112230Z 
2019-09-22T23:28:17.6112631Z ------------------------------------------
2019-09-22T23:28:17.6112822Z stderr:
---
2019-09-22T23:28:17.6115411Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-22T23:28:17.6115633Z 
2019-09-22T23:28:17.6115915Z 
2019-09-22T23:28:17.6116034Z 
2019-09-22T23:28:17.6117545Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-22T23:28:17.6155687Z 
2019-09-22T23:28:17.6155968Z 
2019-09-22T23:28:17.6156258Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-22T23:28:17.6156407Z Build completed unsuccessfully in 1:05:11
2019-09-22T23:28:17.6156407Z Build completed unsuccessfully in 1:05:11
2019-09-22T23:28:17.6182968Z == clock drift check ==
2019-09-22T23:28:17.6199515Z   local time: Sun Sep 22 23:28:17 UTC 2019
2019-09-22T23:28:17.8996362Z   network time: Sun, 22 Sep 2019 23:28:17 GMT
2019-09-22T23:28:17.8996517Z == end clock drift check ==
2019-09-22T23:28:18.7936575Z ##[error]Bash exited with code '1'.
2019-09-22T23:28:18.7970176Z ##[section]Starting: Checkout
2019-09-22T23:28:18.7972505Z ==============================================================================
2019-09-22T23:28:18.7972563Z Task         : Get sources
2019-09-22T23:28:18.7972627Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
