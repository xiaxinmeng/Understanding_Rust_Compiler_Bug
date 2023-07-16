plain
2019-12-11T21:10:25.0576372Z ========================== Starting Command Output ===========================
2019-12-11T21:10:25.0577446Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8ec7e3d2-1471-464b-b41b-8547d959aac6.sh
2019-12-11T21:10:25.0577470Z 
2019-12-11T21:10:25.0579392Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-11T21:10:25.0584179Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67216/merge to s
2019-12-11T21:10:25.0585495Z Task         : Get sources
2019-12-11T21:10:25.0585565Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T21:10:25.0585591Z Version      : 1.0.0
2019-12-11T21:10:25.0585617Z Author       : Microsoft
---
2019-12-11T21:10:26.8567266Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-11T21:10:26.8734435Z ##[command]git config gc.auto 0
2019-12-11T21:10:26.8795225Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-11T21:10:26.9094810Z ##[command]git config --get-all http.proxy
2019-12-11T21:10:26.9222072Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67216/merge:refs/remotes/pull/67216/merge
---
2019-12-11T21:55:08.6762229Z .................................................................................................... 1600/9354
2019-12-11T21:55:12.0186866Z ....F............................................................................................... 1700/9354
2019-12-11T21:55:21.0342451Z ........................................................i........................................... 1800/9354
2019-12-11T21:55:27.0887578Z .................................................................................................... 1900/9354
2019-12-11T21:55:37.4751261Z .........................................iiiii...................................................... 2000/9354
2019-12-11T21:55:44.8923460Z .................................................................................................... 2200/9354
2019-12-11T21:55:46.6523778Z .................................................................................................... 2300/9354
2019-12-11T21:55:49.4291769Z .................................................................................................... 2400/9354
2019-12-11T21:56:05.7632330Z .................................................................................................... 2500/9354
---
2019-12-11T21:58:01.9434051Z .................................................i...............i.................................. 4800/9354
2019-12-11T21:58:07.9811780Z .................................................................................................... 4900/9354
2019-12-11T21:58:14.0035857Z .............................................................................................i...... 5000/9354
2019-12-11T21:58:18.1320739Z .................................................................................................... 5100/9354
2019-12-11T21:58:26.0172820Z ...........................................................ii.ii...........i........................ 5200/9354
2019-12-11T21:58:32.8478896Z .................................................................................................... 5400/9354
2019-12-11T21:58:40.4887559Z .................................................................................................... 5500/9354
2019-12-11T21:58:45.4630949Z .........................................i.......................................................... 5600/9354
2019-12-11T21:58:50.6444152Z .................................................................................................... 5700/9354
2019-12-11T21:58:50.6444152Z .................................................................................................... 5700/9354
2019-12-11T21:58:59.0859742Z .................................................................................................... 5800/9354
2019-12-11T21:59:07.2286237Z .............................ii...i..ii...........i................................................. 5900/9354
2019-12-11T21:59:20.9840301Z .................................................................................................... 6100/9354
2019-12-11T21:59:26.7469446Z .................................................................................................... 6200/9354
2019-12-11T21:59:26.7469446Z .................................................................................................... 6200/9354
2019-12-11T21:59:32.0201444Z .....................................................i..ii.......................................... 6300/9354
2019-12-11T21:59:51.6729078Z .................................................................................................... 6500/9354
2019-12-11T21:59:53.1585370Z .........................i.......................................................................... 6600/9354
2019-12-11T21:59:54.7235551Z .................................................................................................... 6700/9354
2019-12-11T21:59:56.4506916Z ................i................................................................................... 6800/9354
---
2019-12-11T22:01:08.7052020Z .................................................................................................... 7400/9354
2019-12-11T22:01:12.2569931Z .................................................................................................... 7500/9354
2019-12-11T22:01:17.3720026Z .................................................................................................... 7600/9354
2019-12-11T22:01:23.9413259Z .................................................................................................... 7700/9354
2019-12-11T22:01:29.5518177Z ................................iiii................................................................ 7800/9354
2019-12-11T22:01:40.3596789Z .................................................................................................... 8000/9354
2019-12-11T22:01:46.6859895Z .................................................................................................... 8100/9354
2019-12-11T22:01:57.3487762Z .................................................................................................... 8200/9354
2019-12-11T22:02:03.0634451Z .................................................................................................... 8300/9354
---
2019-12-11T22:03:27.2534868Z 1 error[E0716]: temporary value dropped while borrowed
2019-12-11T22:03:27.2535288Z -   --> $DIR/interior-mutability.rs:40:26
2019-12-11T22:03:27.2535705Z +   --> $DIR/interior-mutability.rs:44:26
2019-12-11T22:03:27.2535933Z 3    |
2019-12-11T22:03:27.2536310Z 4 LL |     let x: &'static _ = &X;
2019-12-11T22:03:27.2536755Z 5    |            ----------    ^ creates a temporary which is freed while still in use
2019-12-11T22:03:27.2537401Z 10    | - temporary value is freed at the end of this statement
2019-12-11T22:03:27.2537633Z 11 
2019-12-11T22:03:27.2537821Z 12 error[E0716]: temporary value dropped while borrowed
2019-12-11T22:03:27.2538224Z -   --> $DIR/interior-mutability.rs:41:26
2019-12-11T22:03:27.2538224Z -   --> $DIR/interior-mutability.rs:41:26
2019-12-11T22:03:27.2538628Z +   --> $DIR/interior-mutability.rs:45:26
2019-12-11T22:03:27.2538872Z 14    |
2019-12-11T22:03:27.2539263Z 15 LL |     let y: &'static _ = &Y;
2019-12-11T22:03:27.2539704Z 16    |            ----------    ^ creates a temporary which is freed while still in use
2019-12-11T22:03:27.2540473Z 21    | - temporary value is freed at the end of this statement
2019-12-11T22:03:27.2540755Z 22 
2019-12-11T22:03:27.2540944Z 23 error[E0716]: temporary value dropped while borrowed
2019-12-11T22:03:27.2541367Z -   --> $DIR/interior-mutability.rs:42:26
2019-12-11T22:03:27.2541367Z -   --> $DIR/interior-mutability.rs:42:26
2019-12-11T22:03:27.2541777Z +   --> $DIR/interior-mutability.rs:46:26
2019-12-11T22:03:27.2542025Z 25    |
2019-12-11T22:03:27.2542386Z 26 LL |     let z: &'static _ = &Z;
2019-12-11T22:03:27.2542826Z 27    |            ----------    ^ creates a temporary which is freed while still in use
2019-12-11T22:03:27.2543231Z 
2019-12-11T22:03:27.2543413Z The actual stderr differed from the expected stderr.
2019-12-11T22:03:27.2543854Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/interior-mutability.stderr
2019-12-11T22:03:27.2543854Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/interior-mutability.stderr
2019-12-11T22:03:27.2544495Z To update references, rerun the tests and pass the `--bless` flag
2019-12-11T22:03:27.2545033Z To only update this specific test, also pass `--test-args consts/control-flow/interior-mutability.rs`
2019-12-11T22:03:27.2548447Z error: 1 errors occurred comparing output.
2019-12-11T22:03:27.2550183Z status: exit code: 1
2019-12-11T22:03:27.2550183Z status: exit code: 1
2019-12-11T22:03:27.2551313Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/interior-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/auxiliary" "-A" "unused"
2019-12-11T22:03:27.2552155Z ------------------------------------------
2019-12-11T22:03:27.2552390Z 
2019-12-11T22:03:27.2552822Z ------------------------------------------
2019-12-11T22:03:27.2553073Z stderr:
2019-12-11T22:03:27.2553073Z stderr:
2019-12-11T22:03:27.2553465Z ------------------------------------------
2019-12-11T22:03:27.2553729Z error[E0716]: temporary value dropped while borrowed
2019-12-11T22:03:27.2554152Z   --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:44:26
2019-12-11T22:03:27.2554396Z    |
2019-12-11T22:03:27.2554843Z LL |     let x: &'static _ = &X; //~ ERROR temporary value dropped while borrowed
2019-12-11T22:03:27.2555827Z    |            ----------    ^ creates a temporary which is freed while still in use
2019-12-11T22:03:27.2556491Z    |            type annotation requires that borrow lasts for `'static`
2019-12-11T22:03:27.2556696Z ...
2019-12-11T22:03:27.2556828Z LL | }
2019-12-11T22:03:27.2557185Z    | - temporary value is freed at the end of this statement
2019-12-11T22:03:27.2557185Z    | - temporary value is freed at the end of this statement
2019-12-11T22:03:27.2557336Z 
2019-12-11T22:03:27.2557499Z error[E0716]: temporary value dropped while borrowed
2019-12-11T22:03:27.2557846Z   --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:45:26
2019-12-11T22:03:27.2559937Z    |
2019-12-11T22:03:27.2561263Z LL |     let y: &'static _ = &Y; //~ ERROR temporary value dropped while borrowed
2019-12-11T22:03:27.2561761Z    |            ----------    ^ creates a temporary which is freed while still in use
2019-12-11T22:03:27.2562385Z    |            type annotation requires that borrow lasts for `'static`
2019-12-11T22:03:27.2562385Z    |            type annotation requires that borrow lasts for `'static`
2019-12-11T22:03:27.2562819Z LL |     let z: &'static _ = &Z; //~ ERROR temporary value dropped while borrowed
2019-12-11T22:03:27.2563408Z    | - temporary value is freed at the end of this statement
2019-12-11T22:03:27.2563609Z 
2019-12-11T22:03:27.2563972Z error[E0716]: temporary value dropped while borrowed
2019-12-11T22:03:27.2564381Z   --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:46:26
2019-12-11T22:03:27.2564381Z   --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:46:26
2019-12-11T22:03:27.2565136Z    |
2019-12-11T22:03:27.2565757Z LL |     let z: &'static _ = &Z; //~ ERROR temporary value dropped while borrowed
2019-12-11T22:03:27.2566279Z    |            ----------    ^ creates a temporary which is freed while still in use
2019-12-11T22:03:27.2566895Z    |            type annotation requires that borrow lasts for `'static`
2019-12-11T22:03:27.2567086Z LL | }
2019-12-11T22:03:27.2567482Z    | - temporary value is freed at the end of this statement
2019-12-11T22:03:27.2567658Z 
---
2019-12-11T22:03:27.2572909Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-11T22:03:27.2573076Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-11T22:03:27.2573204Z 
2019-12-11T22:03:27.2573309Z 
2019-12-11T22:03:27.2574658Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-11T22:03:27.2575081Z 
2019-12-11T22:03:27.2575190Z 
2019-12-11T22:03:27.2575329Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-11T22:03:27.2575466Z Build completed unsuccessfully in 0:47:52
2019-12-11T22:03:27.2575466Z Build completed unsuccessfully in 0:47:52
2019-12-11T22:03:27.2621751Z == clock drift check ==
2019-12-11T22:03:27.2635735Z   local time: Wed Dec 11 22:03:27 UTC 2019
2019-12-11T22:03:27.7881409Z   network time: Wed, 11 Dec 2019 22:03:27 GMT
2019-12-11T22:03:27.7885752Z == end clock drift check ==
2019-12-11T22:03:29.1422487Z 
2019-12-11T22:03:29.1493593Z ##[error]Bash exited with code '1'.
2019-12-11T22:03:29.1503390Z ##[section]Finishing: Run build
2019-12-11T22:03:29.1530946Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67216/merge to s
2019-12-11T22:03:29.1532557Z Task         : Get sources
2019-12-11T22:03:29.1532629Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T22:03:29.1532666Z Version      : 1.0.0
2019-12-11T22:03:29.1532698Z Author       : Microsoft
2019-12-11T22:03:29.1532698Z Author       : Microsoft
2019-12-11T22:03:29.1532751Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-11T22:03:29.1532789Z ==============================================================================
2019-12-11T22:03:29.4851647Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-11T22:03:29.4881171Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67216/merge to s
2019-12-11T22:03:29.4965219Z Start cleaning up orphan processes.
2019-12-11T22:03:29.5047875Z Terminate orphan process: pid (4081) (python)
2019-12-11T22:03:29.5250867Z ##[section]Finishing: Finalize Job
2019-12-11T22:03:29.5294836Z ##[section]Finishing: Linux x86_64-gnu-llvm-7
