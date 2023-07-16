plain
2019-10-01T20:36:19.1565620Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T20:36:19.1750980Z ##[command]git config gc.auto 0
2019-10-01T20:36:19.1818592Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T20:36:19.1871009Z ##[command]git config --get-all http.proxy
2019-10-01T20:36:19.1998233Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64967/merge:refs/remotes/pull/64967/merge
---
2019-10-01T21:38:46.5963322Z .................................................................................................... 1400/9086
2019-10-01T21:38:52.9925146Z .........................F.......................................................................... 1500/9086
2019-10-01T21:38:59.5035649Z .................................................................................................... 1600/9086
2019-10-01T21:39:08.9785829Z .................................................................................................... 1700/9086
2019-10-01T21:39:17.2537975Z i...............i................................................................................... 1800/9086
2019-10-01T21:39:24.1063811Z ...........................................................................................iiiii.... 1900/9086
2019-10-01T21:39:45.8669033Z .................................................................................................... 2100/9086
2019-10-01T21:39:48.1664984Z .................................................................................................... 2200/9086
2019-10-01T21:39:50.6434230Z .................................................................................................... 2300/9086
2019-10-01T21:39:57.3883456Z .................................................................................................... 2400/9086
---
2019-10-01T21:42:50.0062842Z ..............................................................................i...............i..... 4700/9086
2019-10-01T21:42:57.7944914Z .................................................................................................... 4800/9086
2019-10-01T21:43:07.6661933Z .................................................................................................... 4900/9086
2019-10-01T21:43:13.5835070Z .................................................................................................... 5000/9086
2019-10-01T21:43:24.6226590Z .....................................................................ii.ii.......................... 5100/9086
2019-10-01T21:43:34.1594384Z .................................................................................................... 5300/9086
2019-10-01T21:43:43.2619144Z .................................................................................................... 5400/9086
2019-10-01T21:43:50.6159434Z ...................................i................................................................ 5500/9086
2019-10-01T21:43:56.9390048Z .................................................................................................... 5600/9086
2019-10-01T21:43:56.9390048Z .................................................................................................... 5600/9086
2019-10-01T21:44:08.6663577Z .................................................................................................... 5700/9086
2019-10-01T21:44:19.1699340Z ...............................ii...i..ii...........i............................................... 5800/9086
2019-10-01T21:44:40.5226278Z .................................................................................................... 6000/9086
2019-10-01T21:44:47.9821156Z .................................................................................................... 6100/9086
2019-10-01T21:44:47.9821156Z .................................................................................................... 6100/9086
2019-10-01T21:45:01.6454158Z ..................................i..ii............................................................. 6200/9086
2019-10-01T21:45:21.1659126Z ..............................................................................................i..... 6400/9086
2019-10-01T21:45:23.2880865Z .................................................................................................... 6500/9086
2019-10-01T21:45:25.4763744Z ..................................................................i................................. 6600/9086
2019-10-01T21:45:28.3713566Z .................................................................................................... 6700/9086
---
2019-10-01T21:49:30.2018550Z normalized stderr:
2019-10-01T21:49:30.2018624Z error[E0597]: `x` does not live long enough
2019-10-01T21:49:30.2020051Z   --> $DIR/generic-slice.rs:15:9
2019-10-01T21:49:30.2020108Z    |
2019-10-01T21:49:30.2020960Z LL | impl<'a, T: 'static> Generic<'a, T> {
2019-10-01T21:49:30.2021263Z ...
2019-10-01T21:49:30.2021300Z LL |         &x
2019-10-01T21:49:30.2021354Z    |         ^^
2019-10-01T21:49:30.2021562Z    |         |
---
2019-10-01T21:49:30.2024651Z 
2019-10-01T21:49:30.2024682Z 
2019-10-01T21:49:30.2024726Z 
2019-10-01T21:49:30.2024773Z The actual stderr differed from the expected stderr.
2019-10-01T21:49:30.2025140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/generic-slice.stderr
2019-10-01T21:49:30.2025438Z To update references, rerun the tests and pass the `--bless` flag
2019-10-01T21:49:30.2026441Z To only update this specific test, also pass `--test-args consts/const-eval/generic-slice.rs`
2019-10-01T21:49:30.2026554Z error: 1 errors occurred comparing output.
2019-10-01T21:49:30.2026600Z status: exit code: 1
2019-10-01T21:49:30.2026600Z status: exit code: 1
2019-10-01T21:49:30.2027392Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/generic-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/auxiliary" "-A" "unused"
2019-10-01T21:49:30.2027766Z ------------------------------------------
2019-10-01T21:49:30.2027822Z 
2019-10-01T21:49:30.2028063Z ------------------------------------------
2019-10-01T21:49:30.2028112Z stderr:
2019-10-01T21:49:30.2028112Z stderr:
2019-10-01T21:49:30.2028344Z ------------------------------------------
2019-10-01T21:49:30.2028413Z error[E0597]: `x` does not live long enough
2019-10-01T21:49:30.2028675Z   --> /checkout/src/test/ui/consts/const-eval/generic-slice.rs:15:9
2019-10-01T21:49:30.2028730Z    |
2019-10-01T21:49:30.2028991Z LL | impl<'a, T: 'static> Generic<'a, T> {
2019-10-01T21:49:30.2029433Z ...
2019-10-01T21:49:30.2029657Z LL |         &x
2019-10-01T21:49:30.2029704Z    |         ^^
2019-10-01T21:49:30.2029740Z    |         |
2019-10-01T21:49:30.2029740Z    |         |
2019-10-01T21:49:30.2029797Z    |         borrowed value does not live long enough
2019-10-01T21:49:30.2030053Z    |         using this value as a constant requires that `x` is borrowed for `'a`
2019-10-01T21:49:30.2030104Z LL |         //~^ ERROR `x` does not live long enough
2019-10-01T21:49:30.2030378Z    |     - `x` dropped here while still borrowed
2019-10-01T21:49:30.2030409Z 
2019-10-01T21:49:30.2030447Z error[E0597]: `x` does not live long enough
2019-10-01T21:49:30.2030698Z   --> /checkout/src/test/ui/consts/const-eval/generic-slice.rs:27:5
2019-10-01T21:49:30.2030698Z   --> /checkout/src/test/ui/consts/const-eval/generic-slice.rs:27:5
2019-10-01T21:49:30.2030903Z    |
2019-10-01T21:49:30.2030939Z LL |     &x
2019-10-01T21:49:30.2030974Z    |     ^^
2019-10-01T21:49:30.2031026Z    |     |
2019-10-01T21:49:30.2031064Z    |     borrowed value does not live long enough
2019-10-01T21:49:30.2031315Z    |     using this value as a static requires that `x` is borrowed for `'static`
2019-10-01T21:49:30.2031385Z LL |     //~^ ERROR `x` does not live long enough
2019-10-01T21:49:30.2031629Z    | - `x` dropped here while still borrowed
2019-10-01T21:49:30.2031673Z 
2019-10-01T21:49:30.2031711Z error: aborting due to 2 previous errors
2019-10-01T21:49:30.2031736Z 
---
2019-10-01T21:49:30.2052335Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-01T21:49:30.2052485Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-01T21:49:30.2072268Z 
2019-10-01T21:49:30.2072377Z 
2019-10-01T21:49:30.2073927Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-01T21:49:30.2074521Z 
2019-10-01T21:49:30.2074550Z 
2019-10-01T21:49:30.2081277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-01T21:49:30.2081365Z Build completed unsuccessfully in 1:05:51
2019-10-01T21:49:30.2081365Z Build completed unsuccessfully in 1:05:51
2019-10-01T21:49:30.2133497Z == clock drift check ==
2019-10-01T21:49:30.2148590Z   local time: Tue Oct  1 21:49:30 UTC 2019
2019-10-01T21:49:30.2394355Z   network time: Tue, 01 Oct 2019 21:49:30 GMT
2019-10-01T21:49:30.2398601Z == end clock drift check ==
2019-10-01T21:49:31.5068248Z ##[error]Bash exited with code '1'.
2019-10-01T21:49:31.5152539Z ##[section]Starting: Checkout
2019-10-01T21:49:31.5154550Z ==============================================================================
2019-10-01T21:49:31.5154623Z Task         : Get sources
2019-10-01T21:49:31.5154669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
