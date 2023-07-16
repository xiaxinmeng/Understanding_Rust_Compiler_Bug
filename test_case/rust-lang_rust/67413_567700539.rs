plain
2019-12-19T21:02:42.8052881Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T21:02:42.8069316Z ##[command]git config gc.auto 0
2019-12-19T21:02:42.8073079Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T21:02:42.8075183Z ##[command]git config --get-all http.proxy
2019-12-19T21:02:42.8080548Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67413/merge:refs/remotes/pull/67413/merge
---
2019-12-19T22:00:59.4992912Z .................................................................................................... 1600/9380
2019-12-19T22:01:03.8035408Z .................................................................................................... 1700/9380
2019-12-19T22:01:15.4094108Z ...................................................................i................................ 1800/9380
2019-12-19T22:01:22.4448821Z .................................................................................................... 1900/9380
2019-12-19T22:01:37.5519035Z ....................................................iiiii........................................... 2000/9380
2019-12-19T22:01:47.6092407Z .................................................................................................... 2200/9380
2019-12-19T22:01:49.9814551Z .................................................................................................... 2300/9380
2019-12-19T22:01:53.1608175Z .................................................................................................... 2400/9380
2019-12-19T22:02:15.2157467Z .................................................................................................... 2500/9380
---
2019-12-19T22:04:48.5082216Z .............................................................i...............i...................... 4800/9380
2019-12-19T22:04:55.8170997Z .................................................................................................... 4900/9380
2019-12-19T22:05:04.1578205Z .................................................................................................... 5000/9380
2019-12-19T22:05:09.2801505Z .....i.............................................................................................. 5100/9380
2019-12-19T22:05:19.4219836Z .......................................................................ii.ii...........i............ 5200/9380
2019-12-19T22:05:27.9858171Z .......i............................................................................................ 5400/9380
2019-12-19T22:05:37.7641217Z .................................................................................................... 5500/9380
2019-12-19T22:05:44.2106774Z .....................................................i.............................................. 5600/9380
2019-12-19T22:05:51.0441970Z .................................................................................................... 5700/9380
2019-12-19T22:05:51.0441970Z .................................................................................................... 5700/9380
2019-12-19T22:06:00.8246800Z .................................................................................................... 5800/9380
2019-12-19T22:06:07.7357208Z .........................................ii...i..ii...........i..................................... 5900/9380
2019-12-19T22:06:29.2364340Z .................................................................................................... 6100/9380
2019-12-19T22:06:37.0281611Z .................................................................................................... 6200/9380
2019-12-19T22:06:37.0281611Z .................................................................................................... 6200/9380
2019-12-19T22:06:47.2521145Z ..................................................................i..ii............................. 6300/9380
2019-12-19T22:07:20.0954968Z .................................................................................................... 6500/9380
2019-12-19T22:07:22.2108377Z ......................................i............................................................. 6600/9380
2019-12-19T22:07:24.3776049Z .................................................................................................... 6700/9380
2019-12-19T22:07:26.7170960Z ............................F.i..................................................................... 6800/9380
---
2019-12-19T22:09:01.5263472Z .................................................................................................... 7400/9380
2019-12-19T22:09:06.4108526Z .................................................................................................... 7500/9380
2019-12-19T22:09:12.0259545Z .................................................................................................... 7600/9380
2019-12-19T22:09:21.0934259Z .................................................................................................... 7700/9380
2019-12-19T22:09:29.8000389Z ...................................................iiii............................................. 7800/9380
2019-12-19T22:09:43.7728401Z .................................................................................................... 8000/9380
2019-12-19T22:09:49.8199039Z .................................................................................................... 8100/9380
2019-12-19T22:10:04.8740708Z .................................................................................................... 8200/9380
2019-12-19T22:10:12.4925708Z .................................................................................................... 8300/9380
---
2019-12-19T22:12:07.0109776Z 
2019-12-19T22:12:07.0110562Z ---- [ui] ui/parser/mismatched-braces/missing-close-brace-in-trait.rs stdout ----
2019-12-19T22:12:07.0110884Z diff of stderr:
2019-12-19T22:12:07.0111136Z 
2019-12-19T22:12:07.0111365Z 13 LL | pub(crate) struct Bar<T>();
2019-12-19T22:12:07.0111611Z 14    |            ^^^^^^ expected one of `async`, `const`, `extern`, `fn`, `type`, or `unsafe`
2019-12-19T22:12:07.0112748Z - error: aborting due to 2 previous errors
2019-12-19T22:12:07.0113059Z + error[E0601]: `main` function not found in crate `missing_close_brace_in_trait`
2019-12-19T22:12:07.0113565Z +   --> $DIR/missing-close-brace-in-trait.rs:1:1
2019-12-19T22:12:07.0113836Z +    |
2019-12-19T22:12:07.0113836Z +    |
2019-12-19T22:12:07.0114074Z + LL | / trait T {
2019-12-19T22:12:07.0114296Z + LL | |     fn foo(&self);
2019-12-19T22:12:07.0114513Z + LL | |
2019-12-19T22:12:07.0114747Z + LL | | pub(crate) struct Bar<T>();
2019-12-19T22:12:07.0114966Z + ...  |
2019-12-19T22:12:07.0115177Z + LL | |
2019-12-19T22:12:07.0120464Z + LL | | fn main() {}
2019-12-19T22:12:07.0121400Z +    | |_________________________________________________________________^ consider adding a `main` function to `$DIR/missing-close-brace-in-trait.rs`
2019-12-19T22:12:07.0124957Z + error: aborting due to 3 previous errors
2019-12-19T22:12:07.0125460Z + 
2019-12-19T22:12:07.0126113Z + For more information about this error, try `rustc --explain E0601`.
2019-12-19T22:12:07.0126400Z 18 
2019-12-19T22:12:07.0126400Z 18 
2019-12-19T22:12:07.0126602Z 
2019-12-19T22:12:07.0126801Z 
2019-12-19T22:12:07.0127076Z The actual stderr differed from the expected stderr.
2019-12-19T22:12:07.0127667Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-trait/missing-close-brace-in-trait.stderr
2019-12-19T22:12:07.0128218Z To update references, rerun the tests and pass the `--bless` flag
2019-12-19T22:12:07.0128844Z To only update this specific test, also pass `--test-args parser/mismatched-braces/missing-close-brace-in-trait.rs`
2019-12-19T22:12:07.0129371Z error: 1 errors occurred comparing output.
2019-12-19T22:12:07.0129604Z status: exit code: 1
2019-12-19T22:12:07.0129604Z status: exit code: 1
2019-12-19T22:12:07.0130790Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-trait/auxiliary" "-A" "unused"
2019-12-19T22:12:07.0133502Z ------------------------------------------
2019-12-19T22:12:07.0133739Z 
2019-12-19T22:12:07.0134209Z ------------------------------------------
2019-12-19T22:12:07.0134397Z stderr:
2019-12-19T22:12:07.0134397Z stderr:
2019-12-19T22:12:07.0134797Z ------------------------------------------
2019-12-19T22:12:07.0136398Z error: this file contains an un-closed delimiter
2019-12-19T22:12:07.0137468Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs:11:66
2019-12-19T22:12:07.0138142Z    |
2019-12-19T22:12:07.0138342Z LL | trait T {
2019-12-19T22:12:07.0139114Z    |         - un-closed delimiter
2019-12-19T22:12:07.0140431Z LL | fn main() {} //~ ERROR this file contains an un-closed delimiter
2019-12-19T22:12:07.0140630Z    |                                                                  ^
2019-12-19T22:12:07.0141220Z 
2019-12-19T22:12:07.0141220Z 
2019-12-19T22:12:07.0141440Z error: expected one of `async`, `const`, `extern`, `fn`, `type`, or `unsafe`, found keyword `struct`
2019-12-19T22:12:07.0142555Z    |
2019-12-19T22:12:07.0142555Z    |
2019-12-19T22:12:07.0143097Z LL | pub(crate) struct Bar<T>();
2019-12-19T22:12:07.0143314Z    |            ^^^^^^ expected one of `async`, `const`, `extern`, `fn`, `type`, or `unsafe`
2019-12-19T22:12:07.0143623Z error[E0601]: `main` function not found in crate `missing_close_brace_in_trait`
2019-12-19T22:12:07.0144440Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs:1:1
2019-12-19T22:12:07.0144706Z    |
2019-12-19T22:12:07.0145571Z LL | / trait T {
2019-12-19T22:12:07.0145571Z LL | / trait T {
2019-12-19T22:12:07.0145786Z LL | |     fn foo(&self);
2019-12-19T22:12:07.0145960Z LL | |
2019-12-19T22:12:07.0146102Z LL | | pub(crate) struct Bar<T>();
2019-12-19T22:12:07.0146397Z LL | |
2019-12-19T22:12:07.0147343Z LL | | fn main() {} //~ ERROR this file contains an un-closed delimiter
2019-12-19T22:12:07.0147949Z    | |_________________________________________________________________^ consider adding a `main` function to `/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs`
2019-12-19T22:12:07.0148555Z 
---
2019-12-19T22:12:07.0152069Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-19T22:12:07.0152267Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-19T22:12:07.0166014Z 
2019-12-19T22:12:07.0204786Z 
2019-12-19T22:12:07.0207473Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-19T22:12:07.0208399Z 
2019-12-19T22:12:07.0208649Z 
2019-12-19T22:12:07.0208949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-19T22:12:07.0209215Z Build completed unsuccessfully in 1:03:23
2019-12-19T22:12:07.0209215Z Build completed unsuccessfully in 1:03:23
2019-12-19T22:12:07.0227236Z == clock drift check ==
2019-12-19T22:12:07.0245873Z   local time: Thu Dec 19 22:12:07 UTC 2019
2019-12-19T22:12:07.3124773Z   network time: Thu, 19 Dec 2019 22:12:07 GMT
2019-12-19T22:12:07.3135559Z == end clock drift check ==
2019-12-19T22:12:08.0675807Z 
2019-12-19T22:12:08.0793146Z ##[error]Bash exited with code '1'.
2019-12-19T22:12:08.0827979Z ##[section]Starting: Checkout
2019-12-19T22:12:08.0829747Z ==============================================================================
2019-12-19T22:12:08.0829805Z Task         : Get sources
2019-12-19T22:12:08.0829855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
