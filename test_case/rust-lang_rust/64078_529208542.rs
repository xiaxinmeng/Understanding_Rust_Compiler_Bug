plain
2019-09-08T13:24:45.5554953Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T13:24:45.5788168Z ##[command]git config gc.auto 0
2019-09-08T13:24:45.5887476Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T13:24:45.5954855Z ##[command]git config --get-all http.proxy
2019-09-08T13:24:45.6116818Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64078/merge:refs/remotes/pull/64078/merge
---
2019-09-08T14:28:03.0157742Z .................................................................................................... 1500/9008
2019-09-08T14:28:08.5174232Z .................................................................................................... 1600/9008
2019-09-08T14:28:21.1586037Z ......................................................i...............i............................. 1700/9008
2019-09-08T14:28:29.0051015Z .................................................................................................... 1800/9008
2019-09-08T14:28:44.5260892Z .............................................iiiii.................................................. 1900/9008
2019-09-08T14:28:54.6472427Z .................................................................................................... 2100/9008
2019-09-08T14:28:57.2120846Z .................................................................................................... 2200/9008
2019-09-08T14:29:00.8208332Z .................................................................................................... 2300/9008
2019-09-08T14:29:08.8439604Z .................................................................................................... 2400/9008
---
2019-09-08T14:32:06.6248126Z ..................................i...............i................................................. 4700/9008
2019-09-08T14:32:18.8661063Z .................................................................................................... 4800/9008
2019-09-08T14:32:25.4105925Z .................................................................................................... 4900/9008
2019-09-08T14:32:36.7621013Z .................................................................................................... 5000/9008
2019-09-08T14:32:42.9449035Z ................ii.ii............................................................................... 5100/9008
2019-09-08T14:32:53.5290442Z .................................................................................................... 5300/9008
2019-09-08T14:33:03.5869790Z ...............................................................................i.................... 5400/9008
2019-09-08T14:33:11.4098730Z .................................................................................................... 5500/9008
2019-09-08T14:33:17.4324116Z .................................................................................................... 5600/9008
2019-09-08T14:33:17.4324116Z .................................................................................................... 5600/9008
2019-09-08T14:33:28.6329581Z .........................................................................ii...i..ii...........i..... 5700/9008
2019-09-08T14:33:53.4463525Z .................................................................................................... 5900/9008
2019-09-08T14:34:02.6559297Z .................................................................................................... 6000/9008
2019-09-08T14:34:02.6559297Z .................................................................................................... 6000/9008
2019-09-08T14:34:09.8884373Z ...........................................................................i..ii.................... 6100/9008
2019-09-08T14:34:38.7461878Z .................................................................................................... 6300/9008
2019-09-08T14:34:41.0012799Z ..................................i................................................................. 6400/9008
2019-09-08T14:34:43.3214286Z .................................................................................................... 6500/9008
2019-09-08T14:34:45.9593111Z ......i............................................................................................. 6600/9008
---
2019-09-08T14:38:49.4595535Z 15 
2019-09-08T14:38:49.4595558Z 
2019-09-08T14:38:49.4595596Z 
2019-09-08T14:38:49.4595632Z The actual stderr differed from the expected stderr.
2019-09-08T14:38:49.4595898Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/borrowck-migrate-to-nll.zflag.stderr
2019-09-08T14:38:49.4596113Z To update references, rerun the tests and pass the `--bless` flag
2019-09-08T14:38:49.4596360Z To only update this specific test, also pass `--test-args borrowck/borrowck-migrate-to-nll.rs`
2019-09-08T14:38:49.4596392Z 
2019-09-08T14:38:49.4596447Z error in revision `zflag`: 1 errors occurred comparing output.
2019-09-08T14:38:49.4596486Z status: exit code: 0
2019-09-08T14:38:49.4597175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "zflag" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/auxiliary" "-A" "unused"
2019-09-08T14:38:49.4597471Z ------------------------------------------
2019-09-08T14:38:49.4597517Z 
2019-09-08T14:38:49.4597704Z ------------------------------------------
2019-09-08T14:38:49.4597743Z stderr:
2019-09-08T14:38:49.4597743Z stderr:
2019-09-08T14:38:49.4597943Z ------------------------------------------
2019-09-08T14:38:49.4597999Z warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-09-08T14:38:49.4598287Z    |
2019-09-08T14:38:49.4598324Z LL |     let x = &mut block;
2019-09-08T14:38:49.4598521Z    |             ---------- mutable borrow occurs here
2019-09-08T14:38:49.4598521Z    |             ---------- mutable borrow occurs here
2019-09-08T14:38:49.4598706Z LL |     let p: &'a u8 = &*block.current;
2019-09-08T14:38:49.4598768Z    |                     ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-09-08T14:38:49.4598840Z LL |     drop(x);
2019-09-08T14:38:49.4598840Z LL |     drop(x);
2019-09-08T14:38:49.4599049Z    |          - mutable borrow later used here
2019-09-08T14:38:49.4599131Z    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-09-08T14:38:49.4599376Z    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-09-08T14:38:49.4599596Z    = note: for more information, try `rustc --explain E0729`
2019-09-08T14:38:49.4599628Z 
---
2019-09-08T14:38:49.4639073Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-08T14:38:49.4639163Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-08T14:38:49.4655120Z 
2019-09-08T14:38:49.4655381Z 
2019-09-08T14:38:49.4657492Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-08T14:38:49.4658106Z 
2019-09-08T14:38:49.4658132Z 
2019-09-08T14:38:49.4665804Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-08T14:38:49.4665903Z Build completed unsuccessfully in 1:06:56
2019-09-08T14:38:49.4665903Z Build completed unsuccessfully in 1:06:56
2019-09-08T14:38:49.4719893Z == clock drift check ==
2019-09-08T14:38:49.4736829Z   local time: Sun Sep  8 14:38:49 UTC 2019
2019-09-08T14:38:49.6269359Z   network time: Sun, 08 Sep 2019 14:38:49 GMT
2019-09-08T14:38:49.6269478Z == end clock drift check ==
2019-09-08T14:38:50.3167878Z ##[error]Bash exited with code '1'.
2019-09-08T14:38:50.3223484Z ##[section]Starting: Checkout
2019-09-08T14:38:50.3225483Z ==============================================================================
2019-09-08T14:38:50.3225553Z Task         : Get sources
2019-09-08T14:38:50.3225604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
