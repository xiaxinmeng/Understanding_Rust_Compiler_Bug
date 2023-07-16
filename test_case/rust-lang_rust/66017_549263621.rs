plain
2019-11-04T07:33:36.4649816Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T07:33:36.4867439Z ##[command]git config gc.auto 0
2019-11-04T07:33:36.4959058Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T07:33:36.5016725Z ##[command]git config --get-all http.proxy
2019-11-04T07:33:36.5188681Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66017/merge:refs/remotes/pull/66017/merge
---
2019-11-04T08:36:03.5521666Z .................................................................................................... 1600/9272
2019-11-04T08:36:09.6274378Z .................................................................................................... 1700/9272
2019-11-04T08:36:23.0570887Z ..............................................................i...............i..................... 1800/9272
2019-11-04T08:36:30.8744386Z .................................................................................................... 1900/9272
2019-11-04T08:36:47.1950181Z ....................................................iiiii........................................... 2000/9272
2019-11-04T08:36:58.7489692Z .................................................................................................... 2200/9272
2019-11-04T08:37:01.5142569Z .................................................................................................... 2300/9272
2019-11-04T08:37:05.2883442Z .................................................................................................... 2400/9272
2019-11-04T08:37:29.9833818Z .................................................................................................... 2500/9272
2019-11-04T08:37:29.9833818Z .................................................................................................... 2500/9272
2019-11-04T08:37:32.8287702Z .................................................................................................... 2600/9272
2019-11-04T08:37:41.2651235Z .................................................................................................... 2700/9272
2019-11-04T08:37:50.7346468Z ....................i............................................................................... 2800/9272
2019-11-04T08:38:00.1182107Z .................................................................................................... 2900/9272
2019-11-04T08:38:04.9442427Z ...................i................................................................................ 3000/9272
2019-11-04T08:38:14.1680442Z .................................................................................................... 3100/9272
2019-11-04T08:38:20.2577733Z .................................................................................................... 3200/9272
2019-11-04T08:38:29.6011067Z .ii................................................................................................. 3300/9272
2019-11-04T08:38:47.6576390Z ..............................................................................................i..... 3500/9272
2019-11-04T08:38:55.5073536Z .........................................i.......................................................... 3600/9272
2019-11-04T08:39:02.5944813Z .................................................................................................... 3700/9272
2019-11-04T08:39:09.4482337Z .................................................................................................... 3800/9272
---
2019-11-04T08:40:32.5776345Z ....................................................i...............i............................... 4800/9272
2019-11-04T08:40:42.1897473Z .................................................................................................... 4900/9272
2019-11-04T08:40:51.3889132Z .................................................................................................... 5000/9272
2019-11-04T08:40:57.8044183Z .................................................................................................... 5100/9272
2019-11-04T08:41:08.9872455Z ....................................................F.ii.ii...........i............................. 5200/9272
2019-11-04T08:41:19.5994856Z .................................................................................................... 5400/9272
2019-11-04T08:41:30.2547092Z .................................................................................................... 5500/9272
2019-11-04T08:41:38.2746359Z ...........................i........................................................................ 5600/9272
2019-11-04T08:41:45.1398195Z .................................................................................................... 5700/9272
2019-11-04T08:41:45.1398195Z .................................................................................................... 5700/9272
2019-11-04T08:41:57.9824047Z .................................................................................................... 5800/9272
2019-11-04T08:42:10.2859174Z ............ii...i..ii...........i.................................................................. 5900/9272
2019-11-04T08:42:32.0104354Z .................................................................................................... 6100/9272
2019-11-04T08:42:39.0612959Z .................................................................................................... 6200/9272
2019-11-04T08:42:39.0612959Z .................................................................................................... 6200/9272
2019-11-04T08:42:53.8775839Z ...............................i..ii................................................................ 6300/9272
2019-11-04T08:43:15.7731816Z ..................................................................................................i. 6500/9272
2019-11-04T08:43:18.2101085Z .................................................................................................... 6600/9272
2019-11-04T08:43:20.5736523Z ............................................................................i....................... 6700/9272
2019-11-04T08:43:23.5468615Z .................................................................................................... 6800/9272
---
2019-11-04T08:48:32.7024059Z failures:
2019-11-04T08:48:32.7070370Z 
2019-11-04T08:48:32.7071471Z ---- [ui] ui/iterators/into-iter-on-arrays-lint.rs stdout ----
2019-11-04T08:48:32.7071764Z 
2019-11-04T08:48:32.7072634Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:9: unexpected warning: '9:11: 9:20: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]'
2019-11-04T08:48:32.7072929Z 
2019-11-04T08:48:32.7073572Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:9: unexpected warning: '9:11: 9:20: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T08:48:32.7073840Z 
2019-11-04T08:48:32.7074531Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:11: unexpected warning: '11:12: 11:21: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]'
2019-11-04T08:48:32.7074827Z 
2019-11-04T08:48:32.7075447Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:11: unexpected warning: '11:12: 11:21: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T08:48:32.7075797Z 
2019-11-04T08:48:32.7077045Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:13: unexpected warning: '13:9: 13:18: this method call currently resolves to `<&[T] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]'
2019-11-04T08:48:32.7077406Z 
2019-11-04T08:48:32.7078070Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:13: unexpected warning: '13:9: 13:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T08:48:32.7078350Z 
2019-11-04T08:48:32.7079291Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:15: unexpected warning: '15:15: 15:24: this method call currently resolves to `<&[T] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]'
2019-11-04T08:48:32.7079996Z 
2019-11-04T08:48:32.7080690Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:15: unexpected warning: '15:15: 15:24: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T08:48:32.7080971Z 
2019-11-04T08:48:32.7081781Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:10: expected warning not found: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter`
2019-11-04T08:48:32.7082061Z 
2019-11-04T08:48:32.7082858Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:12: expected warning not found: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter`
2019-11-04T08:48:32.7083142Z 
2019-11-04T08:48:32.7084089Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:14: expected warning not found: this method call currently resolves to `<&[T] as IntoIterator>::into_iter`
2019-11-04T08:48:32.7084363Z 
2019-11-04T08:48:32.7085129Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:16: expected warning not found: this method call currently resolves to `<&[T] as IntoIterator>::into_iter`
2019-11-04T08:48:32.7085600Z error: 8 unexpected errors found, 4 expected errors not found
2019-11-04T08:48:32.7086461Z status: exit code: 0
2019-11-04T08:48:32.7086461Z status: exit code: 0
2019-11-04T08:48:32.7087530Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/auxiliary"
2019-11-04T08:48:32.7087907Z unexpected errors (from JSON output): [
2019-11-04T08:48:32.7088367Z         line_num: 9,
2019-11-04T08:48:32.7088579Z         kind: Some(
2019-11-04T08:48:32.7088788Z             Warning,
2019-11-04T08:48:32.7089011Z         ),
2019-11-04T08:48:32.7089011Z         ),
2019-11-04T08:48:32.7089250Z         msg: "9:11: 9:20: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]",
2019-11-04T08:48:32.7089717Z     Error {
2019-11-04T08:48:32.7090092Z         line_num: 9,
2019-11-04T08:48:32.7090471Z         kind: Some(
2019-11-04T08:48:32.7090666Z             Warning,
2019-11-04T08:48:32.7090666Z             Warning,
2019-11-04T08:48:32.7090856Z         ),
2019-11-04T08:48:32.7091178Z         msg: "9:11: 9:20: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T08:48:32.7091641Z     Error {
2019-11-04T08:48:32.7092577Z         line_num: 11,
2019-11-04T08:48:32.7092788Z         kind: Some(
2019-11-04T08:48:32.7092926Z             Warning,
2019-11-04T08:48:32.7092926Z             Warning,
2019-11-04T08:48:32.7093066Z         ),
2019-11-04T08:48:32.7093243Z         msg: "11:12: 11:21: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]",
2019-11-04T08:48:32.7093543Z     Error {
2019-11-04T08:48:32.7093673Z         line_num: 11,
2019-11-04T08:48:32.7093802Z         kind: Some(
2019-11-04T08:48:32.7093948Z             Warning,
2019-11-04T08:48:32.7093948Z             Warning,
2019-11-04T08:48:32.7094077Z         ),
2019-11-04T08:48:32.7094338Z         msg: "11:12: 11:21: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T08:48:32.7094726Z     Error {
2019-11-04T08:48:32.7094869Z         line_num: 13,
2019-11-04T08:48:32.7095001Z         kind: Some(
2019-11-04T08:48:32.7095130Z             Warning,
2019-11-04T08:48:32.7095130Z             Warning,
2019-11-04T08:48:32.7095273Z         ),
2019-11-04T08:48:32.7095451Z         msg: "13:9: 13:18: this method call currently resolves to `<&[T] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]",
2019-11-04T08:48:32.7095742Z     Error {
2019-11-04T08:48:32.7096332Z         line_num: 13,
2019-11-04T08:48:32.7096484Z         kind: Some(
2019-11-04T08:48:32.7096637Z             Warning,
2019-11-04T08:48:32.7096637Z             Warning,
2019-11-04T08:48:32.7096771Z         ),
2019-11-04T08:48:32.7096929Z         msg: "13:9: 13:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T08:48:32.7097233Z     Error {
2019-11-04T08:48:32.7097381Z         line_num: 15,
2019-11-04T08:48:32.7097518Z         kind: Some(
2019-11-04T08:48:32.7097651Z             Warning,
2019-11-04T08:48:32.7097651Z             Warning,
2019-11-04T08:48:32.7097797Z         ),
2019-11-04T08:48:32.7097959Z         msg: "15:15: 15:24: this method call currently resolves to `<&[T] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added. [array_into_iter]",
2019-11-04T08:48:32.7098261Z     Error {
2019-11-04T08:48:32.7098395Z         line_num: 15,
2019-11-04T08:48:32.7098528Z         kind: Some(
2019-11-04T08:48:32.7098678Z             Warning,
2019-11-04T08:48:32.7098678Z             Warning,
2019-11-04T08:48:32.7098808Z         ),
2019-11-04T08:48:32.7098958Z         msg: "15:15: 15:24: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T08:48:32.7099266Z ]
2019-11-04T08:48:32.7099387Z 
2019-11-04T08:48:32.7099704Z not found errors (from test file): [
2019-11-04T08:48:32.7099836Z     Error {
2019-11-04T08:48:32.7099836Z     Error {
2019-11-04T08:48:32.7099966Z         line_num: 10,
2019-11-04T08:48:32.7100112Z         kind: Some(
2019-11-04T08:48:32.7100239Z             Warning,
2019-11-04T08:48:32.7100367Z         ),
2019-11-04T08:48:32.7100525Z         msg: "this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter`",
2019-11-04T08:48:32.7100805Z     Error {
2019-11-04T08:48:32.7100936Z         line_num: 12,
2019-11-04T08:48:32.7104953Z         kind: Some(
2019-11-04T08:48:32.7105489Z             Warning,
2019-11-04T08:48:32.7105489Z             Warning,
2019-11-04T08:48:32.7105635Z         ),
2019-11-04T08:48:32.7106281Z         msg: "this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter`",
2019-11-04T08:48:32.7106563Z     Error {
2019-11-04T08:48:32.7106735Z         line_num: 14,
2019-11-04T08:48:32.7106882Z         kind: Some(
2019-11-04T08:48:32.7107015Z             Warning,
2019-11-04T08:48:32.7107015Z             Warning,
2019-11-04T08:48:32.7107164Z         ),
2019-11-04T08:48:32.7107308Z         msg: "this method call currently resolves to `<&[T] as IntoIterator>::into_iter`",
2019-11-04T08:48:32.7107597Z     Error {
2019-11-04T08:48:32.7107732Z         line_num: 16,
2019-11-04T08:48:32.7107903Z         kind: Some(
2019-11-04T08:48:32.7108039Z             Warning,
2019-11-04T08:48:32.7108039Z             Warning,
2019-11-04T08:48:32.7108172Z         ),
2019-11-04T08:48:32.7108329Z         msg: "this method call currently resolves to `<&[T] as IntoIterator>::into_iter`",
2019-11-04T08:48:32.7108600Z ]
2019-11-04T08:48:32.7108730Z 
2019-11-04T08:48:32.7109344Z thread '[ui] ui/iterators/into-iter-on-arrays-lint.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-04T08:48:32.7109732Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-11-04T08:48:32.7112437Z 
2019-11-04T08:48:32.7112841Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-04T08:48:32.7124624Z 
2019-11-04T08:48:32.7124727Z 
2019-11-04T08:48:32.7126779Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-04T08:48:32.7127038Z 
2019-11-04T08:48:32.7127067Z 
2019-11-04T08:48:32.7132977Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-04T08:48:32.7133058Z Build completed unsuccessfully in 1:08:07
2019-11-04T08:48:32.7133058Z Build completed unsuccessfully in 1:08:07
2019-11-04T08:48:32.7186522Z == clock drift check ==
2019-11-04T08:48:32.7200644Z   local time: Mon Nov  4 08:48:32 UTC 2019
2019-11-04T08:48:32.9974627Z   network time: Mon, 04 Nov 2019 08:48:32 GMT
2019-11-04T08:48:32.9975174Z == end clock drift check ==
2019-11-04T08:48:34.3899293Z 
2019-11-04T08:48:34.4008169Z ##[error]Bash exited with code '1'.
2019-11-04T08:48:34.4038486Z ##[section]Starting: Checkout
2019-11-04T08:48:34.4040276Z ==============================================================================
2019-11-04T08:48:34.4040328Z Task         : Get sources
2019-11-04T08:48:34.4040372Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
