plain
2020-01-10T22:25:45.1406274Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T22:25:45.1541771Z ##[command]git config gc.auto 0
2020-01-10T22:25:45.1631719Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T22:25:45.1719626Z ##[command]git config --get-all http.proxy
2020-01-10T22:25:45.1880172Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68096/merge:refs/remotes/pull/68096/merge
---
2020-01-10T23:26:52.1531417Z .............................i...............i...................................................... 4900/9503
2020-01-10T23:27:02.2904213Z .................................................................................................... 5000/9503
2020-01-10T23:27:08.9700244Z ..........................................................................i......................... 5100/9503
2020-01-10T23:27:15.5401214Z .................................................................................................... 5200/9503
2020-01-10T23:27:25.3034929Z .........................................ii.ii...........i.......................................... 5300/9503
2020-01-10T23:27:35.4542588Z .................................................................................................... 5500/9503
2020-01-10T23:27:45.6520709Z .................................................................................................... 5600/9503
2020-01-10T23:27:53.2135175Z .........................i.......................................................................... 5700/9503
2020-01-10T23:27:59.6876083Z .................................................................................................... 5800/9503
2020-01-10T23:27:59.6876083Z .................................................................................................... 5800/9503
2020-01-10T23:28:12.3688653Z .................................................................................................... 5900/9503
2020-01-10T23:28:22.8898107Z ................ii...i..ii...........i.............................................................. 6000/9503
2020-01-10T23:28:41.9315591Z .................................................................................................... 6200/9503
2020-01-10T23:28:50.4759101Z .................................................................................................... 6300/9503
2020-01-10T23:28:50.4759101Z .................................................................................................... 6300/9503
2020-01-10T23:29:05.4024696Z ...........................................i..ii.................................................... 6400/9503
2020-01-10T23:29:28.4623604Z .................................................................................................... 6600/9503
2020-01-10T23:29:30.6988083Z ..................i................................................................................. 6700/9503
2020-01-10T23:29:33.5133274Z .................................................................................................... 6800/9503
2020-01-10T23:29:36.2062711Z ..................i................................................................................. 6900/9503
---
2020-01-10T23:31:19.0856712Z .................................................................................................... 7500/9503
2020-01-10T23:31:23.0350191Z .......................................F............................................................ 7600/9503
2020-01-10T23:31:29.1682114Z .................................................................................................... 7700/9503
2020-01-10T23:31:37.9864020Z .................................................................................................... 7800/9503
2020-01-10T23:31:48.6785795Z .....................................................................iiii........................... 7900/9503
2020-01-10T23:32:04.6223615Z ...i......i......................................................................................... 8100/9503
2020-01-10T23:32:10.1109659Z .................................................................................................... 8200/9503
2020-01-10T23:32:26.0739772Z .................................................................................................... 8300/9503
2020-01-10T23:32:35.0293929Z .................................................................................................... 8400/9503
---
2020-01-10T23:34:39.7742601Z diff of stderr:
2020-01-10T23:34:39.7742791Z 
2020-01-10T23:34:39.7743229Z 2   --> $DIR/impl-items-vis-unresolved.rs:21:13
2020-01-10T23:34:39.7743419Z 3    |
2020-01-10T23:34:39.7743627Z 4 LL |         pub(super) fn new() {}
2020-01-10T23:34:39.7744415Z -    |             ^^^^^ there are too many leading `super` keywords.
2020-01-10T23:34:39.7744767Z +    |             ^^^^^ there are too many leading `super` keywords
2020-01-10T23:34:39.7745025Z 7 error: aborting due to previous error
2020-01-10T23:34:39.7745166Z 8 
2020-01-10T23:34:39.7745272Z 
2020-01-10T23:34:39.7745381Z 
2020-01-10T23:34:39.7745381Z 
2020-01-10T23:34:39.7745510Z The actual stderr differed from the expected stderr.
2020-01-10T23:34:39.7745949Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/impl-items-vis-unresolved/impl-items-vis-unresolved.stderr
2020-01-10T23:34:39.7746342Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T23:34:39.7746826Z To only update this specific test, also pass `--test-args resolve/impl-items-vis-unresolved.rs`
2020-01-10T23:34:39.7747174Z error: 1 errors occurred comparing output.
2020-01-10T23:34:39.7747303Z status: exit code: 1
2020-01-10T23:34:39.7747303Z status: exit code: 1
2020-01-10T23:34:39.7748259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/impl-items-vis-unresolved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/impl-items-vis-unresolved" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/impl-items-vis-unresolved/auxiliary" "-A" "unused"
2020-01-10T23:34:39.7748918Z ------------------------------------------
2020-01-10T23:34:39.7749060Z 
2020-01-10T23:34:39.7749369Z ------------------------------------------
2020-01-10T23:34:39.7749551Z stderr:
2020-01-10T23:34:39.7749551Z stderr:
2020-01-10T23:34:39.7749879Z ------------------------------------------
2020-01-10T23:34:39.7750053Z error[E0433]: failed to resolve: there are too many leading `super` keywords
2020-01-10T23:34:39.7751178Z    |
2020-01-10T23:34:39.7751329Z LL |         pub(super) fn new() {}
2020-01-10T23:34:39.7751329Z LL |         pub(super) fn new() {}
2020-01-10T23:34:39.7751478Z    |             ^^^^^ there are too many leading `super` keywords
2020-01-10T23:34:39.7751757Z error: aborting due to previous error
2020-01-10T23:34:39.7752140Z 
2020-01-10T23:34:39.7752619Z For more information about this error, try `rustc --explain E0433`.
2020-01-10T23:34:39.7752798Z 
---
2020-01-10T23:34:39.7766915Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T23:34:39.7767313Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T23:34:39.7782271Z 
2020-01-10T23:34:39.7782685Z 
2020-01-10T23:34:39.7788482Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T23:34:39.7789049Z 
2020-01-10T23:34:39.7789153Z 
2020-01-10T23:34:39.7792751Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T23:34:39.7792972Z Build completed unsuccessfully in 1:02:40
2020-01-10T23:34:39.7792972Z Build completed unsuccessfully in 1:02:40
2020-01-10T23:34:39.7844507Z == clock drift check ==
2020-01-10T23:34:39.7868205Z   local time: Fri Jan 10 23:34:39 UTC 2020
2020-01-10T23:34:40.3289888Z   network time: Fri, 10 Jan 2020 23:34:40 GMT
2020-01-10T23:34:40.3290950Z == end clock drift check ==
2020-01-10T23:34:40.7066770Z 
2020-01-10T23:34:40.7172255Z ##[error]Bash exited with code '1'.
2020-01-10T23:34:40.7223060Z ##[section]Starting: Checkout
2020-01-10T23:34:40.7224936Z ==============================================================================
2020-01-10T23:34:40.7224988Z Task         : Get sources
2020-01-10T23:34:40.7225049Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
