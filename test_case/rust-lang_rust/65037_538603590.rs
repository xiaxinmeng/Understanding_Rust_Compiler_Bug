plain
2019-10-05T00:00:54.9159622Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T00:00:54.9353193Z ##[command]git config gc.auto 0
2019-10-05T00:00:54.9443786Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T00:00:54.9507543Z ##[command]git config --get-all http.proxy
2019-10-05T00:00:54.9650937Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65037/merge:refs/remotes/pull/65037/merge
---
2019-10-05T01:02:36.7148405Z .................................................................................................... 1500/9113
2019-10-05T01:02:43.6775120Z .................................................................................................... 1600/9113
2019-10-05T01:02:52.9109317Z .................................................................................................... 1700/9113
2019-10-05T01:03:02.3837100Z .......i...............i............................................................................ 1800/9113
2019-10-05T01:03:09.6889409Z ..................................................................................................ii 1900/9113
2019-10-05T01:03:26.1406056Z iii................................................................................................. 2000/9113
2019-10-05T01:03:35.1575955Z .................................................................................................... 2200/9113
2019-10-05T01:03:37.8700306Z .................................................................................................... 2300/9113
2019-10-05T01:03:44.1235881Z .................................................................................................... 2400/9113
2019-10-05T01:03:49.7194028Z .................................................................................................... 2500/9113
---
2019-10-05T01:06:44.2590625Z ........................................................................................i........... 4700/9113
2019-10-05T01:06:52.2823897Z ....i............................................................................................... 4800/9113
2019-10-05T01:07:02.9267856Z .................................................................................................... 4900/9113
2019-10-05T01:07:08.6657862Z .................................................................................................... 5000/9113
2019-10-05T01:07:20.7741829Z .................................................................................ii.ii.............. 5100/9113
2019-10-05T01:07:30.2890985Z .................................................................................................... 5300/9113
2019-10-05T01:07:40.3064233Z .................................................................................................... 5400/9113
2019-10-05T01:07:47.1495367Z ...............................................i.................................................... 5500/9113
2019-10-05T01:07:54.0944915Z .................................................................................................... 5600/9113
2019-10-05T01:07:54.0944915Z .................................................................................................... 5600/9113
2019-10-05T01:08:04.5418314Z .................................................................................................... 5700/9113
2019-10-05T01:08:12.3186919Z ............................................ii...i..ii...........i.................................. 5800/9113
2019-10-05T01:08:37.8892627Z .................................................................................................... 6000/9113
2019-10-05T01:08:47.0061205Z .................................................................................................... 6100/9113
2019-10-05T01:08:47.0061205Z .................................................................................................... 6100/9113
2019-10-05T01:09:03.2218192Z ..................................................i..ii............................................. 6200/9113
2019-10-05T01:09:27.9139421Z .................................................................................................... 6400/9113
2019-10-05T01:09:30.2120977Z ..........i......................................................................................... 6500/9113
2019-10-05T01:09:32.5153908Z ..................................................................................i................. 6600/9113
2019-10-05T01:09:35.2474105Z .................................................................................................... 6700/9113
---
2019-10-05T01:13:46.1054005Z failures:
2019-10-05T01:13:46.1097879Z 
2019-10-05T01:13:46.1099096Z ---- [ui] ui/rfc-2091-track-caller/error-with-trait-decl.rs stdout ----
2019-10-05T01:13:46.1099384Z 
2019-10-05T01:13:46.1100000Z error: /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs:4: unexpected error: '4:5: 4:20: `#[track_caller]` is not supported in trait declarations. [E0738]'
2019-10-05T01:13:46.1100226Z 
2019-10-05T01:13:46.1100737Z error: /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs:4: expected error not found: `#[track_caller]` is not supported in traits yet.
2019-10-05T01:13:46.1101413Z error: 1 unexpected errors found, 1 expected errors not found
2019-10-05T01:13:46.1101579Z status: exit code: 1
2019-10-05T01:13:46.1101579Z status: exit code: 1
2019-10-05T01:13:46.1102794Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-decl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-decl/auxiliary" "-A" "unused"
2019-10-05T01:13:46.1103097Z unexpected errors (from JSON output): [
2019-10-05T01:13:46.1103267Z     Error {
2019-10-05T01:13:46.1103416Z         line_num: 4,
2019-10-05T01:13:46.1103557Z         kind: Some(
2019-10-05T01:13:46.1103849Z         ),
2019-10-05T01:13:46.1103849Z         ),
2019-10-05T01:13:46.1104155Z         msg: "4:5: 4:20: `#[track_caller]` is not supported in trait declarations. [E0738]",
2019-10-05T01:13:46.1104486Z ]
2019-10-05T01:13:46.1104607Z 
2019-10-05T01:13:46.1104764Z not found errors (from test file): [
2019-10-05T01:13:46.1104906Z     Error {
2019-10-05T01:13:46.1104906Z     Error {
2019-10-05T01:13:46.1105044Z         line_num: 4,
2019-10-05T01:13:46.1105198Z         kind: Some(
2019-10-05T01:13:46.1105348Z             Error,
2019-10-05T01:13:46.1105502Z         ),
2019-10-05T01:13:46.1105648Z         msg: "`#[track_caller]` is not supported in traits yet.",
2019-10-05T01:13:46.1105939Z ]
2019-10-05T01:13:46.1106060Z 
2019-10-05T01:13:46.1106581Z thread '[ui] ui/rfc-2091-track-caller/error-with-trait-decl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-10-05T01:13:46.1106824Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-10-05T01:13:46.1110202Z 12 LL |     #[track_caller]
2019-10-05T01:13:46.1110324Z 
2019-10-05T01:13:46.1110443Z 
2019-10-05T01:13:46.1110584Z The actual stderr differed from the expected stderr.
2019-10-05T01:13:46.1111107Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-default-impl/error-with-trait-default-impl.stderr
2019-10-05T01:13:46.1111588Z To update references, rerun the tests and pass the `--bless` flag
2019-10-05T01:13:46.1112092Z To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-trait-default-impl.rs`
2019-10-05T01:13:46.1112422Z error: 1 errors occurred comparing output.
2019-10-05T01:13:46.1112580Z status: exit code: 1
2019-10-05T01:13:46.1112580Z status: exit code: 1
2019-10-05T01:13:46.1113539Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-default-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-default-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-default-impl/auxiliary" "-A" "unused"
2019-10-05T01:13:46.1114196Z ------------------------------------------
2019-10-05T01:13:46.1114388Z 
2019-10-05T01:13:46.1114768Z ------------------------------------------
2019-10-05T01:13:46.1114959Z stderr:
2019-10-05T01:13:46.1114959Z stderr:
2019-10-05T01:13:46.1115333Z ------------------------------------------
2019-10-05T01:13:46.1115701Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-05T01:13:46.1116239Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-default-impl.rs:1:12
2019-10-05T01:13:46.1116445Z    |
2019-10-05T01:13:46.1116621Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-05T01:13:46.1116903Z    |
2019-10-05T01:13:46.1117063Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-05T01:13:46.1117186Z 
2019-10-05T01:13:46.1117330Z error[E0738]: `#[track_caller]` is not supported in trait declarations.
---
2019-10-05T01:13:46.1121107Z ---- [ui] ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs stdout ----
2019-10-05T01:13:46.1121304Z 
2019-10-05T01:13:46.1121452Z error: ui test compiled successfully!
2019-10-05T01:13:46.1121595Z status: exit code: 0
2019-10-05T01:13:46.1122557Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl/auxiliary" "-A" "unused"
2019-10-05T01:13:46.1123207Z ------------------------------------------
2019-10-05T01:13:46.1123383Z 
2019-10-05T01:13:46.1123772Z ------------------------------------------
2019-10-05T01:13:46.1123964Z stderr:
2019-10-05T01:13:46.1123964Z stderr:
2019-10-05T01:13:46.1124328Z ------------------------------------------
2019-10-05T01:13:46.1124544Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-05T01:13:46.1124966Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs:1:12
2019-10-05T01:13:46.1125804Z    |
2019-10-05T01:13:46.1125877Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-05T01:13:46.1125966Z    |
2019-10-05T01:13:46.1126028Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-05T01:13:46.1126070Z 
2019-10-05T01:13:46.1126095Z 
---
2019-10-05T01:13:46.1127683Z 
2019-10-05T01:13:46.1142717Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-05T01:13:46.1157064Z 
2019-10-05T01:13:46.1157167Z 
2019-10-05T01:13:46.1163232Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-05T01:13:46.1163620Z 
2019-10-05T01:13:46.1163653Z 
2019-10-05T01:13:46.1167097Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-05T01:13:46.1167196Z Build completed unsuccessfully in 1:05:09
2019-10-05T01:13:46.1167196Z Build completed unsuccessfully in 1:05:09
2019-10-05T01:13:46.1218424Z == clock drift check ==
2019-10-05T01:13:46.1234883Z   local time: Sat Oct  5 01:13:46 UTC 2019
2019-10-05T01:13:46.4100165Z   network time: Sat, 05 Oct 2019 01:13:46 GMT
2019-10-05T01:13:46.4107514Z == end clock drift check ==
2019-10-05T01:13:47.6634028Z ##[error]Bash exited with code '1'.
2019-10-05T01:13:47.6690016Z ##[section]Starting: Checkout
2019-10-05T01:13:47.6691952Z ==============================================================================
2019-10-05T01:13:47.6692024Z Task         : Get sources
2019-10-05T01:13:47.6692073Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
