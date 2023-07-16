plain
2019-12-23T01:14:20.5727401Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T01:14:21.5377305Z ##[command]git config gc.auto 0
2019-12-23T01:14:21.5380738Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T01:14:21.5383414Z ##[command]git config --get-all http.proxy
2019-12-23T01:14:21.5386450Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67549/merge:refs/remotes/pull/67549/merge
---
2019-12-23T02:15:42.2658959Z .................................................................................................... 1600/9428
2019-12-23T02:15:47.1956523Z .................................................................................................... 1700/9428
2019-12-23T02:15:58.2385497Z .....................................................................................i.............. 1800/9428
2019-12-23T02:16:06.2097338Z .................................................................................................... 1900/9428
2019-12-23T02:16:13.6945568Z ......................................................................ii.iii........................ 2000/9428
2019-12-23T02:16:35.2950346Z .................................................................................................... 2200/9428
2019-12-23T02:16:37.8493620Z .................................................................................................... 2300/9428
2019-12-23T02:16:40.6895369Z .................................................................................................... 2400/9428
2019-12-23T02:16:54.2252161Z .................................................................................................... 2500/9428
---
2019-12-23T02:19:53.0752422Z .i...............i.................................................................................. 4900/9428
2019-12-23T02:20:03.6718415Z .................................................................................................... 5000/9428
2019-12-23T02:20:08.7309752Z .............................................i...................................................... 5100/9428
2019-12-23T02:20:19.0609086Z .................................................................................................... 5200/9428
2019-12-23T02:20:25.2071704Z ............ii.ii...........i....................................................................... 5300/9428
2019-12-23T02:20:34.9131966Z .................................................................................................... 5500/9428
2019-12-23T02:20:46.6342711Z ..............................................................................................i..... 5600/9428
2019-12-23T02:20:55.1266351Z .................................................................................................... 5700/9428
2019-12-23T02:21:00.5727915Z .................................................................................................... 5800/9428
2019-12-23T02:21:00.5727915Z .................................................................................................... 5800/9428
2019-12-23T02:21:10.7005114Z ..................................................................................ii...i..ii........ 5900/9428
2019-12-23T02:21:34.2639819Z .................................................................................................... 6100/9428
2019-12-23T02:21:42.3968591Z .................................................................................................... 6200/9428
2019-12-23T02:21:50.7767564Z .................................................................................................... 6300/9428
2019-12-23T02:21:50.7767564Z .................................................................................................... 6300/9428
2019-12-23T02:22:08.3920213Z .........i..ii...................................................................................... 6400/9428
2019-12-23T02:22:17.3462789Z .............................................FF.F................................................... 6500/9428
2019-12-23T02:22:30.4425501Z .................................................................................................... 6700/9428
2019-12-23T02:22:32.7662509Z ......................................................................................i............. 6800/9428
2019-12-23T02:22:35.5361389Z .................................................................................................... 6900/9428
2019-12-23T02:22:40.0215899Z .................................................................................................... 7000/9428
2019-12-23T02:22:40.0215899Z .................................................................................................... 7000/9428
2019-12-23T02:23:12.8891305Z .................................................................................................... 7100/9428
2019-12-23T02:23:59.8174535Z ...........................................i.......i................................................ 7200/9428
2019-12-23T02:24:06.4001614Z .................................................................................................... 7300/9428
2019-12-23T02:24:11.8119587Z .................................................................................................... 7400/9428
2019-12-23T02:24:16.8897996Z .................................................................................................... 7500/9428
2019-12-23T02:24:22.1785798Z .................................................................................................... 7600/9428
2019-12-23T02:24:29.3350116Z .................................................................................................... 7700/9428
2019-12-23T02:24:40.4715662Z ...................................................................................................i 7800/9428
2019-12-23T02:24:47.3940044Z iii................................................................................................. 7900/9428
2019-12-23T02:25:02.2715810Z .................................................................................................... 8100/9428
2019-12-23T02:25:14.3260772Z .................................................................................................... 8200/9428
2019-12-23T02:25:26.6336002Z .................................................................................................... 8300/9428
2019-12-23T02:25:32.5899948Z .................................................................................................... 8400/9428
---
2019-12-23T02:27:29.6063136Z ---- [ui] ui/or-patterns/exhaustiveness-pass.rs stdout ----
2019-12-23T02:27:29.6063745Z 
2019-12-23T02:27:29.6063910Z error: ui test compiled successfully!
2019-12-23T02:27:29.6064082Z status: exit code: 0
2019-12-23T02:27:29.6067080Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/exhaustiveness-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-pass" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-pass/auxiliary" "-A" "unused"
2019-12-23T02:27:29.6068111Z ------------------------------------------
2019-12-23T02:27:29.6068458Z 
2019-12-23T02:27:29.6068975Z ------------------------------------------
2019-12-23T02:27:29.6069146Z stderr:
---
2019-12-23T02:27:29.6074860Z 24 
2019-12-23T02:27:29.6075240Z - error: or-patterns are not fully implemented yet
2019-12-23T02:27:29.6075635Z -   --> $DIR/exhaustiveness-non-exhaustive.rs:10:10
2019-12-23T02:27:29.6076011Z -    |
2019-12-23T02:27:29.6076396Z - LL |         (0 | _,) => {}
2019-12-23T02:27:29.6077082Z - 
2019-12-23T02:27:29.6077451Z - error: aborting due to 4 previous errors
2019-12-23T02:27:29.6077642Z + error: aborting due to 3 previous errors
2019-12-23T02:27:29.6077774Z 32 
2019-12-23T02:27:29.6077774Z 32 
2019-12-23T02:27:29.6078331Z 33 For more information about this error, try `rustc --explain E0004`.
2019-12-23T02:27:29.6078513Z 34 
2019-12-23T02:27:29.6078653Z 
2019-12-23T02:27:29.6078764Z 
2019-12-23T02:27:29.6078896Z The actual stderr differed from the expected stderr.
2019-12-23T02:27:29.6079381Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive/exhaustiveness-non-exhaustive.stderr
2019-12-23T02:27:29.6080480Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T02:27:29.6080974Z To only update this specific test, also pass `--test-args or-patterns/exhaustiveness-non-exhaustive.rs`
2019-12-23T02:27:29.6081304Z error: 1 errors occurred comparing output.
2019-12-23T02:27:29.6081480Z status: exit code: 1
2019-12-23T02:27:29.6081480Z status: exit code: 1
2019-12-23T02:27:29.6082603Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/exhaustiveness-non-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-non-exhaustive/auxiliary" "-A" "unused"
2019-12-23T02:27:29.6086538Z ------------------------------------------
2019-12-23T02:27:29.6086804Z 
2019-12-23T02:27:29.6087224Z ------------------------------------------
2019-12-23T02:27:29.6087401Z stderr:
2019-12-23T02:27:29.6087401Z stderr:
2019-12-23T02:27:29.6087754Z ------------------------------------------
2019-12-23T02:27:29.6088174Z error[E0004]: non-exhaustive patterns: `(2u8..=std::u8::MAX, _)` not covered
2019-12-23T02:27:29.6088983Z    |
2019-12-23T02:27:29.6089255Z LL |     match (0u8, 0u8) {
2019-12-23T02:27:29.6089255Z LL |     match (0u8, 0u8) {
2019-12-23T02:27:29.6089391Z    |           ^^^^^^^^^^ pattern `(2u8..=std::u8::MAX, _)` not covered
2019-12-23T02:27:29.6090269Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-23T02:27:29.6090430Z 
2019-12-23T02:27:29.6090430Z 
2019-12-23T02:27:29.6091969Z error[E0004]: non-exhaustive patterns: `((4u8..=std::u8::MAX))` not covered
2019-12-23T02:27:29.6092707Z    |
2019-12-23T02:27:29.6092707Z    |
2019-12-23T02:27:29.6092770Z LL |     match ((0u8,),) {
2019-12-23T02:27:29.6092820Z    |           ^^^^^^^^^ pattern `((4u8..=std::u8::MAX))` not covered
2019-12-23T02:27:29.6092918Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-23T02:27:29.6092988Z 
2019-12-23T02:27:29.6092988Z 
2019-12-23T02:27:29.6093423Z error[E0004]: non-exhaustive patterns: `(Some(2u8..=std::u8::MAX))` not covered
2019-12-23T02:27:29.6093948Z    |
2019-12-23T02:27:29.6093948Z    |
2019-12-23T02:27:29.6093987Z LL |     match (Some(0u8),) {
2019-12-23T02:27:29.6094033Z    |           ^^^^^^^^^^^^ pattern `(Some(2u8..=std::u8::MAX))` not covered
2019-12-23T02:27:29.6094137Z    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-12-23T02:27:29.6094170Z 
2019-12-23T02:27:29.6094210Z error: aborting due to 3 previous errors
2019-12-23T02:27:29.6094254Z 
---
2019-12-23T02:27:29.6095915Z 102 
2019-12-23T02:27:29.6096488Z - error: or-patterns are not fully implemented yet
2019-12-23T02:27:29.6096741Z -   --> $DIR/exhaustiveness-unreachable-pattern.rs:10:10
2019-12-23T02:27:29.6096937Z -    |
2019-12-23T02:27:29.6098901Z - LL |         (0 | _,) => {}
2019-12-23T02:27:29.6100143Z - 
2019-12-23T02:27:29.6100393Z - error: aborting due to 17 previous errors
2019-12-23T02:27:29.6100467Z + error: aborting due to 16 previous errors
2019-12-23T02:27:29.6100510Z 110 
2019-12-23T02:27:29.6100510Z 110 
2019-12-23T02:27:29.6100550Z 111 
2019-12-23T02:27:29.6100596Z 
2019-12-23T02:27:29.6100623Z 
2019-12-23T02:27:29.6100670Z The actual stderr differed from the expected stderr.
2019-12-23T02:27:29.6101054Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-unreachable-pattern/exhaustiveness-unreachable-pattern.stderr
2019-12-23T02:27:29.6101368Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T02:27:29.6101681Z To only update this specific test, also pass `--test-args or-patterns/exhaustiveness-unreachable-pattern.rs`
2019-12-23T02:27:29.6101785Z error: 1 errors occurred comparing output.
2019-12-23T02:27:29.6101830Z status: exit code: 1
2019-12-23T02:27:29.6101830Z status: exit code: 1
2019-12-23T02:27:29.6103380Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-unreachable-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/exhaustiveness-unreachable-pattern/auxiliary" "-A" "unused"
2019-12-23T02:27:29.6103844Z ------------------------------------------
2019-12-23T02:27:29.6103877Z 
2019-12-23T02:27:29.6104094Z ------------------------------------------
2019-12-23T02:27:29.6104136Z stderr:
2019-12-23T02:27:29.6104136Z stderr:
2019-12-23T02:27:29.6104366Z ------------------------------------------
2019-12-23T02:27:29.6104410Z error: unreachable pattern
2019-12-23T02:27:29.6104663Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:16:9
2019-12-23T02:27:29.6104729Z    |
2019-12-23T02:27:29.6104773Z LL |         (1,) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6104869Z    |
2019-12-23T02:27:29.6104908Z note: lint level defined here
2019-12-23T02:27:29.6105169Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:4:9
2019-12-23T02:27:29.6105244Z    |
2019-12-23T02:27:29.6105244Z    |
2019-12-23T02:27:29.6105286Z LL | #![deny(unreachable_patterns)]
2019-12-23T02:27:29.6105327Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-12-23T02:27:29.6105353Z 
2019-12-23T02:27:29.6105411Z error: unreachable pattern
2019-12-23T02:27:29.6105667Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:21:9
2019-12-23T02:27:29.6105713Z    |
2019-12-23T02:27:29.6105772Z LL |         (2,) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6105838Z 
2019-12-23T02:27:29.6105875Z error: unreachable pattern
2019-12-23T02:27:29.6106144Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:27:9
2019-12-23T02:27:29.6106190Z    |
2019-12-23T02:27:29.6106190Z    |
2019-12-23T02:27:29.6106232Z LL |         (1 | 2,) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6106322Z 
2019-12-23T02:27:29.6106367Z error: unreachable pattern
2019-12-23T02:27:29.6106620Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:32:9
2019-12-23T02:27:29.6106690Z    |
---
2019-12-23T02:27:29.6108647Z 
2019-12-23T02:27:29.6108696Z error: unreachable pattern
2019-12-23T02:27:29.6108987Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:35:9
2019-12-23T02:27:29.6109056Z    |
2019-12-23T02:27:29.6109103Z LL |         (2 | 1, 4) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6109198Z 
2019-12-23T02:27:29.6109240Z error: unreachable pattern
2019-12-23T02:27:29.6109739Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:37:9
2019-12-23T02:27:29.6109794Z    |
2019-12-23T02:27:29.6109794Z    |
2019-12-23T02:27:29.6109860Z LL |         (1, 4 | 5) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6109935Z 
2019-12-23T02:27:29.6109995Z error: unreachable pattern
2019-12-23T02:27:29.6110279Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:42:9
2019-12-23T02:27:29.6110329Z    |
2019-12-23T02:27:29.6110329Z    |
2019-12-23T02:27:29.6110512Z LL |         (Some(1),) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6110678Z 
2019-12-23T02:27:29.6110720Z error: unreachable pattern
2019-12-23T02:27:29.6111060Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:43:9
2019-12-23T02:27:29.6111111Z    |
2019-12-23T02:27:29.6111111Z    |
2019-12-23T02:27:29.6111157Z LL |         (None,) => {} //~ ERROR unreachable pattern
2019-12-23T02:27:29.6111251Z 
2019-12-23T02:27:29.6111292Z error: unreachable pattern
2019-12-23T02:27:29.6111573Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:48:9
2019-12-23T02:27:29.6111641Z    |
2019-12-23T02:27:29.6111641Z    |
2019-12-23T02:27:29.6111689Z LL |         ((1..=4,),) => {}, //~ ERROR unreachable pattern
2019-12-23T02:27:29.6111764Z 
2019-12-23T02:27:29.6111824Z error: unreachable pattern
2019-12-23T02:27:29.6112112Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:54:12
2019-12-23T02:27:29.6112164Z    |
2019-12-23T02:27:29.6112164Z    |
2019-12-23T02:27:29.6112235Z LL |          | 1,) => {} //~ ERROR unreachable
2019-12-23T02:27:29.6112309Z 
2019-12-23T02:27:29.6112350Z error: unreachable pattern
2019-12-23T02:27:29.6112653Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:61:15
2019-12-23T02:27:29.6112897Z    |
2019-12-23T02:27:29.6112897Z    |
2019-12-23T02:27:29.6112940Z LL |             | 0] => {} //~ ERROR unreachable
2019-12-23T02:27:29.6113027Z 
2019-12-23T02:27:29.6113065Z error: unreachable pattern
2019-12-23T02:27:29.6113325Z   --> /checkout/src/test/ui/or-patterns/exhaustiveness-unreachable-pattern.rs:59:15
2019-12-23T02:27:29.6113389Z    |
---
2019-12-23T02:27:29.6116308Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-23T02:27:29.6116363Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T02:27:29.6116396Z 
2019-12-23T02:27:29.6116421Z 
2019-12-23T02:27:29.6118050Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T02:27:29.6118359Z 
2019-12-23T02:27:29.6118406Z 
2019-12-23T02:27:29.6119248Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T02:27:29.6119914Z Build completed unsuccessfully in 1:06:53
2019-12-23T02:27:29.6119914Z Build completed unsuccessfully in 1:06:53
2019-12-23T02:27:29.6164076Z == clock drift check ==
2019-12-23T02:27:29.6180415Z   local time: Mon Dec 23 02:27:29 UTC 2019
2019-12-23T02:27:29.7747094Z   network time: Mon, 23 Dec 2019 02:27:29 GMT
2019-12-23T02:27:29.7751665Z == end clock drift check ==
2019-12-23T02:27:31.2238426Z 
2019-12-23T02:27:31.2345913Z ##[error]Bash exited with code '1'.
2019-12-23T02:27:31.2384963Z ##[section]Starting: Checkout
2019-12-23T02:27:31.2387317Z ==============================================================================
2019-12-23T02:27:31.2387375Z Task         : Get sources
2019-12-23T02:27:31.2387424Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
