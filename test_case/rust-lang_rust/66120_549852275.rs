plain
2019-11-05T13:29:45.2088465Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-05T13:29:45.2279224Z ##[command]git config gc.auto 0
2019-11-05T13:29:45.2348401Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-05T13:29:45.2407328Z ##[command]git config --get-all http.proxy
2019-11-05T13:29:45.2585701Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66120/merge:refs/remotes/pull/66120/merge
---
2019-11-05T14:29:51.2202262Z .................................................................................................... 1600/9280
2019-11-05T14:29:57.2773720Z ..............................F..................................................................... 1700/9280
2019-11-05T14:30:09.8698631Z ...................................................................i...............i................ 1800/9280
2019-11-05T14:30:17.2075189Z .................................................................................................... 1900/9280
2019-11-05T14:30:32.9423890Z .........................................................iiiii...................................... 2000/9280
2019-11-05T14:30:44.3342920Z .................................................................................................... 2200/9280
2019-11-05T14:30:46.8759902Z .................................................................................................... 2300/9280
2019-11-05T14:30:50.2716930Z .................................................................................................... 2400/9280
2019-11-05T14:31:14.0988437Z .................................................................................................... 2500/9280
---
2019-11-05T14:34:06.1850737Z .........................................................i...............i.......................... 4800/9280
2019-11-05T14:34:14.8083867Z .................................................................................................... 4900/9280
2019-11-05T14:34:23.8281663Z .................................................................................................... 5000/9280
2019-11-05T14:34:29.8897624Z .................................................................................................... 5100/9280
2019-11-05T14:34:40.8707456Z ..........................................................ii.ii...........i......................... 5200/9280
2019-11-05T14:34:50.3537106Z .................................................................................................... 5400/9280
2019-11-05T14:35:00.9885780Z .................................................................................................... 5500/9280
2019-11-05T14:35:08.3684304Z ...............................i.................................................................... 5600/9280
2019-11-05T14:35:15.0950569Z .................................................................................................... 5700/9280
2019-11-05T14:35:15.0950569Z .................................................................................................... 5700/9280
2019-11-05T14:35:27.1082748Z .................................................................................................... 5800/9280
2019-11-05T14:35:38.5693009Z ................ii...i..ii...........i.............................................................. 5900/9280
2019-11-05T14:35:59.4434046Z .................................................................................................... 6100/9280
2019-11-05T14:36:07.9606192Z .................................................................................................... 6200/9280
2019-11-05T14:36:07.9606192Z .................................................................................................... 6200/9280
2019-11-05T14:36:23.8072233Z ...................................i..ii............................................................ 6300/9280
2019-11-05T14:36:44.5898132Z .................................................................................................... 6500/9280
2019-11-05T14:36:46.7504434Z ..i................................................................................................. 6600/9280
2019-11-05T14:36:48.9462069Z ................................................................................i................... 6700/9280
2019-11-05T14:36:51.6431367Z .................................................................................................... 6800/9280
---
2019-11-05T14:41:35.8177315Z ....................................................................................i............... 9200/9280
2019-11-05T14:41:47.3424248Z ................................................................................
2019-11-05T14:41:47.3424522Z failures:
2019-11-05T14:41:47.3460037Z 
2019-11-05T14:41:47.3460584Z ---- [ui] ui/consts/const_in_pattern/warn_corner_cases.rs stdout ----
2019-11-05T14:41:47.3460651Z 
2019-11-05T14:41:47.3461164Z error: /checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs:26: unexpected warning: '26:47: 26:52: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-05T14:41:47.3461415Z 
2019-11-05T14:41:47.3461949Z error: /checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs:31: unexpected warning: '31:47: 31:51: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-05T14:41:47.3462022Z 
2019-11-05T14:41:47.3462479Z error: /checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs:36: unexpected warning: '36:47: 36:58: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-05T14:41:47.3462604Z error: 3 unexpected errors found, 0 expected errors not found
2019-11-05T14:41:47.3462656Z status: exit code: 0
2019-11-05T14:41:47.3462656Z status: exit code: 0
2019-11-05T14:41:47.3463528Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/warn_corner_cases.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/warn_corner_cases/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/warn_corner_cases/auxiliary"
2019-11-05T14:41:47.3463653Z unexpected errors (from JSON output): [
2019-11-05T14:41:47.3463772Z         line_num: 26,
2019-11-05T14:41:47.3463820Z         kind: Some(
2019-11-05T14:41:47.3463867Z             Warning,
2019-11-05T14:41:47.3463929Z         ),
2019-11-05T14:41:47.3463929Z         ),
2019-11-05T14:41:47.3463993Z         msg: "26:47: 26:52: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-05T14:41:47.3464242Z     Error {
2019-11-05T14:41:47.3464288Z         line_num: 31,
2019-11-05T14:41:47.3464335Z         kind: Some(
2019-11-05T14:41:47.3464390Z             Warning,
2019-11-05T14:41:47.3464390Z             Warning,
2019-11-05T14:41:47.3464453Z         ),
2019-11-05T14:41:47.3464516Z         msg: "31:47: 31:51: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-05T14:41:47.3464635Z     Error {
2019-11-05T14:41:47.3464681Z         line_num: 36,
2019-11-05T14:41:47.3464728Z         kind: Some(
2019-11-05T14:41:47.3464793Z             Warning,
2019-11-05T14:41:47.3464793Z             Warning,
2019-11-05T14:41:47.3464838Z         ),
2019-11-05T14:41:47.3464901Z         msg: "36:47: 36:58: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-05T14:41:47.3465026Z ]
2019-11-05T14:41:47.3465056Z 
2019-11-05T14:41:47.3465056Z 
2019-11-05T14:41:47.3465461Z thread '[ui] ui/consts/const_in_pattern/warn_corner_cases.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-05T14:41:47.3465593Z 
2019-11-05T14:41:47.3465621Z 
2019-11-05T14:41:47.3465665Z failures:
2019-11-05T14:41:47.3465665Z failures:
2019-11-05T14:41:47.3465732Z     [ui] ui/consts/const_in_pattern/warn_corner_cases.rs
2019-11-05T14:41:47.3466081Z test result: FAILED. 9236 passed; 1 failed; 43 ignored; 0 measured; 0 filtered out
2019-11-05T14:41:47.3466122Z 
2019-11-05T14:41:47.3492834Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-05T14:41:47.3508625Z 
2019-11-05T14:41:47.3508625Z 
2019-11-05T14:41:47.3508795Z 
2019-11-05T14:41:47.3516058Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-05T14:41:47.3516369Z 
2019-11-05T14:41:47.3516419Z 
2019-11-05T14:41:47.3528516Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-05T14:41:47.3528597Z Build completed unsuccessfully in 1:05:40
2019-11-05T14:41:47.3528597Z Build completed unsuccessfully in 1:05:40
2019-11-05T14:41:47.3596749Z == clock drift check ==
2019-11-05T14:41:47.8894536Z   local time: Tue Nov  5 14:41:47 UTC 2019
2019-11-05T14:41:47.8895607Z   network time: Tue, 05 Nov 2019 14:41:47 GMT
2019-11-05T14:41:47.8895822Z == end clock drift check ==
2019-11-05T14:41:48.8779955Z 
2019-11-05T14:41:48.8915788Z ##[error]Bash exited with code '1'.
2019-11-05T14:41:48.8949613Z ##[section]Starting: Checkout
2019-11-05T14:41:48.8951471Z ==============================================================================
2019-11-05T14:41:48.8951694Z Task         : Get sources
2019-11-05T14:41:48.8951747Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
