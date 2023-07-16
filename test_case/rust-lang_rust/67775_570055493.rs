plain
2020-01-01T12:45:02.3177033Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T12:45:02.3395788Z ##[command]git config gc.auto 0
2020-01-01T12:45:02.3451038Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T12:45:02.3502691Z ##[command]git config --get-all http.proxy
2020-01-01T12:45:02.3639595Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67775/merge:refs/remotes/pull/67775/merge
---
2020-01-01T13:48:53.6956348Z .................................................................................................... 1500/9467
2020-01-01T13:48:59.7013307Z .................................................................................................... 1600/9467
2020-01-01T13:49:04.8031750Z .................................................................................................... 1700/9467
2020-01-01T13:49:14.7134941Z .................................................................................................... 1800/9467
2020-01-01T13:49:23.2548881Z i................................................................................................... 1900/9467
2020-01-01T13:49:30.1706772Z ......................................................................................iiiii......... 2000/9467
2020-01-01T13:49:52.9947588Z .................................................................................................... 2200/9467
2020-01-01T13:49:55.4580414Z .................................................................................................... 2300/9467
2020-01-01T13:49:58.0311406Z .................................................................................................... 2400/9467
2020-01-01T13:50:04.3761844Z .................................................................................................... 2500/9467
---
2020-01-01T13:53:12.0544527Z .................i...............i.................................................................. 4900/9467
2020-01-01T13:53:22.4277320Z .................................................................................................... 5000/9467
2020-01-01T13:53:28.8934008Z ..............................................................i..................................... 5100/9467
2020-01-01T13:53:38.0055752Z .................................................................................................... 5200/9467
2020-01-01T13:53:45.3624128Z .............................ii.ii...........i...................................................... 5300/9467
2020-01-01T13:53:55.1839887Z .................................................................................................... 5500/9467
2020-01-01T13:54:05.8027696Z .................................................................................................... 5600/9467
2020-01-01T13:54:13.0369687Z ............i....................................................................................... 5700/9467
2020-01-01T13:54:19.4726448Z .................................................................................................... 5800/9467
2020-01-01T13:54:19.4726448Z .................................................................................................... 5800/9467
2020-01-01T13:54:30.6827219Z .................................................................................................... 5900/9467
2020-01-01T13:54:42.8463577Z ii...i..ii...........i.............................................................................. 6000/9467
2020-01-01T13:55:01.2148113Z .................................................................................................... 6200/9467
2020-01-01T13:55:08.7639944Z .................................................................................................... 6300/9467
2020-01-01T13:55:08.7639944Z .................................................................................................... 6300/9467
2020-01-01T13:55:26.0501928Z ...........................i..ii.................................................................... 6400/9467
2020-01-01T13:55:47.2233501Z .................................................................................................... 6600/9467
2020-01-01T13:55:49.4313574Z ...i................................................................................................ 6700/9467
2020-01-01T13:55:51.8748932Z .................................................................................................... 6800/9467
2020-01-01T13:55:54.5494132Z ..i................................................................................................. 6900/9467
---
2020-01-01T13:57:37.5747751Z .................................................................................................... 7500/9467
2020-01-01T13:57:42.6395102Z .................................................................................................... 7600/9467
2020-01-01T13:57:48.4800909Z .................................................................................................... 7700/9467
2020-01-01T13:57:59.1127263Z .................................................................................................... 7800/9467
2020-01-01T13:58:07.1177261Z .....................................iiii........................................................... 7900/9467
2020-01-01T13:58:22.5166315Z .................................................................................................... 8100/9467
2020-01-01T13:58:31.4894355Z .................................................................................................... 8200/9467
2020-01-01T13:58:46.3146068Z .................................................................................................... 8300/9467
2020-01-01T13:58:54.5628916Z .................................................................................................... 8400/9467
---
2020-01-01T14:00:55.6401655Z 
2020-01-01T14:00:55.6402255Z ---- [ui] ui/dollar-crate/dollar-crate-is-keyword.rs stdout ----
2020-01-01T14:00:55.6402531Z diff of stderr:
2020-01-01T14:00:55.6402569Z 
2020-01-01T14:00:55.6402611Z 16 LL | m!();
2020-01-01T14:00:55.6403360Z 18 
2020-01-01T14:00:55.6403586Z - warning: `$crate` may not be imported
2020-01-01T14:00:55.6403635Z + error: `$crate` may not be imported
2020-01-01T14:00:55.6403871Z 20   --> $DIR/dollar-crate-is-keyword.rs:9:9
2020-01-01T14:00:55.6403871Z 20   --> $DIR/dollar-crate-is-keyword.rs:9:9
2020-01-01T14:00:55.6403918Z 21    |
2020-01-01T14:00:55.6403960Z 22 LL |         use $crate; // OK
2020-01-01T14:00:55.6404045Z 24 ...
2020-01-01T14:00:55.6404045Z 24 ...
2020-01-01T14:00:55.6404085Z 25 LL | m!();
2020-01-01T14:00:55.6404701Z -    |
2020-01-01T14:00:55.6404701Z -    |
2020-01-01T14:00:55.6405005Z -    = note: `use $crate;` was erroneously allowed and will become a hard error in a future release
2020-01-01T14:00:55.6405277Z - warning: `$crate` may not be imported
2020-01-01T14:00:55.6405352Z + error: `$crate` may not be imported
2020-01-01T14:00:55.6405580Z 31   --> $DIR/dollar-crate-is-keyword.rs:11:9
2020-01-01T14:00:55.6405625Z 32    |
2020-01-01T14:00:55.6405625Z 32    |
2020-01-01T14:00:55.6405684Z 33 LL |         use $crate as $crate;
2020-01-01T14:00:55.6405752Z 35 ...
2020-01-01T14:00:55.6405752Z 35 ...
2020-01-01T14:00:55.6405793Z 36 LL | m!();
2020-01-01T14:00:55.6406206Z -    |
2020-01-01T14:00:55.6406206Z -    |
2020-01-01T14:00:55.6406476Z -    = note: `use $crate;` was erroneously allowed and will become a hard error in a future release
2020-01-01T14:00:55.6406756Z - error: aborting due to 2 previous errors
2020-01-01T14:00:55.6406814Z + error: aborting due to 4 previous errors
2020-01-01T14:00:55.6406869Z 42 
2020-01-01T14:00:55.6406907Z 43 
2020-01-01T14:00:55.6406907Z 43 
2020-01-01T14:00:55.6406932Z 
2020-01-01T14:00:55.6406958Z 
2020-01-01T14:00:55.6407019Z The actual stderr differed from the expected stderr.
2020-01-01T14:00:55.6407347Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword/dollar-crate-is-keyword.stderr
2020-01-01T14:00:55.6407613Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T14:00:55.6407952Z To only update this specific test, also pass `--test-args dollar-crate/dollar-crate-is-keyword.rs`
2020-01-01T14:00:55.6408039Z error: 1 errors occurred comparing output.
2020-01-01T14:00:55.6408087Z status: exit code: 1
2020-01-01T14:00:55.6408087Z status: exit code: 1
2020-01-01T14:00:55.6409012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword/auxiliary" "-A" "unused"
2020-01-01T14:00:55.6409384Z ------------------------------------------
2020-01-01T14:00:55.6409420Z 
2020-01-01T14:00:55.6409678Z ------------------------------------------
2020-01-01T14:00:55.6409726Z stderr:
2020-01-01T14:00:55.6409726Z stderr:
2020-01-01T14:00:55.6409962Z ------------------------------------------
2020-01-01T14:00:55.6410014Z error: expected identifier, found reserved identifier `$crate`
2020-01-01T14:00:55.6410320Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:6:20
2020-01-01T14:00:55.6410374Z    |
2020-01-01T14:00:55.6410427Z LL |             struct $crate {} //~ ERROR expected identifier, found reserved identifier `$crate`
2020-01-01T14:00:55.6410553Z ...
2020-01-01T14:00:55.6410553Z ...
2020-01-01T14:00:55.6410596Z LL | m!();
2020-01-01T14:00:55.6411035Z 
2020-01-01T14:00:55.6411083Z error: expected identifier, found reserved identifier `$crate`
2020-01-01T14:00:55.6411373Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:11:23
2020-01-01T14:00:55.6411424Z    |
2020-01-01T14:00:55.6411424Z    |
2020-01-01T14:00:55.6411478Z LL |         use $crate as $crate; //~ ERROR expected identifier, found reserved identifier `$crate`
2020-01-01T14:00:55.6411599Z ...
2020-01-01T14:00:55.6411599Z ...
2020-01-01T14:00:55.6411641Z LL | m!();
2020-01-01T14:00:55.6412013Z 
2020-01-01T14:00:55.6412059Z error: `$crate` may not be imported
2020-01-01T14:00:55.6412361Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:9:9
2020-01-01T14:00:55.6412427Z    |
2020-01-01T14:00:55.6412427Z    |
2020-01-01T14:00:55.6412472Z LL |         use $crate; // OK
2020-01-01T14:00:55.6412569Z ...
2020-01-01T14:00:55.6412569Z ...
2020-01-01T14:00:55.6412630Z LL | m!();
2020-01-01T14:00:55.6413935Z 
2020-01-01T14:00:55.6413979Z error: `$crate` may not be imported
2020-01-01T14:00:55.6414265Z   --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:11:9
2020-01-01T14:00:55.6414313Z    |
2020-01-01T14:00:55.6414313Z    |
2020-01-01T14:00:55.6414363Z LL |         use $crate as $crate; //~ ERROR expected identifier, found reserved identifier `$crate`
2020-01-01T14:00:55.6414470Z ...
2020-01-01T14:00:55.6414470Z ...
2020-01-01T14:00:55.6414510Z LL | m!();
2020-01-01T14:00:55.6415644Z 
2020-01-01T14:00:55.6415692Z error: aborting due to 4 previous errors
2020-01-01T14:00:55.6415721Z 
2020-01-01T14:00:55.6415746Z 
2020-01-01T14:00:55.6415746Z 
2020-01-01T14:00:55.6416043Z ------------------------------------------
2020-01-01T14:00:55.6416077Z 
2020-01-01T14:00:55.6416102Z 
2020-01-01T14:00:55.6416339Z ---- [ui] ui/imports/import-crate-var.rs stdout ----
2020-01-01T14:00:55.6416388Z 
2020-01-01T14:00:55.6417241Z error: test compilation failed although it shouldn't!
2020-01-01T14:00:55.6417311Z status: exit code: 1
2020-01-01T14:00:55.6418967Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/import-crate-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-crate-var" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/import-crate-var/auxiliary" "-A" "unused"
2020-01-01T14:00:55.6420000Z ------------------------------------------
2020-01-01T14:00:55.6420051Z 
2020-01-01T14:00:55.6420281Z ------------------------------------------
2020-01-01T14:00:55.6420928Z stderr:
---
2020-01-01T14:00:55.6430342Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-01T14:00:55.6430419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-01T14:00:55.6449224Z 
2020-01-01T14:00:55.6449764Z 
2020-01-01T14:00:55.6451998Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-01T14:00:55.6452585Z 
2020-01-01T14:00:55.6453625Z 
2020-01-01T14:00:55.6453948Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-01T14:00:55.6454114Z Build completed unsuccessfully in 1:09:11
2020-01-01T14:00:55.6454114Z Build completed unsuccessfully in 1:09:11
2020-01-01T14:00:55.6516973Z == clock drift check ==
2020-01-01T14:00:55.6534413Z   local time: Wed Jan  1 14:00:55 UTC 2020
2020-01-01T14:00:55.9391060Z   network time: Wed, 01 Jan 2020 14:00:55 GMT
2020-01-01T14:00:55.9392943Z == end clock drift check ==
2020-01-01T14:00:57.1437696Z 
2020-01-01T14:00:57.1552106Z ##[error]Bash exited with code '1'.
2020-01-01T14:00:57.1588944Z ##[section]Starting: Checkout
2020-01-01T14:00:57.1590838Z ==============================================================================
2020-01-01T14:00:57.1590894Z Task         : Get sources
2020-01-01T14:00:57.1590941Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
