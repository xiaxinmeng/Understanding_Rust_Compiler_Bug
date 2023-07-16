plain
2019-09-15T05:36:59.0234459Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-15T05:36:59.0433326Z ##[command]git config gc.auto 0
2019-09-15T05:36:59.0530502Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-15T05:36:59.0588791Z ##[command]git config --get-all http.proxy
2019-09-15T05:36:59.0778536Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-15T06:42:41.7625473Z ...........................................................F........................................ 1500/9018
2019-09-15T06:42:47.9270437Z .................................................................................................... 1600/9018
2019-09-15T06:43:01.1887799Z ............................................................i...............i....................... 1700/9018
2019-09-15T06:43:09.1638823Z .................................................................................................... 1800/9018
2019-09-15T06:43:24.9624999Z ...................................................iiiii............................................ 1900/9018
2019-09-15T06:43:36.7826119Z .................................................................................................... 2100/9018
2019-09-15T06:43:39.4879615Z .................................................................................................... 2200/9018
2019-09-15T06:43:43.1551273Z .................................................................................................... 2300/9018
2019-09-15T06:43:51.8324230Z .................................................................................................... 2400/9018
---
2019-09-15T06:46:58.5395469Z .......................................i...............i............................................ 4700/9018
2019-09-15T06:47:10.8122367Z .................................................................................................... 4800/9018
2019-09-15T06:47:17.7797136Z .................................................................................................... 4900/9018
2019-09-15T06:47:28.4286473Z .................................................................................................... 5000/9018
2019-09-15T06:47:35.5543446Z .......................ii.ii........................................................................ 5100/9018
2019-09-15T06:47:46.4756823Z .................................................................................................... 5300/9018
2019-09-15T06:47:57.0650251Z .......................................................................................i............ 5400/9018
2019-09-15T06:48:05.5215601Z .................................................................................................... 5500/9018
2019-09-15T06:48:11.2091347Z .................................................................................................... 5600/9018
2019-09-15T06:48:11.2091347Z .................................................................................................... 5600/9018
2019-09-15T06:48:21.9822917Z ..................................................................................ii...i..ii........ 5700/9018
2019-09-15T06:48:48.5348349Z .................................................................................................... 5900/9018
2019-09-15T06:48:59.1154685Z .................................................................................................... 6000/9018
2019-09-15T06:48:59.1154685Z .................................................................................................... 6000/9018
2019-09-15T06:49:08.3890733Z ....................................................................................i..ii........... 6100/9018
2019-09-15T06:49:42.3876937Z .................................................................................................... 6300/9018
2019-09-15T06:49:45.0292481Z ...........................................i........................................................ 6400/9018
2019-09-15T06:49:47.3781218Z .................................................................................................... 6500/9018
2019-09-15T06:49:50.0954427Z ...............i.................................................................................... 6600/9018
---
2019-09-15T06:54:06.3328826Z 1 error[E0019]: constant contains unimplemented expression type
2019-09-15T06:54:06.3329407Z -   --> $DIR/const-if.rs:1:20
2019-09-15T06:54:06.3329968Z +   --> $DIR/const-if.rs:1:19
2019-09-15T06:54:06.3330274Z 3    |
2019-09-15T06:54:06.3330791Z - LL | const _X: i32 = if true { 5 } else { 6 };
2019-09-15T06:54:06.3331326Z -    |                    ^^^^
2019-09-15T06:54:06.3331622Z + LL | const _: i32 = if true { 5 } else { 6 };
2019-09-15T06:54:06.3332128Z 6 
2019-09-15T06:54:06.3332347Z 7 error[E0019]: constant contains unimplemented expression type
2019-09-15T06:54:06.3332844Z -   --> $DIR/const-if.rs:1:17
2019-09-15T06:54:06.3333364Z +   --> $DIR/const-if.rs:1:16
2019-09-15T06:54:06.3333364Z +   --> $DIR/const-if.rs:1:16
2019-09-15T06:54:06.3333652Z 9    |
2019-09-15T06:54:06.3334490Z - LL | const _X: i32 = if true { 5 } else { 6 };
2019-09-15T06:54:06.3335149Z -    |                 ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-15T06:54:06.3335485Z + LL | const _: i32 = if true { 5 } else { 6 };
2019-09-15T06:54:06.3335943Z 12 
2019-09-15T06:54:06.3336181Z 13 error: aborting due to 2 previous errors
2019-09-15T06:54:06.3336424Z 14 
2019-09-15T06:54:06.3336619Z 
2019-09-15T06:54:06.3336619Z 
2019-09-15T06:54:06.3336813Z 
2019-09-15T06:54:06.3337064Z The actual stderr differed from the expected stderr.
2019-09-15T06:54:06.3337605Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-if/const-if.stderr
2019-09-15T06:54:06.3338225Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T06:54:06.3338871Z To only update this specific test, also pass `--test-args consts/const-if.rs`
2019-09-15T06:54:06.3339426Z error: 1 errors occurred comparing output.
2019-09-15T06:54:06.3339895Z status: exit code: 1
2019-09-15T06:54:06.3339895Z status: exit code: 1
2019-09-15T06:54:06.3341065Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-if.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-if" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-if/auxiliary" "-A" "unused"
2019-09-15T06:54:06.3342014Z ------------------------------------------
2019-09-15T06:54:06.3342316Z 
2019-09-15T06:54:06.3342817Z ------------------------------------------
2019-09-15T06:54:06.3343118Z stderr:
---
2019-09-15T06:54:06.3369901Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-15T06:54:06.3370242Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-15T06:54:06.3387079Z 
2019-09-15T06:54:06.3387381Z 
2019-09-15T06:54:06.3390060Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-15T06:54:06.3391954Z 
2019-09-15T06:54:06.3391987Z 
2019-09-15T06:54:06.3443778Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-15T06:54:06.3444189Z Build completed unsuccessfully in 1:09:36
2019-09-15T06:54:06.3444189Z Build completed unsuccessfully in 1:09:36
2019-09-15T06:54:06.3452553Z == clock drift check ==
2019-09-15T06:54:06.3468621Z   local time: Sun Sep 15 06:54:06 UTC 2019
2019-09-15T06:54:06.4347292Z   network time: Sun, 15 Sep 2019 06:54:06 GMT
2019-09-15T06:54:06.4347524Z == end clock drift check ==
2019-09-15T06:54:07.2535532Z ##[error]Bash exited with code '1'.
2019-09-15T06:54:07.2611766Z ##[section]Starting: Checkout
2019-09-15T06:54:07.2613826Z ==============================================================================
2019-09-15T06:54:07.2613921Z Task         : Get sources
2019-09-15T06:54:07.2613973Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
