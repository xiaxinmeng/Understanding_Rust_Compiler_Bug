plain
2020-01-13T05:30:49.9028066Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T05:30:49.9110200Z ##[command]git config gc.auto 0
2020-01-13T05:30:49.9185983Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T05:30:49.9247252Z ##[command]git config --get-all http.proxy
2020-01-13T05:30:50.5539966Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68171/merge:refs/remotes/pull/68171/merge
---
2020-01-13T06:28:32.8741686Z ........................................i...............i........................................... 4900/9518
2020-01-13T06:28:41.7971140Z .................................................................................................... 5000/9518
2020-01-13T06:28:47.9003149Z ...................................................................................i................ 5100/9518
2020-01-13T06:28:53.0984069Z .................................................................................................... 5200/9518
2020-01-13T06:29:02.9838156Z ......................................................ii.ii...........i............................. 5300/9518
2020-01-13T06:29:11.9000245Z .................................................................................................... 5500/9518
2020-01-13T06:29:21.8759454Z .................................................................................................... 5600/9518
2020-01-13T06:29:27.9257426Z .......................................i............................................................ 5700/9518
2020-01-13T06:29:34.1816761Z .................................................................................................... 5800/9518
2020-01-13T06:29:34.1816761Z .................................................................................................... 5800/9518
2020-01-13T06:29:44.5128756Z .................................................................................................... 5900/9518
2020-01-13T06:29:53.8969609Z ..............................ii...i..ii...........i................................................ 6000/9518
2020-01-13T06:30:11.7922168Z .................................................................................................... 6200/9518
2020-01-13T06:30:19.5835631Z .................................................................................................... 6300/9518
2020-01-13T06:30:19.5835631Z .................................................................................................... 6300/9518
2020-01-13T06:30:31.7683193Z ..........................................................i..ii..................................... 6400/9518
2020-01-13T06:30:59.4782615Z .................................................................................................... 6600/9518
2020-01-13T06:31:01.6186267Z ..................................i................................................................. 6700/9518
2020-01-13T06:31:03.7953368Z .................................................................................................... 6800/9518
2020-01-13T06:31:06.2739284Z ..................................i................................................................. 6900/9518
---
2020-01-13T06:32:39.1921665Z .................................................................................................... 7500/9518
2020-01-13T06:32:43.4589455Z .................................................................................................... 7600/9518
2020-01-13T06:32:49.4256651Z .................................................................................................... 7700/9518
2020-01-13T06:32:56.4341513Z .................................................................................................... 7800/9518
2020-01-13T06:33:05.9311431Z ...................................................................................iiii............. 7900/9518
2020-01-13T06:33:21.8877954Z .................i......i........................................................................... 8100/9518
2020-01-13T06:33:26.9071010Z .................................................................................................... 8200/9518
2020-01-13T06:33:39.8946205Z .................................................................................................... 8300/9518
2020-01-13T06:33:49.4264580Z .................................................................................................... 8400/9518
---
2020-01-13T06:35:44.9452896Z 18 For more information about an error, try `rustc --explain E0432`.
2020-01-13T06:35:44.9453062Z 
2020-01-13T06:35:44.9453208Z 
2020-01-13T06:35:44.9453518Z The actual stderr differed from the expected stderr.
2020-01-13T06:35:44.9453998Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/privacy2.stderr
2020-01-13T06:35:44.9454476Z To update references, rerun the tests and pass the `--bless` flag
2020-01-13T06:35:44.9454966Z To only update this specific test, also pass `--test-args privacy/privacy2.rs`
2020-01-13T06:35:44.9455344Z error: 1 errors occurred comparing output.
2020-01-13T06:35:44.9455494Z status: exit code: 1
2020-01-13T06:35:44.9455494Z status: exit code: 1
2020-01-13T06:35:44.9456477Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/auxiliary" "-A" "unused"
2020-01-13T06:35:44.9457147Z ------------------------------------------
2020-01-13T06:35:44.9457314Z 
2020-01-13T06:35:44.9457706Z ------------------------------------------
2020-01-13T06:35:44.9457909Z stderr:
---
2020-01-13T06:35:44.9459510Z 
2020-01-13T06:35:44.9459655Z error[E0603]: function `foo` is private
2020-01-13T06:35:44.9460377Z   --> /checkout/src/test/ui/privacy/privacy2.rs:23:20
2020-01-13T06:35:44.9460628Z    |
2020-01-13T06:35:44.9460812Z LL |     use bar::glob::foo;
2020-01-13T06:35:44.9461079Z 
2020-01-13T06:35:44.9461214Z error: requires `sized` lang_item
2020-01-13T06:35:44.9461350Z 
2020-01-13T06:35:44.9461495Z error: requires `sized` lang_item
---
2020-01-13T06:35:44.9467658Z 12 
2020-01-13T06:35:44.9467782Z 
2020-01-13T06:35:44.9467900Z 
2020-01-13T06:35:44.9468038Z The actual stderr differed from the expected stderr.
2020-01-13T06:35:44.9468489Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/privacy3.stderr
2020-01-13T06:35:44.9469072Z To update references, rerun the tests and pass the `--bless` flag
2020-01-13T06:35:44.9469576Z To only update this specific test, also pass `--test-args privacy/privacy3.rs`
2020-01-13T06:35:44.9470159Z error: 1 errors occurred comparing output.
2020-01-13T06:35:44.9470376Z status: exit code: 1
2020-01-13T06:35:44.9470376Z status: exit code: 1
2020-01-13T06:35:44.9471408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/auxiliary" "-A" "unused"
2020-01-13T06:35:44.9472061Z ------------------------------------------
2020-01-13T06:35:44.9473039Z 
2020-01-13T06:35:44.9474286Z ------------------------------------------
2020-01-13T06:35:44.9476398Z stderr:
2020-01-13T06:35:44.9476398Z stderr:
2020-01-13T06:35:44.9477111Z ------------------------------------------
2020-01-13T06:35:44.9477331Z error[E0432]: unresolved import `bar::gpriv`
2020-01-13T06:35:44.9477911Z    |
2020-01-13T06:35:44.9477911Z    |
2020-01-13T06:35:44.9478057Z LL |     use bar::gpriv;
2020-01-13T06:35:44.9478224Z    |         ^^^^^^^^^^ no `gpriv` in `bar`
2020-01-13T06:35:44.9478507Z error: requires `sized` lang_item
2020-01-13T06:35:44.9478629Z 
2020-01-13T06:35:44.9478767Z error: requires `sized` lang_item
2020-01-13T06:35:44.9478888Z 
---
2020-01-13T06:35:44.9483207Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-13T06:35:44.9483398Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-13T06:35:44.9485968Z 
2020-01-13T06:35:44.9488740Z 
2020-01-13T06:35:44.9492421Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-13T06:35:44.9493104Z 
2020-01-13T06:35:44.9493225Z 
2020-01-13T06:35:44.9503781Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-13T06:35:44.9503873Z Build completed unsuccessfully in 0:59:27
2020-01-13T06:35:44.9503873Z Build completed unsuccessfully in 0:59:27
2020-01-13T06:35:44.9556259Z == clock drift check ==
2020-01-13T06:35:44.9577885Z   local time: Mon Jan 13 06:35:44 UTC 2020
2020-01-13T06:35:44.9995220Z   network time: Mon, 13 Jan 2020 06:35:44 GMT
2020-01-13T06:35:44.9995302Z == end clock drift check ==
2020-01-13T06:35:45.4232933Z 
2020-01-13T06:35:45.4330565Z ##[error]Bash exited with code '1'.
2020-01-13T06:35:45.4361950Z ##[section]Starting: Checkout
2020-01-13T06:35:45.4363564Z ==============================================================================
2020-01-13T06:35:45.4363618Z Task         : Get sources
2020-01-13T06:35:45.4363665Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
