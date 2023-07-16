plain
2019-09-16T21:20:41.9134614Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T21:20:41.9362142Z ##[command]git config gc.auto 0
2019-09-16T21:20:41.9417970Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T21:20:41.9471939Z ##[command]git config --get-all http.proxy
2019-09-16T21:20:41.9602403Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64105/merge:refs/remotes/pull/64105/merge
---
2019-09-16T22:22:48.3839425Z .................................................................................................... 1500/9023
2019-09-16T22:22:54.2815449Z .................................................................................................... 1600/9023
2019-09-16T22:23:06.9333926Z ...............................................................i...............i.................... 1700/9023
2019-09-16T22:23:14.1502694Z .................................................................................................... 1800/9023
2019-09-16T22:23:29.5700455Z ..............................................F.......iiiii......................................... 1900/9023
2019-09-16T22:23:41.0781528Z .................................................................................................... 2100/9023
2019-09-16T22:23:43.5850331Z .................................................................................................... 2200/9023
2019-09-16T22:23:46.9932432Z .................................................................................................... 2300/9023
2019-09-16T22:23:55.3279799Z .................................................................................................... 2400/9023
---
2019-09-16T22:26:53.4166270Z ..........................................i...............i......................................... 4700/9023
2019-09-16T22:27:04.0641806Z .................................................................................................... 4800/9023
2019-09-16T22:27:11.1659409Z .................................................................................................... 4900/9023
2019-09-16T22:27:20.7418618Z .................................................................................................... 5000/9023
2019-09-16T22:27:28.5292032Z ..........................ii.ii..................................................................... 5100/9023
2019-09-16T22:27:38.7600914Z .................................................................................................... 5300/9023
2019-09-16T22:27:48.9268213Z ..........................................................................................i......... 5400/9023
2019-09-16T22:27:57.1887175Z .................................................................................................... 5500/9023
2019-09-16T22:28:02.1988377Z .................................................................................................... 5600/9023
2019-09-16T22:28:02.1988377Z .................................................................................................... 5600/9023
2019-09-16T22:28:12.7743433Z .....................................................................................ii...i..ii..... 5700/9023
2019-09-16T22:28:38.2329056Z .................................................................................................... 5900/9023
2019-09-16T22:28:47.0033413Z .................................................................................................... 6000/9023
2019-09-16T22:28:47.0033413Z .................................................................................................... 6000/9023
2019-09-16T22:28:51.5563316Z .......................................................................................i..ii........ 6100/9023
2019-09-16T22:29:20.4882110Z .................................................................................................... 6300/9023
2019-09-16T22:29:24.1291203Z ..............................................i..................................................... 6400/9023
2019-09-16T22:29:26.1496873Z .................................................................................................... 6500/9023
2019-09-16T22:29:28.5772494Z ..................i................................................................................. 6600/9023
---
2019-09-16T22:33:35.1693502Z 
2019-09-16T22:33:35.1694479Z ---- [ui] ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs stdout ----
2019-09-16T22:33:35.1695308Z diff of stderr:
2019-09-16T22:33:35.1695463Z 
2019-09-16T22:33:35.1695553Z 27    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1695610Z 28    |           help: use `&&` instead of `and` for the boolean operator
2019-09-16T22:33:35.1695657Z 29 
2019-09-16T22:33:35.1695738Z + error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-16T22:33:35.1696332Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:22:15
2019-09-16T22:33:35.1696404Z +    |
2019-09-16T22:33:35.1696449Z + LL |     if (a and b) {
2019-09-16T22:33:35.1696528Z +    |               ^ expected one of 8 possible tokens here
2019-09-16T22:33:35.1696573Z + 
2019-09-16T22:33:35.1696625Z 30 error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `or`
2019-09-16T22:33:35.1696957Z 32    |
2019-09-16T22:33:35.1696986Z 
2019-09-16T22:33:35.1697033Z 36    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1697033Z 36    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1697109Z 37    |           help: use `||` instead of `or` for the boolean operator
2019-09-16T22:33:35.1697154Z 38 
2019-09-16T22:33:35.1697203Z + error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-16T22:33:35.1697473Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:31:14
2019-09-16T22:33:35.1697532Z +    |
2019-09-16T22:33:35.1697576Z + LL |     if (a or b) {
2019-09-16T22:33:35.1697658Z +    |              ^ expected one of 8 possible tokens here
2019-09-16T22:33:35.1697703Z + 
2019-09-16T22:33:35.1697752Z 39 error: expected one of `!`, `.`, `::`, `?`, `{`, or an operator, found `and`
2019-09-16T22:33:35.1698072Z 41    |
2019-09-16T22:33:35.1698099Z 
2019-09-16T22:33:35.1698099Z 
2019-09-16T22:33:35.1698148Z 54    |             expected one of `!`, `.`, `::`, `?`, `{`, or an operator here
2019-09-16T22:33:35.1698222Z 55    |             help: use `||` instead of `or` for the boolean operator
2019-09-16T22:33:35.1698631Z - error: aborting due to 6 previous errors
2019-09-16T22:33:35.1698631Z - error: aborting due to 6 previous errors
2019-09-16T22:33:35.1698694Z + error[E0425]: cannot find value `and` in this scope
2019-09-16T22:33:35.1698887Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:22:11
2019-09-16T22:33:35.1698923Z +    |
2019-09-16T22:33:35.1699190Z + LL |     if (a and b) {
2019-09-16T22:33:35.1699289Z 58 
2019-09-16T22:33:35.1699289Z 58 
2019-09-16T22:33:35.1699325Z + error[E0425]: cannot find value `or` in this scope
2019-09-16T22:33:35.1699587Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:31:11
2019-09-16T22:33:35.1699624Z +    |
2019-09-16T22:33:35.1699659Z + LL |     if (a or b) {
2019-09-16T22:33:35.1699751Z + 
2019-09-16T22:33:35.1699786Z + error: aborting due to 10 previous errors
2019-09-16T22:33:35.1699838Z + 
2019-09-16T22:33:35.1700039Z + For more information about this error, try `rustc --explain E0425`.
2019-09-16T22:33:35.1700039Z + For more information about this error, try `rustc --explain E0425`.
2019-09-16T22:33:35.1700078Z 59 
2019-09-16T22:33:35.1700099Z 
2019-09-16T22:33:35.1700139Z 
2019-09-16T22:33:35.1700176Z The actual stderr differed from the expected stderr.
2019-09-16T22:33:35.1700470Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands/issue-54109-and_instead_of_ampersands.stderr
2019-09-16T22:33:35.1700866Z To update references, rerun the tests and pass the `--bless` flag
2019-09-16T22:33:35.1701115Z To only update this specific test, also pass `--test-args did_you_mean/issue-54109-and_instead_of_ampersands.rs`
2019-09-16T22:33:35.1701201Z error: 1 errors occurred comparing output.
2019-09-16T22:33:35.1701238Z status: exit code: 1
2019-09-16T22:33:35.1701238Z status: exit code: 1
2019-09-16T22:33:35.1701993Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands/auxiliary" "-A" "unused"
2019-09-16T22:33:35.1702281Z ------------------------------------------
2019-09-16T22:33:35.1702308Z 
2019-09-16T22:33:35.1702483Z ------------------------------------------
2019-09-16T22:33:35.1702519Z stderr:
2019-09-16T22:33:35.1702519Z stderr:
2019-09-16T22:33:35.1702712Z ------------------------------------------
2019-09-16T22:33:35.1702751Z error: expected `{`, found `and`
2019-09-16T22:33:35.1703023Z    |
2019-09-16T22:33:35.1703060Z LL |     if a and b {
2019-09-16T22:33:35.1703216Z    |     --   ^^^
2019-09-16T22:33:35.1703273Z    |     |    |
2019-09-16T22:33:35.1703273Z    |     |    |
2019-09-16T22:33:35.1703308Z    |     |    expected `{`
2019-09-16T22:33:35.1703351Z    |     |    help: use `&&` instead of `and` for the boolean operator
2019-09-16T22:33:35.1703414Z    |     this `if` statement has a condition, but no block
2019-09-16T22:33:35.1703617Z 
2019-09-16T22:33:35.1703652Z error: expected `{`, found `or`
2019-09-16T22:33:35.1703939Z    |
2019-09-16T22:33:35.1703939Z    |
2019-09-16T22:33:35.1703974Z LL |     if a or b {
2019-09-16T22:33:35.1704133Z    |     --   ^^
2019-09-16T22:33:35.1704227Z    |     |    expected `{`
2019-09-16T22:33:35.1704227Z    |     |    expected `{`
2019-09-16T22:33:35.1704267Z    |     |    help: use `||` instead of `or` for the boolean operator
2019-09-16T22:33:35.1704308Z    |     this `if` statement has a condition, but no block
2019-09-16T22:33:35.1704354Z 
2019-09-16T22:33:35.1704392Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `and`
2019-09-16T22:33:35.1704675Z    |
2019-09-16T22:33:35.1704675Z    |
2019-09-16T22:33:35.1704710Z LL |     if (a and b) {
2019-09-16T22:33:35.1705424Z    |           |
2019-09-16T22:33:35.1705507Z    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1705507Z    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1705560Z    |           help: use `&&` instead of `and` for the boolean operator
2019-09-16T22:33:35.1705592Z 
2019-09-16T22:33:35.1705662Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-16T22:33:35.1707285Z    |
2019-09-16T22:33:35.1707285Z    |
2019-09-16T22:33:35.1707353Z LL |     if (a and b) {
2019-09-16T22:33:35.1707402Z    |               ^ expected one of 8 possible tokens here
2019-09-16T22:33:35.1707433Z 
2019-09-16T22:33:35.1707480Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `or`
2019-09-16T22:33:35.1709079Z    |
2019-09-16T22:33:35.1709079Z    |
2019-09-16T22:33:35.1709117Z LL |     if (a or b) {
2019-09-16T22:33:35.1709221Z    |           |
2019-09-16T22:33:35.1710376Z    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1710376Z    |           expected one of 8 possible tokens here
2019-09-16T22:33:35.1711028Z    |           help: use `||` instead of `or` for the boolean operator
2019-09-16T22:33:35.1711063Z 
2019-09-16T22:33:35.1711101Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-16T22:33:35.1712096Z    |
2019-09-16T22:33:35.1712096Z    |
2019-09-16T22:33:35.1712719Z LL |     if (a or b) {
2019-09-16T22:33:35.1712832Z    |              ^ expected one of 8 possible tokens here
2019-09-16T22:33:35.1712879Z 
2019-09-16T22:33:35.1712917Z error: expected one of `!`, `.`, `::`, `?`, `{`, or an operator, found `and`
2019-09-16T22:33:35.1713225Z    |
2019-09-16T22:33:35.1713270Z LL |     while a and b {
2019-09-16T22:33:35.1713303Z    |             ^^^
2019-09-16T22:33:35.1713343Z    |             |
2019-09-16T22:33:35.1713343Z    |             |
2019-09-16T22:33:35.1713400Z    |             expected one of `!`, `.`, `::`, `?`, `{`, or an operator here
2019-09-16T22:33:35.1713443Z    |             help: use `&&` instead of `and` for the boolean operator
2019-09-16T22:33:35.1713467Z 
2019-09-16T22:33:35.1713524Z error: expected one of `!`, `.`, `::`, `?`, `{`, or an operator, found `or`
2019-09-16T22:33:35.1713956Z    |
2019-09-16T22:33:35.1713956Z    |
2019-09-16T22:33:35.1713989Z LL |     while a or b {
2019-09-16T22:33:35.1714079Z    |             |
2019-09-16T22:33:35.1714079Z    |             |
2019-09-16T22:33:35.1714118Z    |             expected one of `!`, `.`, `::`, `?`, `{`, or an operator here
2019-09-16T22:33:35.1714180Z    |             help: use `||` instead of `or` for the boolean operator
2019-09-16T22:33:35.1714248Z error[E0425]: cannot find value `and` in this scope
2019-09-16T22:33:35.1714493Z   --> /checkout/src/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs:22:11
2019-09-16T22:33:35.1714711Z    |
2019-09-16T22:33:35.1714711Z    |
2019-09-16T22:33:35.1714744Z LL |     if (a and b) {
2019-09-16T22:33:35.1715152Z 
2019-09-16T22:33:35.1716041Z error[E0425]: cannot find value `or` in this scope
2019-09-16T22:33:35.1716428Z   --> /checkout/src/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs:31:11
2019-09-16T22:33:35.1716503Z    |
2019-09-16T22:33:35.1716503Z    |
2019-09-16T22:33:35.1716545Z LL |     if (a or b) {
2019-09-16T22:33:35.1716624Z 
2019-09-16T22:33:35.1716687Z error: aborting due to 10 previous errors
2019-09-16T22:33:35.1716716Z 
2019-09-16T22:33:35.1716964Z For more information about this error, try `rustc --explain E0425`.
---
2019-09-16T22:33:35.1749009Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-16T22:33:35.1749090Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-16T22:33:35.1767661Z 
2019-09-16T22:33:35.1767782Z 
2019-09-16T22:33:35.1769946Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-16T22:33:35.1770560Z 
2019-09-16T22:33:35.1770587Z 
2019-09-16T22:33:35.1779962Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-16T22:33:35.1780034Z Build completed unsuccessfully in 1:05:55
2019-09-16T22:33:35.1780034Z Build completed unsuccessfully in 1:05:55
2019-09-16T22:33:35.1833241Z == clock drift check ==
2019-09-16T22:33:35.1846269Z   local time: Mon Sep 16 22:33:35 UTC 2019
2019-09-16T22:33:35.3355052Z   network time: Mon, 16 Sep 2019 22:33:35 GMT
2019-09-16T22:33:35.3360478Z == end clock drift check ==
2019-09-16T22:33:36.2151188Z ##[error]Bash exited with code '1'.
2019-09-16T22:33:36.2202170Z ##[section]Starting: Checkout
2019-09-16T22:33:36.2203786Z ==============================================================================
2019-09-16T22:33:36.2203846Z Task         : Get sources
2019-09-16T22:33:36.2203884Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
