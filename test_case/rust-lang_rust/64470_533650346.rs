plain
2019-09-20T16:32:32.5209440Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T16:32:33.1130632Z ##[command]git config gc.auto 0
2019-09-20T16:32:33.1137731Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T16:32:33.1139723Z ##[command]git config --get-all http.proxy
2019-09-20T16:32:33.1142329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-20T17:39:07.9933240Z .................................................................................................... 1500/9028
2019-09-20T17:39:15.0176429Z ........................................................................................F........... 1600/9028
2019-09-20T17:39:28.3604607Z ....................................................................i...............i............... 1700/9028
2019-09-20T17:39:35.4297051Z .................................................................................................... 1800/9028
2019-09-20T17:39:51.7503010Z ...........................................................iiiii.................................... 1900/9028
2019-09-20T17:40:04.5525278Z .................................................................................................... 2100/9028
2019-09-20T17:40:07.2549254Z .................................................................................................... 2200/9028
2019-09-20T17:40:10.7629939Z .................................................................................................... 2300/9028
2019-09-20T17:40:19.7557487Z .................................................................................................... 2400/9028
---
2019-09-20T17:43:28.2070395Z ...............................................i...............i.................................... 4700/9028
2019-09-20T17:43:38.7657440Z .................................................................................................... 4800/9028
2019-09-20T17:43:46.8675790Z .................................................................................................... 4900/9028
2019-09-20T17:43:57.0846907Z .................................................................................................... 5000/9028
2019-09-20T17:44:05.4355157Z ...............................ii.ii................................................................ 5100/9028
2019-09-20T17:44:15.6844509Z .................................................................................................... 5300/9028
2019-09-20T17:44:27.1943351Z ...............................................................................................i.... 5400/9028
2019-09-20T17:44:36.1739348Z .................................................................................................... 5500/9028
2019-09-20T17:44:41.3236591Z .................................................................................................... 5600/9028
2019-09-20T17:44:41.3236591Z .................................................................................................... 5600/9028
2019-09-20T17:44:52.4213229Z ..........................................................................................ii...i..ii 5700/9028
2019-09-20T17:45:19.2401366Z .................................................................................................... 5900/9028
2019-09-20T17:45:29.8785706Z .................................................................................................... 6000/9028
2019-09-20T17:45:29.8785706Z .................................................................................................... 6000/9028
2019-09-20T17:45:38.5302514Z ............................................................................................i..ii... 6100/9028
2019-09-20T17:46:08.1413518Z .................................................................................................... 6300/9028
2019-09-20T17:46:12.9013189Z ...................................................i................................................ 6400/9028
2019-09-20T17:46:15.2740287Z .................................................................................................... 6500/9028
2019-09-20T17:46:17.8945148Z .......................i............................................................................ 6600/9028
---
2019-09-20T17:50:38.3581187Z 
2019-09-20T17:50:38.3582089Z ---- [ui] ui/consts/miri_unleashed/enum_discriminants.rs stdout ----
2019-09-20T17:50:38.3582151Z diff of stderr:
2019-09-20T17:50:38.3582205Z 
2019-09-20T17:50:38.3582293Z 22 LL |     if let E1::V2 { .. } = (E1::V1 { f: true }) {
2019-09-20T17:50:38.3582381Z 24 
2019-09-20T17:50:38.3582635Z - warning: skipping const checks
2019-09-20T17:50:38.3582868Z -   --> $DIR/enum_discriminants.rs:108:5
2019-09-20T17:50:38.3583054Z -    |
2019-09-20T17:50:38.3583054Z -    |
2019-09-20T17:50:38.3583283Z - LL |     assert_eq!(OVERFLOW, 0);
2019-09-20T17:50:38.3583892Z -    |
2019-09-20T17:50:38.3584301Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-20T17:50:38.3584501Z - 
2019-09-20T17:50:38.3584712Z - warning: skipping const checks
2019-09-20T17:50:38.3584712Z - warning: skipping const checks
2019-09-20T17:50:38.3584946Z -   --> $DIR/enum_discriminants.rs:108:5
2019-09-20T17:50:38.3585128Z -    |
2019-09-20T17:50:38.3585336Z - LL |     assert_eq!(OVERFLOW, 0);
2019-09-20T17:50:38.3585758Z -    |
2019-09-20T17:50:38.3586072Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-20T17:50:38.3586260Z - 
2019-09-20T17:50:38.3586481Z - warning: skipping const checks
2019-09-20T17:50:38.3586481Z - warning: skipping const checks
2019-09-20T17:50:38.3586697Z -   --> $DIR/enum_discriminants.rs:108:5
2019-09-20T17:50:38.3586879Z -    |
2019-09-20T17:50:38.3587106Z - LL |     assert_eq!(OVERFLOW, 0);
2019-09-20T17:50:38.3587582Z -    |
2019-09-20T17:50:38.3587920Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-20T17:50:38.3588144Z - 
2019-09-20T17:50:38.3588369Z - warning: skipping const checks
2019-09-20T17:50:38.3588369Z - warning: skipping const checks
2019-09-20T17:50:38.3588941Z -   --> $DIR/enum_discriminants.rs:109:5
2019-09-20T17:50:38.3589209Z -    |
2019-09-20T17:50:38.3589425Z - LL |     assert_eq!(MORE_OVERFLOW, 0);
2019-09-20T17:50:38.3589852Z -    |
2019-09-20T17:50:38.3590166Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-20T17:50:38.3590370Z - 
2019-09-20T17:50:38.3590610Z - warning: skipping const checks
2019-09-20T17:50:38.3590610Z - warning: skipping const checks
2019-09-20T17:50:38.3590843Z -   --> $DIR/enum_discriminants.rs:109:5
2019-09-20T17:50:38.3591040Z -    |
2019-09-20T17:50:38.3591297Z - LL |     assert_eq!(MORE_OVERFLOW, 0);
2019-09-20T17:50:38.3591733Z -    |
2019-09-20T17:50:38.3592064Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-20T17:50:38.3592285Z - 
2019-09-20T17:50:38.3592505Z - warning: skipping const checks
2019-09-20T17:50:38.3592505Z - warning: skipping const checks
2019-09-20T17:50:38.3592735Z -   --> $DIR/enum_discriminants.rs:109:5
2019-09-20T17:50:38.3592949Z -    |
2019-09-20T17:50:38.3593414Z - LL |     assert_eq!(MORE_OVERFLOW, 0);
2019-09-20T17:50:38.3593865Z -    |
2019-09-20T17:50:38.3594200Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-20T17:50:38.3594402Z - 
2019-09-20T17:50:38.3594461Z 73 
2019-09-20T17:50:38.3594461Z 73 
2019-09-20T17:50:38.3594492Z 
2019-09-20T17:50:38.3594519Z 
2019-09-20T17:50:38.3594577Z The actual stderr differed from the expected stderr.
2019-09-20T17:50:38.3594924Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/enum_discriminants/enum_discriminants.stderr
2019-09-20T17:50:38.3595193Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T17:50:38.3595480Z To only update this specific test, also pass `--test-args consts/miri_unleashed/enum_discriminants.rs`
2019-09-20T17:50:38.3595579Z error: 1 errors occurred comparing output.
2019-09-20T17:50:38.3595635Z status: exit code: 0
2019-09-20T17:50:38.3595635Z status: exit code: 0
2019-09-20T17:50:38.3596564Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/enum_discriminants.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/enum_discriminants/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/enum_discriminants/auxiliary"
2019-09-20T17:50:38.3596962Z ------------------------------------------
2019-09-20T17:50:38.3596998Z 
2019-09-20T17:50:38.3597236Z ------------------------------------------
2019-09-20T17:50:38.3597282Z stderr:
---
2019-09-20T17:50:38.3598895Z 
2019-09-20T17:50:38.3598957Z warning: skipping const checks
2019-09-20T17:50:38.3599313Z   --> /checkout/src/test/ui/consts/miri_unleashed/enum_discriminants.rs:88:28
2019-09-20T17:50:38.3599363Z    |
2019-09-20T17:50:38.3599408Z LL |     if let E1::V2 { .. } = (E1::V1 { f: true }) {
2019-09-20T17:50:38.3599516Z 
2019-09-20T17:50:38.3599557Z warning: skipping const checks
2019-09-20T17:50:38.3599814Z   --> /checkout/src/test/ui/consts/miri_unleashed/enum_discriminants.rs:88:12
2019-09-20T17:50:38.3599877Z    |
2019-09-20T17:50:38.3599877Z    |
2019-09-20T17:50:38.3599921Z LL |     if let E1::V2 { .. } = (E1::V1 { f: true }) {
2019-09-20T17:50:38.3600009Z 
2019-09-20T17:50:38.3600044Z 
2019-09-20T17:50:38.3600261Z ------------------------------------------
2019-09-20T17:50:38.3600292Z 
---
2019-09-20T17:50:38.3621732Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T17:50:38.3622053Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T17:50:38.3643523Z 
2019-09-20T17:50:38.3643648Z 
2019-09-20T17:50:38.3646376Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T17:50:38.3646706Z 
2019-09-20T17:50:38.3646743Z 
2019-09-20T17:50:38.3652179Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-20T17:50:38.3652554Z Build completed unsuccessfully in 1:10:16
2019-09-20T17:50:38.3652554Z Build completed unsuccessfully in 1:10:16
2019-09-20T17:50:38.3706282Z == clock drift check ==
2019-09-20T17:50:38.3723064Z   local time: Fri Sep 20 17:50:38 UTC 2019
2019-09-20T17:50:38.6527412Z   network time: Fri, 20 Sep 2019 17:50:38 GMT
2019-09-20T17:50:38.6529775Z == end clock drift check ==
2019-09-20T17:50:39.6606929Z ##[error]Bash exited with code '1'.
2019-09-20T17:50:39.6646741Z ##[section]Starting: Checkout
2019-09-20T17:50:39.6648689Z ==============================================================================
2019-09-20T17:50:39.6648745Z Task         : Get sources
2019-09-20T17:50:39.6648791Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
