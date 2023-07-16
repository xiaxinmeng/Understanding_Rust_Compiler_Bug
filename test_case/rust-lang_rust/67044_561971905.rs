plain
2019-12-05T03:59:46.6223518Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-05T03:59:46.6451489Z ##[command]git config gc.auto 0
2019-12-05T03:59:46.6523446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-05T03:59:46.6580831Z ##[command]git config --get-all http.proxy
2019-12-05T03:59:46.6725239Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67044/merge:refs/remotes/pull/67044/merge
---
2019-12-05T04:59:42.5931735Z .................................................................................................... 1600/9324
2019-12-05T04:59:47.1548346Z .................................................................................................... 1700/9324
2019-12-05T04:59:59.3658585Z ..........................................i......................................................... 1800/9324
2019-12-05T05:00:07.5364061Z .................................................................................................... 1900/9324
2019-12-05T05:00:22.0207775Z ...........................iiiii.................................................................... 2000/9324
2019-12-05T05:00:32.6482040Z .................................................................................................... 2200/9324
2019-12-05T05:00:35.2117335Z .................................................................................................... 2300/9324
2019-12-05T05:00:39.8187079Z .................................................................................................... 2400/9324
2019-12-05T05:01:01.7443179Z .................................................................................................... 2500/9324
---
2019-12-05T05:03:40.9359619Z ............................i...............i....................................................... 4800/9324
2019-12-05T05:03:51.3518346Z .................................................................................................... 4900/9324
2019-12-05T05:03:57.7465894Z .................................................................................................... 5000/9324
2019-12-05T05:04:05.7825559Z .................................F.................................................................. 5100/9324
2019-12-05T05:04:13.4827005Z ...................................ii.ii...........i................................................ 5200/9324
2019-12-05T05:04:23.0199812Z .................................................................................................... 5400/9324
2019-12-05T05:04:32.8252193Z .................................................................................................... 5500/9324
2019-12-05T05:04:40.1864667Z .................i.................................................................................. 5600/9324
2019-12-05T05:04:46.3807384Z .................................................................................................... 5700/9324
2019-12-05T05:04:46.3807384Z .................................................................................................... 5700/9324
2019-12-05T05:04:57.8134711Z .................................................................................................... 5800/9324
2019-12-05T05:05:09.5024557Z ...ii...i..ii............i.......................................................................... 5900/9324
2019-12-05T05:05:27.6665354Z .................................................................................................... 6100/9324
2019-12-05T05:05:35.1904858Z .................................................................................................... 6200/9324
2019-12-05T05:05:35.1904858Z .................................................................................................... 6200/9324
2019-12-05T05:05:52.9952104Z ..........................i..ii..................................................................... 6300/9324
2019-12-05T05:06:12.4976504Z ..................................................................................................i. 6500/9324
2019-12-05T05:06:14.6731185Z .................................................................................................... 6600/9324
2019-12-05T05:06:16.9070108Z .........................................................................................i.......... 6700/9324
2019-12-05T05:06:19.5543904Z .................................................................................................... 6800/9324
---
2019-12-05T05:07:59.4468605Z .................................................................................................... 7400/9324
2019-12-05T05:08:05.0704242Z .................................................................................................... 7500/9324
2019-12-05T05:08:11.3255376Z .................................................................................................... 7600/9324
2019-12-05T05:08:22.4817103Z .................................................................................................... 7700/9324
2019-12-05T05:08:29.1162832Z ..iiii.............................................................................................. 7800/9324
2019-12-05T05:08:43.6477937Z .................................................................................................... 8000/9324
2019-12-05T05:08:55.2352325Z .................................................................................................... 8100/9324
2019-12-05T05:09:07.3831652Z .................................................................................................... 8200/9324
2019-12-05T05:09:13.6995719Z .................................................................................................... 8300/9324
---
2019-12-05T05:10:59.4548020Z ............................i....................................................................... 9300/9324
2019-12-05T05:11:09.7005486Z ........................
2019-12-05T05:11:09.7006486Z failures:
2019-12-05T05:11:09.7035352Z 
2019-12-05T05:11:09.7035888Z ---- [ui] ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs stdout ----
2019-12-05T05:11:09.7035996Z 
2019-12-05T05:11:09.7036190Z 1 error[E0308]: mismatched types
2019-12-05T05:11:09.7036190Z 1 error[E0308]: mismatched types
2019-12-05T05:11:09.7036410Z -   --> $DIR/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:5:9
2019-12-05T05:11:09.7036643Z +   --> $DIR/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
2019-12-05T05:11:09.7036683Z 3    |
2019-12-05T05:11:09.7036718Z 4 LL |     let P() = U {};
2019-12-05T05:11:09.7036772Z 5    |         ^^^ expected struct `U`, found struct `P`
2019-12-05T05:11:09.7036954Z 
2019-12-05T05:11:09.7036989Z 8               found struct `P<_>`
2019-12-05T05:11:09.7037249Z 10 error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
2019-12-05T05:11:09.7037249Z 10 error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
2019-12-05T05:11:09.7037608Z -   --> $DIR/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:5:9
2019-12-05T05:11:09.7037811Z +   --> $DIR/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
2019-12-05T05:11:09.7037864Z 12    |
2019-12-05T05:11:09.7038014Z - LL | struct P<T>(T);
2019-12-05T05:11:09.7038054Z + LL | struct P<T>(T); // 1 type parameter wanted
2019-12-05T05:11:09.7039158Z 15 ...
2019-12-05T05:11:09.7039158Z 15 ...
2019-12-05T05:11:09.7039194Z 16 LL |     let P() = U {};
2019-12-05T05:11:09.7039256Z 
2019-12-05T05:11:09.7039471Z The actual stderr differed from the expected stderr.
2019-12-05T05:11:09.7039471Z The actual stderr differed from the expected stderr.
2019-12-05T05:11:09.7039851Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields/issue-67037-pat-tup-scrut-ty-diff-less-fields.stderr
2019-12-05T05:11:09.7040085Z To update references, rerun the tests and pass the `--bless` flag
2019-12-05T05:11:09.7040345Z To only update this specific test, also pass `--test-args issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs`
2019-12-05T05:11:09.7040441Z error: 1 errors occurred comparing output.
2019-12-05T05:11:09.7040478Z status: exit code: 1
2019-12-05T05:11:09.7040478Z status: exit code: 1
2019-12-05T05:11:09.7041359Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields/auxiliary" "-A" "unused"
2019-12-05T05:11:09.7042475Z ------------------------------------------
2019-12-05T05:11:09.7042535Z 
2019-12-05T05:11:09.7042760Z ------------------------------------------
2019-12-05T05:11:09.7042975Z stderr:
2019-12-05T05:11:09.7042975Z stderr:
2019-12-05T05:11:09.7043236Z ------------------------------------------
2019-12-05T05:11:09.7043286Z error[E0308]: mismatched types
2019-12-05T05:11:09.7043557Z   --> /checkout/src/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
2019-12-05T05:11:09.7043626Z    |
2019-12-05T05:11:09.7043675Z LL |     let P() = U {}; //~ ERROR mismatched types
2019-12-05T05:11:09.7043734Z    |         ^^^ expected struct `U`, found struct `P`
2019-12-05T05:11:09.7043838Z    = note: expected struct `U`
2019-12-05T05:11:09.7043838Z    = note: expected struct `U`
2019-12-05T05:11:09.7043883Z               found struct `P<_>`
2019-12-05T05:11:09.7043979Z error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
2019-12-05T05:11:09.7043979Z error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
2019-12-05T05:11:09.7044261Z   --> /checkout/src/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
2019-12-05T05:11:09.7044310Z    |
2019-12-05T05:11:09.7044371Z LL | struct P<T>(T); // 1 type parameter wanted
2019-12-05T05:11:09.7044653Z ...
2019-12-05T05:11:09.7044653Z ...
2019-12-05T05:11:09.7044699Z LL |     let P() = U {}; //~ ERROR mismatched types
2019-12-05T05:11:09.7044792Z 
2019-12-05T05:11:09.7044835Z error: aborting due to 2 previous errors
2019-12-05T05:11:09.7044864Z 
2019-12-05T05:11:09.7044927Z Some errors have detailed explanations: E0023, E0308.
2019-12-05T05:11:09.7044927Z Some errors have detailed explanations: E0023, E0308.
2019-12-05T05:11:09.7045184Z For more information about an error, try `rustc --explain E0023`.
2019-12-05T05:11:09.7045218Z 
2019-12-05T05:11:09.7045622Z ------------------------------------------
2019-12-05T05:11:09.7045648Z 
2019-12-05T05:11:09.7045668Z 
2019-12-05T05:11:09.7045687Z 
2019-12-05T05:11:09.7045719Z failures:
2019-12-05T05:11:09.7045919Z     [ui] ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs
2019-12-05T05:11:09.7046145Z test result: FAILED. 9277 passed; 1 failed; 46 ignored; 0 measured; 0 filtered out
2019-12-05T05:11:09.7046178Z 
2019-12-05T05:11:09.7059810Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-05T05:11:09.7059874Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-05T05:11:09.7059874Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-05T05:11:09.7071982Z 
2019-12-05T05:11:09.7072065Z 
2019-12-05T05:11:09.7074283Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-05T05:11:09.7074560Z 
2019-12-05T05:11:09.7074609Z 
2019-12-05T05:11:09.7080778Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-05T05:11:09.7080850Z Build completed unsuccessfully in 1:04:58
2019-12-05T05:11:09.7080850Z Build completed unsuccessfully in 1:04:58
2019-12-05T05:11:09.7138488Z == clock drift check ==
2019-12-05T05:11:09.7156792Z   local time: Thu Dec  5 05:11:09 UTC 2019
2019-12-05T05:11:10.0035777Z   network time: Thu, 05 Dec 2019 05:11:09 GMT
2019-12-05T05:11:10.0036002Z == end clock drift check ==
2019-12-05T05:11:10.8679267Z 
2019-12-05T05:11:10.8754010Z ##[error]Bash exited with code '1'.
2019-12-05T05:11:10.8796132Z ##[section]Starting: Checkout
2019-12-05T05:11:10.8797768Z ==============================================================================
2019-12-05T05:11:10.8797833Z Task         : Get sources
2019-12-05T05:11:10.8797871Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
