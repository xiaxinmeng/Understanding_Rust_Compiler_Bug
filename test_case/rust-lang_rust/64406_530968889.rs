plain
2019-09-12T18:07:41.3072464Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T18:07:41.9199786Z ##[command]git config gc.auto 0
2019-09-12T18:07:41.9205150Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T18:07:41.9207220Z ##[command]git config --get-all http.proxy
2019-09-12T18:07:41.9211050Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64406/merge:refs/remotes/pull/64406/merge
---
2019-09-12T19:10:55.3176201Z .................................................................................................... 1500/9012
2019-09-12T19:11:01.3299004Z .................................................................................................... 1600/9012
2019-09-12T19:11:14.1219195Z .........................................................i...............i.......................... 1700/9012
2019-09-12T19:11:22.1021666Z .................................................................................................... 1800/9012
2019-09-12T19:11:37.3423226Z ................................................iiiii............................................... 1900/9012
2019-09-12T19:11:48.8003325Z .................................................................................................... 2100/9012
2019-09-12T19:11:51.4130901Z .................................................................................................... 2200/9012
2019-09-12T19:11:55.1187539Z .................................................................................................... 2300/9012
2019-09-12T19:12:03.4489094Z .................................................................................................... 2400/9012
---
2019-09-12T19:15:02.9522636Z ....................................i...............i............................................... 4700/9012
2019-09-12T19:15:14.2867769Z .................................................................................................... 4800/9012
2019-09-12T19:15:21.0277486Z .................................................................................................... 4900/9012
2019-09-12T19:15:31.9837939Z .................................................................................................... 5000/9012
2019-09-12T19:15:38.3408081Z ...................ii.ii............................................................................ 5100/9012
2019-09-12T19:15:48.8966160Z .................................................................................................... 5300/9012
2019-09-12T19:15:58.9566835Z ...................................................................................i................ 5400/9012
2019-09-12T19:16:06.9140793Z .................................................................................................... 5500/9012
2019-09-12T19:16:12.4342454Z .................................................................................................... 5600/9012
2019-09-12T19:16:12.4342454Z .................................................................................................... 5600/9012
2019-09-12T19:16:22.7420095Z ..............................................................................ii...i..ii...........i 5700/9012
2019-09-12T19:16:48.1862681Z .................................................................................................... 5900/9012
2019-09-12T19:16:58.2080772Z .................................................................................................... 6000/9012
2019-09-12T19:16:58.2080772Z .................................................................................................... 6000/9012
2019-09-12T19:17:08.0700426Z ................................................................................i..ii............... 6100/9012
2019-09-12T19:17:39.0535224Z .................................................................................................... 6300/9012
2019-09-12T19:17:40.9314018Z .......................................i............................................................ 6400/9012
2019-09-12T19:17:42.9860555Z .................................................................................................... 6500/9012
2019-09-12T19:17:45.4654646Z ...........i........................................................................................ 6600/9012
---
2019-09-12T19:21:58.7051851Z failures:
2019-09-12T19:21:58.7089379Z 
2019-09-12T19:21:58.7091173Z ---- [ui] ui/feature-gates/feature-gate-abi.rs stdout ----
2019-09-12T19:21:58.7091404Z 
2019-09-12T19:21:58.7091968Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:24: unexpected error: '24:32: 24:34: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7092140Z 
2019-09-12T19:21:58.7092611Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:25: unexpected error: '25:36: 25:38: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7092804Z 
2019-09-12T19:21:58.7093274Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:34: unexpected error: '34:32: 34:35: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7093435Z 
2019-09-12T19:21:58.7093938Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:35: unexpected error: '35:36: 35:39: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7094109Z 
2019-09-12T19:21:58.7094571Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:12: unexpected error: '12:33: 12:35: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7094771Z 
2019-09-12T19:21:58.7095235Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:13: unexpected error: '13:37: 13:39: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7095395Z 
2019-09-12T19:21:58.7095881Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:34: unexpected error: '34:38: 34:40: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7096041Z 
2019-09-12T19:21:58.7096499Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:35: unexpected error: '35:42: 35:44: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7096681Z 
2019-09-12T19:21:58.7097312Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:49: unexpected error: '49:37: 49:39: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7097512Z 
2019-09-12T19:21:58.7098035Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:50: unexpected error: '50:41: 50:43: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7098200Z 
2019-09-12T19:21:58.7098654Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:62: unexpected error: '62:38: 62:40: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7098841Z 
2019-09-12T19:21:58.7099298Z error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:63: unexpected error: '63:42: 63:44: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7099634Z error: 12 unexpected errors found, 0 expected errors not found
2019-09-12T19:21:58.7099783Z status: exit code: 1
2019-09-12T19:21:58.7099783Z status: exit code: 1
2019-09-12T19:21:58.7101143Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "-A" "unused"
2019-09-12T19:21:58.7101459Z unexpected errors (from JSON output): [
2019-09-12T19:21:58.7101635Z     Error {
2019-09-12T19:21:58.7101787Z         line_num: 24,
2019-09-12T19:21:58.7101930Z         kind: Some(
2019-09-12T19:21:58.7102235Z         ),
2019-09-12T19:21:58.7102235Z         ),
2019-09-12T19:21:58.7102698Z         msg: "24:32: 24:34: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7103084Z     Error {
2019-09-12T19:21:58.7103245Z         line_num: 25,
2019-09-12T19:21:58.7103391Z         kind: Some(
2019-09-12T19:21:58.7103674Z             Error,
2019-09-12T19:21:58.7103674Z             Error,
2019-09-12T19:21:58.7103836Z         ),
2019-09-12T19:21:58.7104453Z         msg: "25:36: 25:38: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7105371Z     Error {
2019-09-12T19:21:58.7106390Z         line_num: 34,
2019-09-12T19:21:58.7106458Z         kind: Some(
2019-09-12T19:21:58.7106499Z             Error,
2019-09-12T19:21:58.7106499Z             Error,
2019-09-12T19:21:58.7106539Z         ),
2019-09-12T19:21:58.7106883Z         msg: "34:32: 34:35: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7106988Z     Error {
2019-09-12T19:21:58.7107028Z         line_num: 35,
2019-09-12T19:21:58.7107086Z         kind: Some(
2019-09-12T19:21:58.7107125Z             Error,
2019-09-12T19:21:58.7107125Z             Error,
2019-09-12T19:21:58.7107180Z         ),
2019-09-12T19:21:58.7107460Z         msg: "35:36: 35:39: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7107554Z     Error {
2019-09-12T19:21:58.7107594Z         line_num: 12,
2019-09-12T19:21:58.7107652Z         kind: Some(
2019-09-12T19:21:58.7107693Z             Error,
2019-09-12T19:21:58.7107693Z             Error,
2019-09-12T19:21:58.7107732Z         ),
2019-09-12T19:21:58.7108007Z         msg: "12:33: 12:35: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7108092Z     Error {
2019-09-12T19:21:58.7108131Z         line_num: 13,
2019-09-12T19:21:58.7108188Z         kind: Some(
2019-09-12T19:21:58.7108228Z             Error,
2019-09-12T19:21:58.7108228Z             Error,
2019-09-12T19:21:58.7108266Z         ),
2019-09-12T19:21:58.7108537Z         msg: "13:37: 13:39: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7108621Z     Error {
2019-09-12T19:21:58.7108688Z         line_num: 34,
2019-09-12T19:21:58.7108729Z         kind: Some(
2019-09-12T19:21:58.7108769Z             Error,
2019-09-12T19:21:58.7108769Z             Error,
2019-09-12T19:21:58.7108807Z         ),
2019-09-12T19:21:58.7109088Z         msg: "34:38: 34:40: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7109172Z     Error {
2019-09-12T19:21:58.7109229Z         line_num: 35,
2019-09-12T19:21:58.7109268Z         kind: Some(
2019-09-12T19:21:58.7109308Z             Error,
2019-09-12T19:21:58.7109308Z             Error,
2019-09-12T19:21:58.7109363Z         ),
2019-09-12T19:21:58.7109617Z         msg: "35:42: 35:44: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7109877Z     Error {
2019-09-12T19:21:58.7109935Z         line_num: 49,
2019-09-12T19:21:58.7109975Z         kind: Some(
2019-09-12T19:21:58.7110016Z             Error,
2019-09-12T19:21:58.7110016Z             Error,
2019-09-12T19:21:58.7110073Z         ),
2019-09-12T19:21:58.7110737Z         msg: "49:37: 49:39: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7110875Z     Error {
2019-09-12T19:21:58.7110916Z         line_num: 50,
2019-09-12T19:21:58.7110967Z         kind: Some(
2019-09-12T19:21:58.7111008Z             Error,
2019-09-12T19:21:58.7111008Z             Error,
2019-09-12T19:21:58.7111065Z         ),
2019-09-12T19:21:58.7111395Z         msg: "50:41: 50:43: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7111500Z     Error {
2019-09-12T19:21:58.7111540Z         line_num: 62,
2019-09-12T19:21:58.7111581Z         kind: Some(
2019-09-12T19:21:58.7111648Z             Error,
2019-09-12T19:21:58.7111648Z             Error,
2019-09-12T19:21:58.7111688Z         ),
2019-09-12T19:21:58.7111952Z         msg: "62:38: 62:40: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7112059Z     Error {
2019-09-12T19:21:58.7112100Z         line_num: 63,
2019-09-12T19:21:58.7112141Z         kind: Some(
2019-09-12T19:21:58.7112199Z             Error,
2019-09-12T19:21:58.7112199Z             Error,
2019-09-12T19:21:58.7112248Z         ),
2019-09-12T19:21:58.7112513Z         msg: "63:42: 63:44: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7112716Z ]
2019-09-12T19:21:58.7112745Z 
2019-09-12T19:21:58.7113070Z thread '[ui] ui/feature-gates/feature-gate-abi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1512:13
2019-09-12T19:21:58.7113155Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T19:21:58.7113155Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T19:21:58.7113194Z 
2019-09-12T19:21:58.7113431Z ---- [ui] ui/feature-gates/feature-gate-intrinsics.rs stdout ----
2019-09-12T19:21:58.7113464Z 
2019-09-12T19:21:58.7113980Z error: /checkout/src/test/ui/feature-gates/feature-gate-intrinsics.rs:5: unexpected error: '5:34: 5:36: intrinsic must be in `extern "rust-intrinsic" { ... }` block'
2019-09-12T19:21:58.7114067Z error: 1 unexpected errors found, 0 expected errors not found
2019-09-12T19:21:58.7114139Z status: exit code: 1
2019-09-12T19:21:58.7114139Z status: exit code: 1
2019-09-12T19:21:58.7115117Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-intrinsics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-intrinsics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-intrinsics/auxiliary" "-A" "unused"
2019-09-12T19:21:58.7115244Z unexpected errors (from JSON output): [
2019-09-12T19:21:58.7115291Z     Error {
2019-09-12T19:21:58.7115352Z         line_num: 5,
2019-09-12T19:21:58.7115397Z         kind: Some(
2019-09-12T19:21:58.7115482Z         ),
2019-09-12T19:21:58.7115482Z         ),
2019-09-12T19:21:58.7115778Z         msg: "5:34: 5:36: intrinsic must be in `extern \"rust-intrinsic\" { ... }` block",
2019-09-12T19:21:58.7115870Z ]
2019-09-12T19:21:58.7115915Z 
2019-09-12T19:21:58.7116230Z thread '[ui] ui/feature-gates/feature-gate-intrinsics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1512:13
2019-09-12T19:21:58.7116270Z 
---
2019-09-12T19:21:58.7117157Z 
2019-09-12T19:21:58.7132671Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-12T19:21:58.7151675Z 
2019-09-12T19:21:58.7151753Z 
2019-09-12T19:21:58.7153610Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-12T19:21:58.7181897Z 
2019-09-12T19:21:58.7181930Z 
2019-09-12T19:21:58.7234935Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-12T19:21:58.7235009Z Build completed unsuccessfully in 1:06:47
2019-09-12T19:21:58.7235009Z Build completed unsuccessfully in 1:06:47
2019-09-12T19:21:58.7235087Z == clock drift check ==
2019-09-12T19:21:58.7239142Z   local time: Thu Sep 12 19:21:58 UTC 2019
2019-09-12T19:21:58.8829273Z   network time: Thu, 12 Sep 2019 19:21:58 GMT
2019-09-12T19:21:58.8829386Z == end clock drift check ==
2019-09-12T19:21:59.6622135Z ##[error]Bash exited with code '1'.
2019-09-12T19:21:59.6663083Z ##[section]Starting: Checkout
2019-09-12T19:21:59.6665156Z ==============================================================================
2019-09-12T19:21:59.6665210Z Task         : Get sources
2019-09-12T19:21:59.6665256Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
