plain
2019-09-08T00:45:21.0191927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T00:45:21.0367871Z ##[command]git config gc.auto 0
2019-09-08T00:45:21.0443146Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T00:45:21.0518236Z ##[command]git config --get-all http.proxy
2019-09-08T00:45:21.0654984Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64078/merge:refs/remotes/pull/64078/merge
---
2019-09-08T01:43:40.8936076Z .................................................................................................... 1500/9006
2019-09-08T01:43:46.3328932Z .................................................................................................... 1600/9006
2019-09-08T01:43:58.1068063Z ......................................................i...............i............................. 1700/9006
2019-09-08T01:44:05.3131044Z .................................................................................................... 1800/9006
2019-09-08T01:44:18.7417957Z .............................................iiiii.................................................. 1900/9006
2019-09-08T01:44:28.8671447Z .................................................................................................... 2100/9006
2019-09-08T01:44:31.1839842Z .................................................................................................... 2200/9006
2019-09-08T01:44:34.5078417Z .................................................................................................... 2300/9006
2019-09-08T01:44:41.8659421Z .................................................................................................... 2400/9006
---
2019-09-08T01:47:26.9230375Z .................................i...............i.................................................. 4700/9006
2019-09-08T01:47:38.1881677Z .................................................................................................... 4800/9006
2019-09-08T01:47:44.1552558Z .................................................................................................... 4900/9006
2019-09-08T01:47:54.4999179Z .................................................................................................... 5000/9006
2019-09-08T01:48:00.2539802Z ...............ii.ii................................................................................ 5100/9006
2019-09-08T01:48:10.2669532Z .................................................................................................... 5300/9006
2019-09-08T01:48:19.7977448Z ..............................................................................i..................... 5400/9006
2019-09-08T01:48:26.7965373Z .................................................................................................... 5500/9006
2019-09-08T01:48:32.3917981Z .................................................................................................... 5600/9006
2019-09-08T01:48:32.3917981Z .................................................................................................... 5600/9006
2019-09-08T01:48:42.1978433Z ........................................................................ii...i..ii...........i...... 5700/9006
2019-09-08T01:49:05.3266136Z .................................................................................................... 5900/9006
2019-09-08T01:49:14.1742304Z .................................................................................................... 6000/9006
2019-09-08T01:49:14.1742304Z .................................................................................................... 6000/9006
2019-09-08T01:49:19.7464654Z ..........................................................................i..ii..................... 6100/9006
2019-09-08T01:49:46.8951878Z .................................................................................................... 6300/9006
2019-09-08T01:49:48.6731167Z .................................i.................................................................. 6400/9006
2019-09-08T01:49:50.5860163Z .................................................................................................... 6500/9006
2019-09-08T01:49:52.9394297Z .....i.............................................................................................. 6600/9006
---
2019-09-08T01:53:36.3161350Z failures:
2019-09-08T01:53:36.3185771Z 
2019-09-08T01:53:36.3186237Z ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#zflag stdout ----
2019-09-08T01:53:36.3186303Z 
2019-09-08T01:53:36.3186346Z error in revision `zflag`: ui test compiled successfully!
2019-09-08T01:53:36.3186386Z status: exit code: 0
2019-09-08T01:53:36.3187066Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "zflag" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/auxiliary" "-A" "unused"
2019-09-08T01:53:36.3187678Z ------------------------------------------
2019-09-08T01:53:36.3187714Z 
2019-09-08T01:53:36.3187908Z ------------------------------------------
2019-09-08T01:53:36.3187947Z stderr:
2019-09-08T01:53:36.3187947Z stderr:
2019-09-08T01:53:36.3188147Z ------------------------------------------
2019-09-08T01:53:36.3188196Z warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-09-08T01:53:36.3189060Z    |
2019-09-08T01:53:36.3189109Z LL |     let x = &mut block;
2019-09-08T01:53:36.3189381Z    |             ---------- mutable borrow occurs here
2019-09-08T01:53:36.3189381Z    |             ---------- mutable borrow occurs here
2019-09-08T01:53:36.3189674Z LL |     let p: &'a u8 = &*block.current;
2019-09-08T01:53:36.3189737Z    |                     ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-09-08T01:53:36.3189860Z LL |     drop(x);
2019-09-08T01:53:36.3189860Z LL |     drop(x);
2019-09-08T01:53:36.3190124Z    |          - mutable borrow later used here
2019-09-08T01:53:36.3190253Z    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-09-08T01:53:36.3190324Z    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-09-08T01:53:36.3190605Z    = note: for more information, try `rustc --explain E0729`
2019-09-08T01:53:36.3190666Z 
---
2019-09-08T01:53:36.3221078Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-08T01:53:36.3221207Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-08T01:53:36.3235371Z 
2019-09-08T01:53:36.3235452Z 
2019-09-08T01:53:36.3236935Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-08T01:53:36.3237150Z 
2019-09-08T01:53:36.3237175Z 
2019-09-08T01:53:36.3247485Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-08T01:53:36.3247617Z Build completed unsuccessfully in 1:00:51
2019-09-08T01:53:36.3247617Z Build completed unsuccessfully in 1:00:51
2019-09-08T01:53:36.3297065Z == clock drift check ==
2019-09-08T01:53:36.9486760Z   local time: Sun Sep  8 01:53:36 UTC 2019
2019-09-08T01:53:36.9487348Z   network time: Sun, 08 Sep 2019 01:53:36 GMT
2019-09-08T01:53:36.9487488Z == end clock drift check ==
2019-09-08T01:53:37.5325109Z ##[error]Bash exited with code '1'.
2019-09-08T01:53:37.5361433Z ##[section]Starting: Checkout
2019-09-08T01:53:37.5363837Z ==============================================================================
2019-09-08T01:53:37.5363903Z Task         : Get sources
2019-09-08T01:53:37.5363953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
