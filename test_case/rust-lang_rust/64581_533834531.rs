plain
2019-09-21T21:10:26.1095528Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T21:10:26.1369756Z ##[command]git config gc.auto 0
2019-09-21T21:10:26.1455718Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T21:10:26.1520923Z ##[command]git config --get-all http.proxy
2019-09-21T21:10:26.1683129Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64581/merge:refs/remotes/pull/64581/merge
---
2019-09-21T22:16:16.2613072Z .................................................................................................... 1500/9031
2019-09-21T22:16:22.7173045Z .................................................................................................... 1600/9031
2019-09-21T22:16:36.3064841Z .....................................................................i...............i.............. 1700/9031
2019-09-21T22:16:43.4106129Z .................................................................................................... 1800/9031
2019-09-21T22:16:59.8175380Z ............................................................iiiii................................... 1900/9031
2019-09-21T22:17:12.8124104Z .................................................................................................... 2100/9031
2019-09-21T22:17:15.5022880Z .................................................................................................... 2200/9031
2019-09-21T22:17:18.9958374Z .................................................................................................... 2300/9031
2019-09-21T22:17:28.1660805Z .................................................................................................... 2400/9031
---
2019-09-21T22:20:38.1573403Z ................................................i...............i................................... 4700/9031
2019-09-21T22:20:48.1600262Z .................................................................................................... 4800/9031
2019-09-21T22:20:56.5874986Z .................................................................................................... 4900/9031
2019-09-21T22:21:06.5924869Z .................................................................................................... 5000/9031
2019-09-21T22:21:15.1742911Z ..................................ii.ii............................................................. 5100/9031
2019-09-21T22:21:25.4309974Z .................................................................................................... 5300/9031
2019-09-21T22:21:36.7792837Z ..................................................................................................i. 5400/9031
2019-09-21T22:21:45.7302514Z .................................................................................................... 5500/9031
2019-09-21T22:21:51.0280420Z .................................................................................................... 5600/9031
2019-09-21T22:21:51.0280420Z .................................................................................................... 5600/9031
2019-09-21T22:22:02.6907776Z .............................................................................................ii...i. 5700/9031
2019-09-21T22:22:17.6184619Z .ii...........i..................................................................................... 5800/9031
2019-09-21T22:22:40.2699310Z .................................................................................................... 6000/9031
2019-09-21T22:22:49.9224873Z ...............................................................................................i..ii 6100/9031
2019-09-21T22:23:06.6251873Z .................................................................................................... 6200/9031
2019-09-21T22:23:21.0360441Z .................................................................................................... 6300/9031
---
2019-09-21T22:27:53.1833534Z +    |
2019-09-21T22:27:53.1833595Z + note: any code following this expression is unreachable
2019-09-21T22:27:53.1833842Z +   --> $DIR/try-block-unreachable-code-lint.rs:52:13
2019-09-21T22:27:53.1833887Z +    |
2019-09-21T22:27:53.1833943Z + LL |         Err(return)
2019-09-21T22:27:53.1834023Z 22 
2019-09-21T22:27:53.1834082Z 23 warning: unreachable expression
2019-09-21T22:27:53.1834324Z 24   --> $DIR/try-block-unreachable-code-lint.rs:63:9
2019-09-21T22:27:53.1834357Z 
---
2019-09-21T22:27:53.1835182Z 29 
2019-09-21T22:27:53.1835208Z 
2019-09-21T22:27:53.1835233Z 
2019-09-21T22:27:53.1835276Z The actual stderr differed from the expected stderr.
2019-09-21T22:27:53.1835641Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-unreachable-code-lint/try-block-unreachable-code-lint.stderr
2019-09-21T22:27:53.1835909Z To update references, rerun the tests and pass the `--bless` flag
2019-09-21T22:27:53.1836347Z To only update this specific test, also pass `--test-args try-block/try-block-unreachable-code-lint.rs`
2019-09-21T22:27:53.1836449Z error: 1 errors occurred comparing output.
2019-09-21T22:27:53.1836494Z status: exit code: 0
2019-09-21T22:27:53.1836494Z status: exit code: 0
2019-09-21T22:27:53.1838030Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-unreachable-code-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-unreachable-code-lint" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-unreachable-code-lint/auxiliary" "-A" "unused"
2019-09-21T22:27:53.1838460Z ------------------------------------------
2019-09-21T22:27:53.1838495Z 
2019-09-21T22:27:53.1838747Z ------------------------------------------
2019-09-21T22:27:53.1838812Z stderr:
---
2019-09-21T22:27:53.1879055Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-21T22:27:53.1879147Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T22:27:53.1899122Z 
2019-09-21T22:27:53.1900151Z 
2019-09-21T22:27:53.1901947Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T22:27:53.1902243Z 
2019-09-21T22:27:53.1902290Z 
2019-09-21T22:27:53.1913273Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-21T22:27:53.1913348Z Build completed unsuccessfully in 1:10:03
2019-09-21T22:27:53.1913348Z Build completed unsuccessfully in 1:10:03
2019-09-21T22:27:53.1975630Z == clock drift check ==
2019-09-21T22:27:53.2010091Z   local time: Sat Sep 21 22:27:53 UTC 2019
2019-09-21T22:27:53.2876291Z   network time: Sat, 21 Sep 2019 22:27:53 GMT
2019-09-21T22:27:53.2876477Z == end clock drift check ==
2019-09-21T22:27:54.0060680Z ##[error]Bash exited with code '1'.
2019-09-21T22:27:54.0105061Z ##[section]Starting: Checkout
2019-09-21T22:27:54.0107037Z ==============================================================================
2019-09-21T22:27:54.0107113Z Task         : Get sources
2019-09-21T22:27:54.0107162Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
