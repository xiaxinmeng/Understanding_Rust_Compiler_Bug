plain
2019-07-28T12:02:14.7805937Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-28T12:02:14.7985645Z ##[command]git config gc.auto 0
2019-07-28T12:02:14.8043493Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-28T12:02:14.8097593Z ##[command]git config --get-all http.proxy
2019-07-28T12:02:14.8211674Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63079/merge:refs/remotes/pull/63079/merge
---
2019-07-28T12:02:47.3925305Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-28T12:02:47.3926549Z 
2019-07-28T12:02:47.3928295Z   git checkout -b <new-branch-name>
2019-07-28T12:02:47.3930252Z 
2019-07-28T12:02:47.3932023Z HEAD is now at af6e69129 Merge 4063bc1194f2b7151cfdb2df3979d31fd18ed10a into 9a239ef4ded03d155c72b68b5a2dd7aff013e141
2019-07-28T12:02:47.4078225Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-28T12:02:47.4081424Z ==============================================================================
2019-07-28T12:02:47.4081502Z Task         : Bash
2019-07-28T12:02:47.4081550Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-28T12:58:19.3334003Z .........................................F.......................................................... 1400/8798
2019-07-28T12:58:24.6007652Z .................................................................................................... 1500/8798
2019-07-28T12:58:35.9722837Z .............................................................i...............i...................... 1600/8798
2019-07-28T12:58:42.8926343Z .................................................................................................... 1700/8798
2019-07-28T12:58:56.1942944Z ...............................................iiiii................................................ 1800/8798
2019-07-28T12:59:06.3915279Z .................................................................................................... 2000/8798
2019-07-28T12:59:08.6370991Z .................................................................................................... 2100/8798
2019-07-28T12:59:12.1179596Z .................................................................................................... 2200/8798
2019-07-28T12:59:18.1776783Z .................................................................................................... 2300/8798
---
2019-07-28T13:02:41.9900723Z .................................................................................................... 5200/8798
2019-07-28T13:02:52.1414375Z ...................................................................................................i 5300/8798
2019-07-28T13:02:59.6196308Z .................................................................................................... 5400/8798
2019-07-28T13:03:04.2751217Z .................................................................................................... 5500/8798
2019-07-28T13:03:15.1124135Z ............................................................................................ii...i.. 5600/8798
2019-07-28T13:03:29.4147728Z ii...........i...................................................................................... 5700/8798
2019-07-28T13:03:40.9606411Z .................................................................................................... 5900/8798
2019-07-28T13:03:40.9606411Z .................................................................................................... 5900/8798
2019-07-28T13:03:45.0832283Z ............................................................................................i..ii... 6000/8798
2019-07-28T13:04:12.9531517Z .................................................................................................... 6200/8798
2019-07-28T13:04:14.8107726Z ...................................i................................................................ 6300/8798
2019-07-28T13:04:16.6632954Z .................................................................................................... 6400/8798
2019-07-28T13:04:18.6679140Z ....i............................................................................................... 6500/8798
---
2019-07-28T13:07:55.9737306Z 1 error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9737601Z -   --> $DIR/ub-ref.rs:9:1
2019-07-28T13:07:55.9737915Z +   --> $DIR/ub-ref.rs:8:1
2019-07-28T13:07:55.9738090Z 3    |
2019-07-28T13:07:55.9738209Z 4 LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
2019-07-28T13:07:55.9738336Z 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-07-28T13:07:55.9738462Z 
2019-07-28T13:07:55.9739062Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9739368Z 9 error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9739652Z -   --> $DIR/ub-ref.rs:12:1
2019-07-28T13:07:55.9739976Z +   --> $DIR/ub-ref.rs:11:1
2019-07-28T13:07:55.9740123Z 11    |
2019-07-28T13:07:55.9740123Z 11    |
2019-07-28T13:07:55.9740241Z 12 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
2019-07-28T13:07:55.9740830Z 13    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-07-28T13:07:55.9740981Z 
2019-07-28T13:07:55.9741443Z 15    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9741758Z 17 error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9742040Z -   --> $DIR/ub-ref.rs:15:1
2019-07-28T13:07:55.9743113Z +   --> $DIR/ub-ref.rs:14:1
2019-07-28T13:07:55.9743365Z 19    |
2019-07-28T13:07:55.9743365Z 19    |
2019-07-28T13:07:55.9743511Z 20 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
2019-07-28T13:07:55.9744020Z 21    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
2019-07-28T13:07:55.9744388Z 
2019-07-28T13:07:55.9745000Z 23    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9745469Z 25 error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9746247Z -   --> $DIR/ub-ref.rs:18:1
2019-07-28T13:07:55.9746678Z +   --> $DIR/ub-ref.rs:17:1
2019-07-28T13:07:55.9747774Z 27    |
2019-07-28T13:07:55.9747774Z 27    |
2019-07-28T13:07:55.9748083Z 28 LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
2019-07-28T13:07:55.9748257Z 29    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (created from integer)
2019-07-28T13:07:55.9748343Z 
2019-07-28T13:07:55.9748381Z The actual stderr differed from the expected stderr.
2019-07-28T13:07:55.9748664Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
2019-07-28T13:07:55.9748664Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
2019-07-28T13:07:55.9748888Z To update references, rerun the tests and pass the `--bless` flag
2019-07-28T13:07:55.9749102Z To only update this specific test, also pass `--test-args consts/const-eval/ub-ref.rs`
2019-07-28T13:07:55.9749169Z error: 1 errors occurred comparing output.
2019-07-28T13:07:55.9749224Z status: exit code: 1
2019-07-28T13:07:55.9749224Z status: exit code: 1
2019-07-28T13:07:55.9749833Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/auxiliary" "-A" "unused"
2019-07-28T13:07:55.9750107Z ------------------------------------------
2019-07-28T13:07:55.9750135Z 
2019-07-28T13:07:55.9750326Z ------------------------------------------
2019-07-28T13:07:55.9750364Z stderr:
2019-07-28T13:07:55.9750364Z stderr:
2019-07-28T13:07:55.9750533Z ------------------------------------------
2019-07-28T13:07:55.9750592Z error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9750794Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:8:1
2019-07-28T13:07:55.9750837Z    |
2019-07-28T13:07:55.9750892Z LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
2019-07-28T13:07:55.9751104Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2019-07-28T13:07:55.9751149Z    |
2019-07-28T13:07:55.9751486Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9751564Z error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9751771Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:11:1
2019-07-28T13:07:55.9751812Z    |
2019-07-28T13:07:55.9751812Z    |
2019-07-28T13:07:55.9751852Z LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
2019-07-28T13:07:55.9752135Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-07-28T13:07:55.9752526Z    |
2019-07-28T13:07:55.9752989Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9753085Z error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9753435Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:14:1
2019-07-28T13:07:55.9753500Z    |
2019-07-28T13:07:55.9753500Z    |
2019-07-28T13:07:55.9753551Z LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
2019-07-28T13:07:55.9753943Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
2019-07-28T13:07:55.9754001Z    |
2019-07-28T13:07:55.9754381Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9754571Z error[E0080]: it is undefined behavior to use this value
2019-07-28T13:07:55.9754829Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:17:1
2019-07-28T13:07:55.9754900Z    |
2019-07-28T13:07:55.9754900Z    |
2019-07-28T13:07:55.9755151Z LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
2019-07-28T13:07:55.9755219Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (created from integer)
2019-07-28T13:07:55.9755289Z    |
2019-07-28T13:07:55.9755671Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-28T13:07:55.9755934Z error: aborting due to 4 previous errors
2019-07-28T13:07:55.9755957Z 
2019-07-28T13:07:55.9756138Z For more information about this error, try `rustc --explain E0080`.
2019-07-28T13:07:55.9756189Z 
---
2019-07-28T13:07:55.9771631Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:534:22
2019-07-28T13:07:55.9771707Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-28T13:07:55.9791065Z 
2019-07-28T13:07:55.9791202Z 
2019-07-28T13:07:55.9793719Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-28T13:07:55.9794201Z 
2019-07-28T13:07:55.9794322Z 
2019-07-28T13:07:55.9798260Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-28T13:07:55.9798944Z Build completed unsuccessfully in 0:59:03
2019-07-28T13:07:55.9798944Z Build completed unsuccessfully in 0:59:03
2019-07-28T13:07:56.7363896Z ##[error]Bash exited with code '1'.
2019-07-28T13:07:56.7398499Z ##[section]Starting: Checkout
2019-07-28T13:07:56.7399893Z ==============================================================================
2019-07-28T13:07:56.7399933Z Task         : Get sources
2019-07-28T13:07:56.7399984Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
