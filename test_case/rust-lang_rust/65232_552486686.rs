plain
2019-11-11T14:13:15.1576502Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T14:13:15.1768153Z ##[command]git config gc.auto 0
2019-11-11T14:13:15.1822857Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T14:13:15.1855333Z ##[command]git config --get-all http.proxy
2019-11-11T14:13:15.2012362Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65232/merge:refs/remotes/pull/65232/merge
---
2019-11-11T15:07:40.8233505Z .................................................................................................... 1400/9225
2019-11-11T15:07:46.5733984Z .................................................................................................... 1500/9225
2019-11-11T15:07:52.1154842Z .................................................................................................... 1600/9225
2019-11-11T15:08:00.7540114Z .................................................................................................... 1700/9225
2019-11-11T15:08:08.6262271Z ..i................................................................................................. 1800/9225
2019-11-11T15:08:14.8139133Z ......................................................................................iiiii......... 1900/9225
2019-11-11T15:08:34.3191944Z .................................................................................................... 2100/9225
2019-11-11T15:08:36.4470163Z .................................................................................................... 2200/9225
2019-11-11T15:08:38.7134379Z .................................................................................................... 2300/9225
2019-11-11T15:08:47.4838379Z .................................................................................................... 2400/9225
---
2019-11-11T15:11:27.5864845Z ...............................................................................i...............i.... 4700/9225
2019-11-11T15:11:34.3487281Z .................................................................................................... 4800/9225
2019-11-11T15:11:42.5444706Z .................................................................................................... 4900/9225
2019-11-11T15:11:47.1862820Z .................................................................................................... 5000/9225
2019-11-11T15:11:57.6723122Z ..................................................................................ii.ii...........i. 5100/9225
2019-11-11T15:12:05.7303677Z .................i.................................................................................. 5300/9225
2019-11-11T15:12:14.6746396Z .................................................................................................... 5400/9225
2019-11-11T15:12:21.0122275Z ................................................................i................................... 5500/9225
2019-11-11T15:12:27.6395903Z .................................................................................................... 5600/9225
2019-11-11T15:12:27.6395903Z .................................................................................................... 5600/9225
2019-11-11T15:12:35.8454811Z .................................................................................................... 5700/9225
2019-11-11T15:12:43.0986403Z .................................................ii...i..ii...........i............................. 5800/9225
2019-11-11T15:13:04.2365706Z .................................................................................................... 6000/9225
2019-11-11T15:13:11.8766614Z .................................................................................................... 6100/9225
2019-11-11T15:13:11.8766614Z .................................................................................................... 6100/9225
2019-11-11T15:13:16.7273441Z ....................................................................i..ii........................... 6200/9225
2019-11-11T15:13:43.2487225Z .................................................................................................... 6400/9225
2019-11-11T15:13:45.0658864Z ....................................i............................................................... 6500/9225
2019-11-11T15:13:47.0691202Z .................................................................................................... 6600/9225
2019-11-11T15:13:49.2123423Z ....................i............................................................................... 6700/9225
---
2019-11-11T15:18:13.0656749Z 
2019-11-11T15:18:13.0657390Z ---- [ui] ui/coherence/coherence-subtyping.rs#old stdout ----
2019-11-11T15:18:13.0657704Z diff of stderr:
2019-11-11T15:18:13.0657893Z 
2019-11-11T15:18:13.0658426Z 1 error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T15:18:13.0658934Z -   --> $DIR/coherence-subtyping.rs:18:1
2019-11-11T15:18:13.0659917Z +   --> $DIR/coherence-subtyping.rs:16:1
2019-11-11T15:18:13.0660230Z 3    |
2019-11-11T15:18:13.0660739Z 4 LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T15:18:13.0661278Z 5    | --------------------------------------------------------- first implementation here
2019-11-11T15:18:13.0661737Z 
2019-11-11T15:18:13.0661956Z The actual stderr differed from the expected stderr.
2019-11-11T15:18:13.0661956Z The actual stderr differed from the expected stderr.
2019-11-11T15:18:13.0662494Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.old/coherence-subtyping.old.stderr
2019-11-11T15:18:13.0663407Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T15:18:13.0663881Z To only update this specific test, also pass `--test-args coherence/coherence-subtyping.rs`
2019-11-11T15:18:13.0664092Z 
2019-11-11T15:18:13.0664295Z error in revision `old`: 1 errors occurred comparing output.
2019-11-11T15:18:13.0664473Z status: exit code: 1
2019-11-11T15:18:13.0665356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "old" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.old" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.old/auxiliary" "-A" "unused"
2019-11-11T15:18:13.0666105Z ------------------------------------------
2019-11-11T15:18:13.0666325Z 
2019-11-11T15:18:13.0666710Z ------------------------------------------
2019-11-11T15:18:13.0666953Z stderr:
2019-11-11T15:18:13.0666953Z stderr:
2019-11-11T15:18:13.0667313Z ------------------------------------------
2019-11-11T15:18:13.0667777Z error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T15:18:13.0668458Z    |
2019-11-11T15:18:13.0668458Z    |
2019-11-11T15:18:13.0668833Z LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T15:18:13.0670565Z    | --------------------------------------------------------- first implementation here
2019-11-11T15:18:13.0670904Z ...
2019-11-11T15:18:13.0671417Z LL | impl TheTrait for for<'a> fn(&'a u8, &'a u8) -> &'a u8 {
2019-11-11T15:18:13.0672007Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
2019-11-11T15:18:13.0672287Z    |
2019-11-11T15:18:13.0672943Z    = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
2019-11-11T15:18:13.0673692Z error: aborting due to previous error
2019-11-11T15:18:13.0673859Z 
2019-11-11T15:18:13.0674248Z For more information about this error, try `rustc --explain E0119`.
2019-11-11T15:18:13.0674449Z 
2019-11-11T15:18:13.0674449Z 
2019-11-11T15:18:13.0674807Z ------------------------------------------
2019-11-11T15:18:13.0675027Z 
2019-11-11T15:18:13.0675175Z 
2019-11-11T15:18:13.0675564Z ---- [ui] ui/coherence/coherence-subtyping.rs#re stdout ----
2019-11-11T15:18:13.0675806Z diff of stderr:
2019-11-11T15:18:13.0675970Z 
2019-11-11T15:18:13.0676397Z 1 error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T15:18:13.0676824Z -   --> $DIR/coherence-subtyping.rs:18:1
2019-11-11T15:18:13.0677377Z +   --> $DIR/coherence-subtyping.rs:16:1
2019-11-11T15:18:13.0677598Z 3    |
2019-11-11T15:18:13.0677999Z 4 LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T15:18:13.0678445Z 5    | --------------------------------------------------------- first implementation here
2019-11-11T15:18:13.0678826Z 
2019-11-11T15:18:13.0679347Z The actual stderr differed from the expected stderr.
2019-11-11T15:18:13.0679347Z The actual stderr differed from the expected stderr.
2019-11-11T15:18:13.0680060Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.re/coherence-subtyping.re.stderr
2019-11-11T15:18:13.0681278Z To update references, rerun the tests and pass the `--bless` flag
2019-11-11T15:18:13.0682016Z To only update this specific test, also pass `--test-args coherence/coherence-subtyping.rs`
2019-11-11T15:18:13.0682205Z 
2019-11-11T15:18:13.0682374Z error in revision `re`: 1 errors occurred comparing output.
2019-11-11T15:18:13.0682543Z status: exit code: 1
2019-11-11T15:18:13.0683550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "re" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.re" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-subtyping.re/auxiliary" "-A" "unused"
2019-11-11T15:18:13.0684070Z ------------------------------------------
2019-11-11T15:18:13.0684223Z 
2019-11-11T15:18:13.0684527Z ------------------------------------------
2019-11-11T15:18:13.0684674Z stderr:
2019-11-11T15:18:13.0684674Z stderr:
2019-11-11T15:18:13.0684979Z ------------------------------------------
2019-11-11T15:18:13.0685448Z error[E0119]: conflicting implementations of trait `TheTrait` for type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`:
2019-11-11T15:18:13.0685991Z    |
2019-11-11T15:18:13.0685991Z    |
2019-11-11T15:18:13.0686299Z LL | impl TheTrait for for<'a,'b> fn(&'a u8, &'b u8) -> &'a u8 {
2019-11-11T15:18:13.0686676Z    | --------------------------------------------------------- first implementation here
2019-11-11T15:18:13.0686821Z ...
2019-11-11T15:18:13.0687129Z LL | impl TheTrait for for<'a> fn(&'a u8, &'a u8) -> &'a u8 {
2019-11-11T15:18:13.0688951Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
2019-11-11T15:18:13.0689883Z    |
2019-11-11T15:18:13.0690527Z    = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
2019-11-11T15:18:13.0690898Z error: aborting due to previous error
2019-11-11T15:18:13.0691021Z 
2019-11-11T15:18:13.0691452Z For more information about this error, try `rustc --explain E0119`.
2019-11-11T15:18:13.0691624Z 
---
2019-11-11T15:18:13.0703972Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-11T15:18:13.0704213Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-11T15:18:13.0707297Z 
2019-11-11T15:18:13.0707354Z 
2019-11-11T15:18:13.0715697Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-11T15:18:13.0751514Z 
2019-11-11T15:18:13.0751897Z 
2019-11-11T15:18:13.0752216Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-11T15:18:13.0752479Z Build completed unsuccessfully in 0:58:56
2019-11-11T15:18:13.0752479Z Build completed unsuccessfully in 0:58:56
2019-11-11T15:18:13.0768789Z == clock drift check ==
2019-11-11T15:18:13.0780266Z   local time: Mon Nov 11 15:18:13 UTC 2019
2019-11-11T15:18:13.3551207Z   network time: Mon, 11 Nov 2019 15:18:13 GMT
2019-11-11T15:18:13.3551391Z == end clock drift check ==
2019-11-11T15:18:14.2136442Z 
2019-11-11T15:18:14.2251436Z ##[error]Bash exited with code '1'.
2019-11-11T15:18:14.2286196Z ##[section]Starting: Checkout
2019-11-11T15:18:14.2287789Z ==============================================================================
2019-11-11T15:18:14.2287851Z Task         : Get sources
2019-11-11T15:18:14.2287893Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
