plain
2019-10-03T18:47:59.0592524Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T18:47:59.0762781Z ##[command]git config gc.auto 0
2019-10-03T18:47:59.0843078Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T18:47:59.0904023Z ##[command]git config --get-all http.proxy
2019-10-03T18:47:59.1057899Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65068/merge:refs/remotes/pull/65068/merge
---
2019-10-03T19:51:35.8635323Z .................................................................................................... 1500/9098
2019-10-03T19:51:42.9262973Z .................................................................................................... 1600/9098
2019-10-03T19:51:52.6297043Z .................................................................................................... 1700/9098
2019-10-03T19:52:02.0553428Z .....i...............i.............................................................................. 1800/9098
2019-10-03T19:52:09.3663060Z ................................................................................................iiii 1900/9098
2019-10-03T19:52:26.5900159Z i................................................................................................... 2000/9098
2019-10-03T19:52:35.5033255Z .................................................................................................... 2200/9098
2019-10-03T19:52:38.2501429Z .................................................................................................... 2300/9098
2019-10-03T19:52:44.7862991Z .................................................................................................... 2400/9098
2019-10-03T19:52:50.5291687Z .................................................................................................... 2500/9098
---
2019-10-03T19:55:49.5788517Z ...................................................................................i...............i 4700/9098
2019-10-03T19:55:58.1479456Z .................................................................................................... 4800/9098
2019-10-03T19:56:08.8864401Z .................................................................................................... 4900/9098
2019-10-03T19:56:14.6748707Z .................................................................................................... 5000/9098
2019-10-03T19:56:27.0948227Z ...........................................................................ii.ii.................... 5100/9098
2019-10-03T19:56:36.9407340Z .................................................................................................... 5300/9098
2019-10-03T19:56:46.9567469Z .................................................................................................... 5400/9098
2019-10-03T19:56:54.1327424Z ..........................................i......................................................... 5500/9098
2019-10-03T19:57:01.0972801Z .................................................................................................... 5600/9098
2019-10-03T19:57:01.0972801Z .................................................................................................... 5600/9098
2019-10-03T19:57:12.1199612Z .................................................................................................... 5700/9098
2019-10-03T19:57:22.6740583Z .......................................ii...i..ii...........i....................................... 5800/9098
2019-10-03T19:57:46.1705835Z .................................................................................................... 6000/9098
2019-10-03T19:57:55.7843815Z .................................................................................................... 6100/9098
2019-10-03T19:57:55.7843815Z .................................................................................................... 6100/9098
2019-10-03T19:58:09.0334048Z ............................................i..ii................................................... 6200/9098
2019-10-03T19:58:31.4434179Z .................................................................................................... 6400/9098
2019-10-03T19:58:33.6730957Z ....i............................................................................................... 6500/9098
2019-10-03T19:58:35.9642522Z ............................................................................i....................... 6600/9098
2019-10-03T19:58:38.7490385Z .................................................................................................... 6700/9098
---
2019-10-03T20:02:53.7735281Z failures:
2019-10-03T20:02:53.7775321Z 
2019-10-03T20:02:53.7776459Z ---- [ui] ui/lifetimes/lifetime-mismatch-between-trait-and-impl.rs stdout ----
2019-10-03T20:02:53.7776533Z normalized stderr:
2019-10-03T20:02:53.7777111Z error: `impl` item doesn't match `trait` item
2019-10-03T20:02:53.7777684Z   --> $DIR/lifetime-mismatch-between-trait-and-impl.rs:6:5
2019-10-03T20:02:53.7777896Z    |
2019-10-03T20:02:53.7778312Z LL |     fn foo<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
2019-10-03T20:02:53.7778775Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&i32, &i32) -> &i32
2019-10-03T20:02:53.7779007Z    |
2019-10-03T20:02:53.7779332Z    = note: expected: fn(&i32, &'a i32) -> &'a i32
2019-10-03T20:02:53.7779739Z               found: fn(&i32, &i32) -> &i32
2019-10-03T20:02:53.7780031Z error: aborting due to previous error
2019-10-03T20:02:53.7780062Z 
2019-10-03T20:02:53.7780090Z 
2019-10-03T20:02:53.7780117Z 
2019-10-03T20:02:53.7780117Z 
2019-10-03T20:02:53.7780160Z 
2019-10-03T20:02:53.7780209Z The actual stderr differed from the expected stderr.
2019-10-03T20:02:53.7780617Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-mismatch-between-trait-and-impl/lifetime-mismatch-between-trait-and-impl.stderr
2019-10-03T20:02:53.7780900Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T20:02:53.7781472Z To only update this specific test, also pass `--test-args lifetimes/lifetime-mismatch-between-trait-and-impl.rs`
2019-10-03T20:02:53.7782263Z error: 1 errors occurred comparing output.
2019-10-03T20:02:53.7782324Z status: exit code: 1
2019-10-03T20:02:53.7782324Z status: exit code: 1
2019-10-03T20:02:53.7783318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-mismatch-between-trait-and-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-mismatch-between-trait-and-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-mismatch-between-trait-and-impl/auxiliary" "-A" "unused"
2019-10-03T20:02:53.7783700Z ------------------------------------------
2019-10-03T20:02:53.7783749Z 
2019-10-03T20:02:53.7783998Z ------------------------------------------
2019-10-03T20:02:53.7784067Z stderr:
2019-10-03T20:02:53.7784067Z stderr:
2019-10-03T20:02:53.7784640Z ------------------------------------------
2019-10-03T20:02:53.7784860Z error: `impl` item doesn't match `trait` item
2019-10-03T20:02:53.7785625Z   --> /checkout/src/test/ui/lifetimes/lifetime-mismatch-between-trait-and-impl.rs:6:5
2019-10-03T20:02:53.7785820Z    |
2019-10-03T20:02:53.7786140Z LL |     fn foo<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
2019-10-03T20:02:53.7786429Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&i32, &i32) -> &i32
2019-10-03T20:02:53.7786622Z    |
2019-10-03T20:02:53.7786980Z    = note: expected: fn(&i32, &'a i32) -> &'a i32
2019-10-03T20:02:53.7787238Z               found: fn(&i32, &i32) -> &i32
2019-10-03T20:02:53.7787465Z error: aborting due to previous error
2019-10-03T20:02:53.7787527Z 
2019-10-03T20:02:53.7787578Z 
2019-10-03T20:02:53.7787865Z ------------------------------------------
---
2019-10-03T20:02:53.7821693Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T20:02:53.7822027Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T20:02:53.7839706Z 
2019-10-03T20:02:53.7839803Z 
2019-10-03T20:02:53.7844440Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T20:02:53.7844930Z 
2019-10-03T20:02:53.7844966Z 
2019-10-03T20:02:54.4989159Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T20:02:54.4989541Z Build completed unsuccessfully in 1:07:26
2019-10-03T20:02:54.4989541Z Build completed unsuccessfully in 1:07:26
2019-10-03T20:02:54.4989688Z == clock drift check ==
2019-10-03T20:02:54.4989764Z   local time: Thu Oct  3 20:02:53 UTC 2019
2019-10-03T20:02:54.4989816Z   network time: Thu, 03 Oct 2019 20:02:53 GMT
2019-10-03T20:02:54.4989864Z == end clock drift check ==
2019-10-03T20:02:54.9551787Z ##[error]Bash exited with code '1'.
2019-10-03T20:02:54.9603562Z ##[section]Starting: Checkout
2019-10-03T20:02:54.9605887Z ==============================================================================
2019-10-03T20:02:54.9605962Z Task         : Get sources
2019-10-03T20:02:54.9606026Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
