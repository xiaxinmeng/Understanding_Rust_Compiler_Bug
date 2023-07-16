plain
2019-07-29T18:15:22.3119123Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T18:15:22.3337320Z ##[command]git config gc.auto 0
2019-07-29T18:15:22.3424773Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T18:15:22.3477419Z ##[command]git config --get-all http.proxy
2019-07-29T18:15:22.3622612Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62984/merge:refs/remotes/pull/62984/merge
---
2019-07-29T18:15:56.3734681Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T18:15:56.3735286Z 
2019-07-29T18:15:56.3736491Z   git checkout -b <new-branch-name>
2019-07-29T18:15:56.3737034Z 
2019-07-29T18:15:56.3738876Z HEAD is now at f5c378cc6 Merge 33d1bc567f73714244be55aca846ab7d71fc9af8 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-29T18:15:56.3891055Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T18:15:56.3893721Z ==============================================================================
2019-07-29T18:15:56.3893919Z Task         : Bash
2019-07-29T18:15:56.3893958Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T19:19:14.1387149Z .................................................................................................... 1400/8804
2019-07-29T19:19:20.3921964Z .................................................................................................... 1500/8804
2019-07-29T19:19:33.5916201Z ................................................................i...............i................... 1600/8804
2019-07-29T19:19:41.7114995Z .................................................................................................... 1700/8804
2019-07-29T19:19:56.8674975Z ..................................................iiiii............................................. 1800/8804
2019-07-29T19:20:08.6751472Z .................................................................................................... 2000/8804
2019-07-29T19:20:11.3865388Z .................................................................................................... 2100/8804
2019-07-29T19:20:15.6172087Z .................................................................................................... 2200/8804
2019-07-29T19:20:22.5739212Z .................................................................................................... 2300/8804
---
2019-07-29T19:24:18.6033524Z .................................................................................................... 5200/8804
2019-07-29T19:24:29.7517699Z .................................................................................................... 5300/8804
2019-07-29T19:24:37.7209645Z ..i................................................................................................. 5400/8804
2019-07-29T19:24:43.4043444Z .................................................................................................... 5500/8804
2019-07-29T19:24:56.3773358Z ................................................................................................ii.. 5600/8804
2019-07-29T19:25:11.7751539Z .i..ii...........i.................................................................................. 5700/8804
2019-07-29T19:25:27.5836128Z .................................................................................................... 5900/8804
2019-07-29T19:25:32.6321655Z ................................................................................................i..i 6000/8804
2019-07-29T19:25:47.5062869Z i................................................................................................... 6100/8804
2019-07-29T19:26:04.2556995Z .................................................................................................... 6200/8804
---
2019-07-29T19:30:22.3541140Z normalized stderr:
2019-07-29T19:30:22.3541211Z warning: unnecessary trailing semicolons
2019-07-29T19:30:22.3541428Z   --> $DIR/block-expr-precedence.rs:60:21
2019-07-29T19:30:22.3541469Z    |
2019-07-29T19:30:22.3541668Z LL |   if (true) { 12; };;; -num;
2019-07-29T19:30:22.3541714Z    |                     ^^ help: remove these semicolons
2019-07-29T19:30:22.3541753Z    |
2019-07-29T19:30:22.3541814Z    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-29T19:30:22.3541864Z 
2019-07-29T19:30:22.3541885Z 
2019-07-29T19:30:22.3541905Z 
2019-07-29T19:30:22.3542140Z The actual stderr differed from the expected stderr.
2019-07-29T19:30:22.3542140Z The actual stderr differed from the expected stderr.
2019-07-29T19:30:22.3542459Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expr-precedence/block-expr-precedence.stderr
2019-07-29T19:30:22.3542669Z To update references, rerun the tests and pass the `--bless` flag
2019-07-29T19:30:22.3542912Z To only update this specific test, also pass `--test-args block-expr-precedence.rs`
2019-07-29T19:30:22.3543105Z error: 1 errors occurred comparing output.
2019-07-29T19:30:22.3543163Z status: exit code: 0
2019-07-29T19:30:22.3543163Z status: exit code: 0
2019-07-29T19:30:22.3543800Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-expr-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expr-precedence/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expr-precedence/auxiliary" "-A" "unused"
2019-07-29T19:30:22.3544080Z ------------------------------------------
2019-07-29T19:30:22.3544108Z 
2019-07-29T19:30:22.3544734Z ------------------------------------------
2019-07-29T19:30:22.3544789Z stderr:
2019-07-29T19:30:22.3544789Z stderr:
2019-07-29T19:30:22.3545029Z ------------------------------------------
2019-07-29T19:30:22.3545080Z warning: unnecessary trailing semicolons
2019-07-29T19:30:22.3545333Z   --> /checkout/src/test/ui/block-expr-precedence.rs:60:21
2019-07-29T19:30:22.3545383Z    |
2019-07-29T19:30:22.3545591Z LL |   if (true) { 12; };;; -num;
2019-07-29T19:30:22.3545662Z    |                     ^^ help: remove these semicolons
2019-07-29T19:30:22.3545707Z    |
2019-07-29T19:30:22.3545753Z    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-29T19:30:22.3545831Z 
2019-07-29T19:30:22.3546044Z ------------------------------------------
2019-07-29T19:30:22.3546076Z 
2019-07-29T19:30:22.3546111Z 
---
2019-07-29T19:30:22.3590128Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-29T19:30:22.3590295Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T19:30:22.3609911Z 
2019-07-29T19:30:22.3613288Z 
2019-07-29T19:30:22.3618253Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T19:30:22.3618679Z 
2019-07-29T19:30:22.3618725Z 
2019-07-29T19:30:22.3618829Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T19:30:22.3618877Z Build completed unsuccessfully in 1:08:02
2019-07-29T19:30:22.3618877Z Build completed unsuccessfully in 1:08:02
2019-07-29T19:30:23.0965346Z ##[error]Bash exited with code '1'.
2019-07-29T19:30:23.1002730Z ##[section]Starting: Checkout
2019-07-29T19:30:23.1004315Z ==============================================================================
2019-07-29T19:30:23.1004363Z Task         : Get sources
2019-07-29T19:30:23.1004404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
