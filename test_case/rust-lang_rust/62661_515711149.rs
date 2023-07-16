plain
2019-07-27T19:19:44.8176350Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T19:19:44.8347416Z ##[command]git config gc.auto 0
2019-07-27T19:19:44.8423060Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T19:19:44.8485628Z ##[command]git config --get-all http.proxy
2019-07-27T19:19:44.8621721Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62661/merge:refs/remotes/pull/62661/merge
---
2019-07-27T19:20:19.2289688Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T19:20:19.2289737Z 
2019-07-27T19:20:19.2289955Z   git checkout -b <new-branch-name>
2019-07-27T19:20:19.2289984Z 
2019-07-27T19:20:19.2290031Z HEAD is now at 0262b3fef Merge 89e14d18df1c4e7b471f20ac3df7f4b22962b5c1 into 0e9b465d729d07101b29b4d096d83edf9be82df0
2019-07-27T19:20:19.2426106Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T19:20:19.2428872Z ==============================================================================
2019-07-27T19:20:19.2428950Z Task         : Bash
2019-07-27T19:20:19.2428998Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T20:20:40.6194659Z .................................................................................................... 700/5874
2019-07-27T20:20:44.7747801Z .................................................................................................... 800/5874
2019-07-27T20:20:50.2472234Z .................................................................................................... 900/5874
2019-07-27T20:20:55.2508851Z .................................................................................................... 1000/5874
2019-07-27T20:21:00.7215240Z i...........i....................................................................................... 1100/5874
2019-07-27T20:21:04.6339813Z ..............................iiiii................................................................. 1200/5874
2019-07-27T20:21:10.6906565Z .................................................................................................... 1400/5874
2019-07-27T20:21:13.4010840Z .................................................................................................... 1500/5874
2019-07-27T20:21:17.1057946Z .................................................................................................... 1600/5874
2019-07-27T20:21:19.8149599Z .................................................................................................... 1700/5874
---
2019-07-27T20:22:33.5486663Z .................................................................................................... 3400/5874
2019-07-27T20:22:38.6769136Z .................................................................................................... 3500/5874
2019-07-27T20:22:42.7054064Z ..........................i......................................................................... 3600/5874
2019-07-27T20:22:47.0355713Z .................................................................................................... 3700/5874
2019-07-27T20:22:50.5048467Z ....ii...i..ii...................................................................................... 3800/5874
2019-07-27T20:22:59.3086632Z .................................................................................................... 4000/5874
2019-07-27T20:23:03.1886472Z ........................ii.......................................................................... 4100/5874
2019-07-27T20:23:05.5278785Z .............................................i...................................................... 4200/5874
2019-07-27T20:23:07.6008814Z .................................................................................................... 4300/5874
---
2019-07-27T20:24:31.0288345Z 
2019-07-27T20:24:31.0288791Z ---- [ui] ui/traits/reservation-impls/reservation-impl-coherence-conflict.rs stdout ----
2019-07-27T20:24:31.0288852Z diff of stderr:
2019-07-27T20:24:31.0288895Z 
2019-07-27T20:24:31.0289172Z 5    | ---------------------- first implementation here
2019-07-27T20:24:31.0289274Z 6 LL | impl<T: MyTrait> OtherTrait for T {}
2019-07-27T20:24:31.0289558Z -    |
2019-07-27T20:24:31.0289785Z -    = note: this impl is reserved
2019-07-27T20:24:31.0289839Z 10 
2019-07-27T20:24:31.0289905Z 11 error: aborting due to previous error
2019-07-27T20:24:31.0289905Z 11 error: aborting due to previous error
2019-07-27T20:24:31.0289947Z 12 
2019-07-27T20:24:31.0289973Z 
2019-07-27T20:24:31.0289998Z 
2019-07-27T20:24:31.0290061Z The actual stderr differed from the expected stderr.
2019-07-27T20:24:31.0290422Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-coherence-conflict/reservation-impl-coherence-conflict.stderr
2019-07-27T20:24:31.0290675Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T20:24:31.0291000Z To only update this specific test, also pass `--test-args traits/reservation-impls/reservation-impl-coherence-conflict.rs`
2019-07-27T20:24:31.0291083Z error: 1 errors occurred comparing output.
2019-07-27T20:24:31.0291147Z status: exit code: 1
2019-07-27T20:24:31.0291147Z status: exit code: 1
2019-07-27T20:24:31.0292116Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/reservation-impls/reservation-impl-coherence-conflict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-coherence-conflict" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/reservation-impls/reservation-impl-coherence-conflict/auxiliary" "-A" "unused"
2019-07-27T20:24:31.0292522Z ------------------------------------------
2019-07-27T20:24:31.0292557Z 
2019-07-27T20:24:31.0292801Z ------------------------------------------
2019-07-27T20:24:31.0292847Z stderr:
2019-07-27T20:24:31.0292847Z stderr:
2019-07-27T20:24:31.0293056Z ------------------------------------------
2019-07-27T20:24:31.0293128Z error[E0119]: conflicting implementations of trait `OtherTrait` for type `()`:
2019-07-27T20:24:31.0293426Z   --> /checkout/src/test/ui/traits/reservation-impls/reservation-impl-coherence-conflict.rs:13:1
2019-07-27T20:24:31.0293480Z    |
2019-07-27T20:24:31.0293808Z LL | impl OtherTrait for () {}
2019-07-27T20:24:31.0294103Z    | ---------------------- first implementation here
2019-07-27T20:24:31.0294154Z LL | impl<T: MyTrait> OtherTrait for T {}
2019-07-27T20:24:31.0294256Z 
2019-07-27T20:24:31.0294298Z error: aborting due to previous error
2019-07-27T20:24:31.0294327Z 
2019-07-27T20:24:31.0294593Z For more information about this error, try `rustc --explain E0119`.
---
2019-07-27T20:24:31.0296016Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-27T20:24:31.0296093Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T20:24:31.0296127Z 
2019-07-27T20:24:31.0296151Z 
2019-07-27T20:24:31.0297620Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-27T20:24:31.0297878Z 
2019-07-27T20:24:31.0297908Z 
2019-07-27T20:24:31.0297954Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-27T20:24:31.0298110Z Build completed unsuccessfully in 0:56:30
2019-07-27T20:24:31.0298110Z Build completed unsuccessfully in 0:56:30
2019-07-27T20:24:31.4390883Z ##[error]Bash exited with code '1'.
2019-07-27T20:24:31.4425778Z ##[section]Starting: Checkout
2019-07-27T20:24:31.4427574Z ==============================================================================
2019-07-27T20:24:31.4427631Z Task         : Get sources
2019-07-27T20:24:31.4427699Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
