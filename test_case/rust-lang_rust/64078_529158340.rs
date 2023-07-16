plain
2019-09-07T23:30:39.5791128Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T23:30:39.5987913Z ##[command]git config gc.auto 0
2019-09-07T23:30:39.6053493Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T23:30:39.6108423Z ##[command]git config --get-all http.proxy
2019-09-07T23:30:39.6237889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64078/merge:refs/remotes/pull/64078/merge
---
2019-09-08T00:30:18.4024545Z .................................................................................................... 1500/9006
2019-09-08T00:30:24.5504721Z .................................................................................................... 1600/9006
2019-09-08T00:30:38.0276079Z ......................................................i...............i............................. 1700/9006
2019-09-08T00:30:46.2555525Z .................................................................................................... 1800/9006
2019-09-08T00:31:01.6724598Z .............................................iiiii.................................................. 1900/9006
2019-09-08T00:31:13.4251994Z .................................................................................................... 2100/9006
2019-09-08T00:31:16.1350895Z .................................................................................................... 2200/9006
2019-09-08T00:31:19.9855589Z .................................................................................................... 2300/9006
2019-09-08T00:31:28.2837839Z .................................................................................................... 2400/9006
---
2019-09-08T00:34:35.0319768Z .................................i...............i.................................................. 4700/9006
2019-09-08T00:34:47.4201260Z .................................................................................................... 4800/9006
2019-09-08T00:34:53.9136928Z .................................................................................................... 4900/9006
2019-09-08T00:35:05.1283050Z .................................................................................................... 5000/9006
2019-09-08T00:35:11.4047499Z ...............ii.ii................................................................................ 5100/9006
2019-09-08T00:35:22.5898931Z .................................................................................................... 5300/9006
2019-09-08T00:35:33.0945384Z ..............................................................................i..................... 5400/9006
2019-09-08T00:35:41.0554356Z .................................................................................................... 5500/9006
2019-09-08T00:35:47.3948231Z .................................................................................................... 5600/9006
2019-09-08T00:35:47.3948231Z .................................................................................................... 5600/9006
2019-09-08T00:35:58.3651616Z ........................................................................ii...i..ii...........i...... 5700/9006
2019-09-08T00:36:24.7280792Z .................................................................................................... 5900/9006
2019-09-08T00:36:34.6186127Z .................................................................................................... 6000/9006
2019-09-08T00:36:34.6186127Z .................................................................................................... 6000/9006
2019-09-08T00:36:40.4043684Z ..........................................................................i..ii..................... 6100/9006
2019-09-08T00:37:10.8373884Z .................................................................................................... 6300/9006
2019-09-08T00:37:13.0466332Z .................................i.................................................................. 6400/9006
2019-09-08T00:37:15.3662274Z .................................................................................................... 6500/9006
2019-09-08T00:37:18.1243841Z .....i.............................................................................................. 6600/9006
---
2019-09-08T00:41:33.7726577Z 
2019-09-08T00:41:33.7728640Z ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#edition stdout ----
2019-09-08T00:41:33.7729081Z diff of stderr:
2019-09-08T00:41:33.7729270Z 
2019-09-08T00:41:33.7729472Z 1 error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-09-08T00:41:33.7730853Z +   --> $DIR/borrowck-migrate-to-nll.rs:29:21
2019-09-08T00:41:33.7730944Z 3    |
2019-09-08T00:41:33.7730995Z 4 LL |     let x = &mut block;
2019-09-08T00:41:33.7731329Z 5    |             ---------- mutable borrow occurs here
2019-09-08T00:41:33.7731329Z 5    |             ---------- mutable borrow occurs here
2019-09-08T00:41:33.7731372Z 
2019-09-08T00:41:33.7731401Z 
2019-09-08T00:41:33.7731466Z The actual stderr differed from the expected stderr.
2019-09-08T00:41:33.7731838Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/borrowck-migrate-to-nll.edition.stderr
2019-09-08T00:41:33.7732117Z To update references, rerun the tests and pass the `--bless` flag
2019-09-08T00:41:33.7732469Z To only update this specific test, also pass `--test-args borrowck/borrowck-migrate-to-nll.rs`
2019-09-08T00:41:33.7732507Z 
2019-09-08T00:41:33.7732553Z error in revision `edition`: 1 errors occurred comparing output.
2019-09-08T00:41:33.7732598Z status: exit code: 1
2019-09-08T00:41:33.7733897Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/auxiliary" "-A" "unused"
2019-09-08T00:41:33.7734662Z ------------------------------------------
2019-09-08T00:41:33.7734700Z 
2019-09-08T00:41:33.7734975Z ------------------------------------------
2019-09-08T00:41:33.7735025Z stderr:
2019-09-08T00:41:33.7735025Z stderr:
2019-09-08T00:41:33.7735264Z ------------------------------------------
2019-09-08T00:41:33.7735467Z error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-09-08T00:41:33.7735846Z    |
2019-09-08T00:41:33.7735910Z LL |     let x = &mut block;
2019-09-08T00:41:33.7736172Z    |             ---------- mutable borrow occurs here
2019-09-08T00:41:33.7736172Z    |             ---------- mutable borrow occurs here
2019-09-08T00:41:33.7736420Z LL |     let p: &'a u8 = &*block.current;
2019-09-08T00:41:33.7736493Z    |                     ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-09-08T00:41:33.7736582Z LL |     drop(x);
2019-09-08T00:41:33.7736582Z LL |     drop(x);
2019-09-08T00:41:33.7736834Z    |          - mutable borrow later used here
2019-09-08T00:41:33.7736944Z error: aborting due to previous error
2019-09-08T00:41:33.7736973Z 
2019-09-08T00:41:33.7737402Z For more information about this error, try `rustc --explain E0502`.
2019-09-08T00:41:33.7737457Z 
2019-09-08T00:41:33.7737457Z 
2019-09-08T00:41:33.7737694Z ------------------------------------------
2019-09-08T00:41:33.7737728Z 
2019-09-08T00:41:33.7737761Z 
2019-09-08T00:41:33.7738037Z ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#zflag stdout ----
2019-09-08T00:41:33.7738072Z 
2019-09-08T00:41:33.7738116Z error in revision `zflag`: ui test compiled successfully!
2019-09-08T00:41:33.7738160Z status: exit code: 0
2019-09-08T00:41:33.7739010Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "zflag" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/auxiliary" "-A" "unused"
2019-09-08T00:41:33.7739380Z ------------------------------------------
2019-09-08T00:41:33.7739415Z 
2019-09-08T00:41:33.7739652Z ------------------------------------------
2019-09-08T00:41:33.7739716Z stderr:
2019-09-08T00:41:33.7739716Z stderr:
2019-09-08T00:41:33.7739945Z ------------------------------------------
2019-09-08T00:41:33.7740002Z warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-09-08T00:41:33.7740341Z    |
2019-09-08T00:41:33.7740383Z LL |     let x = &mut block;
2019-09-08T00:41:33.7740652Z    |             ---------- mutable borrow occurs here
2019-09-08T00:41:33.7740652Z    |             ---------- mutable borrow occurs here
2019-09-08T00:41:33.7740903Z LL |     let p: &'a u8 = &*block.current;
2019-09-08T00:41:33.7740955Z    |                     ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-09-08T00:41:33.7741056Z LL |     drop(x);
2019-09-08T00:41:33.7741056Z LL |     drop(x);
2019-09-08T00:41:33.7741292Z    |          - mutable borrow later used here
2019-09-08T00:41:33.7741418Z    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-09-08T00:41:33.7741480Z    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-09-08T00:41:33.7741770Z    = note: for more information, try `rustc --explain E0729`
2019-09-08T00:41:33.7741808Z 
---
2019-09-08T00:41:33.7789053Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-08T00:41:33.7789186Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-08T00:41:33.7809434Z 
2019-09-08T00:41:33.7809548Z 
2019-09-08T00:41:33.7811540Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-08T00:41:33.7811834Z 
2019-09-08T00:41:33.7811865Z 
2019-09-08T00:41:33.7817522Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-08T00:41:33.7817806Z Build completed unsuccessfully in 1:04:04
2019-09-08T00:41:33.7817806Z Build completed unsuccessfully in 1:04:04
2019-09-08T00:41:33.7870790Z == clock drift check ==
2019-09-08T00:41:33.7885963Z   local time: Sun Sep  8 00:41:33 UTC 2019
2019-09-08T00:41:33.9407848Z   network time: Sun, 08 Sep 2019 00:41:33 GMT
2019-09-08T00:41:33.9408276Z == end clock drift check ==
2019-09-08T00:41:34.7188054Z ##[error]Bash exited with code '1'.
2019-09-08T00:41:34.7254546Z ##[section]Starting: Checkout
2019-09-08T00:41:34.7256619Z ==============================================================================
2019-09-08T00:41:34.7256670Z Task         : Get sources
2019-09-08T00:41:34.7256716Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
