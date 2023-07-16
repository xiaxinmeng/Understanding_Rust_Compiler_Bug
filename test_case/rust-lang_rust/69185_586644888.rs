plain
2020-02-15T20:38:51.6398166Z ========================== Starting Command Output ===========================
2020-02-15T20:38:51.6402605Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/eea98c0a-7650-4427-9db7-c3e230226dac.sh
2020-02-15T20:38:51.6552344Z 
2020-02-15T20:38:51.6628125Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T20:38:51.6634897Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T20:38:51.6636598Z Task         : Get sources
2020-02-15T20:38:51.6636689Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T20:38:51.6636725Z Version      : 1.0.0
2020-02-15T20:38:51.6636761Z Author       : Microsoft
---
2020-02-15T20:38:57.1582905Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T20:38:57.1856931Z ##[command]git config gc.auto 0
2020-02-15T20:38:57.1935761Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T20:38:57.2000586Z ##[command]git config --get-all http.proxy
2020-02-15T20:38:57.2162689Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69185/merge:refs/remotes/pull/69185/merge
---
2020-02-15T21:42:25.5359270Z .................................................................................................... 1700/9664
2020-02-15T21:42:30.4780321Z .................................................................................................... 1800/9664
2020-02-15T21:42:42.8205771Z ............................................i....................................................... 1900/9664
2020-02-15T21:42:51.4580091Z .................................................................................................... 2000/9664
2020-02-15T21:43:06.4518458Z ..................................iiiii............................................................. 2100/9664
2020-02-15T21:43:16.8368925Z .................................................................................................... 2300/9664
2020-02-15T21:43:19.3993425Z .................................................................................................... 2400/9664
2020-02-15T21:43:24.0714004Z .................................................................................................... 2500/9664
2020-02-15T21:43:46.4704787Z .................................................................................................... 2600/9664
---
2020-02-15T21:46:35.1499715Z .......i............................................................................................ 5000/9664
2020-02-15T21:46:45.2880732Z .................................................................................................... 5100/9664
2020-02-15T21:46:50.6220055Z .................................i.................................................................. 5200/9664
2020-02-15T21:47:01.4053033Z .................................................................................................... 5300/9664
2020-02-15T21:47:07.6017054Z .........ii.ii........i...i......................................................................... 5400/9664
2020-02-15T21:47:16.8052264Z .................................................................................................... 5600/9664
2020-02-15T21:47:28.8801511Z .................................................................................................... 5700/9664
2020-02-15T21:47:37.1645966Z .i.................................................................................................. 5800/9664
2020-02-15T21:47:42.8178988Z ...................................................................................................i 5900/9664
2020-02-15T21:47:42.8178988Z ...................................................................................................i 5900/9664
2020-02-15T21:47:53.8501453Z .............................................................................................ii...i. 6000/9664
2020-02-15T21:48:06.7948553Z .ii...........i..................................................................................... 6100/9664
2020-02-15T21:48:25.0422794Z .................................................................................................... 6300/9664
2020-02-15T21:48:31.9254966Z .................................................................................................... 6400/9664
2020-02-15T21:48:31.9254966Z .................................................................................................... 6400/9664
2020-02-15T21:48:49.8487594Z .....................i..ii.......................................................................... 6500/9664
2020-02-15T21:49:11.7345623Z .................................................................................................... 6700/9664
2020-02-15T21:49:14.1303100Z .........i.......................................................................................... 6800/9664
2020-02-15T21:49:16.5165166Z .................................................................................................... 6900/9664
2020-02-15T21:49:19.0201319Z ...................i................................................................................ 7000/9664
---
2020-02-15T21:51:06.7899037Z .................................................................................................... 7700/9664
2020-02-15T21:51:12.7789864Z .................................................................................................... 7800/9664
2020-02-15T21:51:19.4866894Z .................................................................................................... 7900/9664
2020-02-15T21:51:30.6376867Z .................................................................................................... 8000/9664
2020-02-15T21:51:36.7922800Z .iiiiiii.i.......................................................................................... 8100/9664
2020-02-15T21:51:52.0399543Z .................................................................................................... 8300/9664
2020-02-15T21:52:03.5920202Z .................................................................................................... 8400/9664
2020-02-15T21:52:16.6063188Z .................................................................................................... 8500/9664
2020-02-15T21:52:22.9093915Z .................................................................................................... 8600/9664
---
2020-02-15T21:54:51.7101183Z  finished in 7.743
2020-02-15T21:54:51.7277032Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:54:51.8882689Z 
2020-02-15T21:54:51.8883571Z running 178 tests
2020-02-15T21:54:54.8970693Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-15T21:54:57.3301724Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-15T21:54:57.3305957Z 
2020-02-15T21:54:57.3306025Z  finished in 5.602
2020-02-15T21:54:57.3516325Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:54:57.5136967Z 
---
2020-02-15T21:54:59.5765047Z  finished in 2.224
2020-02-15T21:54:59.5967788Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:54:59.7614374Z 
2020-02-15T21:54:59.7616826Z running 9 tests
2020-02-15T21:54:59.7618092Z iiiiiiiii
2020-02-15T21:54:59.7618572Z 
2020-02-15T21:54:59.7624579Z  finished in 0.165
2020-02-15T21:54:59.7798329Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:54:59.9439644Z 
2020-02-15T21:54:59.9439644Z 
2020-02-15T21:54:59.9440469Z running 115 tests
2020-02-15T21:55:18.3226533Z .................................................................................................... 100/115
2020-02-15T21:55:20.4858393Z .............F.
2020-02-15T21:55:20.4858616Z failures:
2020-02-15T21:55:20.4860967Z 
2020-02-15T21:55:20.4864603Z ---- [incremental] incremental/warnings-reemitted.rs stdout ----
2020-02-15T21:55:20.4864667Z 
2020-02-15T21:55:20.4864948Z error in revision `cfail1`: test compilation failed although it shouldn't!
2020-02-15T21:55:20.4865103Z status: exit code: 1
2020-02-15T21:55:20.4866417Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/warnings-reemitted.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted/warnings-reemitted.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Coverflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted/auxiliary"
2020-02-15T21:55:20.4866806Z ------------------------------------------
2020-02-15T21:55:20.4867051Z 
2020-02-15T21:55:20.4867334Z ------------------------------------------
2020-02-15T21:55:20.4867381Z stderr:
2020-02-15T21:55:20.4867381Z stderr:
2020-02-15T21:55:20.4867601Z ------------------------------------------
2020-02-15T21:55:20.4867670Z error: this arithmetic operation will overflow
2020-02-15T21:55:20.4868023Z   --> /checkout/src/test/incremental/warnings-reemitted.rs:8:13
2020-02-15T21:55:20.4868071Z    |
2020-02-15T21:55:20.4868135Z LL |     let _ = 255u8 + 1; //~ WARNING attempt to add with overflow
2020-02-15T21:55:20.4868306Z    |             ^^^^^^^^^ attempt to add with overflow
2020-02-15T21:55:20.4868357Z    |
2020-02-15T21:55:20.4868537Z    = note: `#[deny(overflow)]` on by default
2020-02-15T21:55:20.4868606Z error: aborting due to previous error
2020-02-15T21:55:20.4868633Z 
2020-02-15T21:55:20.4868672Z 
2020-02-15T21:55:20.4869012Z ------------------------------------------
---
2020-02-15T21:55:20.4875890Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-15T21:55:20.4875947Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-15T21:55:20.4884897Z 
2020-02-15T21:55:20.4885199Z 
2020-02-15T21:55:20.4890810Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-15T21:55:20.4891059Z 
2020-02-15T21:55:20.4891106Z 
2020-02-15T21:55:20.4912949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-15T21:55:20.4913172Z Build completed unsuccessfully in 1:08:57
2020-02-15T21:55:20.4913172Z Build completed unsuccessfully in 1:08:57
2020-02-15T21:55:20.4964185Z == clock drift check ==
2020-02-15T21:55:20.4987832Z   local time: Sat Feb 15 21:55:20 UTC 2020
2020-02-15T21:55:20.6609744Z   network time: Sat, 15 Feb 2020 21:55:20 GMT
2020-02-15T21:55:20.6613980Z == end clock drift check ==
2020-02-15T21:55:23.0893377Z 
2020-02-15T21:55:23.1006966Z ##[error]Bash exited with code '1'.
2020-02-15T21:55:23.1026479Z ##[section]Finishing: Run build
2020-02-15T21:55:23.1051311Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T21:55:23.1053347Z Task         : Get sources
2020-02-15T21:55:23.1053412Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T21:55:23.1053462Z Version      : 1.0.0
2020-02-15T21:55:23.1053650Z Author       : Microsoft
2020-02-15T21:55:23.1053650Z Author       : Microsoft
2020-02-15T21:55:23.1053697Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T21:55:23.1053763Z ==============================================================================
2020-02-15T21:55:23.5624210Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T21:55:23.5667474Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T21:55:23.5825128Z Cleaning up task key
2020-02-15T21:55:23.5826019Z Start cleaning up orphan processes.
2020-02-15T21:55:23.6371488Z Terminate orphan process: pid (5405) (python)
2020-02-15T21:55:23.6413957Z ##[section]Finishing: Finalize Job
