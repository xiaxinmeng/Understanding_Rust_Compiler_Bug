plain
2019-11-11T22:02:27.2063660Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T22:02:27.2238856Z ##[command]git config gc.auto 0
2019-11-11T22:02:27.2316300Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T22:02:27.2368903Z ##[command]git config --get-all http.proxy
2019-11-11T22:02:27.2496935Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65232/merge:refs/remotes/pull/65232/merge
---
2019-11-11T22:56:51.7647391Z .................................................................................................... 1400/9228
2019-11-11T22:56:57.6557540Z .................................................................................................... 1500/9228
2019-11-11T22:57:03.5510691Z .................................................................................................... 1600/9228
2019-11-11T22:57:12.4182452Z .................................................................................................... 1700/9228
2019-11-11T22:57:20.8519757Z ..i................................................................................................. 1800/9228
2019-11-11T22:57:27.3664908Z ......................................................................................iiiii......... 1900/9228
2019-11-11T22:57:47.6022956Z .................................................................................................... 2100/9228
2019-11-11T22:57:50.0103305Z .................................................................................................... 2200/9228
2019-11-11T22:57:52.4679351Z .................................................................................................... 2300/9228
2019-11-11T22:58:02.3423869Z .................................................................................................... 2400/9228
---
2019-11-11T23:00:56.2664681Z ..................................................................................i...............i. 4700/9228
2019-11-11T23:01:03.2843761Z .................................................................................................... 4800/9228
2019-11-11T23:01:12.3901829Z .................................................................................................... 4900/9228
2019-11-11T23:01:17.7407805Z .................................................................................................... 5000/9228
2019-11-11T23:01:29.2102733Z .....................................................................................ii.ii.......... 5100/9228
2019-11-11T23:01:33.0580251Z .i.................................................................................................. 5200/9228
2019-11-11T23:01:47.6396253Z .................................................................................................... 5400/9228
2019-11-11T23:01:54.6297852Z ...................................................................i................................ 5500/9228
2019-11-11T23:02:02.0087804Z .................................................................................................... 5600/9228
2019-11-11T23:02:09.9690698Z .................................................................................................... 5700/9228
2019-11-11T23:02:09.9690698Z .................................................................................................... 5700/9228
2019-11-11T23:02:19.0329938Z ....................................................ii...i..ii...........i.......................... 5800/9228
2019-11-11T23:02:41.6383994Z .................................................................................................... 6000/9228
2019-11-11T23:02:50.0969625Z .................................................................................................... 6100/9228
2019-11-11T23:02:50.0969625Z .................................................................................................... 6100/9228
2019-11-11T23:02:55.4023674Z .......................................................................i..ii........................ 6200/9228
2019-11-11T23:03:24.6834541Z .................................................................................................... 6400/9228
2019-11-11T23:03:26.7767033Z .......................................i............................................................ 6500/9228
2019-11-11T23:03:28.9522987Z .................................................................................................... 6600/9228
2019-11-11T23:03:31.2108286Z .......................i............................................................................ 6700/9228
---
2019-11-11T23:08:18.6817240Z 
2019-11-11T23:08:18.6818373Z ---- [ui] ui/coherence/coherence-subtyping.rs#old stdout ----
2019-11-11T23:08:18.6818600Z diff of stderr:
2019-11-11T23:08:18.6818730Z 
2019-11-11T23:08:18.6819194Z 1 error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T23:08:18.6819571Z -   --> $DIR/coherence-subtyping.rs:18:1
2019-11-11T23:08:18.6819962Z +   --> $DIR/coherence-subtyping.rs:16:1
2019-11-11T23:08:18.6820155Z 3    |
2019-11-11T23:08:18.6820659Z 4 LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T23:08:18.6821041Z 5    | --------------------------------------------------------- first implementation here
2019-11-11T23:08:18.6821319Z 
2019-11-11T23:08:18.6821450Z The actual stderr differed from the expected stderr.
2019-11-11T23:08:18.6821450Z The actual stderr differed from the expected stderr.
2019-11-11T23:08:18.6821893Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.old/coherence-subtyping.old.stderr
2019-11-11T23:08:18.6822277Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T23:08:18.6822709Z To only update this specific test, also pass `--test-args coherence/coherence-subtyping.rs`
2019-11-11T23:08:18.6822889Z 
2019-11-11T23:08:18.6823030Z error in revision `old`: 1 errors occurred comparing output.
2019-11-11T23:08:18.6823163Z status: exit code: 1
2019-11-11T23:08:18.6824253Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "old" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.old" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.old/auxiliary" "-A" "unused"
2019-11-11T23:08:18.6825233Z ------------------------------------------
2019-11-11T23:08:18.6825807Z 
2019-11-11T23:08:18.6826598Z ------------------------------------------
2019-11-11T23:08:18.6826820Z stderr:
2019-11-11T23:08:18.6826820Z stderr:
2019-11-11T23:08:18.6827197Z ------------------------------------------
2019-11-11T23:08:18.6827697Z error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T23:08:18.6828683Z    |
2019-11-11T23:08:18.6828683Z    |
2019-11-11T23:08:18.6829371Z LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T23:08:18.6829771Z    | --------------------------------------------------------- first implementation here
2019-11-11T23:08:18.6829958Z ...
2019-11-11T23:08:18.6830318Z LL | impl TheTrait for for<'a> fn(&'a u8, &'a u8) -> &'a u8 {
2019-11-11T23:08:18.6830774Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
2019-11-11T23:08:18.6830970Z    |
2019-11-11T23:08:18.6831519Z    = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
2019-11-11T23:08:18.6831824Z error: aborting due to previous error
2019-11-11T23:08:18.6831937Z 
2019-11-11T23:08:18.6833457Z For more information about this error, try `rustc --explain E0119`.
2019-11-11T23:08:18.6833654Z 
2019-11-11T23:08:18.6833654Z 
2019-11-11T23:08:18.6833977Z ------------------------------------------
2019-11-11T23:08:18.6834116Z 
2019-11-11T23:08:18.6834225Z 
2019-11-11T23:08:18.6835166Z ---- [ui] ui/coherence/coherence-subtyping.rs#re stdout ----
2019-11-11T23:08:18.6835393Z diff of stderr:
2019-11-11T23:08:18.6835523Z 
2019-11-11T23:08:18.6836001Z 1 error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T23:08:18.6836401Z -   --> $DIR/coherence-subtyping.rs:18:1
2019-11-11T23:08:18.6836784Z +   --> $DIR/coherence-subtyping.rs:16:1
2019-11-11T23:08:18.6837190Z 3    |
2019-11-11T23:08:18.6837640Z 4 LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T23:08:18.6838107Z 5    | --------------------------------------------------------- first implementation here
2019-11-11T23:08:18.6838533Z 
2019-11-11T23:08:18.6838681Z The actual stderr differed from the expected stderr.
2019-11-11T23:08:18.6838681Z The actual stderr differed from the expected stderr.
2019-11-11T23:08:18.6839118Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.re/coherence-subtyping.re.stderr
2019-11-11T23:08:18.6839489Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T23:08:18.6840164Z To only update this specific test, also pass `--test-args coherence/coherence-subtyping.rs`
2019-11-11T23:08:18.6840492Z 
2019-11-11T23:08:18.6840643Z error in revision `re`: 1 errors occurred comparing output.
2019-11-11T23:08:18.6840810Z status: exit code: 1
2019-11-11T23:08:18.6841722Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "re" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.re" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.re/auxiliary" "-A" "unused"
2019-11-11T23:08:18.6842478Z ------------------------------------------
2019-11-11T23:08:18.6843135Z 
2019-11-11T23:08:18.6843767Z ------------------------------------------
2019-11-11T23:08:18.6845074Z stderr:
2019-11-11T23:08:18.6845074Z stderr:
2019-11-11T23:08:18.6845646Z ------------------------------------------
2019-11-11T23:08:18.6846138Z error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T23:08:18.6846520Z    |
2019-11-11T23:08:18.6846520Z    |
2019-11-11T23:08:18.6846766Z LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T23:08:18.6847033Z    | --------------------------------------------------------- first implementation here
2019-11-11T23:08:18.6847094Z ...
2019-11-11T23:08:18.6847355Z LL | impl TheTrait for for<'a> fn(&'a u8, &'a u8) -> &'a u8 {
2019-11-11T23:08:18.6847669Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
2019-11-11T23:08:18.6847722Z    |
2019-11-11T23:08:18.6848022Z    = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
2019-11-11T23:08:18.6848276Z error: aborting due to previous error
2019-11-11T23:08:18.6848302Z 
2019-11-11T23:08:18.6848541Z For more information about this error, try `rustc --explain E0119`.
2019-11-11T23:08:18.6848572Z 
---
2019-11-11T23:08:18.6857779Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-11T23:08:18.6857861Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-11T23:08:18.6875997Z 
2019-11-11T23:08:18.6876125Z 
2019-11-11T23:08:18.6877963Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-11T23:08:18.6878396Z 
2019-11-11T23:08:18.6878425Z 
2019-11-11T23:08:18.6917714Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-11T23:08:18.6917811Z Build completed unsuccessfully in 0:59:36
2019-11-11T23:08:18.6917811Z Build completed unsuccessfully in 0:59:36
2019-11-11T23:08:18.6937667Z == clock drift check ==
2019-11-11T23:08:18.6953391Z   local time: Mon Nov 11 23:08:18 UTC 2019
2019-11-11T23:08:18.8458366Z   network time: Mon, 11 Nov 2019 23:08:18 GMT
2019-11-11T23:08:18.8462861Z == end clock drift check ==
2019-11-11T23:08:19.5991515Z 
2019-11-11T23:08:19.6102299Z ##[error]Bash exited with code '1'.
2019-11-11T23:08:19.6142956Z ##[section]Starting: Checkout
2019-11-11T23:08:19.6145278Z ==============================================================================
2019-11-11T23:08:19.6145336Z Task         : Get sources
2019-11-11T23:08:19.6145403Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
