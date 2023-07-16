plain
2019-08-27T19:31:43.3732557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T19:31:43.3965265Z ##[command]git config gc.auto 0
2019-08-27T19:31:43.4021575Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T19:31:43.4070740Z ##[command]git config --get-all http.proxy
2019-08-27T19:31:43.4205513Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63956/merge:refs/remotes/pull/63956/merge
---
2019-08-27T19:32:19.7717686Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T19:32:19.7718307Z 
2019-08-27T19:32:19.7719061Z   git checkout -b <new-branch-name>
2019-08-27T19:32:19.7719290Z 
2019-08-27T19:32:19.7719568Z HEAD is now at 15dddcbe6 Merge 464657bebbc1b6cef8521fd49d43149aae371250 into 0396aace27eea97c3603e9683e921807dff2a314
2019-08-27T19:32:19.7871140Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T19:32:19.7873990Z ==============================================================================
2019-08-27T19:32:19.7874073Z Task         : Bash
2019-08-27T19:32:19.7874114Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T20:35:10.6156788Z .................................................................................................... 1500/8969
2019-08-27T20:35:16.3763916Z .................................................................................................... 1600/8969
2019-08-27T20:35:29.4013479Z .............................................i...............i...................................... 1700/8969
2019-08-27T20:35:37.8244618Z .................................................................................................... 1800/8969
2019-08-27T20:35:52.4623717Z ....................................iiiii........................................................... 1900/8969
2019-08-27T20:36:03.2468389Z .................................................................................................... 2100/8969
2019-08-27T20:36:05.8025863Z .................................................................................................... 2200/8969
2019-08-27T20:36:10.1599376Z .................................................................................................... 2300/8969
2019-08-27T20:36:17.5299338Z .................................................................................................... 2400/8969
---
2019-08-27T20:39:19.9753661Z .......................i...............i............................................................ 4700/8969
2019-08-27T20:39:32.4609120Z .................................................................................................... 4800/8969
2019-08-27T20:39:38.7046985Z .................................................................................................... 4900/8969
2019-08-27T20:39:49.8593821Z .................................................................................................... 5000/8969
2019-08-27T20:39:55.2242314Z ....ii.ii........................................................................................... 5100/8969
2019-08-27T20:40:09.3863013Z ...........................................................................................F........ 5300/8969
2019-08-27T20:40:16.7870293Z ...................................................................i................................ 5400/8969
2019-08-27T20:40:24.2682759Z .................................................................................................... 5500/8969
2019-08-27T20:40:31.4057105Z .................................................................................................... 5600/8969
2019-08-27T20:40:31.4057105Z .................................................................................................... 5600/8969
2019-08-27T20:40:41.9830000Z .............................................................ii...i..ii...........i................. 5700/8969
2019-08-27T20:41:04.1062787Z .................................................................................................... 5900/8969
2019-08-27T20:41:09.2739013Z .................................................................................................... 6000/8969
2019-08-27T20:41:09.2739013Z .................................................................................................... 6000/8969
2019-08-27T20:41:16.1304913Z ..............................................................i..ii................................. 6100/8969
2019-08-27T20:41:45.3807032Z .................................................................................................... 6300/8969
2019-08-27T20:41:47.5211122Z .................i.................................................................................. 6400/8969
2019-08-27T20:41:49.6796687Z .........................................................................................i.......... 6500/8969
2019-08-27T20:41:52.4834904Z .................................................................................................... 6600/8969
---
2019-08-27T20:45:54.2893254Z 
2019-08-27T20:45:54.2893856Z ---- [ui] ui/lto-duplicate-symbols.rs stdout ----
2019-08-27T20:45:54.2894060Z diff of stderr:
2019-08-27T20:45:54.2894167Z 
2019-08-27T20:45:54.2894515Z 1 warning: Linking globals named 'foo': symbol multiply defined!
2019-08-27T20:45:54.2894868Z 2 
2019-08-27T20:45:54.2895240Z - error: failed to load bc of "lto_duplicate_symbols1.3a1fbbbh-cgu.0": 
2019-08-27T20:45:54.2896174Z + error: failed to load bc of "lto_duplicate_symbols2.3a1fbbbh-cgu.0": 
2019-08-27T20:45:54.2896646Z 5 error: aborting due to previous error
2019-08-27T20:45:54.2896782Z 6 
2019-08-27T20:45:54.2896893Z 
2019-08-27T20:45:54.2897008Z 
2019-08-27T20:45:54.2897008Z 
2019-08-27T20:45:54.2897169Z The actual stderr differed from the expected stderr.
2019-08-27T20:45:54.2897646Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
2019-08-27T20:45:54.2898090Z To update references, rerun the tests and pass the `--bless` flag
2019-08-27T20:45:54.2898528Z To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`
2019-08-27T20:45:54.2898852Z error: 1 errors occurred comparing output.
2019-08-27T20:45:54.2899170Z status: exit code: 1
2019-08-27T20:45:54.2899170Z status: exit code: 1
2019-08-27T20:45:54.2900439Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary" "-A" "unused"
2019-08-27T20:45:54.2901508Z ------------------------------------------
2019-08-27T20:45:54.2901910Z 
2019-08-27T20:45:54.2902308Z ------------------------------------------
2019-08-27T20:45:54.2902482Z stderr:
2019-08-27T20:45:54.2902482Z stderr:
2019-08-27T20:45:54.2902868Z ------------------------------------------
2019-08-27T20:45:54.2903296Z warning: Linking globals named 'foo': symbol multiply defined!
2019-08-27T20:45:54.2903473Z 
2019-08-27T20:45:54.2903857Z error: failed to load bc of "lto_duplicate_symbols2.3a1fbbbh-cgu.0": 
2019-08-27T20:45:54.2904182Z error: aborting due to previous error
2019-08-27T20:45:54.2904300Z 
2019-08-27T20:45:54.2904417Z 
2019-08-27T20:45:54.2904764Z ------------------------------------------
---
2019-08-27T20:45:54.2938931Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-27T20:45:54.2939367Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-27T20:45:54.2990465Z 
2019-08-27T20:45:54.2990969Z 
2019-08-27T20:45:54.2992528Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-27T20:45:54.2993130Z 
2019-08-27T20:45:54.2993284Z 
2019-08-27T20:45:54.3003061Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-27T20:45:54.3003324Z Build completed unsuccessfully in 1:07:02
2019-08-27T20:45:54.3003324Z Build completed unsuccessfully in 1:07:02
2019-08-27T20:45:54.3056200Z == clock drift check ==
2019-08-27T20:45:54.3074241Z   local time: Tue Aug 27 20:45:54 UTC 2019
2019-08-27T20:45:54.4590616Z   network time: Tue, 27 Aug 2019 20:45:54 GMT
2019-08-27T20:45:54.4593195Z == end clock drift check ==
2019-08-27T20:45:55.5523334Z ##[error]Bash exited with code '1'.
2019-08-27T20:45:55.5564662Z ##[section]Starting: Checkout
2019-08-27T20:45:55.5567311Z ==============================================================================
2019-08-27T20:45:55.5567367Z Task         : Get sources
2019-08-27T20:45:55.5567414Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
