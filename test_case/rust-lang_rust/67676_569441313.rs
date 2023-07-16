plain
2019-12-28T17:27:18.1118153Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-28T17:27:18.1298465Z ##[command]git config gc.auto 0
2019-12-28T17:27:18.1377490Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-28T17:27:18.1430160Z ##[command]git config --get-all http.proxy
2019-12-28T17:27:18.1580660Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67676/merge:refs/remotes/pull/67676/merge
---
2019-12-28T18:26:28.7079707Z ...............................................................................F.................... 1600/9463
2019-12-28T18:26:33.4241473Z .................................................................................................... 1700/9463
2019-12-28T18:26:42.6243395Z ..................................................................................................i. 1800/9463
2019-12-28T18:26:50.4612687Z .................................................................................................... 1900/9463
2019-12-28T18:26:57.0101800Z ....................................................................................iiiii........... 2000/9463
2019-12-28T18:27:18.2133960Z .................................................................................................... 2200/9463
2019-12-28T18:27:20.4855608Z .................................................................................................... 2300/9463
2019-12-28T18:27:22.8315033Z .................................................................................................... 2400/9463
2019-12-28T18:27:28.7227740Z .................................................................................................... 2500/9463
---
2019-12-28T18:30:22.7369018Z ...............i...............i.................................................................... 4900/9463
2019-12-28T18:30:32.3059956Z .................................................................................................... 5000/9463
2019-12-28T18:30:37.7479782Z ............................................................i....................................... 5100/9463
2019-12-28T18:30:45.6673000Z .................................................................................................... 5200/9463
2019-12-28T18:30:52.9851194Z ...........................ii.ii...........i........................................................ 5300/9463
2019-12-28T18:31:01.9812257Z .................................................................................................... 5500/9463
2019-12-28T18:31:12.3597859Z .................................................................................................... 5600/9463
2019-12-28T18:31:19.1314195Z .........i.......................................................................................... 5700/9463
2019-12-28T18:31:25.2022733Z .................................................................................................... 5800/9463
2019-12-28T18:31:25.2022733Z .................................................................................................... 5800/9463
2019-12-28T18:31:35.3470986Z .................................................................................................ii. 5900/9463
2019-12-28T18:31:46.9957656Z ..i..ii...........i................................................................................. 6000/9463
2019-12-28T18:32:04.2791506Z .................................................................................................... 6200/9463
2019-12-28T18:32:11.3846526Z .................................................................................................... 6300/9463
2019-12-28T18:32:11.3846526Z .................................................................................................... 6300/9463
2019-12-28T18:32:28.3287818Z ........................i..ii....................................................................... 6400/9463
2019-12-28T18:32:47.8706563Z .................................................................................................... 6600/9463
2019-12-28T18:32:49.9105159Z .i.................................................................................................. 6700/9463
2019-12-28T18:32:52.1618035Z .................................................................................................... 6800/9463
2019-12-28T18:32:54.6014463Z .i.................................................................................................. 6900/9463
---
2019-12-28T18:34:30.5465696Z .................................................................................................... 7500/9463
2019-12-28T18:34:35.4382010Z .................................................................................................... 7600/9463
2019-12-28T18:34:40.8621672Z .................................................................................................... 7700/9463
2019-12-28T18:34:50.6512452Z .................................................................................................... 7800/9463
2019-12-28T18:34:58.1091298Z .................................iiii............................................................... 7900/9463
2019-12-28T18:35:12.4790670Z .................................................................................................... 8100/9463
2019-12-28T18:35:21.1718477Z .................................................................................................... 8200/9463
2019-12-28T18:35:35.1816930Z .................................................................................................... 8300/9463
2019-12-28T18:35:42.4830027Z .................................................................................................... 8400/9463
---
2019-12-28T18:37:34.7760054Z 
2019-12-28T18:37:34.7761031Z ---- [ui] ui/consts/const-prop-overflowing-casts.rs stdout ----
2019-12-28T18:37:34.7761395Z diff of stderr:
2019-12-28T18:37:34.7761658Z 
2019-12-28T18:37:34.7761945Z 1 error: truncating cast: the value 2147483648 requires 32 bits but the target type is only 16 bits
2019-12-28T18:37:34.7763021Z +   --> $DIR/const-prop-overflowing-casts.rs:6:13
2019-12-28T18:37:34.7763900Z 3    |
2019-12-28T18:37:34.7763900Z 3    |
2019-12-28T18:37:34.7764094Z 4 LL |     let _ = (1u32 << 31) as u16;
2019-12-28T18:37:34.7764490Z 
2019-12-28T18:37:34.7764676Z 7    = note: `#[deny(const_err)]` on by default
2019-12-28T18:37:34.7764859Z 8 
2019-12-28T18:37:34.7764859Z 8 
2019-12-28T18:37:34.7765069Z 9 error: truncating cast: the value 32768 requires 16 bits but the target type is only 8 bits
2019-12-28T18:37:34.7765905Z +   --> $DIR/const-prop-overflowing-casts.rs:7:13
2019-12-28T18:37:34.7766175Z 11    |
2019-12-28T18:37:34.7766175Z 11    |
2019-12-28T18:37:34.7766367Z 12 LL |     let _ = (1u16 << 15) as u8;
2019-12-28T18:37:34.7766965Z 
2019-12-28T18:37:34.7767423Z 14 
2019-12-28T18:37:34.7767423Z 14 
2019-12-28T18:37:34.7767597Z 15 error: truncating cast: the value 65535 requires 16 bits but the target type is only 8 bits
2019-12-28T18:37:34.7768736Z +   --> $DIR/const-prop-overflowing-casts.rs:8:13
2019-12-28T18:37:34.7768939Z 17    |
2019-12-28T18:37:34.7768939Z 17    |
2019-12-28T18:37:34.7769123Z 18 LL |     let _ = (!0u16) as u8;
2019-12-28T18:37:34.7769356Z 
2019-12-28T18:37:34.7769647Z 
2019-12-28T18:37:34.7769800Z The actual stderr differed from the expected stderr.
2019-12-28T18:37:34.7770955Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-overflowing-casts/const-prop-overflowing-casts.stderr
2019-12-28T18:37:34.7770955Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-overflowing-casts/const-prop-overflowing-casts.stderr
2019-12-28T18:37:34.7771431Z To update references, rerun the tests and pass the `--bless` flag
2019-12-28T18:37:34.7771897Z To only update this specific test, also pass `--test-args consts/const-prop-overflowing-casts.rs`
2019-12-28T18:37:34.7772251Z error: 1 errors occurred comparing output.
2019-12-28T18:37:34.7772398Z status: exit code: 1
2019-12-28T18:37:34.7772398Z status: exit code: 1
2019-12-28T18:37:34.7773439Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-prop-overflowing-casts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-overflowing-casts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-overflowing-casts/auxiliary" "-A" "unused"
2019-12-28T18:37:34.7774163Z ------------------------------------------
2019-12-28T18:37:34.7774441Z 
2019-12-28T18:37:34.7774819Z ------------------------------------------
2019-12-28T18:37:34.7774995Z stderr:
2019-12-28T18:37:34.7774995Z stderr:
2019-12-28T18:37:34.7775279Z ------------------------------------------
2019-12-28T18:37:34.7775458Z error: truncating cast: the value 2147483648 requires 32 bits but the target type is only 16 bits
2019-12-28T18:37:34.7776077Z    |
2019-12-28T18:37:34.7776077Z    |
2019-12-28T18:37:34.7776226Z LL |     let _ = (1u32 << 31) as u16; //~ ERROR truncating cast: the value 2147483648 requires 32 bits but the target type is only 16 bits
2019-12-28T18:37:34.7776471Z    |
2019-12-28T18:37:34.7776606Z    = note: `#[deny(const_err)]` on by default
2019-12-28T18:37:34.7776705Z 
2019-12-28T18:37:34.7776705Z 
2019-12-28T18:37:34.7776823Z error: truncating cast: the value 32768 requires 16 bits but the target type is only 8 bits
2019-12-28T18:37:34.7777360Z    |
2019-12-28T18:37:34.7777360Z    |
2019-12-28T18:37:34.7777506Z LL |     let _ = (1u16 << 15) as u8; //~ ERROR truncating cast: the value 32768 requires 16 bits but the target type is only 8 bits
2019-12-28T18:37:34.7777733Z 
2019-12-28T18:37:34.7777733Z 
2019-12-28T18:37:34.7777867Z error: truncating cast: the value 65535 requires 16 bits but the target type is only 8 bits
2019-12-28T18:37:34.7778352Z    |
2019-12-28T18:37:34.7778352Z    |
2019-12-28T18:37:34.7778496Z LL |     let _ = (!0u16) as u8; //~ ERROR truncating cast: the value 65535 requires 16 bits but the target type is only 8 bits
2019-12-28T18:37:34.7778719Z 
2019-12-28T18:37:34.7778858Z error: aborting due to 3 previous errors
2019-12-28T18:37:34.7778959Z 
2019-12-28T18:37:34.7779054Z 
---
2019-12-28T18:37:34.7786699Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-28T18:37:34.7786948Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-28T18:37:34.7790896Z 
2019-12-28T18:37:34.7791113Z 
2019-12-28T18:37:34.7796383Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-28T18:37:34.7796905Z 
2019-12-28T18:37:34.7797037Z 
2019-12-28T18:37:34.7801263Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-28T18:37:34.7801496Z Build completed unsuccessfully in 1:03:51
2019-12-28T18:37:34.7801496Z Build completed unsuccessfully in 1:03:51
2019-12-28T18:37:34.7858704Z == clock drift check ==
2019-12-28T18:37:34.7876434Z   local time: Sat Dec 28 18:37:34 UTC 2019
2019-12-28T18:37:34.8325510Z   network time: Sat, 28 Dec 2019 18:37:34 GMT
2019-12-28T18:37:34.8330360Z == end clock drift check ==
2019-12-28T18:37:35.9142645Z 
2019-12-28T18:37:35.9262448Z ##[error]Bash exited with code '1'.
2019-12-28T18:37:35.9298566Z ##[section]Starting: Checkout
2019-12-28T18:37:35.9300563Z ==============================================================================
2019-12-28T18:37:35.9300620Z Task         : Get sources
2019-12-28T18:37:35.9300671Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
