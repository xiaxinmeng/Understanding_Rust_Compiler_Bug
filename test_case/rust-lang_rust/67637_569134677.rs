plain
2019-12-26T20:08:14.1172235Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T20:08:14.1189107Z ##[command]git config gc.auto 0
2019-12-26T20:08:14.1192513Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T20:08:14.1196184Z ##[command]git config --get-all http.proxy
2019-12-26T20:08:14.1198835Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67637/merge:refs/remotes/pull/67637/merge
---
2019-12-26T21:03:53.4518510Z .................................................................................................... 1600/9456
2019-12-26T21:03:58.2216570Z .................................................................................................... 1700/9456
2019-12-26T21:04:06.9387904Z .............................................................................................i...... 1800/9456
2019-12-26T21:04:14.5028007Z .................................................................................................... 1900/9456
2019-12-26T21:04:20.6224493Z ...............................................................................iiiii................ 2000/9456
2019-12-26T21:04:40.3142319Z .................................................................................................... 2200/9456
2019-12-26T21:04:42.5206493Z .................................................................................................... 2300/9456
2019-12-26T21:04:44.8338543Z .................................................................................................... 2400/9456
2019-12-26T21:04:50.4424702Z .................................................................................................... 2500/9456
---
2019-12-26T21:07:34.4070946Z ..........i...............i......................................................................... 4900/9456
2019-12-26T21:07:43.7818678Z .................................................................................................... 5000/9456
2019-12-26T21:07:48.4900667Z ......................................................i............................................. 5100/9456
2019-12-26T21:07:57.2712367Z .................................................................................................... 5200/9456
2019-12-26T21:08:03.0293000Z .....................ii.ii...........i.............................................................. 5300/9456
2019-12-26T21:08:11.2806965Z .................................................................................................... 5500/9456
2019-12-26T21:08:21.3339832Z .................................................................................................... 5600/9456
2019-12-26T21:08:27.9238112Z ...i................................................................................................ 5700/9456
2019-12-26T21:08:33.0867268Z .................................................................................................... 5800/9456
2019-12-26T21:08:33.0867268Z .................................................................................................... 5800/9456
2019-12-26T21:08:42.3050036Z ...........................................................................................ii...i..i 5900/9456
2019-12-26T21:08:53.8728540Z i...........i....................................................................................... 6000/9456
2019-12-26T21:09:09.7154580Z .................................................................................................... 6200/9456
2019-12-26T21:09:16.4082290Z .................................................................................................... 6300/9456
2019-12-26T21:09:16.4082290Z .................................................................................................... 6300/9456
2019-12-26T21:09:31.8639371Z ..................i..ii............................................................................. 6400/9456
2019-12-26T21:09:50.0025069Z ...............................................................................................i.... 6600/9456
2019-12-26T21:09:51.8808575Z .................................................................................................... 6700/9456
2019-12-26T21:09:54.0000298Z ...............................................................................................i.... 6800/9456
2019-12-26T21:09:56.4218202Z .................................................................................................... 6900/9456
---
2019-12-26T21:11:26.3424593Z .................................................................................................... 7500/9456
2019-12-26T21:11:30.8090525Z .......................................................F............................................ 7600/9456
2019-12-26T21:11:36.8890782Z .................................................................................................... 7700/9456
2019-12-26T21:11:47.5831107Z .................................................................................................... 7800/9456
2019-12-26T21:11:52.9279523Z ...........................iiii..................................................................... 7900/9456
2019-12-26T21:12:06.0117168Z .................................................................................................... 8100/9456
2019-12-26T21:12:14.8047232Z .................................................................................................... 8200/9456
2019-12-26T21:12:27.4031420Z .................................................................................................... 8300/9456
2019-12-26T21:12:34.1141879Z .................................................................................................... 8400/9456
---
2019-12-26T21:14:20.0646282Z 14   --> $DIR/resolve-primitive-fallback.rs:3:5
2019-12-26T21:14:20.0646647Z 
2019-12-26T21:14:20.0646811Z 
2019-12-26T21:14:20.0646955Z The actual stderr differed from the expected stderr.
2019-12-26T21:14:20.0647479Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-primitive-fallback/resolve-primitive-fallback.stderr
2019-12-26T21:14:20.0648302Z To update references, rerun the tests and pass the `--bless` flag
2019-12-26T21:14:20.0648722Z To only update this specific test, also pass `--test-args resolve/resolve-primitive-fallback.rs`
2019-12-26T21:14:20.0648980Z error: 1 errors occurred comparing output.
2019-12-26T21:14:20.0649111Z status: exit code: 1
2019-12-26T21:14:20.0649111Z status: exit code: 1
2019-12-26T21:14:20.0649950Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-primitive-fallback.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-primitive-fallback" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-primitive-fallback/auxiliary" "-A" "unused"
2019-12-26T21:14:20.0650462Z ------------------------------------------
2019-12-26T21:14:20.0650609Z 
2019-12-26T21:14:20.0650902Z ------------------------------------------
2019-12-26T21:14:20.0651039Z stderr:
---
2019-12-26T21:14:20.0657554Z ---- [ui] ui/shadow-bool.rs stdout ----
2019-12-26T21:14:20.0657863Z 
2019-12-26T21:14:20.0658154Z error: test compilation failed although it shouldn't!
2019-12-26T21:14:20.0658296Z status: exit code: 1
2019-12-26T21:14:20.0661807Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/shadow-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadow-bool" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadow-bool/auxiliary" "-A" "unused"
2019-12-26T21:14:20.0662887Z ------------------------------------------
2019-12-26T21:14:20.0662916Z 
2019-12-26T21:14:20.0663085Z ------------------------------------------
2019-12-26T21:14:20.0663136Z stderr:
---
2019-12-26T21:14:20.0663767Z 
2019-12-26T21:14:20.0663827Z error[E0601]: `main` function not found in crate `shadow_bool`
2019-12-26T21:14:20.0664635Z   --> /checkout/src/test/ui/shadow-bool.rs:3:1
2019-12-26T21:14:20.0664690Z    |
2019-12-26T21:14:20.0664750Z LL | / mod bar {
2019-12-26T21:14:20.0664795Z LL | |     pub trait QueryId {
2019-12-26T21:14:20.0664843Z LL | |         const SOME_PROPERTY: bool;
2019-12-26T21:14:20.0664946Z ...  |
2019-12-26T21:14:20.0664993Z LL | |     const SOME_PROPERTY: core::primitive::bool = true;
2019-12-26T21:14:20.0665039Z LL | | }
2019-12-26T21:14:20.0665039Z LL | | }
2019-12-26T21:14:20.0665321Z    | |_^ consider adding a `main` function to `/checkout/src/test/ui/shadow-bool.rs`
2019-12-26T21:14:20.0665402Z error: aborting due to previous error
2019-12-26T21:14:20.0665432Z 
2019-12-26T21:14:20.0665700Z For more information about this error, try `rustc --explain E0601`.
2019-12-26T21:14:20.0665736Z 
---
2019-12-26T21:14:20.0667131Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-26T21:14:20.0667190Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T21:14:20.0680721Z 
2019-12-26T21:14:20.0680801Z 
2019-12-26T21:14:20.0682359Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-26T21:14:20.0682646Z 
2019-12-26T21:14:20.0682669Z 
2019-12-26T21:14:20.0689699Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-26T21:14:20.0689937Z Build completed unsuccessfully in 0:59:42
2019-12-26T21:14:20.0689937Z Build completed unsuccessfully in 0:59:42
2019-12-26T21:14:20.0739096Z == clock drift check ==
2019-12-26T21:14:20.0756807Z   local time: Thu Dec 26 21:14:20 UTC 2019
2019-12-26T21:14:20.2298537Z   network time: Thu, 26 Dec 2019 21:14:20 GMT
2019-12-26T21:14:20.2298624Z == end clock drift check ==
2019-12-26T21:14:21.2017420Z 
2019-12-26T21:14:21.2107535Z ##[error]Bash exited with code '1'.
2019-12-26T21:14:21.2142778Z ##[section]Starting: Checkout
2019-12-26T21:14:21.2144822Z ==============================================================================
2019-12-26T21:14:21.2144881Z Task         : Get sources
2019-12-26T21:14:21.2144966Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
