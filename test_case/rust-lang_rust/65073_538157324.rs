plain
2019-10-03T21:26:26.1656000Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T21:26:26.1850642Z ##[command]git config gc.auto 0
2019-10-03T21:26:26.1926691Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T21:26:26.1986513Z ##[command]git config --get-all http.proxy
2019-10-03T21:26:26.2130004Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65073/merge:refs/remotes/pull/65073/merge
---
2019-10-03T22:31:53.5303388Z .................................................................................................... 1500/9099
2019-10-03T22:32:00.9754339Z .................................................................................................... 1600/9099
2019-10-03T22:32:10.8071416Z .................................................................................................... 1700/9099
2019-10-03T22:32:20.8037784Z .......i...............i............................................................................ 1800/9099
2019-10-03T22:32:28.6773684Z ..................................................................................................ii 1900/9099
2019-10-03T22:32:46.3114094Z iii................................................................................................. 2000/9099
2019-10-03T22:32:55.9622921Z .................................................................................................... 2200/9099
2019-10-03T22:32:58.9152954Z .................................................................................................... 2300/9099
2019-10-03T22:33:05.9203276Z .................................................................................................... 2400/9099
2019-10-03T22:33:11.9823967Z .................................................................................................... 2500/9099
---
2019-10-03T22:36:20.0603916Z .....................................................................................i.............. 4700/9099
2019-10-03T22:36:28.6631358Z .i.................................................................................................. 4800/9099
2019-10-03T22:36:39.5355065Z .................................................................................................... 4900/9099
2019-10-03T22:36:45.6792263Z .................................................................................................... 5000/9099
2019-10-03T22:36:58.4832324Z .............................................................................ii.ii.................. 5100/9099
2019-10-03T22:37:08.9227827Z .................................................................................................... 5300/9099
2019-10-03T22:37:19.5635865Z .................................................................................................... 5400/9099
2019-10-03T22:37:27.1604492Z ...........................................i........................................................ 5500/9099
2019-10-03T22:37:34.4531191Z .................................................................................................... 5600/9099
2019-10-03T22:37:34.4531191Z .................................................................................................... 5600/9099
2019-10-03T22:37:46.0917577Z ....................................................F............................................... 5700/9099
2019-10-03T22:37:54.4486513Z ........................................ii...i..ii...........i...................................... 5800/9099
2019-10-03T22:38:22.4886917Z .................................................................................................... 6000/9099
2019-10-03T22:38:32.9178229Z .................................................................................................... 6100/9099
2019-10-03T22:38:32.9178229Z .................................................................................................... 6100/9099
2019-10-03T22:38:48.5745418Z .............................................i..ii.................................................. 6200/9099
2019-10-03T22:39:12.2829514Z .................................................................................................... 6400/9099
2019-10-03T22:39:14.7672565Z .....i.............................................................................................. 6500/9099
2019-10-03T22:39:17.2041220Z .............................................................................i...................... 6600/9099
2019-10-03T22:39:20.3128125Z .................................................................................................... 6700/9099
---
2019-10-03T22:43:48.0087372Z 
2019-10-03T22:43:48.0087914Z ---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
2019-10-03T22:43:48.0087974Z diff of stderr:
2019-10-03T22:43:48.0088008Z 
2019-10-03T22:43:48.0088049Z 1 error: rustc_peek: bit not set
2019-10-03T22:43:48.0088499Z +   --> $DIR/indirect-mutation-offset.rs:34:14
2019-10-03T22:43:48.0088541Z 3    |
2019-10-03T22:43:48.0088541Z 3    |
2019-10-03T22:43:48.0088602Z 4 LL |     unsafe { rustc_peek(x) };
2019-10-03T22:43:48.0088691Z 
2019-10-03T22:43:48.0088716Z 
2019-10-03T22:43:48.0088778Z The actual stderr differed from the expected stderr.
2019-10-03T22:43:48.0089078Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2019-10-03T22:43:48.0089078Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2019-10-03T22:43:48.0089316Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T22:43:48.0089599Z To only update this specific test, also pass `--test-args mir-dataflow/indirect-mutation-offset.rs`
2019-10-03T22:43:48.0089675Z error: 1 errors occurred comparing output.
2019-10-03T22:43:48.0089735Z status: exit code: 1
2019-10-03T22:43:48.0089735Z status: exit code: 1
2019-10-03T22:43:48.0090479Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/auxiliary" "-A" "unused"
2019-10-03T22:43:48.0090799Z ------------------------------------------
2019-10-03T22:43:48.0090830Z 
2019-10-03T22:43:48.0091042Z ------------------------------------------
2019-10-03T22:43:48.0091084Z stderr:
2019-10-03T22:43:48.0091084Z stderr:
2019-10-03T22:43:48.0091274Z ------------------------------------------
2019-10-03T22:43:48.0091335Z error: rustc_peek: bit not set
2019-10-03T22:43:48.0091609Z    |
2019-10-03T22:43:48.0091609Z    |
2019-10-03T22:43:48.0091675Z LL |     unsafe { rustc_peek(x) }; //~ ERROR rustc_peek: bit not set
2019-10-03T22:43:48.0091759Z 
2019-10-03T22:43:48.0091759Z 
2019-10-03T22:43:48.0091800Z error: stop_after_dataflow ended compilation
2019-10-03T22:43:48.0091885Z error: aborting due to 2 previous errors
2019-10-03T22:43:48.0091913Z 
2019-10-03T22:43:48.0091937Z 
2019-10-03T22:43:48.0092294Z ------------------------------------------
---
2019-10-03T22:43:48.0132632Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T22:43:48.0132713Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T22:43:48.0149487Z 
2019-10-03T22:43:48.0150249Z 
2019-10-03T22:43:48.0152094Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T22:43:48.0152612Z 
2019-10-03T22:43:48.0152759Z 
2019-10-03T22:43:48.0158062Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T22:43:48.0158339Z Build completed unsuccessfully in 1:09:48
2019-10-03T22:43:48.0158339Z Build completed unsuccessfully in 1:09:48
2019-10-03T22:43:48.0214439Z == clock drift check ==
2019-10-03T22:43:48.0231342Z   local time: Thu Oct  3 22:43:48 UTC 2019
2019-10-03T22:43:48.1873403Z   network time: Thu, 03 Oct 2019 22:43:48 GMT
2019-10-03T22:43:48.1874101Z == end clock drift check ==
2019-10-03T22:43:49.2844694Z ##[error]Bash exited with code '1'.
2019-10-03T22:43:49.2914803Z ##[section]Starting: Checkout
2019-10-03T22:43:49.2916950Z ==============================================================================
2019-10-03T22:43:49.2917001Z Task         : Get sources
2019-10-03T22:43:49.2917060Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
