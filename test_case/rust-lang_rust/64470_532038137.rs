plain
2019-09-17T01:36:09.4724272Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T01:36:09.5015113Z ##[command]git config gc.auto 0
2019-09-17T01:36:09.5063813Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T01:36:09.5123552Z ##[command]git config --get-all http.proxy
2019-09-17T01:36:09.5298115Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-17T02:49:08.5704184Z .................................................................................................... 1500/9024
2019-09-17T02:49:15.4221683Z ...................................................................................F................ 1600/9024
2019-09-17T02:49:29.7757919Z .................................................................i...............i.................. 1700/9024
2019-09-17T02:49:37.8901694Z .................................................................................................... 1800/9024
2019-09-17T02:49:55.1396321Z ........................................................iiiii....................................... 1900/9024
2019-09-17T02:50:08.6273263Z .................................................................................................... 2100/9024
2019-09-17T02:50:11.4843041Z .................................................................................................... 2200/9024
2019-09-17T02:50:15.3260808Z .................................................................................................... 2300/9024
2019-09-17T02:50:24.9061934Z .................................................................................................... 2400/9024
---
2019-09-17T02:53:45.9019407Z ............................................i...............i....................................... 4700/9024
2019-09-17T02:53:57.5321878Z .................................................................................................... 4800/9024
2019-09-17T02:54:05.6970318Z .................................................................................................... 4900/9024
2019-09-17T02:54:16.7994555Z .................................................................................................... 5000/9024
2019-09-17T02:54:25.6089402Z ............................ii.ii................................................................... 5100/9024
2019-09-17T02:54:36.8679194Z .................................................................................................... 5300/9024
2019-09-17T02:54:48.6514370Z ............................................................................................i....... 5400/9024
2019-09-17T02:54:58.0068501Z .................................................................................................... 5500/9024
2019-09-17T02:55:03.4067765Z .................................................................................................... 5600/9024
2019-09-17T02:55:03.4067765Z .................................................................................................... 5600/9024
2019-09-17T02:55:15.4786728Z .......................................................................................ii...i..ii... 5700/9024
2019-09-17T02:55:44.5953471Z .................................................................................................... 5900/9024
2019-09-17T02:55:56.2376336Z .................................................................................................... 6000/9024
2019-09-17T02:55:56.2376336Z .................................................................................................... 6000/9024
2019-09-17T02:56:06.3440368Z .........................................................................................i..ii...... 6100/9024
2019-09-17T02:56:41.4256817Z .................................................................................................... 6300/9024
2019-09-17T02:56:46.4854283Z ................................................i................................................... 6400/9024
2019-09-17T02:56:49.0434085Z .................................................................................................... 6500/9024
2019-09-17T02:56:51.9417302Z ....................i............................................................................... 6600/9024
---
2019-09-17T03:01:36.3434683Z 
2019-09-17T03:01:36.3434826Z 7 warning: skipping const checks
2019-09-17T03:01:36.3435901Z 8   --> $DIR/assoc_const.rs:19:25
2019-09-17T03:01:36.3436112Z 9    |
2019-09-17T03:01:36.3436476Z - LL |     const X: Vec<u32> = Vec::new();
2019-09-17T03:01:36.3436879Z -    |                         ^^^^^^^^^^
2019-09-17T03:01:36.3437466Z + LL |     const X: Vec<u32> = vec![1];
2019-09-17T03:01:36.3437934Z +    |
2019-09-17T03:01:36.3438512Z +    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-17T03:01:36.3439494Z 12 
2019-09-17T03:01:36.3439558Z 13 warning: skipping const checks
---
2019-09-17T03:01:36.3440340Z 29 error[E0080]: erroneous constant used
2019-09-17T03:01:36.3440604Z -   --> $DIR/assoc_const.rs:31:13
2019-09-17T03:01:36.3440812Z +   --> $DIR/assoc_const.rs:32:13
2019-09-17T03:01:36.3440856Z 31    |
2019-09-17T03:01:36.3441009Z 32 LL |     let y = <String as Bar<Vec<u32>, String>>::F;
2019-09-17T03:01:36.3441061Z 33    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-09-17T03:01:36.3441139Z 
2019-09-17T03:01:36.3441186Z The actual stderr differed from the expected stderr.
2019-09-17T03:01:36.3441714Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/assoc_const.stderr
2019-09-17T03:01:36.3441714Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/assoc_const.stderr
2019-09-17T03:01:36.3442071Z To update references, rerun the tests and pass the `--bless` flag
2019-09-17T03:01:36.3442391Z To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`
2019-09-17T03:01:36.3442477Z error: 1 errors occurred comparing output.
2019-09-17T03:01:36.3443006Z status: exit code: 1
2019-09-17T03:01:36.3443006Z status: exit code: 1
2019-09-17T03:01:36.3443882Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary" "-A" "unused"
2019-09-17T03:01:36.3444418Z ------------------------------------------
2019-09-17T03:01:36.3444452Z 
2019-09-17T03:01:36.3444680Z ------------------------------------------
2019-09-17T03:01:36.3444726Z stderr:
2019-09-17T03:01:36.3444726Z stderr:
2019-09-17T03:01:36.3444932Z ------------------------------------------
2019-09-17T03:01:36.3444995Z warning: skipping const checks
2019-09-17T03:01:36.3445247Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:12:20
2019-09-17T03:01:36.3445297Z    |
2019-09-17T03:01:36.3445363Z LL |     const F: u32 = (U::X, 42).1; //~ WARN skipping const checks
2019-09-17T03:01:36.3445440Z 
2019-09-17T03:01:36.3445482Z warning: skipping const checks
2019-09-17T03:01:36.3445746Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:19:25
2019-09-17T03:01:36.3445793Z    |
2019-09-17T03:01:36.3445793Z    |
2019-09-17T03:01:36.3445846Z LL |     const X: Vec<u32> = vec![1];
2019-09-17T03:01:36.3445960Z    |
2019-09-17T03:01:36.3446276Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-17T03:01:36.3446318Z 
2019-09-17T03:01:36.3446385Z warning: skipping const checks
2019-09-17T03:01:36.3446385Z warning: skipping const checks
2019-09-17T03:01:36.3446630Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:19:25
2019-09-17T03:01:36.3446678Z    |
2019-09-17T03:01:36.3446737Z LL |     const X: Vec<u32> = vec![1];
2019-09-17T03:01:36.3446822Z    |
2019-09-17T03:01:36.3447151Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-17T03:01:36.3447192Z 
2019-09-17T03:01:36.3447234Z warning: skipping const checks
2019-09-17T03:01:36.3447234Z warning: skipping const checks
2019-09-17T03:01:36.3447484Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:19:25
2019-09-17T03:01:36.3447555Z    |
2019-09-17T03:01:36.3447596Z LL |     const X: Vec<u32> = vec![1];
2019-09-17T03:01:36.3447701Z    |
2019-09-17T03:01:36.3448012Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-09-17T03:01:36.3448051Z 
2019-09-17T03:01:36.3448111Z error[E0080]: erroneous constant used
2019-09-17T03:01:36.3448111Z error[E0080]: erroneous constant used
2019-09-17T03:01:36.3448353Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:32:13
2019-09-17T03:01:36.3448399Z    |
2019-09-17T03:01:36.3448448Z LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ ERROR erroneous constant
2019-09-17T03:01:36.3448519Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-09-17T03:01:36.3448594Z error: aborting due to previous error
2019-09-17T03:01:36.3448622Z 
2019-09-17T03:01:36.3448970Z For more information about this error, try `rustc --explain E0080`.
2019-09-17T03:01:36.3449021Z 
---
2019-09-17T03:01:36.3476061Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-17T03:01:36.3476189Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-17T03:01:36.3488378Z 
2019-09-17T03:01:36.3488469Z 
2019-09-17T03:01:36.3499823Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-17T03:01:36.3500317Z 
2019-09-17T03:01:36.3500488Z 
2019-09-17T03:01:37.2685050Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-17T03:01:37.2694154Z Build completed unsuccessfully in 1:17:00
2019-09-17T03:01:37.2694154Z Build completed unsuccessfully in 1:17:00
2019-09-17T03:01:37.2694728Z == clock drift check ==
2019-09-17T03:01:37.2694916Z   local time: Tue Sep 17 03:01:36 UTC 2019
2019-09-17T03:01:37.2695105Z   network time: Tue, 17 Sep 2019 03:01:36 GMT
2019-09-17T03:01:37.2695251Z == end clock drift check ==
2019-09-17T03:01:37.4667926Z ##[error]Bash exited with code '1'.
2019-09-17T03:01:37.4706752Z ##[section]Starting: Checkout
2019-09-17T03:01:37.4708895Z ==============================================================================
2019-09-17T03:01:37.4708951Z Task         : Get sources
2019-09-17T03:01:37.4709035Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
