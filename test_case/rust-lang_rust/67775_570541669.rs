plain
2020-01-03T09:51:18.3339187Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-03T09:51:18.3512238Z ##[command]git config gc.auto 0
2020-01-03T09:51:19.0506001Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-03T09:51:19.0512345Z ##[command]git config --get-all http.proxy
2020-01-03T09:51:19.0519617Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67775/merge:refs/remotes/pull/67775/merge
---
2020-01-03T10:51:13.9027522Z .................................................................................................... 1500/9467
2020-01-03T10:51:19.6942247Z .................................................................................................... 1600/9467
2020-01-03T10:51:24.5811597Z .................................................................................................... 1700/9467
2020-01-03T10:51:33.8855523Z .................................................................................................... 1800/9467
2020-01-03T10:51:42.0013805Z i................................................................................................... 1900/9467
2020-01-03T10:51:48.6312261Z ......................................................................................iiiii......... 2000/9467
2020-01-03T10:52:10.1262315Z .................................................................................................... 2200/9467
2020-01-03T10:52:12.4962765Z .................................................................................................... 2300/9467
2020-01-03T10:52:14.9484704Z .................................................................................................... 2400/9467
2020-01-03T10:52:21.0123691Z .................................................................................................... 2500/9467
---
2020-01-03T10:55:18.6554360Z .................i...............i.................................................................. 4900/9467
2020-01-03T10:55:28.5486369Z .................................................................................................... 5000/9467
2020-01-03T10:55:34.0908075Z ..............................................................i..................................... 5100/9467
2020-01-03T10:55:42.2244627Z .................................................................................................... 5200/9467
2020-01-03T10:55:49.7397286Z .............................ii.ii...........i...................................................... 5300/9467
2020-01-03T10:55:58.9180860Z .................................................................................................... 5500/9467
2020-01-03T10:56:08.9162101Z .................................................................................................... 5600/9467
2020-01-03T10:56:15.9793410Z ............i....................................................................................... 5700/9467
2020-01-03T10:56:22.1320847Z .................................................................................................... 5800/9467
2020-01-03T10:56:22.1320847Z .................................................................................................... 5800/9467
2020-01-03T10:56:32.8362454Z .................................................................................................... 5900/9467
2020-01-03T10:56:44.5909891Z ii...i..ii...........i.............................................................................. 6000/9467
2020-01-03T10:57:02.4741623Z .................................................................................................... 6200/9467
2020-01-03T10:57:09.7153398Z .................................................................................................... 6300/9467
2020-01-03T10:57:09.7153398Z .................................................................................................... 6300/9467
2020-01-03T10:57:23.3719916Z ...........................i..ii.................................................................... 6400/9467
2020-01-03T10:57:43.2735090Z .................................................................................................... 6600/9467
2020-01-03T10:57:45.3932220Z ..i................................................................................................. 6700/9467
2020-01-03T10:57:47.7329257Z .................................................................................................... 6800/9467
2020-01-03T10:57:50.3165124Z ..i................................................................................................. 6900/9467
---
2020-01-03T10:59:27.6937859Z .................................................................................................... 7500/9467
2020-01-03T10:59:32.6604849Z .................................................................................................... 7600/9467
2020-01-03T10:59:38.0778698Z .................................................................................................... 7700/9467
2020-01-03T10:59:48.2442499Z .................................................................................................... 7800/9467
2020-01-03T10:59:55.7638165Z ....................................iiii............................................................ 7900/9467
2020-01-03T11:00:10.6625921Z .................................................................................................... 8100/9467
2020-01-03T11:00:19.2965559Z .................................................................................................... 8200/9467
2020-01-03T11:00:33.3930313Z .................................................................................................... 8300/9467
2020-01-03T11:00:41.3735705Z .................................................................................................... 8400/9467
---
2020-01-03T11:02:36.1005204Z 38 
2020-01-03T11:02:36.1005309Z 
2020-01-03T11:02:36.1005417Z 
2020-01-03T11:02:36.1005544Z The actual stderr differed from the expected stderr.
2020-01-03T11:02:36.1006180Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword/dollar-crate-is-keyword.stderr
2020-01-03T11:02:36.1006561Z To update references, rerun the tests and pass the `--bless` flag
2020-01-03T11:02:36.1007034Z To only update this specific test, also pass `--test-args dollar-crate/dollar-crate-is-keyword.rs`
2020-01-03T11:02:36.1007342Z error: 1 errors occurred comparing output.
2020-01-03T11:02:36.1007499Z status: exit code: 1
2020-01-03T11:02:36.1007499Z status: exit code: 1
2020-01-03T11:02:36.1008520Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword/auxiliary" "-A" "unused"
2020-01-03T11:02:36.1009128Z ------------------------------------------
2020-01-03T11:02:36.1009297Z 
2020-01-03T11:02:36.1009641Z ------------------------------------------
2020-01-03T11:02:36.1009810Z stderr:
2020-01-03T11:02:36.1009810Z stderr:
2020-01-03T11:02:36.1010164Z ------------------------------------------
2020-01-03T11:02:36.1010332Z error: expected identifier, found reserved identifier `$crate`
2020-01-03T11:02:36.1010697Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:6:20
2020-01-03T11:02:36.1010888Z    |
2020-01-03T11:02:36.1011029Z LL |             struct $crate {} //~ ERROR expected identifier, found reserved identifier `$crate`
2020-01-03T11:02:36.1011797Z ...
2020-01-03T11:02:36.1011797Z ...
2020-01-03T11:02:36.1011942Z LL | m!();
2020-01-03T11:02:36.1012515Z 
2020-01-03T11:02:36.1012662Z error: expected identifier, found reserved identifier `$crate`
2020-01-03T11:02:36.1013067Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:10:23
2020-01-03T11:02:36.1013241Z    |
2020-01-03T11:02:36.1013241Z    |
2020-01-03T11:02:36.1013402Z LL |         use $crate as $crate; //~ ERROR expected identifier, found reserved identifier `$crate`
2020-01-03T11:02:36.1013721Z ...
2020-01-03T11:02:36.1013721Z ...
2020-01-03T11:02:36.1013876Z LL | m!();
2020-01-03T11:02:36.1014366Z 
2020-01-03T11:02:36.1014508Z error: `$crate` may not be imported
2020-01-03T11:02:36.1015761Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:9:9
2020-01-03T11:02:36.1018299Z    |
2020-01-03T11:02:36.1018299Z    |
2020-01-03T11:02:36.1018757Z LL |         use $crate; //~ ERROR `$crate` may not be imported
2020-01-03T11:02:36.1020215Z ...
2020-01-03T11:02:36.1020215Z ...
2020-01-03T11:02:36.1020280Z LL | m!();
2020-01-03T11:02:36.1020624Z 
2020-01-03T11:02:36.1020681Z error: `$crate` may not be imported
2020-01-03T11:02:36.1020904Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:10:9
2020-01-03T11:02:36.1021078Z    |
2020-01-03T11:02:36.1021078Z    |
2020-01-03T11:02:36.1021133Z LL |         use $crate as $crate; //~ ERROR expected identifier, found reserved identifier `$crate`
2020-01-03T11:02:36.1021861Z ...
2020-01-03T11:02:36.1021861Z ...
2020-01-03T11:02:36.1021908Z LL | m!();
2020-01-03T11:02:36.1022239Z 
2020-01-03T11:02:36.1022283Z error: aborting due to 4 previous errors
2020-01-03T11:02:36.1022312Z 
2020-01-03T11:02:36.1022356Z 
---
2020-01-03T11:02:36.1023665Z 10 
2020-01-03T11:02:36.1023692Z 
2020-01-03T11:02:36.1023718Z 
2020-01-03T11:02:36.1023763Z The actual stderr differed from the expected stderr.
2020-01-03T11:02:36.1024090Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-crate-var/import-crate-var.stderr
2020-01-03T11:02:36.1024343Z To update references, rerun the tests and pass the `--bless` flag
2020-01-03T11:02:36.1024619Z To only update this specific test, also pass `--test-args imports/import-crate-var.rs`
2020-01-03T11:02:36.1024718Z error: 1 errors occurred comparing output.
2020-01-03T11:02:36.1024927Z status: exit code: 1
2020-01-03T11:02:36.1024927Z status: exit code: 1
2020-01-03T11:02:36.1025688Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/import-crate-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-crate-var" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-crate-var/auxiliary" "-A" "unused"
2020-01-03T11:02:36.1025985Z ------------------------------------------
2020-01-03T11:02:36.1026021Z 
2020-01-03T11:02:36.1026210Z ------------------------------------------
2020-01-03T11:02:36.1026250Z stderr:
---
2020-01-03T11:02:36.1028838Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-03T11:02:36.1028901Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-03T11:02:36.1038480Z 
2020-01-03T11:02:36.1038600Z 
2020-01-03T11:02:36.1040334Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-03T11:02:36.1040761Z 
2020-01-03T11:02:36.1040789Z 
2020-01-03T11:02:36.1045191Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-03T11:02:36.1045256Z Build completed unsuccessfully in 1:04:58
2020-01-03T11:02:36.1045256Z Build completed unsuccessfully in 1:04:58
2020-01-03T11:02:36.1104259Z == clock drift check ==
2020-01-03T11:02:36.1123104Z   local time: Fri Jan  3 11:02:36 UTC 2020
2020-01-03T11:02:36.2092164Z   network time: Fri, 03 Jan 2020 11:02:36 GMT
2020-01-03T11:02:36.2094773Z == end clock drift check ==
2020-01-03T11:02:37.5148785Z 
2020-01-03T11:02:37.5229552Z ##[error]Bash exited with code '1'.
2020-01-03T11:02:37.5275942Z ##[section]Starting: Checkout
2020-01-03T11:02:37.5277586Z ==============================================================================
2020-01-03T11:02:37.5277637Z Task         : Get sources
2020-01-03T11:02:37.5277681Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
