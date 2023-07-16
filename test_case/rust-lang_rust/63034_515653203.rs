plain
2019-07-27T04:06:34.3443571Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T04:06:34.3667527Z ##[command]git config gc.auto 0
2019-07-27T04:06:34.3712150Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T04:06:34.3763801Z ##[command]git config --get-all http.proxy
2019-07-27T04:06:34.3906271Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63034/merge:refs/remotes/pull/63034/merge
---
2019-07-27T04:07:06.6229567Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T04:07:06.6229597Z 
2019-07-27T04:07:06.6229808Z   git checkout -b <new-branch-name>
2019-07-27T04:07:06.6229856Z 
2019-07-27T04:07:06.6229921Z HEAD is now at 520f1b0de Merge fd1583738cbdd24f9ad0fdbc273be2d6496e9413 into 09e39897587dca70f0b15093d425a682c392349c
2019-07-27T04:07:06.6374560Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T04:07:06.6377371Z ==============================================================================
2019-07-27T04:07:06.6377445Z Task         : Bash
2019-07-27T04:07:06.6377493Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T05:09:15.3829107Z .................................................................................................... 700/5870
2019-07-27T05:09:19.4552917Z .................................................................................................... 800/5870
2019-07-27T05:09:25.1056259Z .................................................................................................... 900/5870
2019-07-27T05:09:30.2507540Z .................................................................................................... 1000/5870
2019-07-27T05:09:35.8449927Z i...........i....................................................................................... 1100/5870
2019-07-27T05:09:39.8620066Z ..............................iiiii................................................................. 1200/5870
2019-07-27T05:09:45.9588391Z .................................................................................................... 1400/5870
2019-07-27T05:09:48.6677913Z .................................................................................................... 1500/5870
2019-07-27T05:09:52.4815025Z .................................................................................................... 1600/5870
2019-07-27T05:09:55.1500778Z .................................................................................................... 1700/5870
---
2019-07-27T05:11:09.4534356Z .................................................................................................... 3400/5870
2019-07-27T05:11:14.5040559Z .................................................................................................... 3500/5870
2019-07-27T05:11:18.4724693Z ..........................i......................................................................... 3600/5870
2019-07-27T05:11:22.8288084Z .................................................................................................... 3700/5870
2019-07-27T05:11:26.3156207Z ....ii...i..ii...................................................................................... 3800/5870
2019-07-27T05:11:35.2422553Z .................................................................................................... 4000/5870
2019-07-27T05:11:39.1200471Z .......................ii........................................................................... 4100/5870
2019-07-27T05:11:41.3919664Z ............................................i....................................................... 4200/5870
2019-07-27T05:11:43.5816488Z .................................................................................................... 4300/5870
---
2019-07-27T05:13:05.9126948Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-27T05:13:05.9127041Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T05:13:05.9138391Z 
2019-07-27T05:13:05.9138480Z 
2019-07-27T05:13:05.9144119Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-27T05:13:05.9144426Z 
2019-07-27T05:13:05.9144456Z 
2019-07-27T05:13:05.9155017Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-27T05:13:05.9155285Z Build completed unsuccessfully in 0:58:12
2019-07-27T05:13:05.9155285Z Build completed unsuccessfully in 0:58:12
2019-07-27T05:13:06.9925786Z ##[error]Bash exited with code '1'.
2019-07-27T05:13:06.9969208Z ##[section]Starting: Checkout
2019-07-27T05:13:06.9971575Z ==============================================================================
2019-07-27T05:13:06.9971633Z Task         : Get sources
2019-07-27T05:13:06.9971680Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
