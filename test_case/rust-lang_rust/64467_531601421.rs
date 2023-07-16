plain
2019-09-15T20:18:32.6562600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-15T20:18:32.6765789Z ##[command]git config gc.auto 0
2019-09-15T20:18:32.6858876Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-15T20:18:32.6931235Z ##[command]git config --get-all http.proxy
2019-09-15T20:18:32.7085569Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64467/merge:refs/remotes/pull/64467/merge
---
2019-09-15T21:22:49.4259938Z .................................................................................................... 1500/9020
2019-09-15T21:22:55.6460107Z .................................................................................................... 1600/9020
2019-09-15T21:23:08.8186339Z ..............................................................i...............i..................... 1700/9020
2019-09-15T21:23:16.2692686Z .................................................................................................... 1800/9020
2019-09-15T21:23:32.2088255Z .....................................................iiiii.......................................... 1900/9020
2019-09-15T21:23:44.1694212Z .................................................................................................... 2100/9020
2019-09-15T21:23:46.8451346Z .................................................................................................... 2200/9020
2019-09-15T21:23:50.4659162Z .................................................................................................... 2300/9020
2019-09-15T21:23:59.1230067Z .................................................................................................... 2400/9020
---
2019-09-15T21:27:04.8516956Z .........................................i...............i.......................................... 4700/9020
2019-09-15T21:27:16.5129691Z .................................................................................................... 4800/9020
2019-09-15T21:27:23.4319454Z .................................................................................................... 4900/9020
2019-09-15T21:27:33.4564358Z .................................................................................................... 5000/9020
2019-09-15T21:27:41.2165960Z .........................ii.ii...................................................................... 5100/9020
2019-09-15T21:27:52.0283341Z .................................................................................................... 5300/9020
2019-09-15T21:28:02.4488103Z .........................................................................................i.......... 5400/9020
2019-09-15T21:28:10.7688274Z .................................................................................................... 5500/9020
2019-09-15T21:28:16.0850697Z .................................................................................................... 5600/9020
2019-09-15T21:28:16.0850697Z .................................................................................................... 5600/9020
2019-09-15T21:28:27.1028422Z ....................................................................................ii...i..ii...... 5700/9020
2019-09-15T21:28:53.9640993Z .................................................................................................... 5900/9020
2019-09-15T21:29:04.5746540Z .................................................................................................... 6000/9020
2019-09-15T21:29:04.5746540Z .................................................................................................... 6000/9020
2019-09-15T21:29:10.8587824Z ......................................................................................i..ii......... 6100/9020
2019-09-15T21:29:41.2922759Z .................................................................................................... 6300/9020
2019-09-15T21:29:44.4671236Z .............................................i...................................................... 6400/9020
2019-09-15T21:29:46.7278347Z .................................................................................................... 6500/9020
2019-09-15T21:29:49.4009134Z .................i.................................................................................. 6600/9020
---
2019-09-15T21:34:03.8920394Z 
2019-09-15T21:34:03.8921250Z ---- [ui] ui/conditional-compilation/cfg-arg-invalid-6.rs stdout ----
2019-09-15T21:34:03.8921500Z diff of stderr:
2019-09-15T21:34:03.8921634Z 
2019-09-15T21:34:03.8922062Z - error: invalid `--cfg` argument: `a{` (argument value must be a string)
2019-09-15T21:34:03.8922779Z + error: invalid `--cfg` argument: `a{` (expected `key` or `key="value"`)
2019-09-15T21:34:03.8923092Z 3 
2019-09-15T21:34:03.8923203Z 
2019-09-15T21:34:03.8923306Z 
2019-09-15T21:34:03.8923434Z The actual stderr differed from the expected stderr.
2019-09-15T21:34:03.8923434Z The actual stderr differed from the expected stderr.
2019-09-15T21:34:03.8923885Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-arg-invalid-6/cfg-arg-invalid-6.stderr
2019-09-15T21:34:03.8924274Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T21:34:03.8924695Z To only update this specific test, also pass `--test-args conditional-compilation/cfg-arg-invalid-6.rs`
2019-09-15T21:34:03.8924989Z error: 1 errors occurred comparing output.
2019-09-15T21:34:03.8925112Z status: exit code: 1
2019-09-15T21:34:03.8925112Z status: exit code: 1
2019-09-15T21:34:03.8925971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-arg-invalid-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-arg-invalid-6" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "a{" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-arg-invalid-6/auxiliary" "-A" "unused"
2019-09-15T21:34:03.8926553Z ------------------------------------------
2019-09-15T21:34:03.8926716Z 
2019-09-15T21:34:03.8927068Z ------------------------------------------
2019-09-15T21:34:03.8927232Z stderr:
2019-09-15T21:34:03.8927232Z stderr:
2019-09-15T21:34:03.8927547Z ------------------------------------------
2019-09-15T21:34:03.8927944Z error: invalid `--cfg` argument: `a{` (expected `key` or `key="value"`)
2019-09-15T21:34:03.8928205Z 
2019-09-15T21:34:03.8928593Z ------------------------------------------
2019-09-15T21:34:03.8929512Z 
2019-09-15T21:34:03.8929736Z 
---
2019-09-15T21:34:03.8964081Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-15T21:34:03.8964356Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-15T21:34:03.8979445Z 
2019-09-15T21:34:03.8979526Z 
2019-09-15T21:34:03.8989092Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-15T21:34:03.8996656Z 
2019-09-15T21:34:03.8996707Z 
2019-09-15T21:34:03.9003134Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-15T21:34:03.9003427Z Build completed unsuccessfully in 1:08:38
2019-09-15T21:34:03.9003427Z Build completed unsuccessfully in 1:08:38
2019-09-15T21:34:03.9065427Z == clock drift check ==
2019-09-15T21:34:03.9090693Z   local time: Sun Sep 15 21:34:03 UTC 2019
2019-09-15T21:34:03.9954307Z   network time: Sun, 15 Sep 2019 21:34:03 GMT
2019-09-15T21:34:03.9957444Z == end clock drift check ==
2019-09-15T21:34:04.7877779Z ##[error]Bash exited with code '1'.
2019-09-15T21:34:04.7919065Z ##[section]Starting: Checkout
2019-09-15T21:34:04.7921629Z ==============================================================================
2019-09-15T21:34:04.7921712Z Task         : Get sources
2019-09-15T21:34:04.7921765Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
