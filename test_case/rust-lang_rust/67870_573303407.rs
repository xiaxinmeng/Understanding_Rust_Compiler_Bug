plain
2020-01-11T09:09:37.1031348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T09:09:37.1144532Z ##[command]git config gc.auto 0
2020-01-11T09:09:37.1223867Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T09:09:37.1278881Z ##[command]git config --get-all http.proxy
2020-01-11T09:09:37.1420402Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67870/merge:refs/remotes/pull/67870/merge
---
2020-01-11T10:10:42.5520951Z .......................................i...............i............................................ 4900/9515
2020-01-11T10:10:52.3391491Z .................................................................................................... 5000/9515
2020-01-11T10:10:58.9452054Z ....................................................................................i............... 5100/9515
2020-01-11T10:11:04.7090601Z .................................................................................................... 5200/9515
2020-01-11T10:11:15.5104769Z .......................................................ii.ii...........i............................ 5300/9515
2020-01-11T10:11:25.3645255Z .................................................................................................... 5500/9515
2020-01-11T10:11:35.9791353Z .................................................................................................... 5600/9515
2020-01-11T10:11:42.9287027Z .......................................i............................................................ 5700/9515
2020-01-11T10:11:49.8711589Z .................................................................................................... 5800/9515
2020-01-11T10:11:49.8711589Z .................................................................................................... 5800/9515
2020-01-11T10:12:01.1827342Z .................................................................................................... 5900/9515
2020-01-11T10:12:11.6606388Z ..............................ii...i...ii..........i................................................ 6000/9515
2020-01-11T10:12:31.4322771Z .................................................................................................... 6200/9515
2020-01-11T10:12:40.1272252Z .................................................................................................... 6300/9515
2020-01-11T10:12:40.1272252Z .................................................................................................... 6300/9515
2020-01-11T10:12:51.7154305Z .........................................................i..ii...................................... 6400/9515
2020-01-11T10:13:20.1863012Z .................................................................................................... 6600/9515
2020-01-11T10:13:22.4526561Z ................................i................................................................... 6700/9515
2020-01-11T10:13:24.9992571Z .................................................................................................... 6800/9515
2020-01-11T10:13:27.7773988Z ................................i................................................................... 6900/9515
---
2020-01-11T10:15:08.7518700Z .................................................................................................... 7500/9515
2020-01-11T10:15:13.4886091Z .................................................................................................... 7600/9515
2020-01-11T10:15:19.8215823Z .................................................................................................... 7700/9515
2020-01-11T10:15:27.6612976Z .................................................................................................... 7800/9515
2020-01-11T10:15:37.9354449Z .................................................................................iiii............... 7900/9515
2020-01-11T10:15:55.1760181Z ...............i......i............................................................................. 8100/9515
2020-01-11T10:16:00.7019671Z .................................................................................................... 8200/9515
2020-01-11T10:16:15.0175030Z .................................................................................................... 8300/9515
2020-01-11T10:16:25.2075578Z .................................................................................................... 8400/9515
---
2020-01-11T10:18:29.6077960Z 16 
2020-01-11T10:18:29.6078017Z 
2020-01-11T10:18:29.6078067Z 
2020-01-11T10:18:29.6078135Z The actual stderr differed from the expected stderr.
2020-01-11T10:18:29.6078507Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/privacy3.stderr
2020-01-11T10:18:29.6078860Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T10:18:29.6079166Z To only update this specific test, also pass `--test-args privacy/privacy3.rs`
2020-01-11T10:18:29.6079271Z error: 1 errors occurred comparing output.
2020-01-11T10:18:29.6079328Z status: exit code: 1
2020-01-11T10:18:29.6079328Z status: exit code: 1
2020-01-11T10:18:29.6080456Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/auxiliary" "-A" "unused"
2020-01-11T10:18:29.6080874Z ------------------------------------------
2020-01-11T10:18:29.6080927Z 
2020-01-11T10:18:29.6081158Z ------------------------------------------
2020-01-11T10:18:29.6081205Z stderr:
2020-01-11T10:18:29.6081205Z stderr:
2020-01-11T10:18:29.6081442Z ------------------------------------------
2020-01-11T10:18:29.6081501Z error[E0432]: unresolved import `bar::gpriv`
2020-01-11T10:18:29.6081793Z    |
2020-01-11T10:18:29.6081793Z    |
2020-01-11T10:18:29.6081854Z LL |     use bar::gpriv;
2020-01-11T10:18:29.6081901Z    |         ^^^^^^^^^^ no `gpriv` in `bar`
2020-01-11T10:18:29.6081993Z error: requires `sized` lang_item
2020-01-11T10:18:29.6082024Z 
2020-01-11T10:18:29.6082069Z error: aborting due to 2 previous errors
2020-01-11T10:18:29.6082098Z 
---
2020-01-11T10:18:29.6085278Z 22 For more information about an error, try `rustc --explain E0432`.
2020-01-11T10:18:29.6085326Z 
2020-01-11T10:18:29.6085353Z 
2020-01-11T10:18:29.6085400Z The actual stderr differed from the expected stderr.
2020-01-11T10:18:29.6085740Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/privacy2.stderr
2020-01-11T10:18:29.6086018Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T10:18:29.6086334Z To only update this specific test, also pass `--test-args privacy/privacy2.rs`
2020-01-11T10:18:29.6086419Z error: 1 errors occurred comparing output.
2020-01-11T10:18:29.6086464Z status: exit code: 1
2020-01-11T10:18:29.6086464Z status: exit code: 1
2020-01-11T10:18:29.6087283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/auxiliary" "-A" "unused"
2020-01-11T10:18:29.6087652Z ------------------------------------------
2020-01-11T10:18:29.6087689Z 
2020-01-11T10:18:29.6087930Z ------------------------------------------
2020-01-11T10:18:29.6088004Z stderr:
---
2020-01-11T10:18:29.6089237Z 
2020-01-11T10:18:29.6089280Z error[E0603]: function `foo` is private
2020-01-11T10:18:29.6089557Z   --> /checkout/src/test/ui/privacy/privacy2.rs:23:20
2020-01-11T10:18:29.6089623Z    |
2020-01-11T10:18:29.6089668Z LL |     use bar::glob::foo;
2020-01-11T10:18:29.6089744Z 
2020-01-11T10:18:29.6089803Z error: requires `sized` lang_item
2020-01-11T10:18:29.6089833Z 
2020-01-11T10:18:29.6089876Z error: aborting due to 3 previous errors
---
2020-01-11T10:18:29.6158428Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T10:18:29.6159611Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T10:18:29.6183163Z 
2020-01-11T10:18:29.6193401Z 
2020-01-11T10:18:29.6204321Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T10:18:29.6204647Z 
2020-01-11T10:18:29.6204679Z 
2020-01-11T10:18:29.6204762Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-11T10:18:29.6204838Z Build completed unsuccessfully in 1:02:43
2020-01-11T10:18:29.6204838Z Build completed unsuccessfully in 1:02:43
2020-01-11T10:18:29.6298367Z == clock drift check ==
2020-01-11T10:18:29.6321507Z   local time: Sat Jan 11 10:18:29 UTC 2020
2020-01-11T10:18:29.9206946Z   network time: Sat, 11 Jan 2020 10:18:29 GMT
2020-01-11T10:18:29.9210951Z == end clock drift check ==
2020-01-11T10:18:30.6466809Z 
2020-01-11T10:18:30.6573367Z ##[error]Bash exited with code '1'.
2020-01-11T10:18:30.6604782Z ##[section]Starting: Checkout
2020-01-11T10:18:30.6606486Z ==============================================================================
2020-01-11T10:18:30.6606560Z Task         : Get sources
2020-01-11T10:18:30.6606607Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
