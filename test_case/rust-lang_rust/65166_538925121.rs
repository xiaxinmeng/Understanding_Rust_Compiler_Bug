plain
2019-10-07T08:38:27.6728705Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T08:38:27.6908979Z ##[command]git config gc.auto 0
2019-10-07T08:38:28.2052449Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T08:38:28.2059012Z ##[command]git config --get-all http.proxy
2019-10-07T08:38:28.2064616Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65166/merge:refs/remotes/pull/65166/merge
---
2019-10-07T09:39:32.0243310Z .................................................................................................... 1600/9118
2019-10-07T09:39:40.0083400Z .................................................................................................... 1700/9118
2019-10-07T09:39:50.6142752Z ...............i...............i.................................................................... 1800/9118
2019-10-07T09:39:57.6914177Z .................................................................................................... 1900/9118
2019-10-07T09:40:12.7817228Z ......iiiii......................................................................................... 2000/9118
2019-10-07T09:40:22.4265639Z .................................................................................................... 2200/9118
2019-10-07T09:40:24.9124335Z .................................................................................................... 2300/9118
2019-10-07T09:40:30.5973880Z .................................................................................................... 2400/9118
2019-10-07T09:40:36.6390402Z .................................................................................................... 2500/9118
---
2019-10-07T09:43:26.1702191Z ...............................................................................................i.... 4700/9118
2019-10-07T09:43:33.0803620Z ...........i........................................................................................ 4800/9118
2019-10-07T09:43:43.9826470Z .................................................................................................... 4900/9118
2019-10-07T09:43:49.5270378Z .................................................................................................... 5000/9118
2019-10-07T09:44:01.0773501Z .........................................................................................ii.ii...... 5100/9118
2019-10-07T09:44:10.6268193Z .................................................................................................... 5300/9118
2019-10-07T09:44:19.8943032Z .................................................................................................... 5400/9118
2019-10-07T09:44:26.7050357Z .......................................................i............................................ 5500/9118
2019-10-07T09:44:33.3991741Z .................................................................................................... 5600/9118
2019-10-07T09:44:33.3991741Z .................................................................................................... 5600/9118
2019-10-07T09:44:41.5324755Z .................................................................................................... 5700/9118
2019-10-07T09:44:50.6768832Z ....................................................ii...i..ii...........i.......................... 5800/9118
2019-10-07T09:45:15.5933484Z .................................................................................................... 6000/9118
2019-10-07T09:45:21.5561129Z .................................................................................................... 6100/9118
2019-10-07T09:45:21.5561129Z .................................................................................................... 6100/9118
2019-10-07T09:45:28.4540151Z ..........................................................i..ii..................................... 6200/9118
2019-10-07T09:45:55.2204283Z .................................................................................................... 6400/9118
2019-10-07T09:45:57.3456025Z ..................i................................................................................. 6500/9118
2019-10-07T09:45:59.5305245Z ...........................................................................................i........ 6600/9118
2019-10-07T09:46:02.1620324Z .................................................................................................... 6700/9118
---
2019-10-07T09:50:04.5364260Z ---- [ui] ui/async-await/async-borrowck-escaping-block-error.rs stdout ----
2019-10-07T09:50:04.5364301Z 
2019-10-07T09:50:04.5364377Z error: fixed code is still producing diagnostics
2019-10-07T09:50:04.5364614Z status: exit code: 0
2019-10-07T09:50:04.5365409Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/auxiliary"
2019-10-07T09:50:04.5365962Z ------------------------------------------
2019-10-07T09:50:04.5366014Z 
2019-10-07T09:50:04.5366204Z ------------------------------------------
2019-10-07T09:50:04.5366244Z stderr:
2019-10-07T09:50:04.5366244Z stderr:
2019-10-07T09:50:04.5366444Z ------------------------------------------
2019-10-07T09:50:04.5366486Z warning: function is never used: `foo`
2019-10-07T09:50:04.5366709Z   --> /checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.fixed:5:1
2019-10-07T09:50:04.5366772Z    |
2019-10-07T09:50:04.5366977Z LL | fn foo() -> Box<impl std::future::Future<Output = u32>> {
2019-10-07T09:50:04.5367081Z    |
2019-10-07T09:50:04.5367230Z    = note: `#[warn(dead_code)]` on by default
2019-10-07T09:50:04.5367271Z 
2019-10-07T09:50:04.5367317Z warning: the feature `async_await` has been stable since 1.39.0 and no longer requires an attribute to enable
---
2019-10-07T09:50:04.5406119Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-07T09:50:04.5406199Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-07T09:50:04.5424374Z 
2019-10-07T09:50:04.5424488Z 
2019-10-07T09:50:04.5425930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-07T09:50:04.5426174Z 
2019-10-07T09:50:04.5426200Z 
2019-10-07T09:50:04.5438090Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-07T09:50:04.5438193Z Build completed unsuccessfully in 1:04:34
2019-10-07T09:50:04.5438193Z Build completed unsuccessfully in 1:04:34
2019-10-07T09:50:04.5494988Z == clock drift check ==
2019-10-07T09:50:04.5533037Z   local time: Mon Oct  7 09:50:04 UTC 2019
2019-10-07T09:50:04.8307318Z   network time: Mon, 07 Oct 2019 09:50:04 GMT
2019-10-07T09:50:04.8307577Z == end clock drift check ==
2019-10-07T09:50:06.0143602Z ##[error]Bash exited with code '1'.
2019-10-07T09:50:06.0193806Z ##[section]Starting: Checkout
2019-10-07T09:50:06.0195845Z ==============================================================================
2019-10-07T09:50:06.0195896Z Task         : Get sources
2019-10-07T09:50:06.0195943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
