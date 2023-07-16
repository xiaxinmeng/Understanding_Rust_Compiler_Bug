plain
2020-01-01T23:51:42.8996566Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T23:51:42.9213514Z ##[command]git config gc.auto 0
2020-01-01T23:51:42.9680327Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T23:51:42.9737376Z ##[command]git config --get-all http.proxy
2020-01-01T23:51:42.9898189Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67783/merge:refs/remotes/pull/67783/merge
---
2020-01-02T00:50:37.6901846Z .................................................................................................... 1500/9468
2020-01-02T00:50:43.4884252Z .................................................................................................... 1600/9468
2020-01-02T00:50:48.2441235Z .................................................................................................... 1700/9468
2020-01-02T00:50:57.4797880Z .................................................................................................... 1800/9468
2020-01-02T00:51:05.5873875Z i................................................................................................... 1900/9468
2020-01-02T00:51:12.2441853Z ......................................................................................iiiii......... 2000/9468
2020-01-02T00:51:33.5824060Z .................................................................................................... 2200/9468
2020-01-02T00:51:35.9908826Z .................................................................................................... 2300/9468
2020-01-02T00:51:38.4208216Z .................................................................................................... 2400/9468
2020-01-02T00:51:44.3676859Z .................................................................................................... 2500/9468
---
2020-01-02T00:54:39.6264184Z .................i...............i.................................................................. 4900/9468
2020-01-02T00:54:49.3518705Z .................................................................................................... 5000/9468
2020-01-02T00:54:54.8400870Z ..............................................................i..................................... 5100/9468
2020-01-02T00:55:02.7549773Z .................................................................................................... 5200/9468
2020-01-02T00:55:10.2506682Z .............................ii.ii...........i...................................................... 5300/9468
2020-01-02T00:55:19.4809688Z .................................................................................................... 5500/9468
2020-01-02T00:55:29.3763114Z .................................................................................................... 5600/9468
2020-01-02T00:55:36.3387408Z ............i....................................................................................... 5700/9468
2020-01-02T00:55:42.4082621Z ........................................................F........................................... 5800/9468
2020-01-02T00:55:42.4082621Z ........................................................F........................................... 5800/9468
2020-01-02T00:55:52.8578426Z .................................................................................................... 5900/9468
2020-01-02T00:56:04.3997544Z .ii...i..ii...........i............................................................................. 6000/9468
2020-01-02T00:56:21.6271915Z .................................................................................................... 6200/9468
2020-01-02T00:56:28.8551753Z .................................................................................................... 6300/9468
2020-01-02T00:56:28.8551753Z .................................................................................................... 6300/9468
2020-01-02T00:56:55.1356682Z ............................i..ii................................................................... 6400/9468
2020-01-02T00:57:13.9451048Z .................................................................................................... 6600/9468
2020-01-02T00:57:16.0705521Z ...i................................................................................................ 6700/9468
2020-01-02T00:57:18.4327117Z .................................................................................................... 6800/9468
2020-01-02T00:57:20.9647376Z ...i................................................................................................ 6900/9468
---
2020-01-02T00:58:56.6369650Z .................................................................................................... 7500/9468
2020-01-02T00:59:01.4406485Z .................................................................................................... 7600/9468
2020-01-02T00:59:06.7357915Z .................................................................................................... 7700/9468
2020-01-02T00:59:17.5360308Z .................................................................................................... 7800/9468
2020-01-02T00:59:25.3707716Z .....................................iiii........................................................... 7900/9468
2020-01-02T00:59:40.7684245Z .................................................................................................... 8100/9468
2020-01-02T00:59:49.9405733Z .................................................................................................... 8200/9468
2020-01-02T01:00:04.7348795Z .................................................................................................... 8300/9468
2020-01-02T01:00:12.9476728Z .................................................................................................... 8400/9468
---
2020-01-02T01:02:09.7140743Z 
2020-01-02T01:02:09.7141485Z ---- [ui] ui/match/match-same-name-enum-variant.rs stdout ----
2020-01-02T01:02:09.7141758Z diff of stderr:
2020-01-02T01:02:09.7141903Z 
2020-01-02T01:02:09.7142080Z 1 warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7142508Z -   --> $DIR/match-same-name-enum-variant.rs:14:9
2020-01-02T01:02:09.7142971Z +   --> $DIR/match-same-name-enum-variant.rs:17:9
2020-01-02T01:02:09.7143313Z 4 LL |         Bar => println!("A"),
2020-01-02T01:02:09.7143313Z 4 LL |         Bar => println!("A"),
2020-01-02T01:02:09.7143482Z 5    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-02T01:02:09.7143773Z 6 
2020-01-02T01:02:09.7143773Z 6 
2020-01-02T01:02:09.7143937Z 7 warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7144338Z -   --> $DIR/match-same-name-enum-variant.rs:16:9
2020-01-02T01:02:09.7144768Z +   --> $DIR/match-same-name-enum-variant.rs:19:9
2020-01-02T01:02:09.7144991Z 9    |
2020-01-02T01:02:09.7145137Z 10 LL |         Baz => println!("B"),
2020-01-02T01:02:09.7145289Z 11    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-02T01:02:09.7145572Z 12 
2020-01-02T01:02:09.7145572Z 12 
2020-01-02T01:02:09.7145723Z 13 warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7146125Z -   --> $DIR/match-same-name-enum-variant.rs:23:9
2020-01-02T01:02:09.7146535Z +   --> $DIR/match-same-name-enum-variant.rs:26:9
2020-01-02T01:02:09.7146910Z 16 LL |         Bar => println!("A"),
2020-01-02T01:02:09.7146910Z 16 LL |         Bar => println!("A"),
2020-01-02T01:02:09.7147058Z 17    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-02T01:02:09.7147341Z 18 
2020-01-02T01:02:09.7147341Z 18 
2020-01-02T01:02:09.7147499Z 19 warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7147906Z -   --> $DIR/match-same-name-enum-variant.rs:25:9
2020-01-02T01:02:09.7148329Z +   --> $DIR/match-same-name-enum-variant.rs:28:9
2020-01-02T01:02:09.7148521Z 21    |
2020-01-02T01:02:09.7148685Z 22 LL |         Baz => println!("B"),
2020-01-02T01:02:09.7148837Z 23    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-02T01:02:09.7149083Z 
2020-01-02T01:02:09.7149244Z The actual stderr differed from the expected stderr.
2020-01-02T01:02:09.7149244Z The actual stderr differed from the expected stderr.
2020-01-02T01:02:09.7149722Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-same-name-enum-variant/match-same-name-enum-variant.stderr
2020-01-02T01:02:09.7150541Z To update references, rerun the tests and pass the `--bless` flag
2020-01-02T01:02:09.7151081Z To only update this specific test, also pass `--test-args match/match-same-name-enum-variant.rs`
2020-01-02T01:02:09.7151617Z error: 1 errors occurred comparing output.
2020-01-02T01:02:09.7151833Z status: exit code: 0
2020-01-02T01:02:09.7151833Z status: exit code: 0
2020-01-02T01:02:09.7152982Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-same-name-enum-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-same-name-enum-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-same-name-enum-variant/auxiliary" "-A" "unused"
2020-01-02T01:02:09.7153697Z ------------------------------------------
2020-01-02T01:02:09.7153881Z 
2020-01-02T01:02:09.7154295Z ------------------------------------------
2020-01-02T01:02:09.7154519Z stderr:
2020-01-02T01:02:09.7154519Z stderr:
2020-01-02T01:02:09.7154910Z ------------------------------------------
2020-01-02T01:02:09.7155152Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7155592Z   --> /checkout/src/test/ui/match/match-same-name-enum-variant.rs:17:9
2020-01-02T01:02:09.7155976Z LL |         Bar => println!("A"),
2020-01-02T01:02:09.7155976Z LL |         Bar => println!("A"),
2020-01-02T01:02:09.7156132Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-02T01:02:09.7156265Z 
2020-01-02T01:02:09.7156442Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7156894Z   --> /checkout/src/test/ui/match/match-same-name-enum-variant.rs:19:9
2020-01-02T01:02:09.7157093Z    |
2020-01-02T01:02:09.7157270Z LL |         Baz => println!("B"),
2020-01-02T01:02:09.7158093Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-02T01:02:09.7158282Z 
2020-01-02T01:02:09.7158473Z warning[E0170]: pattern binding `Bar` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7158963Z   --> /checkout/src/test/ui/match/match-same-name-enum-variant.rs:26:9
2020-01-02T01:02:09.7159336Z LL |         Bar => println!("A"),
2020-01-02T01:02:09.7159336Z LL |         Bar => println!("A"),
2020-01-02T01:02:09.7159481Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Bar`
2020-01-02T01:02:09.7159606Z 
2020-01-02T01:02:09.7159774Z warning[E0170]: pattern binding `Baz` is named the same as one of the variants of the type `Foo`
2020-01-02T01:02:09.7160184Z   --> /checkout/src/test/ui/match/match-same-name-enum-variant.rs:28:9
2020-01-02T01:02:09.7160467Z    |
2020-01-02T01:02:09.7160971Z LL |         Baz => println!("B"),
2020-01-02T01:02:09.7161209Z    |         ^^^ help: to match on the variant, qualify the path: `Foo::Baz`
2020-01-02T01:02:09.7161905Z 
2020-01-02T01:02:09.7162643Z ------------------------------------------
2020-01-02T01:02:09.7163395Z 
2020-01-02T01:02:09.7163463Z 
---
2020-01-02T01:02:09.7171719Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-02T01:02:09.7171827Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-02T01:02:09.7198923Z 
2020-01-02T01:02:09.7203406Z 
2020-01-02T01:02:09.7207367Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-02T01:02:09.7208492Z 
2020-01-02T01:02:09.7208566Z 
2020-01-02T01:02:09.7216914Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-02T01:02:09.7217052Z Build completed unsuccessfully in 1:03:54
2020-01-02T01:02:09.7217052Z Build completed unsuccessfully in 1:03:54
2020-01-02T01:02:09.7285986Z == clock drift check ==
2020-01-02T01:02:09.7307414Z   local time: Thu Jan  2 01:02:09 UTC 2020
2020-01-02T01:02:10.0055025Z   network time: Thu, 02 Jan 2020 01:02:10 GMT
2020-01-02T01:02:10.0058339Z == end clock drift check ==
2020-01-02T01:02:10.8593949Z 
2020-01-02T01:02:10.8701204Z ##[error]Bash exited with code '1'.
2020-01-02T01:02:10.8737532Z ##[section]Starting: Checkout
2020-01-02T01:02:10.8739420Z ==============================================================================
2020-01-02T01:02:10.8739477Z Task         : Get sources
2020-01-02T01:02:10.8739539Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
