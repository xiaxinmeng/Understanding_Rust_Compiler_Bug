plain
2020-01-12T04:33:40.2436360Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-12T04:33:40.2446337Z ##[command]git config gc.auto 0
2020-01-12T04:33:40.2448658Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-12T04:33:40.2450556Z ##[command]git config --get-all http.proxy
2020-01-12T04:33:40.2453353Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68143/merge:refs/remotes/pull/68143/merge
---
2020-01-12T05:29:47.0677081Z .................................................................................................... 1600/9517
2020-01-12T05:29:52.4135379Z .................................................................................................... 1700/9517
2020-01-12T05:30:01.4228023Z .................................................................................................... 1800/9517
2020-01-12T05:30:10.5766468Z ........i........................................................................................... 1900/9517
2020-01-12T05:30:17.6540976Z ..................................................................................................ii 2000/9517
2020-01-12T05:30:34.2555835Z iii................................................................................................. 2100/9517
2020-01-12T05:30:42.7446944Z .................................................................................................... 2300/9517
2020-01-12T05:30:45.2735083Z .................................................................................................... 2400/9517
2020-01-12T05:30:51.2184254Z .................................................................................................... 2500/9517
2020-01-12T05:31:12.1017480Z .................................................................................................... 2600/9517
---
2020-01-12T05:33:57.1281698Z .........................................i...............i.......................................... 4900/9517
2020-01-12T05:34:06.5534011Z .................................................................................................... 5000/9517
2020-01-12T05:34:13.4321484Z ....................................................................................i............... 5100/9517
2020-01-12T05:34:19.0756937Z .................................................................................................... 5200/9517
2020-01-12T05:34:29.7741324Z .......................................................ii.ii...........i............................ 5300/9517
2020-01-12T05:34:39.3004806Z .................................................................................................... 5500/9517
2020-01-12T05:34:49.6671119Z .................................................................................................... 5600/9517
2020-01-12T05:34:56.3960693Z ........................................i........................................................... 5700/9517
2020-01-12T05:35:03.3239147Z .................................................................................................... 5800/9517
2020-01-12T05:35:03.3239147Z .................................................................................................... 5800/9517
2020-01-12T05:35:14.4537499Z .................................................................................................... 5900/9517
2020-01-12T05:35:24.5572448Z ...............................ii...i..ii...........i............................................... 6000/9517
2020-01-12T05:35:43.6866823Z .................................................................................................... 6200/9517
2020-01-12T05:35:52.0972267Z .................................................................................................... 6300/9517
2020-01-12T05:35:52.0972267Z .................................................................................................... 6300/9517
2020-01-12T05:36:04.9560048Z ...........................................................i..ii.................................... 6400/9517
2020-01-12T05:36:33.5128432Z .................................................................................................... 6600/9517
2020-01-12T05:36:35.7055807Z ..................................i................................................................. 6700/9517
2020-01-12T05:36:37.9721627Z .................................................................................................... 6800/9517
2020-01-12T05:36:40.6261252Z ..................................i................................................................. 6900/9517
---
2020-01-12T05:38:21.0323832Z .................................................................................................... 7500/9517
2020-01-12T05:38:25.5244152Z .................................................................................................... 7600/9517
2020-01-12T05:38:31.7008898Z .................................................................................................... 7700/9517
2020-01-12T05:38:39.1171726Z .................................................................................................... 7800/9517
2020-01-12T05:38:49.2982240Z ...................................................................................iiii............. 7900/9517
2020-01-12T05:39:06.3968665Z .................i.......i.......................................................................... 8100/9517
2020-01-12T05:39:11.8571327Z .................................................................................................... 8200/9517
2020-01-12T05:39:25.4747957Z .................................................................................................... 8300/9517
2020-01-12T05:39:36.1234844Z .................................................................................................... 8400/9517
---
2020-01-12T05:41:38.9546662Z 1 error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9546929Z -   --> $DIR/const-param-elided-lifetime.rs:4:19
2020-01-12T05:41:38.9547186Z +   --> $DIR/const-param-elided-lifetime.rs:9:19
2020-01-12T05:41:38.9547232Z 3    |
2020-01-12T05:41:38.9547275Z 4 LL | struct A<const N: &u8>;
2020-01-12T05:41:38.9547371Z 
2020-01-12T05:41:38.9547410Z 6 
2020-01-12T05:41:38.9547471Z 7 error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9547715Z -   --> $DIR/const-param-elided-lifetime.rs:8:15
2020-01-12T05:41:38.9547715Z -   --> $DIR/const-param-elided-lifetime.rs:8:15
2020-01-12T05:41:38.9547936Z +   --> $DIR/const-param-elided-lifetime.rs:13:15
2020-01-12T05:41:38.9547983Z 9    |
2020-01-12T05:41:38.9548039Z 10 LL | impl<const N: &u8> A<N> {
2020-01-12T05:41:38.9548127Z 
2020-01-12T05:41:38.9548166Z 12 
2020-01-12T05:41:38.9548227Z 13 error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9548451Z -   --> $DIR/const-param-elided-lifetime.rs:9:21
2020-01-12T05:41:38.9548451Z -   --> $DIR/const-param-elided-lifetime.rs:9:21
2020-01-12T05:41:38.9548969Z +   --> $DIR/const-param-elided-lifetime.rs:14:21
2020-01-12T05:41:38.9549046Z 15    |
2020-01-12T05:41:38.9549089Z 16 LL |     fn foo<const M: &u8>(&self) {}
2020-01-12T05:41:38.9549183Z 
2020-01-12T05:41:38.9549222Z 18 
2020-01-12T05:41:38.9549477Z 19 error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9549759Z -   --> $DIR/const-param-elided-lifetime.rs:13:15
2020-01-12T05:41:38.9549759Z -   --> $DIR/const-param-elided-lifetime.rs:13:15
2020-01-12T05:41:38.9550004Z +   --> $DIR/const-param-elided-lifetime.rs:18:15
2020-01-12T05:41:38.9550049Z 21    |
2020-01-12T05:41:38.9550091Z 22 LL | impl<const N: &u8> B for A<N> {}
2020-01-12T05:41:38.9550290Z 
2020-01-12T05:41:38.9550336Z 24 
2020-01-12T05:41:38.9550383Z 25 error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9550661Z -   --> $DIR/const-param-elided-lifetime.rs:16:17
2020-01-12T05:41:38.9550661Z -   --> $DIR/const-param-elided-lifetime.rs:16:17
2020-01-12T05:41:38.9550880Z +   --> $DIR/const-param-elided-lifetime.rs:21:17
2020-01-12T05:41:38.9550924Z 27    |
2020-01-12T05:41:38.9550980Z 28 LL | fn bar<const N: &u8>() {}
2020-01-12T05:41:38.9551059Z 
2020-01-12T05:41:38.9551112Z 30 
2020-01-12T05:41:38.9551171Z 31 warning: the feature `const_generics` is incomplete and may cause the compiler to crash
2020-01-12T05:41:38.9551399Z -   --> $DIR/const-param-elided-lifetime.rs:1:12
2020-01-12T05:41:38.9551399Z -   --> $DIR/const-param-elided-lifetime.rs:1:12
2020-01-12T05:41:38.9551617Z +   --> $DIR/const-param-elided-lifetime.rs:6:12
2020-01-12T05:41:38.9551676Z 33    |
2020-01-12T05:41:38.9551718Z 34 LL | #![feature(const_generics)]
2020-01-12T05:41:38.9551762Z 35    |            ^^^^^^^^^^^^^^
2020-01-12T05:41:38.9551814Z 
2020-01-12T05:41:38.9551841Z 
2020-01-12T05:41:38.9551884Z The actual stderr differed from the expected stderr.
2020-01-12T05:41:38.9552215Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime/const-param-elided-lifetime.stderr
2020-01-12T05:41:38.9552476Z To update references, rerun the tests and pass the `--bless` flag
2020-01-12T05:41:38.9552756Z To only update this specific test, also pass `--test-args const-generics/const-param-elided-lifetime.rs`
2020-01-12T05:41:38.9552858Z error: 1 errors occurred comparing output.
2020-01-12T05:41:38.9552902Z status: exit code: 1
2020-01-12T05:41:38.9552902Z status: exit code: 1
2020-01-12T05:41:38.9553761Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime/auxiliary" "-A" "unused"
2020-01-12T05:41:38.9554083Z ------------------------------------------
2020-01-12T05:41:38.9554130Z 
2020-01-12T05:41:38.9554338Z ------------------------------------------
2020-01-12T05:41:38.9554393Z stderr:
2020-01-12T05:41:38.9554393Z stderr:
2020-01-12T05:41:38.9554611Z ------------------------------------------
2020-01-12T05:41:38.9554663Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9554915Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:9:19
2020-01-12T05:41:38.9555179Z    |
2020-01-12T05:41:38.9555229Z LL | struct A<const N: &u8>;
2020-01-12T05:41:38.9555321Z 
2020-01-12T05:41:38.9555379Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9555681Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:13:15
2020-01-12T05:41:38.9555729Z    |
2020-01-12T05:41:38.9555729Z    |
2020-01-12T05:41:38.9555794Z LL | impl<const N: &u8> A<N> { //~ ERROR `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9556313Z 
2020-01-12T05:41:38.9556379Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9562063Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:14:21
2020-01-12T05:41:38.9562142Z    |
2020-01-12T05:41:38.9562142Z    |
2020-01-12T05:41:38.9563145Z LL |     fn foo<const M: &u8>(&self) {}
2020-01-12T05:41:38.9563258Z 
2020-01-12T05:41:38.9563438Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9563882Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:18:15
2020-01-12T05:41:38.9563934Z    |
2020-01-12T05:41:38.9563934Z    |
2020-01-12T05:41:38.9563978Z LL | impl<const N: &u8> B for A<N> {}
2020-01-12T05:41:38.9564071Z 
2020-01-12T05:41:38.9564116Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-01-12T05:41:38.9564376Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:21:17
2020-01-12T05:41:38.9564450Z    |
2020-01-12T05:41:38.9564450Z    |
2020-01-12T05:41:38.9564490Z LL | fn bar<const N: &u8>() {}
2020-01-12T05:41:38.9564583Z 
2020-01-12T05:41:38.9564631Z warning: the feature `const_generics` is incomplete and may cause the compiler to crash
2020-01-12T05:41:38.9566877Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:6:12
2020-01-12T05:41:38.9566967Z    |
---
2020-01-12T05:41:38.9568814Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-12T05:41:38.9568882Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-12T05:41:38.9568933Z 
2020-01-12T05:41:38.9568958Z 
2020-01-12T05:41:38.9570579Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-12T05:41:38.9570994Z 
2020-01-12T05:41:38.9571024Z 
2020-01-12T05:41:38.9627049Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-12T05:41:38.9627366Z Build completed unsuccessfully in 1:02:08
2020-01-12T05:41:38.9627366Z Build completed unsuccessfully in 1:02:08
2020-01-12T05:41:38.9632690Z == clock drift check ==
2020-01-12T05:41:38.9653832Z   local time: Sun Jan 12 05:41:38 UTC 2020
2020-01-12T05:41:39.2509793Z   network time: Sun, 12 Jan 2020 05:41:39 GMT
2020-01-12T05:41:39.2509995Z == end clock drift check ==
2020-01-12T05:41:39.7459916Z 
2020-01-12T05:41:39.7556127Z ##[error]Bash exited with code '1'.
2020-01-12T05:41:39.7598868Z ##[section]Starting: Checkout
2020-01-12T05:41:39.7600978Z ==============================================================================
2020-01-12T05:41:39.7601046Z Task         : Get sources
2020-01-12T05:41:39.7601093Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
