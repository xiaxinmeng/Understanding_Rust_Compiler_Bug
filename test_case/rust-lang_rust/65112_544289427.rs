plain
2019-10-20T19:10:55.3671330Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T19:10:55.3897497Z ##[command]git config gc.auto 0
2019-10-20T19:10:55.3979086Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T19:10:55.4035267Z ##[command]git config --get-all http.proxy
2019-10-20T19:10:55.4208179Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65112/merge:refs/remotes/pull/65112/merge
---
2019-10-20T20:16:32.5289877Z .................................................................................................... 1600/9203
2019-10-20T20:16:38.2069027Z .................................................................................................... 1700/9203
2019-10-20T20:16:51.9348008Z ................................i...............i................................................... 1800/9203
2019-10-20T20:16:59.7967996Z .................................................................................................... 1900/9203
2019-10-20T20:17:14.6893791Z ......................iiiii......................................................................... 2000/9203
2019-10-20T20:17:25.7306986Z .................................................................................................... 2200/9203
2019-10-20T20:17:28.4653305Z .................................................................................................... 2300/9203
2019-10-20T20:17:33.8142291Z .................................................................................................... 2400/9203
2019-10-20T20:17:57.4783840Z .................................................................................................... 2500/9203
---
2019-10-20T20:21:01.1980998Z .........................i...............i.......................................................... 4800/9203
2019-10-20T20:21:13.7542786Z .................................................................................................... 4900/9203
2019-10-20T20:21:20.2892997Z .................................................................................................... 5000/9203
2019-10-20T20:21:29.9470442Z .................................................................................................... 5100/9203
2019-10-20T20:21:37.7471665Z .........................ii.ii...................................................................... 5200/9203
2019-10-20T20:21:48.2607725Z .................................................................................................... 5400/9203
2019-10-20T20:21:59.1547752Z ...........................................................................................i........ 5500/9203
2019-10-20T20:22:07.7161902Z .................................................................................................... 5600/9203
2019-10-20T20:22:12.7810188Z .................................................................................................... 5700/9203
2019-10-20T20:22:12.7810188Z .................................................................................................... 5700/9203
2019-10-20T20:22:24.0179592Z ........................................................................................ii...i..ii.. 5800/9203
2019-10-20T20:22:52.0067060Z .................................................................................................... 6000/9203
2019-10-20T20:23:01.9731715Z .................................................................................................... 6100/9203
2019-10-20T20:23:07.9464094Z .................................................................................................... 6200/9203
2019-10-20T20:23:07.9464094Z .................................................................................................... 6200/9203
2019-10-20T20:23:22.4495322Z ..........i..ii..................................................................................... 6300/9203
2019-10-20T20:23:43.2814212Z ......................................................................i............................. 6500/9203
2019-10-20T20:23:45.6092138Z .................................................................................................... 6600/9203
2019-10-20T20:23:48.1471781Z .............................................i...................................................... 6700/9203
2019-10-20T20:23:51.9519058Z .................................................................................................... 6800/9203
---
2019-10-20T20:28:08.5469738Z 
2019-10-20T20:28:08.5470004Z 21 error: lifetime parameter `'b` never used
2019-10-20T20:28:08.5470242Z 22   --> $DIR/zero-uses-in-fn.rs:18:17
2019-10-20T20:28:08.5470311Z 23    |
2019-10-20T20:28:08.5470555Z - LL | fn november<'a, 'b>(s: &'a str) -> (&'a str) {
2019-10-20T20:28:08.5470799Z + LL | fn november<'a, 'b>(s: &'a str) -> &'a str {
2019-10-20T20:28:08.5471041Z 25    |               --^^
2019-10-20T20:28:08.5471139Z 27    |               help: elide the unused lifetime
2019-10-20T20:28:08.5471170Z 
2019-10-20T20:28:08.5471458Z 
2019-10-20T20:28:08.5471609Z The actual stderr differed from the expected stderr.
2019-10-20T20:28:08.5471609Z The actual stderr differed from the expected stderr.
2019-10-20T20:28:08.5471993Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/zero-uses-in-fn/zero-uses-in-fn.stderr
2019-10-20T20:28:08.5472075Z diff of fixed:
2019-10-20T20:28:08.5472107Z 
2019-10-20T20:28:08.5472147Z 15     s
2019-10-20T20:28:08.5472187Z 16 }
2019-10-20T20:28:08.5472247Z 17 
2019-10-20T20:28:08.5472487Z - fn november<'a>(s: &'a str) -> (&'a str) {
2019-10-20T20:28:08.5472723Z + fn november<'a>(s: &'a str) -> &'a str {
2019-10-20T20:28:08.5472988Z 19     //~^ ERROR lifetime parameter `'b` never used
2019-10-20T20:28:08.5473203Z 20     //~| HELP elide the unused lifetime
2019-10-20T20:28:08.5473270Z 
2019-10-20T20:28:08.5473313Z 
2019-10-20T20:28:08.5473356Z The actual fixed differed from the expected fixed.
2019-10-20T20:28:08.5473356Z The actual fixed differed from the expected fixed.
2019-10-20T20:28:08.5473680Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/zero-uses-in-fn/zero-uses-in-fn.fixed
2019-10-20T20:28:08.5473965Z To update references, rerun the tests and pass the `--bless` flag
2019-10-20T20:28:08.5474277Z To only update this specific test, also pass `--test-args single-use-lifetime/zero-uses-in-fn.rs`
2019-10-20T20:28:08.5474364Z error: 2 errors occurred comparing output.
2019-10-20T20:28:08.5474429Z status: exit code: 1
2019-10-20T20:28:08.5474429Z status: exit code: 1
2019-10-20T20:28:08.5475213Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/zero-uses-in-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/zero-uses-in-fn/auxiliary" "-A" "unused"
2019-10-20T20:28:08.5475583Z ------------------------------------------
2019-10-20T20:28:08.5475618Z 
2019-10-20T20:28:08.5476051Z ------------------------------------------
2019-10-20T20:28:08.5476100Z stderr:
2019-10-20T20:28:08.5476100Z stderr:
2019-10-20T20:28:08.5476327Z ------------------------------------------
2019-10-20T20:28:08.5476574Z error: lifetime parameter `'a` never used
2019-10-20T20:28:08.5476841Z   --> /checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.rs:8:14
2019-10-20T20:28:08.5476895Z    |
2019-10-20T20:28:08.5477128Z LL | fn september<'a>() {}
2019-10-20T20:28:08.5477662Z    |             -^^- help: elide the unused lifetime
2019-10-20T20:28:08.5477762Z note: lint level defined here
2019-10-20T20:28:08.5478047Z   --> /checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.rs:5:9
2019-10-20T20:28:08.5478098Z    |
2019-10-20T20:28:08.5478140Z LL | #![deny(unused_lifetimes)]
2019-10-20T20:28:08.5478140Z LL | #![deny(unused_lifetimes)]
2019-10-20T20:28:08.5478204Z    |         ^^^^^^^^^^^^^^^^
2019-10-20T20:28:08.5478253Z 
2019-10-20T20:28:08.5478487Z error: lifetime parameter `'a` never used
2019-10-20T20:28:08.5478751Z   --> /checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.rs:12:12
2019-10-20T20:28:08.5478821Z    |
2019-10-20T20:28:08.5479056Z LL | fn october<'a, 'b, T>(s: &'b T) -> &'b T {
2019-10-20T20:28:08.5479268Z    |            ^^--
2019-10-20T20:28:08.5479381Z    |            help: elide the unused lifetime
2019-10-20T20:28:08.5479412Z 
2019-10-20T20:28:08.5479641Z error: lifetime parameter `'b` never used
2019-10-20T20:28:08.5479925Z   --> /checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.rs:18:17
2019-10-20T20:28:08.5479925Z   --> /checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.rs:18:17
2019-10-20T20:28:08.5479974Z    |
2019-10-20T20:28:08.5480209Z LL | fn november<'a, 'b>(s: &'a str) -> &'a str {
2019-10-20T20:28:08.5480445Z    |               --^^
2019-10-20T20:28:08.5480539Z    |               help: elide the unused lifetime
2019-10-20T20:28:08.5480882Z 
2019-10-20T20:28:08.5481014Z error: aborting due to 3 previous errors
2019-10-20T20:28:08.5481048Z 
---
2019-10-20T20:28:08.5512048Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-20T20:28:08.5512139Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-20T20:28:08.5536472Z 
2019-10-20T20:28:08.5536600Z 
2019-10-20T20:28:08.5539037Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-20T20:28:08.5539383Z 
2019-10-20T20:28:08.5539416Z 
2019-10-20T20:28:08.5550265Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-20T20:28:08.5550363Z Build completed unsuccessfully in 1:10:02
2019-10-20T20:28:08.5550363Z Build completed unsuccessfully in 1:10:02
2019-10-20T20:28:08.5602954Z == clock drift check ==
2019-10-20T20:28:08.5618053Z   local time: Sun Oct 20 20:28:08 UTC 2019
2019-10-20T20:28:08.7124316Z   network time: Sun, 20 Oct 2019 20:28:08 GMT
2019-10-20T20:28:08.7124629Z == end clock drift check ==
2019-10-20T20:28:09.9774767Z 
2019-10-20T20:28:09.9881942Z ##[error]Bash exited with code '1'.
2019-10-20T20:28:09.9926414Z ##[section]Starting: Checkout
2019-10-20T20:28:09.9928905Z ==============================================================================
2019-10-20T20:28:09.9929000Z Task         : Get sources
2019-10-20T20:28:09.9929047Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
