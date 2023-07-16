plain
2019-11-12T00:50:47.5249375Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T00:50:47.5265307Z ##[command]git config gc.auto 0
2019-11-12T00:50:47.5277615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T00:50:47.5280347Z ##[command]git config --get-all http.proxy
2019-11-12T00:50:47.5283340Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66319/merge:refs/remotes/pull/66319/merge
---
2019-11-12T01:49:03.6328464Z .................................................................................................... 1400/9229
2019-11-12T01:49:09.7713455Z .................................................................................................... 1500/9229
2019-11-12T01:49:15.6518862Z .................................................................................................... 1600/9229
2019-11-12T01:49:24.5273326Z .................................................................................................... 1700/9229
2019-11-12T01:49:32.8070113Z ..i................................................................................................. 1800/9229
2019-11-12T01:49:39.4228955Z ......................................................................................iiiii......... 1900/9229
2019-11-12T01:50:00.3707016Z .................................................................................................... 2100/9229
2019-11-12T01:50:02.7446815Z .................................................................................................... 2200/9229
2019-11-12T01:50:05.1707570Z .................................................................................................... 2300/9229
2019-11-12T01:50:14.5762296Z .................................................................................................... 2400/9229
---
2019-11-12T01:53:04.9482773Z ...................................................................................i...............i 4700/9229
2019-11-12T01:53:11.8589317Z .................................................................................................... 4800/9229
2019-11-12T01:53:20.8333445Z .................................................................................................... 4900/9229
2019-11-12T01:53:26.0321360Z .................................................................................................... 5000/9229
2019-11-12T01:53:37.2458275Z ......................................................................................ii.ii......... 5100/9229
2019-11-12T01:53:40.9256263Z ..i................................................................................................. 5200/9229
2019-11-12T01:53:54.0216815Z .................................................................................................... 5400/9229
2019-11-12T01:54:01.9577970Z ....................................................................i............................... 5500/9229
2019-11-12T01:54:09.2655336Z .................................................................................................... 5600/9229
2019-11-12T01:54:16.8436776Z .................................................................................................... 5700/9229
2019-11-12T01:54:16.8436776Z .................................................................................................... 5700/9229
2019-11-12T01:54:25.9164641Z .....................................................ii...i..ii...........i......................... 5800/9229
2019-11-12T01:54:47.7905717Z .................................................................................................... 6000/9229
2019-11-12T01:54:55.3500766Z .................................................................................................... 6100/9229
2019-11-12T01:54:55.3500766Z .................................................................................................... 6100/9229
2019-11-12T01:55:00.4156476Z ........................................................................i..ii....................... 6200/9229
2019-11-12T01:55:29.3084312Z .................................................................................................... 6400/9229
2019-11-12T01:55:31.4973006Z ........................................i........................................................... 6500/9229
2019-11-12T01:55:33.6749847Z .................................................................................................... 6600/9229
2019-11-12T01:55:36.0038975Z ........................i........................................................................... 6700/9229
---
2019-11-12T02:00:16.7627720Z failures:
2019-11-12T02:00:16.7662985Z 
2019-11-12T02:00:16.7663524Z ---- [ui] ui/feature-gates/feature-gate-cfg_accessible.rs stdout ----
2019-11-12T02:00:16.7663592Z normalized stderr:
2019-11-12T02:00:16.7663870Z error[E0537]: invalid predicate `accessible`
2019-11-12T02:00:16.7664211Z   --> $DIR/feature-gate-cfg_accessible.rs:1:7
2019-11-12T02:00:16.7664265Z    |
2019-11-12T02:00:16.7664335Z LL | #[cfg(accessible(::std::mem::ManuallyDrop))]
2019-11-12T02:00:16.7664438Z 
2019-11-12T02:00:16.7664438Z 
2019-11-12T02:00:16.7664491Z error[E0601]: `main` function not found in crate `feature_gate_cfg_accessible`
2019-11-12T02:00:16.7664784Z   --> $DIR/feature-gate-cfg_accessible.rs:1:1
2019-11-12T02:00:16.7664840Z    |
2019-11-12T02:00:16.7664890Z LL | / #[cfg(accessible(::std::mem::ManuallyDrop))]
2019-11-12T02:00:16.7664956Z LL | | fn main() {}
2019-11-12T02:00:16.7665268Z    | |____________^ consider adding a `main` function to `$DIR/feature-gate-cfg_accessible.rs`
2019-11-12T02:00:16.7665358Z error: aborting due to 2 previous errors
2019-11-12T02:00:16.7665405Z 
2019-11-12T02:00:16.7665455Z Some errors have detailed explanations: E0537, E0601.
2019-11-12T02:00:16.7665752Z For more information about an error, try `rustc --explain E0537`.
2019-11-12T02:00:16.7665752Z For more information about an error, try `rustc --explain E0537`.
2019-11-12T02:00:16.7665790Z 
2019-11-12T02:00:16.7665837Z 
2019-11-12T02:00:16.7665865Z 
2019-11-12T02:00:16.7665913Z The actual stderr differed from the expected stderr.
2019-11-12T02:00:16.7666468Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg_accessible/feature-gate-cfg_accessible.stderr
2019-11-12T02:00:16.7687584Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T02:00:16.7687983Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg_accessible.rs`
2019-11-12T02:00:16.7688106Z error: 1 errors occurred comparing output.
2019-11-12T02:00:16.7688158Z status: exit code: 1
2019-11-12T02:00:16.7688158Z status: exit code: 1
2019-11-12T02:00:16.7689118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg_accessible.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg_accessible" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg_accessible/auxiliary" "-A" "unused"
2019-11-12T02:00:16.7689544Z ------------------------------------------
2019-11-12T02:00:16.7689586Z 
2019-11-12T02:00:16.7689863Z ------------------------------------------
2019-11-12T02:00:16.7689917Z stderr:
2019-11-12T02:00:16.7689917Z stderr:
2019-11-12T02:00:16.7690194Z ------------------------------------------
2019-11-12T02:00:16.7690254Z error[E0537]: invalid predicate `accessible`
2019-11-12T02:00:16.7690570Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg_accessible.rs:1:7
2019-11-12T02:00:16.7690639Z    |
2019-11-12T02:00:16.7690709Z LL | #[cfg(accessible(::std::mem::ManuallyDrop))] //~^ ERROR: currently unstable
2019-11-12T02:00:16.7690803Z 
2019-11-12T02:00:16.7690803Z 
2019-11-12T02:00:16.7690871Z error[E0601]: `main` function not found in crate `feature_gate_cfg_accessible`
2019-11-12T02:00:16.7691209Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg_accessible.rs:1:1
2019-11-12T02:00:16.7691266Z    |
2019-11-12T02:00:16.7691333Z LL | / #[cfg(accessible(::std::mem::ManuallyDrop))] //~^ ERROR: currently unstable
2019-11-12T02:00:16.7691389Z LL | | fn main() {}
2019-11-12T02:00:16.7691756Z    | |____________^ consider adding a `main` function to `/checkout/src/test/ui/feature-gates/feature-gate-cfg_accessible.rs`
2019-11-12T02:00:16.7691864Z error: aborting due to 2 previous errors
2019-11-12T02:00:16.7691899Z 
2019-11-12T02:00:16.7691950Z Some errors have detailed explanations: E0537, E0601.
2019-11-12T02:00:16.7692415Z For more information about an error, try `rustc --explain E0537`.
---
2019-11-12T02:00:16.7700850Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T02:00:16.7700948Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T02:00:16.7722460Z 
2019-11-12T02:00:16.7722540Z 
2019-11-12T02:00:16.7725892Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T02:00:16.7726324Z 
2019-11-12T02:00:16.7726368Z 
2019-11-12T02:00:16.7726897Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T02:00:16.7726962Z Build completed unsuccessfully in 1:02:59
2019-11-12T02:00:16.7726962Z Build completed unsuccessfully in 1:02:59
2019-11-12T02:00:16.7777111Z == clock drift check ==
2019-11-12T02:00:16.7808978Z   local time: Tue Nov 12 02:00:16 UTC 2019
2019-11-12T02:00:17.0435283Z   network time: Tue, 12 Nov 2019 02:00:17 GMT
2019-11-12T02:00:17.0440824Z == end clock drift check ==
2019-11-12T02:00:17.9292040Z 
2019-11-12T02:00:17.9400461Z ##[error]Bash exited with code '1'.
2019-11-12T02:00:17.9455693Z ##[section]Starting: Checkout
2019-11-12T02:00:17.9457614Z ==============================================================================
2019-11-12T02:00:17.9457675Z Task         : Get sources
2019-11-12T02:00:17.9457741Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
