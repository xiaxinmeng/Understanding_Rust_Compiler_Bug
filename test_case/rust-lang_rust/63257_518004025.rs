plain
2019-08-04T12:18:21.5130765Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-04T12:18:21.5308987Z ##[command]git config gc.auto 0
2019-08-04T12:18:21.5379898Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-04T12:18:21.5437791Z ##[command]git config --get-all http.proxy
2019-08-04T12:18:21.5566227Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63257/merge:refs/remotes/pull/63257/merge
---
2019-08-04T12:18:55.3577944Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-04T12:18:55.3578157Z 
2019-08-04T12:18:55.3578408Z   git checkout -b <new-branch-name>
2019-08-04T12:18:55.3578438Z 
2019-08-04T12:18:55.3578503Z HEAD is now at 10d3f7ced Merge d13738b02758399e2728a1c56a784e376e58d68b into 5170a3f45af6a7a2a2e46a115daca8f31379b3a8
2019-08-04T12:18:55.3730701Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-04T12:18:55.3733409Z ==============================================================================
2019-08-04T12:18:55.3733469Z Task         : Bash
2019-08-04T12:18:55.3733529Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-04T13:20:00.7249277Z .................................................................................................... 1400/8827
2019-08-04T13:20:06.5368431Z .................................................................................................... 1500/8827
2019-08-04T13:20:19.0378767Z ....................................................................i...............i............... 1600/8827
2019-08-04T13:20:26.2603324Z .................................................................................................... 1700/8827
2019-08-04T13:20:41.6373779Z ......................................................iiiii......................................... 1800/8827
2019-08-04T13:20:53.1913324Z .................................................................................................... 2000/8827
2019-08-04T13:20:55.7341246Z .................................................................................................... 2100/8827
2019-08-04T13:20:59.0364305Z .................................................................................................... 2200/8827
2019-08-04T13:21:06.8321897Z .................................................................................................... 2300/8827
---
2019-08-04T13:24:54.4677210Z .......................................................F............................................ 5200/8827
2019-08-04T13:25:02.7356745Z .....................................................................i.............................. 5300/8827
2019-08-04T13:25:10.0780197Z .................................................................................................... 5400/8827
2019-08-04T13:25:17.0457697Z .................................................................................................... 5500/8827
2019-08-04T13:25:28.3045229Z ...............................................................ii...i..ii...........i............... 5600/8827
2019-08-04T13:25:53.3638498Z .................................................................................................... 5800/8827
2019-08-04T13:25:58.4117952Z .................................................................................................... 5900/8827
2019-08-04T13:25:58.4117952Z .................................................................................................... 5900/8827
2019-08-04T13:26:04.5413549Z ................................................................i..ii............................... 6000/8827
2019-08-04T13:26:33.6320889Z .................................................................................................... 6200/8827
2019-08-04T13:26:35.7830523Z .......i............................................................................................ 6300/8827
2019-08-04T13:26:37.9430282Z ...............................................................................i.................... 6400/8827
2019-08-04T13:26:40.5971123Z .................................................................................................... 6500/8827
---
2019-08-04T13:30:42.0438262Z failures:
2019-08-04T13:30:42.0465379Z 
2019-08-04T13:30:42.0466249Z ---- [ui] ui/lint/unused_parens_json_suggestion.rs stdout ----
2019-08-04T13:30:42.0466461Z 
2019-08-04T13:30:42.0466879Z error: test compilation failed although it shouldn't!
2019-08-04T13:30:42.0467114Z status: exit code: 1
2019-08-04T13:30:42.0468246Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion/auxiliary" "-A" "unused"
2019-08-04T13:30:42.0469092Z ------------------------------------------
2019-08-04T13:30:42.0469301Z 
2019-08-04T13:30:42.0469693Z ------------------------------------------
2019-08-04T13:30:42.0470012Z stderr:
2019-08-04T13:30:42.0470012Z stderr:
2019-08-04T13:30:42.0470479Z ------------------------------------------
2019-08-04T13:30:42.0470896Z error: argument for --error-format must be `human`, `json` or `short` (instead was `-Zunstable-options`)
2019-08-04T13:30:42.0471205Z 
2019-08-04T13:30:42.0471535Z ------------------------------------------
2019-08-04T13:30:42.0471697Z 
2019-08-04T13:30:42.0471815Z 
---
2019-08-04T13:30:42.0521173Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-04T13:30:42.0521487Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-04T13:30:42.0548330Z 
2019-08-04T13:30:42.0548858Z 
2019-08-04T13:30:42.0554363Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-04T13:30:42.0555040Z 
2019-08-04T13:30:42.0555079Z 
2019-08-04T13:30:42.0566785Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-04T13:30:42.0567122Z Build completed unsuccessfully in 1:05:30
2019-08-04T13:30:42.0567122Z Build completed unsuccessfully in 1:05:30
2019-08-04T13:30:42.8436422Z ##[error]Bash exited with code '1'.
2019-08-04T13:30:42.8478864Z ##[section]Starting: Checkout
2019-08-04T13:30:42.8480717Z ==============================================================================
2019-08-04T13:30:42.8480768Z Task         : Get sources
2019-08-04T13:30:42.8480814Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
