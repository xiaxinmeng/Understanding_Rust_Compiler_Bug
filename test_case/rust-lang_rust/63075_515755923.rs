plain
2019-07-28T10:32:09.1969185Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-28T10:32:09.2171136Z ##[command]git config gc.auto 0
2019-07-28T10:32:09.2239819Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-28T10:32:09.2285797Z ##[command]git config --get-all http.proxy
2019-07-28T10:32:09.2419883Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63075/merge:refs/remotes/pull/63075/merge
---
2019-07-28T10:32:43.4644887Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-28T10:32:43.4644996Z 
2019-07-28T10:32:43.4645395Z   git checkout -b <new-branch-name>
2019-07-28T10:32:43.4645429Z 
2019-07-28T10:32:43.4651055Z HEAD is now at 713a6d267 Merge 3677c5be56168508fea082e1651c774e34600ca8 into 9a239ef4ded03d155c72b68b5a2dd7aff013e141
2019-07-28T10:32:43.4814927Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-28T10:32:43.4817925Z ==============================================================================
2019-07-28T10:32:43.4818005Z Task         : Bash
2019-07-28T10:32:43.4818058Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-28T11:34:34.3265090Z ........................................F........................................................... 1400/8798
2019-07-28T11:34:40.2725241Z .................................................................................................... 1500/8798
2019-07-28T11:34:53.0998088Z .............................................................i...............i...................... 1600/8798
2019-07-28T11:35:01.0463441Z .................................................................................................... 1700/8798
2019-07-28T11:35:15.9124612Z ...............................................iiiii................................................ 1800/8798
2019-07-28T11:35:27.3163624Z .................................................................................................... 2000/8798
2019-07-28T11:35:29.9733238Z .................................................................................................... 2100/8798
2019-07-28T11:35:33.9946110Z .................................................................................................... 2200/8798
2019-07-28T11:35:41.2462624Z .................................................................................................... 2300/8798
---
2019-07-28T11:39:29.7443622Z .................................................................................................... 5200/8798
2019-07-28T11:39:40.7045545Z ...................................................................................................i 5300/8798
2019-07-28T11:39:48.8983783Z .................................................................................................... 5400/8798
2019-07-28T11:39:54.0710793Z .................................................................................................... 5500/8798
2019-07-28T11:40:05.9025998Z ............................................................................................ii...i.. 5600/8798
2019-07-28T11:40:21.5972451Z ii...........i...................................................................................... 5700/8798
2019-07-28T11:40:35.1826536Z .................................................................................................... 5900/8798
2019-07-28T11:40:35.1826536Z .................................................................................................... 5900/8798
2019-07-28T11:40:40.0122295Z ............................................................................................i..ii... 6000/8798
2019-07-28T11:41:11.3057445Z .................................................................................................... 6200/8798
2019-07-28T11:41:13.5530439Z ...................................i................................................................ 6300/8798
2019-07-28T11:41:15.6923628Z .................................................................................................... 6400/8798
2019-07-28T11:41:18.0487799Z ....i............................................................................................... 6500/8798
---
2019-07-28T11:45:21.8911626Z 
2019-07-28T11:45:21.8912568Z ---- [ui] ui/consts/const-eval/ub-nonnull.rs stdout ----
2019-07-28T11:45:21.8913025Z diff of stderr:
2019-07-28T11:45:21.8913086Z 
2019-07-28T11:45:21.8913858Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8914007Z 9 error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8914267Z -   --> $DIR/ub-nonnull.rs:14:1
2019-07-28T11:45:21.8914498Z -    |
2019-07-28T11:45:21.8914498Z -    |
2019-07-28T11:45:21.8914746Z - LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
2019-07-28T11:45:21.8914953Z - LL | |
2019-07-28T11:45:21.8915230Z - LL | |     let ptr: &(u8, u8, u8) = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
2019-07-28T11:45:21.8915640Z - LL | |     let out_of_bounds_ptr = &ptr.2; // use address-of-field for pointer arithmetic
2019-07-28T11:45:21.8916084Z - LL | |     mem::transmute(out_of_bounds_ptr)
2019-07-28T11:45:21.8916250Z - LL | | } };
2019-07-28T11:45:21.8916553Z -    | |____^ type validation failed: encountered a potentially NULL pointer, but expected something that cannot possibly fail to be greater or equal to 1
2019-07-28T11:45:21.8916721Z -    |
2019-07-28T11:45:21.8917059Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8917462Z - error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8917663Z 23   --> $DIR/ub-nonnull.rs:21:1
2019-07-28T11:45:21.8917720Z 24    |
2019-07-28T11:45:21.8917720Z 24    |
2019-07-28T11:45:21.8917761Z 25 LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
2019-07-28T11:45:21.8917839Z 59    |
2019-07-28T11:45:21.8917839Z 59    |
2019-07-28T11:45:21.8918183Z 60    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8918528Z - error: aborting due to 7 previous errors
2019-07-28T11:45:21.8918572Z + error: aborting due to 6 previous errors
2019-07-28T11:45:21.8918608Z 63 
2019-07-28T11:45:21.8918848Z 64 For more information about this error, try `rustc --explain E0080`.
2019-07-28T11:45:21.8918848Z 64 For more information about this error, try `rustc --explain E0080`.
2019-07-28T11:45:21.8918888Z 65 
2019-07-28T11:45:21.8918912Z 
2019-07-28T11:45:21.8918934Z 
2019-07-28T11:45:21.8919156Z The actual stderr differed from the expected stderr.
2019-07-28T11:45:21.8919472Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.stderr
2019-07-28T11:45:21.8919814Z To update references, rerun the tests and pass the `--bless` flag
2019-07-28T11:45:21.8920098Z To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`
2019-07-28T11:45:21.8920172Z error: 1 errors occurred comparing output.
2019-07-28T11:45:21.8920230Z status: exit code: 1
2019-07-28T11:45:21.8920230Z status: exit code: 1
2019-07-28T11:45:21.8921210Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary" "-A" "unused"
2019-07-28T11:45:21.8921546Z ------------------------------------------
2019-07-28T11:45:21.8921578Z 
2019-07-28T11:45:21.8921802Z ------------------------------------------
2019-07-28T11:45:21.8921846Z stderr:
2019-07-28T11:45:21.8921846Z stderr:
2019-07-28T11:45:21.8922048Z ------------------------------------------
2019-07-28T11:45:21.8922095Z error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8922837Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:11:1
2019-07-28T11:45:21.8922900Z    |
2019-07-28T11:45:21.8922948Z LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
2019-07-28T11:45:21.8923028Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-07-28T11:45:21.8923081Z    |
2019-07-28T11:45:21.8923505Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8923611Z error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8923878Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:21:1
2019-07-28T11:45:21.8923930Z    |
2019-07-28T11:45:21.8923930Z    |
2019-07-28T11:45:21.8923978Z LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
2019-07-28T11:45:21.8924055Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-07-28T11:45:21.8924107Z    |
2019-07-28T11:45:21.8924504Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8924621Z error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8924878Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:23:1
2019-07-28T11:45:21.8924945Z    |
2019-07-28T11:45:21.8924945Z    |
2019-07-28T11:45:21.8924994Z LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
2019-07-28T11:45:21.8925055Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-07-28T11:45:21.8925124Z    |
2019-07-28T11:45:21.8925507Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8925731Z error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8926061Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:30:1
2019-07-28T11:45:21.8926133Z    |
2019-07-28T11:45:21.8926133Z    |
2019-07-28T11:45:21.8926175Z LL | const UNINIT: NonZeroU8 = unsafe { Transmute { uninit: () }.out };
2019-07-28T11:45:21.8926291Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected something greater or equal to 1
2019-07-28T11:45:21.8926355Z    |
2019-07-28T11:45:21.8926725Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8926824Z error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8927038Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:38:1
2019-07-28T11:45:21.8927081Z    |
2019-07-28T11:45:21.8927081Z    |
2019-07-28T11:45:21.8927141Z LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
2019-07-28T11:45:21.8927203Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
2019-07-28T11:45:21.8927271Z    |
2019-07-28T11:45:21.8927613Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8927710Z error[E0080]: it is undefined behavior to use this value
2019-07-28T11:45:21.8927922Z   --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:44:1
2019-07-28T11:45:21.8927966Z    |
2019-07-28T11:45:21.8927966Z    |
2019-07-28T11:45:21.8928024Z LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
2019-07-28T11:45:21.8928081Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
2019-07-28T11:45:21.8928135Z    |
2019-07-28T11:45:21.8928609Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T11:45:21.8928701Z error: aborting due to 6 previous errors
2019-07-28T11:45:21.8928748Z 
2019-07-28T11:45:21.8928978Z For more information about this error, try `rustc --explain E0080`.
2019-07-28T11:45:21.8929011Z 
---
2019-07-28T11:45:21.8966109Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:534:22
2019-07-28T11:45:21.8966206Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-28T11:45:21.8980195Z 
2019-07-28T11:45:21.8980291Z 
2019-07-28T11:45:21.8995360Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-28T11:45:21.8995750Z 
2019-07-28T11:45:21.8995782Z 
2019-07-28T11:45:21.9001063Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-28T11:45:21.9001154Z Build completed unsuccessfully in 1:06:06
2019-07-28T11:45:21.9001154Z Build completed unsuccessfully in 1:06:06
2019-07-28T11:45:22.6621725Z ##[error]Bash exited with code '1'.
2019-07-28T11:45:22.6669837Z ##[section]Starting: Checkout
2019-07-28T11:45:22.6671530Z ==============================================================================
2019-07-28T11:45:22.6671600Z Task         : Get sources
2019-07-28T11:45:22.6671644Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
