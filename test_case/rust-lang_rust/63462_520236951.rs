plain
2019-08-11T14:13:25.3162210Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T14:13:25.3341033Z ##[command]git config gc.auto 0
2019-08-11T14:13:25.3419024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T14:13:25.3483648Z ##[command]git config --get-all http.proxy
2019-08-11T14:13:25.3623251Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63462/merge:refs/remotes/pull/63462/merge
---
2019-08-11T14:13:59.9067129Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T14:13:59.9067170Z 
2019-08-11T14:13:59.9067694Z   git checkout -b <new-branch-name>
2019-08-11T14:13:59.9067725Z 
2019-08-11T14:13:59.9067775Z HEAD is now at 036dc5877 Merge 9dbfc67696a3832e19458700ee503b6103df3450 into 2b78e10ac1454d2d4190c575f6ece03f484ac398
2019-08-11T14:13:59.9221795Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T14:13:59.9224107Z ==============================================================================
2019-08-11T14:13:59.9224173Z Task         : Bash
2019-08-11T14:13:59.9224210Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T15:15:30.9229144Z .................................................................................................... 1300/8873
2019-08-11T15:15:37.6661774Z .................................................................................................... 1400/8873
2019-08-11T15:15:43.9765109Z .................................................................................................... 1500/8873
2019-08-11T15:15:54.7275220Z ....................................................................................i............... 1600/8873
2019-08-11T15:16:02.5400008Z i................................................................................................... 1700/8873
2019-08-11T15:16:09.0692375Z ................F...........................................................iiiii................... 1800/8873
2019-08-11T15:16:31.4679062Z .................................................................................................... 2000/8873
2019-08-11T15:16:33.9691223Z .................................................................................................... 2100/8873
2019-08-11T15:16:36.7184180Z .................................................................................................... 2200/8873
2019-08-11T15:16:44.5691425Z .................................................................................................... 2300/8873
---
2019-08-11T15:20:42.1272416Z .................................................................................................... 5300/8873
2019-08-11T15:20:49.4393879Z ........i........................................................................................... 5400/8873
2019-08-11T15:20:54.9181069Z .................................................................................................... 5500/8873
2019-08-11T15:21:07.3578276Z .................................................................................................... 5600/8873
2019-08-11T15:21:21.7115566Z ...ii...i..ii...........i........................................................................... 5700/8873
2019-08-11T15:21:36.9436608Z .................................................................................................... 5900/8873
2019-08-11T15:21:41.6597020Z .................................................................................................... 6000/8873
2019-08-11T15:21:41.6597020Z .................................................................................................... 6000/8873
2019-08-11T15:21:56.1110945Z ....i..ii........................................................................................... 6100/8873
2019-08-11T15:22:15.1078460Z ...............................................i.................................................... 6300/8873
2019-08-11T15:22:17.2010417Z .................................................................................................... 6400/8873
2019-08-11T15:22:19.6678702Z ...................i................................................................................ 6500/8873
2019-08-11T15:22:24.2111184Z .................................................................................................... 6600/8873
---
2019-08-11T15:26:24.9887319Z failures:
2019-08-11T15:26:24.9914907Z 
2019-08-11T15:26:24.9915344Z ---- [ui] ui/deriving/deriving-hash.rs stdout ----
2019-08-11T15:26:24.9915586Z 
2019-08-11T15:26:24.9915839Z error: test compilation failed although it shouldn't!
2019-08-11T15:26:24.9915884Z status: exit code: 1
2019-08-11T15:26:24.9916537Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deriving/deriving-hash.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-hash/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-hash/auxiliary" "-A" "unused"
2019-08-11T15:26:24.9916835Z ------------------------------------------
2019-08-11T15:26:24.9916866Z 
2019-08-11T15:26:24.9917054Z ------------------------------------------
2019-08-11T15:26:24.9917111Z stderr:
2019-08-11T15:26:24.9917111Z stderr:
2019-08-11T15:26:24.9917306Z ------------------------------------------
2019-08-11T15:26:24.9917354Z error[E0194]: type parameter `__H` shadows another type parameter of the same name
2019-08-11T15:26:24.9917641Z    |
2019-08-11T15:26:24.9917641Z    |
2019-08-11T15:26:24.9917683Z LL | #[derive(Hash)] enum Collision<__H> { __H { __H__H: __H } }
2019-08-11T15:26:24.9917955Z    |          |
2019-08-11T15:26:24.9917995Z    |          shadows another type parameter
2019-08-11T15:26:24.9917995Z    |          shadows another type parameter
2019-08-11T15:26:24.9918036Z    |          first `__H` declared here
2019-08-11T15:26:24.9918510Z error: aborting due to previous error
2019-08-11T15:26:24.9918540Z 
2019-08-11T15:26:24.9918835Z For more information about this error, try `rustc --explain E0194`.
2019-08-11T15:26:24.9918890Z 
---
2019-08-11T15:26:24.9950644Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-11T15:26:24.9950728Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-11T15:26:24.9974129Z 
2019-08-11T15:26:24.9974206Z 
2019-08-11T15:26:24.9977302Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-11T15:26:24.9977600Z 
2019-08-11T15:26:24.9977630Z 
2019-08-11T15:26:24.9981143Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-11T15:26:24.9981237Z Build completed unsuccessfully in 1:06:05
2019-08-11T15:26:24.9981237Z Build completed unsuccessfully in 1:06:05
2019-08-11T15:26:25.8104202Z ##[error]Bash exited with code '1'.
2019-08-11T15:26:25.8147538Z ##[section]Starting: Checkout
2019-08-11T15:26:25.8181223Z ==============================================================================
2019-08-11T15:26:25.8181303Z Task         : Get sources
2019-08-11T15:26:25.8181355Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
