plain
2019-08-13T00:49:15.8870884Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T00:49:15.9049632Z ##[command]git config gc.auto 0
2019-08-13T00:49:15.9122297Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T00:49:15.9171555Z ##[command]git config --get-all http.proxy
2019-08-13T00:49:15.9308213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63470/merge:refs/remotes/pull/63470/merge
---
2019-08-13T00:49:51.4556209Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T00:49:51.4556234Z 
2019-08-13T00:49:51.4556405Z   git checkout -b <new-branch-name>
2019-08-13T00:49:51.4556446Z 
2019-08-13T00:49:51.4556486Z HEAD is now at 0b563536d Merge cc0bdabef81d6a5f66caeefb3744145f22f7fa7f into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T00:49:51.4729816Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T00:49:51.4733004Z ==============================================================================
2019-08-13T00:49:51.4733071Z Task         : Bash
2019-08-13T00:49:51.4733109Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T01:46:59.6090570Z .................................................................................................... 1300/8875
2019-08-13T01:47:06.2606012Z .................................................................................................... 1400/8875
2019-08-13T01:47:11.8981421Z .................................................................................................... 1500/8875
2019-08-13T01:47:21.9724751Z ....................................................................................i............... 1600/8875
2019-08-13T01:47:29.3827132Z i................................................................................................... 1700/8875
2019-08-13T01:47:35.8508370Z ...........................................................................iiiii.................... 1800/8875
2019-08-13T01:47:57.0335565Z .................................................................................................... 2000/8875
2019-08-13T01:47:59.4164377Z .................................................................................................... 2100/8875
2019-08-13T01:48:02.0424979Z .................................................................................................... 2200/8875
2019-08-13T01:48:09.4670696Z .................................................................................................... 2300/8875
---
2019-08-13T01:51:58.4298800Z .................................................................................................... 5300/8875
2019-08-13T01:52:05.5776011Z ........i........................................................................................... 5400/8875
2019-08-13T01:52:10.8147842Z .................................................................................................... 5500/8875
2019-08-13T01:52:22.8010324Z .................................................................................................... 5600/8875
2019-08-13T01:52:36.4495964Z ...ii...i..ii...........i........................................................................... 5700/8875
2019-08-13T01:52:51.0155298Z .................................................................................................... 5900/8875
2019-08-13T01:52:55.6030007Z .................................................................................................... 6000/8875
2019-08-13T01:52:55.6030007Z .................................................................................................... 6000/8875
2019-08-13T01:53:09.1527375Z ....i..ii........................................................................................... 6100/8875
2019-08-13T01:53:27.2263613Z ...............................................i.................................................... 6300/8875
2019-08-13T01:53:29.3326601Z .................................................................................................... 6400/8875
2019-08-13T01:53:31.7576693Z ...................i................................................................................ 6500/8875
2019-08-13T01:53:36.1665260Z .................................................................................................... 6600/8875
---
2019-08-13T01:57:25.5834481Z 
2019-08-13T01:57:25.5835094Z ---- [ui] ui/lint/lint-qualification.rs stdout ----
2019-08-13T01:57:25.5835363Z diff of stderr:
2019-08-13T01:57:25.5835483Z 
2019-08-13T01:57:25.5835812Z + warning: use of deprecated item 'try': use the `?` operator instead
2019-08-13T01:57:25.5836164Z +   --> $DIR/lint-qualification.rs:13:36
2019-08-13T01:57:25.5836316Z +    |
2019-08-13T01:57:25.5836652Z + LL |     let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345
2019-08-13T01:57:25.5836926Z +    |
2019-08-13T01:57:25.5837059Z +    = note: `#[warn(deprecated)]` on by default
2019-08-13T01:57:25.5837178Z + 
2019-08-13T01:57:25.5837289Z 1 error: unnecessary qualification
2019-08-13T01:57:25.5837289Z 1 error: unnecessary qualification
2019-08-13T01:57:25.5837592Z 2   --> $DIR/lint-qualification.rs:10:5
2019-08-13T01:57:25.5837755Z 3    |
2019-08-13T01:57:25.5837856Z 
2019-08-13T01:57:25.5837970Z 
2019-08-13T01:57:25.5838092Z The actual stderr differed from the expected stderr.
2019-08-13T01:57:25.5838454Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification/lint-qualification.stderr
2019-08-13T01:57:25.5839779Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T01:57:25.5840264Z To only update this specific test, also pass `--test-args lint/lint-qualification.rs`
2019-08-13T01:57:25.5840600Z error: 1 errors occurred comparing output.
2019-08-13T01:57:25.5840748Z status: exit code: 1
2019-08-13T01:57:25.5840748Z status: exit code: 1
2019-08-13T01:57:25.5841826Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-qualification.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification/auxiliary" "-A" "unused"
2019-08-13T01:57:25.5842783Z ------------------------------------------
2019-08-13T01:57:25.5842934Z 
2019-08-13T01:57:25.5843234Z ------------------------------------------
2019-08-13T01:57:25.5843405Z stderr:
2019-08-13T01:57:25.5843405Z stderr:
2019-08-13T01:57:25.5843693Z ------------------------------------------
2019-08-13T01:57:25.5844021Z warning: use of deprecated item 'try': use the `?` operator instead
2019-08-13T01:57:25.5844707Z    |
2019-08-13T01:57:25.5844707Z    |
2019-08-13T01:57:25.5845016Z LL |     let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345
2019-08-13T01:57:25.5845326Z    |
2019-08-13T01:57:25.5845456Z    = note: `#[warn(deprecated)]` on by default
2019-08-13T01:57:25.5845563Z 
2019-08-13T01:57:25.5845678Z error: unnecessary qualification
2019-08-13T01:57:25.5845678Z error: unnecessary qualification
2019-08-13T01:57:25.5845988Z   --> /checkout/src/test/ui/lint/lint-qualification.rs:10:5
2019-08-13T01:57:25.5846164Z    |
2019-08-13T01:57:25.5846286Z LL |     foo::bar(); //~ ERROR: unnecessary qualification
2019-08-13T01:57:25.5846534Z    |
2019-08-13T01:57:25.5846667Z note: lint level defined here
2019-08-13T01:57:25.5846988Z   --> /checkout/src/test/ui/lint/lint-qualification.rs:1:9
2019-08-13T01:57:25.5847140Z    |
---
2019-08-13T01:57:25.5872356Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-13T01:57:25.5872836Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-13T01:57:25.5887552Z 
2019-08-13T01:57:25.5887816Z 
2019-08-13T01:57:25.5890275Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-13T01:57:25.5891302Z 
2019-08-13T01:57:25.5891556Z 
2019-08-13T01:57:25.5896908Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-13T01:57:25.5896995Z Build completed unsuccessfully in 1:01:46
2019-08-13T01:57:25.5896995Z Build completed unsuccessfully in 1:01:46
2019-08-13T01:57:26.2658301Z ##[error]Bash exited with code '1'.
2019-08-13T01:57:26.2716469Z ##[section]Starting: Checkout
2019-08-13T01:57:26.2717898Z ==============================================================================
2019-08-13T01:57:26.2717938Z Task         : Get sources
2019-08-13T01:57:26.2717973Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
