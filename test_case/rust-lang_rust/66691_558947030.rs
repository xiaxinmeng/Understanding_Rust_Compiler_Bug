plain
2019-11-27T05:12:58.1860438Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T05:12:59.1024192Z ##[command]git config gc.auto 0
2019-11-27T05:12:59.1028968Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T05:12:59.1032615Z ##[command]git config --get-all http.proxy
2019-11-27T05:12:59.1037541Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66691/merge:refs/remotes/pull/66691/merge
---
2019-11-27T06:06:36.2047370Z .................................................................................................... 1600/9297
2019-11-27T06:06:40.7478804Z .................................................................................................... 1700/9297
2019-11-27T06:06:52.4379495Z ..................................i................................................................. 1800/9297
2019-11-27T06:06:59.6396497Z .................................................................................................... 1900/9297
2019-11-27T06:07:12.5217819Z ...................iiiii............................................................................ 2000/9297
2019-11-27T06:07:21.6646291Z .................................................................................................... 2200/9297
2019-11-27T06:07:23.9091996Z .................................................................................................... 2300/9297
2019-11-27T06:07:28.5293098Z .................................................................................................... 2400/9297
2019-11-27T06:07:47.7595953Z .................................................................................................... 2500/9297
---
2019-11-27T06:10:16.3298542Z ....................i...............i............................................................... 4800/9297
2019-11-27T06:10:25.7320500Z .................................................................................................... 4900/9297
2019-11-27T06:10:31.0244475Z .................................................................................................... 5000/9297
2019-11-27T06:10:38.7717832Z .................................................................................................... 5100/9297
2019-11-27T06:10:45.5267133Z .........................ii.ii...........i.......................................................... 5200/9297
2019-11-27T06:10:53.8310615Z .................................................................................................... 5400/9297
2019-11-27T06:11:03.1564240Z ...........................................................................................F...F.... 5500/9297
2019-11-27T06:11:09.3596932Z .......i............................................................................................ 5600/9297
2019-11-27T06:11:15.0795488Z ............F....................................................................................... 5700/9297
2019-11-27T06:11:15.0795488Z ............F....................................................................................... 5700/9297
2019-11-27T06:11:25.0958169Z .............................................................................................ii...i. 5800/9297
2019-11-27T06:11:37.0742049Z .ii...........i..................................................................................... 5900/9297
2019-11-27T06:11:53.9517351Z .................................................................................................... 6100/9297
2019-11-27T06:11:57.5466202Z .................................................................................................... 6200/9297
2019-11-27T06:11:57.5466202Z .................................................................................................... 6200/9297
2019-11-27T06:12:10.4885255Z ................i..ii............................................................................... 6300/9297
2019-11-27T06:12:28.4248281Z ....................................................................................i............... 6500/9297
2019-11-27T06:12:30.5309540Z .................................................................................................... 6600/9297
2019-11-27T06:12:32.5934562Z ...........................................................................i........................ 6700/9297
2019-11-27T06:12:35.1036431Z .................................................................................................... 6800/9297
---
2019-11-27T06:16:52.7832213Z failures:
2019-11-27T06:16:52.7868688Z 
2019-11-27T06:16:52.7869599Z ---- [ui] ui/macros/macro-comma-support-rpass.rs#core stdout ----
2019-11-27T06:16:52.7870047Z 
2019-11-27T06:16:52.7870743Z error in revision `core`: test compilation failed although it shouldn't!
2019-11-27T06:16:52.7871110Z status: exit code: 1
2019-11-27T06:16:52.7872167Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/auxiliary"
2019-11-27T06:16:52.7873195Z ------------------------------------------
2019-11-27T06:16:52.7873475Z 
2019-11-27T06:16:52.7873937Z ------------------------------------------
2019-11-27T06:16:52.7874204Z stderr:
---
2019-11-27T06:16:52.7878562Z 
2019-11-27T06:16:52.7878756Z 
2019-11-27T06:16:52.7879269Z ---- [ui] ui/macros/macro-comma-support-rpass.rs#std stdout ----
2019-11-27T06:16:52.7879533Z 
2019-11-27T06:16:52.7880202Z error in revision `std`: test compilation failed although it shouldn't!
2019-11-27T06:16:52.7880488Z status: exit code: 1
2019-11-27T06:16:52.7881457Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/auxiliary"
2019-11-27T06:16:52.7882233Z ------------------------------------------
2019-11-27T06:16:52.7882490Z 
2019-11-27T06:16:52.7882950Z ------------------------------------------
2019-11-27T06:16:52.7883213Z stderr:
---
2019-11-27T06:16:52.7888539Z 
2019-11-27T06:16:52.7889008Z 7 error: cannot find a built-in macro with name `line`
2019-11-27T06:16:52.7889532Z 8   --> <::core::macros::builtin::line macros>:1:1
2019-11-27T06:16:52.7889792Z 9    |
2019-11-27T06:16:52.7890196Z - LL | () => { }
2019-11-27T06:16:52.7893329Z -    | ^^^^^^^^^
2019-11-27T06:16:52.7894248Z + LL | () => { } ;
2019-11-27T06:16:52.7894586Z 12 
2019-11-27T06:16:52.7894746Z 13 error: aborting due to 2 previous errors
2019-11-27T06:16:52.7894872Z 14 
2019-11-27T06:16:52.7894980Z 
2019-11-27T06:16:52.7894980Z 
2019-11-27T06:16:52.7895103Z 
2019-11-27T06:16:52.7895236Z The actual stderr differed from the expected stderr.
2019-11-27T06:16:52.7895751Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unknown-builtin/unknown-builtin.stderr
2019-11-27T06:16:52.7896210Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T06:16:52.7896648Z To only update this specific test, also pass `--test-args macros/unknown-builtin.rs`
2019-11-27T06:16:52.7896980Z error: 1 errors occurred comparing output.
2019-11-27T06:16:52.7897109Z status: exit code: 1
2019-11-27T06:16:52.7897109Z status: exit code: 1
2019-11-27T06:16:52.7899036Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/unknown-builtin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unknown-builtin" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unknown-builtin/auxiliary" "-A" "unused"
2019-11-27T06:16:52.7899774Z ------------------------------------------
2019-11-27T06:16:52.7900104Z 
2019-11-27T06:16:52.7900455Z ------------------------------------------
2019-11-27T06:16:52.7900648Z stderr:
2019-11-27T06:16:52.7900648Z stderr:
2019-11-27T06:16:52.7900982Z ------------------------------------------
2019-11-27T06:16:52.7901360Z error: cannot find a built-in macro with name `unknown`
2019-11-27T06:16:52.7901766Z   --> /checkout/src/test/ui/macros/unknown-builtin.rs:6:1
2019-11-27T06:16:52.7901940Z    |
2019-11-27T06:16:52.7902349Z LL | macro_rules! unknown { () => () } //~ ERROR cannot find a built-in macro with name `unknown`
2019-11-27T06:16:52.7902640Z 
2019-11-27T06:16:52.7902996Z error: cannot find a built-in macro with name `line`
2019-11-27T06:16:52.7903552Z   --> <::core::macros::builtin::line macros>:1:1
2019-11-27T06:16:52.7903730Z    |
2019-11-27T06:16:52.7903730Z    |
2019-11-27T06:16:52.7903872Z LL | () => { } ;
2019-11-27T06:16:52.7904103Z 
2019-11-27T06:16:52.7904247Z error: aborting due to 2 previous errors
2019-11-27T06:16:52.7904354Z 
2019-11-27T06:16:52.7904469Z 
---
2019-11-27T06:16:52.7940611Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-27T06:16:52.7940815Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T06:16:52.7940999Z 
2019-11-27T06:16:52.7941112Z 
2019-11-27T06:16:52.7942772Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-27T06:16:52.7943663Z 
2019-11-27T06:16:52.7943688Z 
2019-11-27T06:16:52.7943729Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-27T06:16:52.7943783Z Build completed unsuccessfully in 0:58:15
2019-11-27T06:16:52.7943783Z Build completed unsuccessfully in 0:58:15
2019-11-27T06:16:52.7966478Z == clock drift check ==
2019-11-27T06:16:52.7983124Z   local time: Wed Nov 27 06:16:52 UTC 2019
2019-11-27T06:16:53.3325626Z   network time: Wed, 27 Nov 2019 06:16:53 GMT
2019-11-27T06:16:53.3328383Z == end clock drift check ==
2019-11-27T06:16:54.0992582Z 
2019-11-27T06:16:54.1119141Z ##[error]Bash exited with code '1'.
2019-11-27T06:16:54.1152743Z ##[section]Starting: Checkout
2019-11-27T06:16:54.1154887Z ==============================================================================
2019-11-27T06:16:54.1154943Z Task         : Get sources
2019-11-27T06:16:54.1154992Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
