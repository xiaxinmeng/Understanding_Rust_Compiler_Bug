plain
2020-03-04T14:22:40.7145597Z ========================== Starting Command Output ===========================
2020-03-04T14:22:40.7152631Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fd1f4e58-4a0b-4abf-9d2b-09f97b26b08f.sh
2020-03-04T14:22:40.7153220Z 
2020-03-04T14:22:40.7160543Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-04T14:22:40.7183024Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69033/merge to s
2020-03-04T14:22:40.7187622Z Task         : Get sources
2020-03-04T14:22:40.7187962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T14:22:40.7188279Z Version      : 1.0.0
2020-03-04T14:22:40.7188526Z Author       : Microsoft
---
2020-03-04T14:22:41.7006971Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-04T14:22:41.7012075Z ##[command]git config gc.auto 0
2020-03-04T14:22:41.7015486Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-04T14:22:41.7018404Z ##[command]git config --get-all http.proxy
2020-03-04T14:22:41.7023891Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69033/merge:refs/remotes/pull/69033/merge
---
2020-03-04T15:25:20.0311715Z .................................................................................................... 1700/9728
2020-03-04T15:25:24.3703543Z .................................................................................................... 1800/9728
2020-03-04T15:25:35.2430529Z .......................................................i............................................ 1900/9728
2020-03-04T15:25:42.3437713Z .................................................................................................... 2000/9728
2020-03-04T15:25:55.6442976Z .............................................iiiii.................................................. 2100/9728
2020-03-04T15:26:05.0418012Z .................................................................................................... 2300/9728
2020-03-04T15:26:07.1408128Z .................................................................................................... 2400/9728
2020-03-04T15:26:10.4890771Z .................................................................................................... 2500/9728
2020-03-04T15:26:30.9999060Z .................................................................................................... 2600/9728
---
2020-03-04T15:29:01.6684682Z ......i...............i............................................................................. 5000/9728
2020-03-04T15:29:11.0766020Z .................................................................................................... 5100/9728
2020-03-04T15:29:15.5968968Z .................................................i.................................................. 5200/9728
2020-03-04T15:29:23.7625560Z .................................................................................................... 5300/9728
2020-03-04T15:29:30.7184851Z ............................ii.ii........i...i...................................................... 5400/9728
2020-03-04T15:29:38.5894065Z .................................................................................................... 5600/9728
2020-03-04T15:29:47.8966959Z .................................................................................................... 5700/9728
2020-03-04T15:29:55.0244017Z ...................i................................................................................ 5800/9728
2020-03-04T15:30:00.4190281Z .................................................................................................... 5900/9728
2020-03-04T15:30:00.4190281Z .................................................................................................... 5900/9728
2020-03-04T15:30:10.7737948Z .................................................................................................... 6000/9728
2020-03-04T15:30:21.2264505Z ...........ii...i..ii...........i................................................................... 6100/9728
2020-03-04T15:30:36.4239379Z .................................................................................................... 6300/9728
2020-03-04T15:30:43.1291861Z .................................................................................................... 6400/9728
2020-03-04T15:30:43.1291861Z .................................................................................................... 6400/9728
2020-03-04T15:30:56.1105794Z ..........................................i..ii..................................................... 6500/9728
2020-03-04T15:31:16.7377274Z .................................................................................................... 6700/9728
2020-03-04T15:31:18.7336911Z ..................................i................................................................. 6800/9728
2020-03-04T15:31:20.6595789Z .................................................................................................... 6900/9728
2020-03-04T15:31:22.7814130Z ................................................................i................................... 7000/9728
---
2020-03-04T15:32:57.7752442Z .................................................................................................... 7700/9728
2020-03-04T15:33:02.7602234Z .................................................................................................... 7800/9728
2020-03-04T15:33:07.6876660Z .................................................................................................... 7900/9728
2020-03-04T15:33:15.5539600Z ..........i......................................................................................... 8000/9728
2020-03-04T15:33:23.5057054Z ...........................................................iiiiiiiii.i.............................. 8100/9728
2020-03-04T15:33:37.1212879Z ..i......i.......................................................................................... 8300/9728
2020-03-04T15:33:42.1211242Z .................................................................................................... 8400/9728
2020-03-04T15:33:55.6748618Z .................................................................................................... 8500/9728
2020-03-04T15:34:03.4388155Z .................................................................................................... 8600/9728
---
2020-03-04T15:35:55.3390788Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-04T15:35:55.3391262Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-04T15:35:55.3396656Z 
2020-03-04T15:35:55.3396837Z 
2020-03-04T15:35:55.3403565Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-04T15:35:55.3406796Z 
2020-03-04T15:35:55.3406914Z 
2020-03-04T15:35:55.3407218Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-04T15:35:55.3407609Z Build completed unsuccessfully in 1:04:41
2020-03-04T15:35:55.3407609Z Build completed unsuccessfully in 1:04:41
2020-03-04T15:35:55.3455786Z == clock drift check ==
2020-03-04T15:35:55.3477912Z   local time: Wed Mar  4 15:35:55 UTC 2020
2020-03-04T15:35:55.6460939Z   network time: Wed, 04 Mar 2020 15:35:55 GMT
2020-03-04T15:35:55.6461638Z == end clock drift check ==
2020-03-04T15:35:56.2222710Z 
2020-03-04T15:35:56.2302006Z ##[error]Bash exited with code '1'.
2020-03-04T15:35:56.2315086Z ##[section]Finishing: Run build
2020-03-04T15:35:56.2364147Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69033/merge to s
2020-03-04T15:35:56.2369185Z Task         : Get sources
2020-03-04T15:35:56.2369512Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T15:35:56.2369794Z Version      : 1.0.0
2020-03-04T15:35:56.2369991Z Author       : Microsoft
2020-03-04T15:35:56.2369991Z Author       : Microsoft
2020-03-04T15:35:56.2370343Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-04T15:35:56.2370703Z ==============================================================================
2020-03-04T15:35:56.5467912Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-04T15:35:56.5512608Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69033/merge to s
2020-03-04T15:35:56.5620155Z Cleaning up task key
2020-03-04T15:35:56.5621566Z Start cleaning up orphan processes.
2020-03-04T15:35:56.5849731Z Terminate orphan process: pid (3735) (python)
2020-03-04T15:35:56.6071164Z ##[section]Finishing: Finalize Job
