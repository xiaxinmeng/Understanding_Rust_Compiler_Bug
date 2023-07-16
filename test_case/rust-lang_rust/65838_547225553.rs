plain
2019-10-29T01:03:57.1723600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T01:03:57.1914548Z ##[command]git config gc.auto 0
2019-10-29T01:03:57.1988382Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T01:03:57.2042294Z ##[command]git config --get-all http.proxy
2019-10-29T01:03:57.2179629Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65838/merge:refs/remotes/pull/65838/merge
---
2019-10-29T02:05:08.6493612Z .................................................................................................... 1600/9262
2019-10-29T02:05:14.8401762Z .................................................................................................... 1700/9262
2019-10-29T02:05:28.0289455Z ..........................................................i...............i......................... 1800/9262
2019-10-29T02:05:36.2541404Z .................................................................................................... 1900/9262
2019-10-29T02:05:52.3321545Z ................................................iiiii............................................... 2000/9262
2019-10-29T02:06:03.9615919Z .................................................................................................... 2200/9262
2019-10-29T02:06:06.7582108Z .................................................................................................... 2300/9262
2019-10-29T02:06:10.9245658Z .................................................................................................... 2400/9262
2019-10-29T02:06:36.5907670Z .................................................................................................... 2500/9262
---
2019-10-29T02:09:38.1350424Z .................................................i...............i.................................. 4800/9262
2019-10-29T02:09:47.8290231Z .................................................................................................... 4900/9262
2019-10-29T02:09:57.1103718Z .................................................................................................... 5000/9262
2019-10-29T02:10:03.8727938Z .................................................................................................... 5100/9262
2019-10-29T02:10:15.2239632Z ..................................................ii.ii...........i................................. 5200/9262
2019-10-29T02:10:25.6192772Z .................................................................................................... 5400/9262
2019-10-29T02:10:35.5752517Z .................................................................................................... 5500/9262
2019-10-29T02:10:43.9138668Z ....................i............................................................................... 5600/9262
2019-10-29T02:10:50.3029389Z .................................................................................................... 5700/9262
2019-10-29T02:10:50.3029389Z .................................................................................................... 5700/9262
2019-10-29T02:11:02.7353329Z .................................................................................................... 5800/9262
2019-10-29T02:11:15.5857698Z .....ii...i..ii...........i......................................................................... 5900/9262
2019-10-29T02:11:38.2661347Z .................................................................................................... 6100/9262
2019-10-29T02:11:44.7104330Z .................................................................................................... 6200/9262
2019-10-29T02:11:44.7104330Z .................................................................................................... 6200/9262
2019-10-29T02:11:59.6534680Z ........................i..ii....................................................................... 6300/9262
2019-10-29T02:12:20.9428047Z ..........................................................................................i......... 6500/9262
2019-10-29T02:12:23.3862986Z .................................................................................................... 6600/9262
2019-10-29T02:12:25.8115801Z ...............................................................F..F.i............................... 6700/9262
2019-10-29T02:12:28.9099939Z .................................................................................................... 6800/9262
---
2019-10-29T02:16:35.3137493Z ..................................................................i................................. 9200/9262
2019-10-29T02:16:46.7984760Z ..............................................................
2019-10-29T02:16:46.7985688Z failures:
2019-10-29T02:16:46.8022566Z 
2019-10-29T02:16:46.8023319Z ---- [ui] ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs stdout ----
2019-10-29T02:16:46.8024042Z 
2019-10-29T02:16:46.8024199Z 7 LL |
2019-10-29T02:16:46.8024341Z 8    |                                                     ^
2019-10-29T02:16:46.8024737Z 9 
2019-10-29T02:16:46.8024737Z 9 
2019-10-29T02:16:46.8025248Z - error: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `trait`
2019-10-29T02:16:46.8025477Z + error: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found keyword `trait`
2019-10-29T02:16:46.8025872Z 11   --> $DIR/missing-close-brace-in-impl-trait.rs:5:1
2019-10-29T02:16:46.8026082Z 12    |
2019-10-29T02:16:46.8026244Z 13 LL | impl T for () {
2019-10-29T02:16:46.8026484Z 
2019-10-29T02:16:46.8026643Z The actual stderr differed from the expected stderr.
2019-10-29T02:16:46.8026643Z The actual stderr differed from the expected stderr.
2019-10-29T02:16:46.8027140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait/missing-close-brace-in-impl-trait.stderr
2019-10-29T02:16:46.8027854Z To update references, rerun the tests and pass the `--bless` flag
2019-10-29T02:16:46.8028450Z To only update this specific test, also pass `--test-args parser/mismatched-braces/missing-close-brace-in-impl-trait.rs`
2019-10-29T02:16:46.8028804Z error: 1 errors occurred comparing output.
2019-10-29T02:16:46.8028954Z status: exit code: 1
2019-10-29T02:16:46.8028954Z status: exit code: 1
2019-10-29T02:16:46.8029972Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait/auxiliary" "-A" "unused"
2019-10-29T02:16:46.8030896Z ------------------------------------------
2019-10-29T02:16:46.8031062Z 
2019-10-29T02:16:46.8031434Z ------------------------------------------
2019-10-29T02:16:46.8031631Z stderr:
2019-10-29T02:16:46.8031631Z stderr:
2019-10-29T02:16:46.8031998Z ------------------------------------------
2019-10-29T02:16:46.8032400Z error: this file contains an un-closed delimiter
2019-10-29T02:16:46.8033023Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs:12:53
2019-10-29T02:16:46.8033219Z    |
2019-10-29T02:16:46.8033363Z LL | impl T for () { //~ ERROR cannot find trait `T` in this scope
2019-10-29T02:16:46.8033730Z    |               - un-closed delimiter
2019-10-29T02:16:46.8034251Z LL | //~ ERROR this file contains an un-closed delimiter
2019-10-29T02:16:46.8034420Z    |                                                     ^
2019-10-29T02:16:46.8034536Z 
2019-10-29T02:16:46.8034536Z 
2019-10-29T02:16:46.8034696Z error: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found keyword `trait`
2019-10-29T02:16:46.8035094Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs:5:1
2019-10-29T02:16:46.8035266Z    |
2019-10-29T02:16:46.8035442Z LL | impl T for () { //~ ERROR cannot find trait `T` in this scope
2019-10-29T02:16:46.8035932Z LL | 
2019-10-29T02:16:46.8036084Z LL | fn foo(&self) {}
2019-10-29T02:16:46.8036520Z    |                 -
2019-10-29T02:16:46.8036737Z    |                 |
2019-10-29T02:16:46.8036737Z    |                 |
2019-10-29T02:16:46.8036876Z    |                 expected one of 10 possible tokens here
2019-10-29T02:16:46.8037015Z    |                 help: `}` may belong here
2019-10-29T02:16:46.8037160Z LL | 
2019-10-29T02:16:46.8037292Z LL | trait T { //~ ERROR expected one of
2019-10-29T02:16:46.8037555Z 
2019-10-29T02:16:46.8038129Z error[E0405]: cannot find trait `T` in this scope
2019-10-29T02:16:46.8038129Z error[E0405]: cannot find trait `T` in this scope
2019-10-29T02:16:46.8038603Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs:1:6
2019-10-29T02:16:46.8038806Z    |
2019-10-29T02:16:46.8038946Z LL | impl T for () { //~ ERROR cannot find trait `T` in this scope
2019-10-29T02:16:46.8039087Z    |      ^ not found in this scope
2019-10-29T02:16:46.8039363Z error: aborting due to 3 previous errors
2019-10-29T02:16:46.8039480Z 
2019-10-29T02:16:46.8039873Z For more information about this error, try `rustc --explain E0405`.
2019-10-29T02:16:46.8040059Z 
2019-10-29T02:16:46.8040059Z 
2019-10-29T02:16:46.8040407Z ------------------------------------------
2019-10-29T02:16:46.8040905Z 
2019-10-29T02:16:46.8041314Z 
2019-10-29T02:16:46.8042351Z ---- [ui] ui/parser/mismatched-braces/missing-close-brace-in-trait.rs stdout ----
2019-10-29T02:16:46.8042459Z 
2019-10-29T02:16:46.8042500Z 7 LL | fn main() {}
2019-10-29T02:16:46.8042548Z 8    |                                                                  ^
2019-10-29T02:16:46.8042592Z 9 
2019-10-29T02:16:46.8042592Z 9 
2019-10-29T02:16:46.8042876Z - error: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`
2019-10-29T02:16:46.8042936Z + error: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found keyword `pub`
2019-10-29T02:16:46.8043547Z 11   --> $DIR/missing-close-brace-in-trait.rs:4:1
2019-10-29T02:16:46.8043612Z 12    |
2019-10-29T02:16:46.8043652Z 13 LL | trait T {
2019-10-29T02:16:46.8043840Z 
2019-10-29T02:16:46.8043902Z The actual stderr differed from the expected stderr.
2019-10-29T02:16:46.8043902Z The actual stderr differed from the expected stderr.
2019-10-29T02:16:46.8044505Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-trait/missing-close-brace-in-trait.stderr
2019-10-29T02:16:46.8044763Z To update references, rerun the tests and pass the `--bless` flag
2019-10-29T02:16:46.8045188Z To only update this specific test, also pass `--test-args parser/mismatched-braces/missing-close-brace-in-trait.rs`
2019-10-29T02:16:46.8045283Z error: 1 errors occurred comparing output.
2019-10-29T02:16:46.8045351Z status: exit code: 1
2019-10-29T02:16:46.8045351Z status: exit code: 1
2019-10-29T02:16:46.8046192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-trait/auxiliary" "-A" "unused"
2019-10-29T02:16:46.8046568Z ------------------------------------------
2019-10-29T02:16:46.8046608Z 
2019-10-29T02:16:46.8046862Z ------------------------------------------
2019-10-29T02:16:46.8046910Z stderr:
2019-10-29T02:16:46.8046910Z stderr:
2019-10-29T02:16:46.8047141Z ------------------------------------------
2019-10-29T02:16:46.8047658Z error: this file contains an un-closed delimiter
2019-10-29T02:16:46.8047937Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs:10:66
2019-10-29T02:16:46.8047992Z    |
2019-10-29T02:16:46.8048060Z LL | trait T {
2019-10-29T02:16:46.8048273Z    |         - un-closed delimiter
2019-10-29T02:16:46.8048740Z LL | fn main() {} //~ ERROR this file contains an un-closed delimiter
2019-10-29T02:16:46.8048799Z    |                                                                  ^
2019-10-29T02:16:46.8048832Z 
2019-10-29T02:16:46.8048832Z 
2019-10-29T02:16:46.8048882Z error: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found keyword `pub`
2019-10-29T02:16:46.8049173Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-trait.rs:4:1
2019-10-29T02:16:46.8049227Z    |
2019-10-29T02:16:46.8049272Z LL | trait T {
2019-10-29T02:16:46.8049566Z LL |     fn foo(&self);
2019-10-29T02:16:46.8049777Z    |                   -
2019-10-29T02:16:46.8049845Z    |                   |
2019-10-29T02:16:46.8049896Z    |                   expected one of 7 possible tokens here
2019-10-29T02:16:46.8049896Z    |                   expected one of 7 possible tokens here
2019-10-29T02:16:46.8049949Z    |                   help: `}` may belong here
2019-10-29T02:16:46.8050011Z LL | 
2019-10-29T02:16:46.8050079Z LL | pub(crate) struct Bar<T>(); //~ ERROR expected one of
2019-10-29T02:16:46.8050160Z 
2019-10-29T02:16:46.8050223Z error: aborting due to 2 previous errors
2019-10-29T02:16:46.8050256Z 
2019-10-29T02:16:46.8050283Z 
2019-10-29T02:16:46.8050283Z 
2019-10-29T02:16:46.8050526Z ------------------------------------------
2019-10-29T02:16:46.8050561Z 
2019-10-29T02:16:46.8050605Z 
2019-10-29T02:16:46.8050632Z 
2019-10-29T02:16:46.8050674Z failures:
2019-10-29T02:16:46.8050939Z     [ui] ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs
2019-10-29T02:16:46.8051228Z     [ui] ui/parser/mismatched-braces/missing-close-brace-in-trait.rs
2019-10-29T02:16:46.8051548Z test result: FAILED. 9219 passed; 2 failed; 41 ignored; 0 measured; 0 filtered out
2019-10-29T02:16:46.8051586Z 
2019-10-29T02:16:46.8055792Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T02:16:46.8056019Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T02:16:46.8056019Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T02:16:46.8080467Z 
2019-10-29T02:16:46.8080657Z 
2019-10-29T02:16:46.8082571Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T02:16:46.8082856Z 
2019-10-29T02:16:46.8082887Z 
2019-10-29T02:16:46.8086604Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-29T02:16:46.8086708Z Build completed unsuccessfully in 1:05:44
2019-10-29T02:16:46.8086708Z Build completed unsuccessfully in 1:05:44
2019-10-29T02:16:46.8141618Z == clock drift check ==
2019-10-29T02:16:46.8153153Z   local time: Tue Oct 29 02:16:46 UTC 2019
2019-10-29T02:16:46.9608179Z   network time: Tue, 29 Oct 2019 02:16:46 GMT
2019-10-29T02:16:46.9611038Z == end clock drift check ==
2019-10-29T02:16:48.0784272Z 
2019-10-29T02:16:48.0909441Z ##[error]Bash exited with code '1'.
2019-10-29T02:16:48.0956440Z ##[section]Starting: Checkout
2019-10-29T02:16:48.0958609Z ==============================================================================
2019-10-29T02:16:48.0958667Z Task         : Get sources
2019-10-29T02:16:48.0958733Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
