plain
2019-12-16T21:33:39.6166042Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T21:33:39.6375443Z ##[command]git config gc.auto 0
2019-12-16T21:33:39.6433604Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T21:33:39.6489412Z ##[command]git config --get-all http.proxy
2019-12-16T21:33:39.6597272Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67362/merge:refs/remotes/pull/67362/merge
---
2019-12-16T22:30:42.9800836Z .................................................................................................... 1600/9380
2019-12-16T22:30:47.7986676Z .................................................................................................... 1700/9380
2019-12-16T22:30:59.8754396Z ...................................................................i................................ 1800/9380
2019-12-16T22:31:07.1872533Z .................................................................................................... 1900/9380
2019-12-16T22:31:21.3958210Z ....................................................iiiii........................................... 2000/9380
2019-12-16T22:31:31.9075376Z .................................................................................................... 2200/9380
2019-12-16T22:31:34.4123795Z .................................................................................................... 2300/9380
2019-12-16T22:31:37.6550223Z .................................................................................................... 2400/9380
2019-12-16T22:31:58.0546291Z .................................................................................................... 2500/9380
---
2019-12-16T22:34:28.9863511Z .............................................................i...............i...................... 4800/9380
2019-12-16T22:34:36.1268611Z .................................................................................................... 4900/9380
2019-12-16T22:34:43.5270134Z .................................................................................................... 5000/9380
2019-12-16T22:34:48.3500925Z .....i.............................................................................................. 5100/9380
2019-12-16T22:34:57.9761116Z .......................................................................ii.ii...........i............ 5200/9380
2019-12-16T22:35:06.3662458Z .......i............................................................................................ 5400/9380
2019-12-16T22:35:15.6974976Z .................................................................................................... 5500/9380
2019-12-16T22:35:22.0908796Z .....................................................i.............................................. 5600/9380
2019-12-16T22:35:28.7070847Z .................................................................................................... 5700/9380
2019-12-16T22:35:28.7070847Z .................................................................................................... 5700/9380
2019-12-16T22:35:38.0744610Z .................................................................................................... 5800/9380
2019-12-16T22:35:44.9217415Z .........................................ii...i..ii...........i..................................... 5900/9380
2019-12-16T22:36:06.8144496Z .................................................................................................... 6100/9380
2019-12-16T22:36:14.4566570Z .................................................................................................... 6200/9380
2019-12-16T22:36:14.4566570Z .................................................................................................... 6200/9380
2019-12-16T22:36:20.1037123Z ..................................................................i..ii............................. 6300/9380
2019-12-16T22:36:46.1598183Z .................................................................................................... 6500/9380
2019-12-16T22:36:48.3207817Z ......................................i............................................................. 6600/9380
2019-12-16T22:36:50.5212930Z .................................................................................................... 6700/9380
2019-12-16T22:36:52.9491418Z ..............................i..................................................................... 6800/9380
---
2019-12-16T22:38:23.0366767Z .................................................................................................... 7400/9380
2019-12-16T22:38:28.0295826Z .................................................................................................... 7500/9380
2019-12-16T22:38:33.4179077Z .................................................................................................... 7600/9380
2019-12-16T22:38:42.0134562Z .................................................................................................... 7700/9380
2019-12-16T22:38:50.5551006Z ...................................................iiii............................................. 7800/9380
2019-12-16T22:39:04.7404867Z .................................................................................................... 8000/9380
2019-12-16T22:39:10.6887733Z .................................................................................................... 8100/9380
2019-12-16T22:39:25.5554951Z .................................................................................................... 8200/9380
2019-12-16T22:39:33.5510638Z .................................................................................................... 8300/9380
---
2019-12-16T22:41:26.8452735Z ---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
2019-12-16T22:41:26.8453006Z diff of stderr:
2019-12-16T22:41:26.8454628Z 
2019-12-16T22:41:26.8454714Z 6    |
2019-12-16T22:41:26.8454769Z 7    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-16T22:41:26.8454820Z 8 
2019-12-16T22:41:26.8455270Z - error[E0275]: overflow evaluating the requirement `Runtime<RootDatabase>: std::panic::RefUnwindSafe`
2019-12-16T22:41:26.8455512Z -   --> $DIR/cycle-cache-err-60010.rs:31:5
2019-12-16T22:41:26.8456352Z - LL |     type Storage;
2019-12-16T22:41:26.8456788Z -    |          ------- associated type defined here
2019-12-16T22:41:26.8457119Z - ...
2019-12-16T22:41:26.8457119Z - ...
2019-12-16T22:41:26.8457330Z - LL | impl Database for RootDatabase {
2019-12-16T22:41:26.8457777Z -    | ------------------------------ in this `impl` item
2019-12-16T22:41:26.8457976Z - LL |     type Storage = SalsaStorage;
2019-12-16T22:41:26.8460597Z -    |
2019-12-16T22:41:26.8460597Z -    |
2019-12-16T22:41:26.8460950Z -    = note: required because it appears within the type `RootDatabase`
2019-12-16T22:41:26.8461223Z -    = note: required because of the requirements on the impl of `SourceDatabase` for `RootDatabase`
2019-12-16T22:41:26.8461467Z -    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-16T22:41:26.8461683Z -    = note: required because it appears within the type `SalsaStorage`
2019-12-16T22:41:26.8462064Z - error: aborting due to 2 previous errors
2019-12-16T22:41:26.8462275Z + error: aborting due to previous error
2019-12-16T22:41:26.8462344Z 26 
2019-12-16T22:41:26.8462599Z 27 For more information about this error, try `rustc --explain E0275`.
2019-12-16T22:41:26.8462599Z 27 For more information about this error, try `rustc --explain E0275`.
2019-12-16T22:41:26.8462642Z 28 
2019-12-16T22:41:26.8462668Z 
2019-12-16T22:41:26.8462708Z 
2019-12-16T22:41:26.8462748Z The actual stderr differed from the expected stderr.
2019-12-16T22:41:26.8463028Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/cycle-cache-err-60010.stderr
2019-12-16T22:41:26.8463267Z To update references, rerun the tests and pass the `--bless` flag
2019-12-16T22:41:26.8463508Z To only update this specific test, also pass `--test-args traits/cycle-cache-err-60010.rs`
2019-12-16T22:41:26.8463577Z error: 1 errors occurred comparing output.
2019-12-16T22:41:26.8463634Z status: exit code: 1
2019-12-16T22:41:26.8463634Z status: exit code: 1
2019-12-16T22:41:26.8464333Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
2019-12-16T22:41:26.8464750Z ------------------------------------------
2019-12-16T22:41:26.8464781Z 
2019-12-16T22:41:26.8464986Z ------------------------------------------
2019-12-16T22:41:26.8465025Z stderr:
2019-12-16T22:41:26.8465025Z stderr:
2019-12-16T22:41:26.8465210Z ------------------------------------------
2019-12-16T22:41:26.8465274Z error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
2019-12-16T22:41:26.8465490Z   --> /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:27:5
2019-12-16T22:41:26.8465544Z    |
2019-12-16T22:41:26.8465611Z LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data, //~ ERROR overflow
2019-12-16T22:41:26.8465696Z    |
2019-12-16T22:41:26.8465696Z    |
2019-12-16T22:41:26.8465757Z    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
2019-12-16T22:41:26.8465824Z error: aborting due to previous error
2019-12-16T22:41:26.8465848Z 
2019-12-16T22:41:26.8466082Z For more information about this error, try `rustc --explain E0275`.
2019-12-16T22:41:26.8466112Z 
---
2019-12-16T22:41:26.8505905Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-16T22:41:26.8506390Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-16T22:41:26.8520257Z 
2019-12-16T22:41:26.8520659Z 
2019-12-16T22:41:26.8522431Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-16T22:41:26.8522925Z 
2019-12-16T22:41:26.8523039Z 
2019-12-16T22:41:26.8570357Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-16T22:41:26.8570873Z Build completed unsuccessfully in 1:01:47
2019-12-16T22:41:26.8570873Z Build completed unsuccessfully in 1:01:47
2019-12-16T22:41:26.8582362Z == clock drift check ==
2019-12-16T22:41:26.8595834Z   local time: Mon Dec 16 22:41:26 UTC 2019
2019-12-16T22:41:27.1302458Z   network time: Mon, 16 Dec 2019 22:41:27 GMT
2019-12-16T22:41:27.1307869Z == end clock drift check ==
2019-12-16T22:41:28.4327172Z 
2019-12-16T22:41:28.4387033Z ##[error]Bash exited with code '1'.
2019-12-16T22:41:28.4427609Z ##[section]Starting: Checkout
2019-12-16T22:41:28.4429163Z ==============================================================================
2019-12-16T22:41:28.4429210Z Task         : Get sources
2019-12-16T22:41:28.4429266Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
