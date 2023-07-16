plain
2019-08-10T11:39:24.2883778Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T11:39:24.3091346Z ##[command]git config gc.auto 0
2019-08-10T11:39:25.2032084Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T11:39:25.2047286Z ##[command]git config --get-all http.proxy
2019-08-10T11:39:25.2054584Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63343/merge:refs/remotes/pull/63343/merge
---
2019-08-10T11:39:58.6689818Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T11:39:58.6689851Z 
2019-08-10T11:39:58.6690079Z   git checkout -b <new-branch-name>
2019-08-10T11:39:58.6690112Z 
2019-08-10T11:39:58.6690196Z HEAD is now at d4c6231f2 Merge 5af094bf3ece5114425903e0f593de525af0642f into d19a359444295bab01de7ff44a9d72302e573bc9
2019-08-10T11:39:58.6871714Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T11:39:58.6874932Z ==============================================================================
2019-08-10T11:39:58.6874997Z Task         : Bash
2019-08-10T11:39:58.6875064Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T12:41:21.3986275Z .................................................................................................... 1300/8869
2019-08-10T12:41:28.0664282Z .................................................................................................... 1400/8869
2019-08-10T12:41:34.2352795Z .................................................................................................... 1500/8869
2019-08-10T12:41:44.7275246Z ....................................................................................i............... 1600/8869
2019-08-10T12:41:52.4299470Z i................................................................................................... 1700/8869
2019-08-10T12:41:59.1878204Z ...........................................................................iiiii.................... 1800/8869
2019-08-10T12:42:21.4216097Z .................................................................................................... 2000/8869
2019-08-10T12:42:23.8757780Z .................................................................................................... 2100/8869
2019-08-10T12:42:26.5470554Z .................................................................................................... 2200/8869
2019-08-10T12:42:34.1632856Z .................................................................................................... 2300/8869
---
2019-08-10T12:46:30.5929843Z .................................................................................................... 5300/8869
2019-08-10T12:46:37.9253027Z .....i.............................................................................................. 5400/8869
2019-08-10T12:46:43.0644699Z .................................................................................................... 5500/8869
2019-08-10T12:46:55.5123023Z .................................................................................................... 5600/8869
2019-08-10T12:47:09.2516381Z ii...i..ii...........i.............................................................................. 5700/8869
2019-08-10T12:47:25.7968713Z .................................................................................................... 5900/8869
2019-08-10T12:47:30.4738886Z .................................................................................................... 6000/8869
2019-08-10T12:47:30.4738886Z .................................................................................................... 6000/8869
2019-08-10T12:47:44.7632561Z .i..ii.............................................................................................. 6100/8869
2019-08-10T12:48:03.4341475Z ............................................i....................................................... 6300/8869
2019-08-10T12:48:05.5611818Z .................................................................................................... 6400/8869
2019-08-10T12:48:08.0923778Z ................i................................................................................... 6500/8869
2019-08-10T12:48:12.6622975Z .................................................................................................... 6600/8869
---
2019-08-10T12:52:12.5368681Z 
2019-08-10T12:52:12.5369375Z ---- [ui] ui/init-unsafe.rs stdout ----
2019-08-10T12:52:12.5369681Z diff of stderr:
2019-08-10T12:52:12.5369878Z 
2019-08-10T12:52:12.5370414Z + warning: use of deprecated item 'std::intrinsics::init': superseded by MaybeUninit, removal planned
2019-08-10T12:52:12.5370906Z +   --> $DIR/init-unsafe.rs:3:23
2019-08-10T12:52:12.5371187Z +    |
2019-08-10T12:52:12.5372606Z + LL | use std::intrinsics::{init};
2019-08-10T12:52:12.5373711Z +    |
2019-08-10T12:52:12.5373945Z +    = note: `#[warn(deprecated)]` on by default
2019-08-10T12:52:12.5374172Z + 
2019-08-10T12:52:12.5374172Z + 
2019-08-10T12:52:12.5374781Z + warning: use of deprecated item 'std::intrinsics::init': superseded by MaybeUninit, removal planned
2019-08-10T12:52:12.5375425Z +   --> $DIR/init-unsafe.rs:7:17
2019-08-10T12:52:12.5375820Z +    |
2019-08-10T12:52:12.5376168Z + LL |     let stuff = init::<isize>();
2019-08-10T12:52:12.5376552Z + 
2019-08-10T12:52:12.5376764Z 1 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2019-08-10T12:52:12.5377175Z 2   --> $DIR/init-unsafe.rs:7:17
2019-08-10T12:52:12.5377432Z 3    |
2019-08-10T12:52:12.5377432Z 3    |
2019-08-10T12:52:12.5377631Z 
2019-08-10T12:52:12.5377801Z 
2019-08-10T12:52:12.5377997Z The actual stderr differed from the expected stderr.
2019-08-10T12:52:12.5378647Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-unsafe/init-unsafe.stderr
2019-08-10T12:52:12.5379210Z To update references, rerun the tests and pass the `--bless` flag
2019-08-10T12:52:12.5379714Z To only update this specific test, also pass `--test-args init-unsafe.rs`
2019-08-10T12:52:12.5380169Z error: 1 errors occurred comparing output.
2019-08-10T12:52:12.5380362Z status: exit code: 1
2019-08-10T12:52:12.5380362Z status: exit code: 1
2019-08-10T12:52:12.5381194Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/init-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-unsafe" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-unsafe/auxiliary" "-A" "unused"
2019-08-10T12:52:12.5384290Z ------------------------------------------
2019-08-10T12:52:12.5384527Z 
2019-08-10T12:52:12.5385172Z ------------------------------------------
2019-08-10T12:52:12.5385370Z stderr:
2019-08-10T12:52:12.5385370Z stderr:
2019-08-10T12:52:12.5385689Z ------------------------------------------
2019-08-10T12:52:12.5386077Z warning: use of deprecated item 'std::intrinsics::init': superseded by MaybeUninit, removal planned
2019-08-10T12:52:12.5386799Z    |
2019-08-10T12:52:12.5386799Z    |
2019-08-10T12:52:12.5386922Z LL | use std::intrinsics::{init};
2019-08-10T12:52:12.5387188Z    |
2019-08-10T12:52:12.5387308Z    = note: `#[warn(deprecated)]` on by default
2019-08-10T12:52:12.5387434Z 
2019-08-10T12:52:12.5387434Z 
2019-08-10T12:52:12.5387842Z warning: use of deprecated item 'std::intrinsics::init': superseded by MaybeUninit, removal planned
2019-08-10T12:52:12.5388387Z    |
2019-08-10T12:52:12.5388387Z    |
2019-08-10T12:52:12.5388525Z LL |     let stuff = init::<isize>(); //~ ERROR call to unsafe function is unsafe
2019-08-10T12:52:12.5388783Z 
2019-08-10T12:52:12.5388909Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2019-08-10T12:52:12.5389258Z   --> /checkout/src/test/ui/init-unsafe.rs:7:17
2019-08-10T12:52:12.5389933Z    |
2019-08-10T12:52:12.5389933Z    |
2019-08-10T12:52:12.5390094Z LL |     let stuff = init::<isize>(); //~ ERROR call to unsafe function is unsafe
2019-08-10T12:52:12.5390245Z    |                 ^^^^^^^^^^^^^^^ call to unsafe function
2019-08-10T12:52:12.5390374Z    |
2019-08-10T12:52:12.5390781Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2019-08-10T12:52:12.5391092Z error: aborting due to previous error
2019-08-10T12:52:12.5391197Z 
2019-08-10T12:52:12.5392057Z For more information about this error, try `rustc --explain E0133`.
2019-08-10T12:52:12.5392284Z 
---
2019-08-10T12:52:12.5404018Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-10T12:52:12.5404402Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-10T12:52:12.5422129Z 
2019-08-10T12:52:12.5429917Z 
2019-08-10T12:52:12.5435639Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-10T12:52:12.5436335Z 
2019-08-10T12:52:12.5436471Z 
2019-08-10T12:52:12.5436704Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-10T12:52:12.5436879Z Build completed unsuccessfully in 1:06:15
2019-08-10T12:52:12.5436879Z Build completed unsuccessfully in 1:06:15
2019-08-10T12:52:13.3452866Z ##[error]Bash exited with code '1'.
2019-08-10T12:52:13.3494568Z ##[section]Starting: Checkout
2019-08-10T12:52:13.3496461Z ==============================================================================
2019-08-10T12:52:13.3496513Z Task         : Get sources
2019-08-10T12:52:13.3496572Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
