plain
2020-04-09T19:44:24.5709454Z ========================== Starting Command Output ===========================
2020-04-09T19:44:24.5716208Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/19d3e660-d315-4478-9037-fe556bafcd18.sh
2020-04-09T19:44:24.5716788Z 
2020-04-09T19:44:24.5723009Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T19:44:24.5743848Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-04-09T19:44:24.5747634Z Task         : Get sources
2020-04-09T19:44:24.5747964Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T19:44:24.5748304Z Version      : 1.0.0
2020-04-09T19:44:24.5748523Z Author       : Microsoft
---
2020-04-09T19:44:26.0922464Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T19:44:26.0932694Z ##[command]git config gc.auto 0
2020-04-09T19:44:26.0939705Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T19:44:26.0946472Z ##[command]git config --get-all http.proxy
2020-04-09T19:44:26.0957653Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70450/merge:refs/remotes/pull/70450/merge
---
2020-04-09T19:48:30.1604414Z Looks like docker image is the same as before, not uploading
2020-04-09T19:48:38.0181328Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T19:48:38.0502345Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T19:48:38.0538946Z == clock drift check ==
2020-04-09T19:48:38.0548508Z   local time: Thu Apr  9 19:48:38 UTC 2020
2020-04-09T19:48:38.2637065Z   network time: Thu, 09 Apr 2020 19:48:38 GMT
2020-04-09T19:48:38.2663565Z Starting sccache server...
2020-04-09T19:48:38.3572353Z configure: processing command line
2020-04-09T19:48:38.3572667Z configure: 
2020-04-09T19:48:38.3573525Z configure: rust.dist-src        := False
---
2020-04-09T19:54:28.5253611Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T19:54:30.2367618Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T19:54:32.0238013Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T19:54:34.6236179Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T19:54:43.4658239Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T19:54:48.1958726Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T19:54:53.4272485Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T19:54:58.2861253Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T19:55:06.7975713Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T20:20:50.6949594Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T20:20:52.7795458Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T20:20:55.0545462Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T20:20:57.5520452Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T20:21:09.4665512Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T20:21:13.3699690Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T20:21:19.4987436Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T20:21:25.9100314Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T20:21:37.8008939Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T20:50:36.3793296Z .................................................................................................... 1600/9881
2020-04-09T20:50:43.2396810Z .................................................................................................... 1700/9881
2020-04-09T20:50:47.7929242Z .................................................................................................... 1800/9881
2020-04-09T20:50:57.4045344Z .................................................................................................... 1900/9881
2020-04-09T20:51:06.3705628Z i................................................................................................... 2000/9881
2020-04-09T20:51:13.4626378Z ..........................................................................................iiiii..... 2100/9881
2020-04-09T20:51:36.9350894Z .................................................................................................... 2300/9881
2020-04-09T20:51:39.2425295Z .................................................................................................... 2400/9881
2020-04-09T20:51:41.6732023Z .................................................................................................... 2500/9881
2020-04-09T20:51:48.0612030Z .................................................................................................... 2600/9881
---
2020-04-09T20:54:59.5075462Z ................................................................i...............i................... 5000/9881
2020-04-09T20:55:07.4040758Z .................................................................................................... 5100/9881
2020-04-09T20:55:16.1219554Z .................................................................................................... 5200/9881
2020-04-09T20:55:21.8656590Z .........i.......................................................................................... 5300/9881
2020-04-09T20:55:32.5720136Z ..................................................................................................ii 5400/9881
2020-04-09T20:55:37.8069607Z .ii........i...i.................................................................................... 5500/9881
2020-04-09T20:55:46.7007368Z ...........................................i........................................................ 5700/9881
2020-04-09T20:55:57.7239614Z ...............................................................ii................................... 5800/9881
2020-04-09T20:56:05.1022621Z ..i................................................................................................. 5900/9881
2020-04-09T20:56:10.8867005Z .................................................................................................... 6000/9881
2020-04-09T20:56:10.8867005Z .................................................................................................... 6000/9881
2020-04-09T20:56:21.3622540Z ................................................................................................ii.. 6100/9881
2020-04-09T20:56:34.3064696Z .i..ii...........i.................................................................................. 6200/9881
2020-04-09T20:56:51.5459568Z .................................................................................................... 6400/9881
2020-04-09T20:56:57.9870113Z .................................................................................................... 6500/9881
2020-04-09T20:56:57.9870113Z .................................................................................................... 6500/9881
2020-04-09T20:57:13.8548061Z ..........................i..ii..................................................................... 6600/9881
2020-04-09T20:57:36.9241035Z .................................................................................................... 6800/9881
2020-04-09T20:57:39.2698172Z ..........................i......................................................................... 6900/9881
2020-04-09T20:57:41.4962056Z .................................................................................................... 7000/9881
2020-04-09T20:57:43.9106043Z .................................................................i.................................. 7100/9881
---
2020-04-09T20:59:32.8191545Z .................................................................................................... 7800/9881
2020-04-09T20:59:37.3006562Z .................................................................................................... 7900/9881
2020-04-09T20:59:44.2876417Z .................................................................................................... 8000/9881
2020-04-09T20:59:52.0443907Z ..............................i..................................................................... 8100/9881
2020-04-09T21:00:01.1895907Z ..............................................................................iiiiii.iiii.i......... 8200/9881
2020-04-09T21:00:18.7998703Z .......................i......i..................................................................... 8400/9881
2020-04-09T21:00:23.4882202Z .................................................................................................... 8500/9881
2020-04-09T21:00:35.3914887Z .................................................................................................... 8600/9881
2020-04-09T21:00:48.9928085Z .................................................................................................... 8700/9881
---
2020-04-09T21:02:57.1149818Z 
2020-04-09T21:02:57.1151095Z ---- [ui] ui/async-await/mutually-recursive-async-impl-trait-type.rs stdout ----
2020-04-09T21:02:57.1151434Z diff of stderr:
2020-04-09T21:02:57.1151763Z 
2020-04-09T21:02:57.1152261Z + error[E0391]: cycle detected when processing `rec_1::{{opaque}}#0`
2020-04-09T21:02:57.1153577Z +    |
2020-04-09T21:02:57.1153577Z +    |
2020-04-09T21:02:57.1153780Z + LL | async fn rec_1() {
2020-04-09T21:02:57.1154298Z +    |
2020-04-09T21:02:57.1154298Z +    |
2020-04-09T21:02:57.1154817Z + note: ...which requires borrow-checking `rec_1`...
2020-04-09T21:02:57.1155789Z +    |
2020-04-09T21:02:57.1155789Z +    |
2020-04-09T21:02:57.1155987Z + LL | async fn rec_1() {
2020-04-09T21:02:57.1156308Z +    | ^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1156897Z + note: ...which requires borrow-checking `rec_1::{{closure}}#0`...
2020-04-09T21:02:57.1158072Z +    |
2020-04-09T21:02:57.1158072Z +    |
2020-04-09T21:02:57.1158279Z + LL |   async fn rec_1() {
2020-04-09T21:02:57.1158492Z +    |  __________________^
2020-04-09T21:02:57.1158836Z + LL | |     rec_2().await;
2020-04-09T21:02:57.1159041Z + LL | | }
2020-04-09T21:02:57.1159225Z +    | |_^
2020-04-09T21:02:57.1159616Z + note: ...which requires processing `rec_1::{{closure}}#0`...
2020-04-09T21:02:57.1160764Z +    |
2020-04-09T21:02:57.1160764Z +    |
2020-04-09T21:02:57.1160973Z + LL |   async fn rec_1() {
2020-04-09T21:02:57.1161177Z +    |  __________________^
2020-04-09T21:02:57.1161401Z + LL | |     rec_2().await;
2020-04-09T21:02:57.1161578Z + LL | | }
2020-04-09T21:02:57.1161846Z +    | |_^
2020-04-09T21:02:57.1162176Z + note: ...which requires const checking `rec_1::{{closure}}#0`...
2020-04-09T21:02:57.1163058Z +    |
2020-04-09T21:02:57.1163058Z +    |
2020-04-09T21:02:57.1163249Z + LL |   async fn rec_1() {
2020-04-09T21:02:57.1163452Z +    |  __________________^
2020-04-09T21:02:57.1163653Z + LL | |     rec_2().await;
2020-04-09T21:02:57.1163848Z + LL | | }
2020-04-09T21:02:57.1164003Z +    | |_^
2020-04-09T21:02:57.1164839Z +    = note: ...which requires computing whether `[static generator@$DIR/mutually-recursive-async-impl-trait-type.rs:5:18: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]` is freeze...
2020-04-09T21:02:57.1166493Z +    = note: ...which requires evaluating trait selection obligation `[static generator@$DIR/mutually-recursive-async-impl-trait-type.rs:5:18: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1167373Z + note: ...which requires processing `rec_2::{{opaque}}#0`...
2020-04-09T21:02:57.1168591Z +    |
2020-04-09T21:02:57.1168591Z +    |
2020-04-09T21:02:57.1168798Z + LL | async fn rec_2() {
2020-04-09T21:02:57.1169018Z +    |                  ^
2020-04-09T21:02:57.1169615Z + note: ...which requires borrow-checking `rec_2`...
2020-04-09T21:02:57.1170740Z +    |
2020-04-09T21:02:57.1170740Z +    |
2020-04-09T21:02:57.1170937Z + LL | async fn rec_2() {
2020-04-09T21:02:57.1171250Z +    | ^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1171827Z + note: ...which requires borrow-checking `rec_2::{{closure}}#0`...
2020-04-09T21:02:57.1173008Z +    |
2020-04-09T21:02:57.1173008Z +    |
2020-04-09T21:02:57.1173218Z + LL |   async fn rec_2() {
2020-04-09T21:02:57.1173527Z +    |  __________________^
2020-04-09T21:02:57.1173780Z + LL | |     rec_1().await;
2020-04-09T21:02:57.1174055Z + LL | | }
2020-04-09T21:02:57.1174245Z +    | |_^
2020-04-09T21:02:57.1174533Z + note: ...which requires processing `rec_2::{{closure}}#0`...
2020-04-09T21:02:57.1175556Z +    |
2020-04-09T21:02:57.1175556Z +    |
2020-04-09T21:02:57.1175764Z + LL |   async fn rec_2() {
2020-04-09T21:02:57.1176074Z +    |  __________________^
2020-04-09T21:02:57.1176326Z + LL | |     rec_1().await;
2020-04-09T21:02:57.1176741Z + LL | | }
2020-04-09T21:02:57.1176916Z +    | |_^
2020-04-09T21:02:57.1177313Z + note: ...which requires const checking `rec_2::{{closure}}#0`...
2020-04-09T21:02:57.1178477Z +    |
2020-04-09T21:02:57.1178477Z +    |
2020-04-09T21:02:57.1178694Z + LL |   async fn rec_2() {
2020-04-09T21:02:57.1179003Z +    |  __________________^
2020-04-09T21:02:57.1179255Z + LL | |     rec_1().await;
2020-04-09T21:02:57.1179432Z + LL | | }
2020-04-09T21:02:57.1179695Z +    | |_^
2020-04-09T21:02:57.1180618Z +    = note: ...which requires computing whether `[static generator@$DIR/mutually-recursive-async-impl-trait-type.rs:9:18: 11:2 {std::future::ResumeTy, impl std::future::Future, ()}]` is freeze...
2020-04-09T21:02:57.1182104Z +    = note: ...which requires evaluating trait selection obligation `[static generator@$DIR/mutually-recursive-async-impl-trait-type.rs:9:18: 11:2 {std::future::ResumeTy, impl std::future::Future, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1183051Z +    = note: ...which again requires processing `rec_1::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1183826Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1184978Z +    |
2020-04-09T21:02:57.1184978Z +    |
2020-04-09T21:02:57.1185179Z + LL | async fn rec_1() {
2020-04-09T21:02:57.1185687Z + 
2020-04-09T21:02:57.1185930Z 1 error[E0733]: recursion in an `async fn` requires boxing
2020-04-09T21:02:57.1186635Z 2   --> $DIR/mutually-recursive-async-impl-trait-type.rs:5:18
2020-04-09T21:02:57.1187037Z 3    |
---
2020-04-09T21:02:57.1271939Z 20 
2020-04-09T21:02:57.1272046Z 
2020-04-09T21:02:57.1272142Z 
2020-04-09T21:02:57.1275059Z The actual stderr differed from the expected stderr.
2020-04-09T21:02:57.1275990Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/mutually-recursive-async-impl-trait-type.stderr
2020-04-09T21:02:57.1276728Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T21:02:57.1277414Z To only update this specific test, also pass `--test-args async-await/mutually-recursive-async-impl-trait-type.rs`
2020-04-09T21:02:57.1277891Z error: 1 errors occurred comparing output.
2020-04-09T21:02:57.1278133Z status: exit code: 1
2020-04-09T21:02:57.1278133Z status: exit code: 1
2020-04-09T21:02:57.1280565Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/auxiliary"
2020-04-09T21:02:57.1282429Z ------------------------------------------
2020-04-09T21:02:57.1282610Z 
2020-04-09T21:02:57.1282996Z ------------------------------------------
2020-04-09T21:02:57.1283337Z stderr:
2020-04-09T21:02:57.1283337Z stderr:
2020-04-09T21:02:57.1296818Z ------------------------------------------
2020-04-09T21:02:57.1297188Z error[E0391]: cycle detected when processing `rec_1::{{opaque}}#0`
2020-04-09T21:02:57.1298500Z    |
2020-04-09T21:02:57.1298500Z    |
2020-04-09T21:02:57.1298757Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1299174Z    |
2020-04-09T21:02:57.1299174Z    |
2020-04-09T21:02:57.1299641Z note: ...which requires borrow-checking `rec_1`...
2020-04-09T21:02:57.1300527Z    |
2020-04-09T21:02:57.1300527Z    |
2020-04-09T21:02:57.1300750Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1301019Z    | ^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1301515Z note: ...which requires borrow-checking `rec_1::{{closure}}#0`...
2020-04-09T21:02:57.1302474Z    |
2020-04-09T21:02:57.1302474Z    |
2020-04-09T21:02:57.1302705Z LL |   async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1302976Z    |  __________________^
2020-04-09T21:02:57.1303195Z LL | |     rec_2().await;
2020-04-09T21:02:57.1303511Z    | |_^
2020-04-09T21:02:57.1303511Z    | |_^
2020-04-09T21:02:57.1303783Z note: ...which requires processing `rec_1::{{closure}}#0`...
2020-04-09T21:02:57.1304706Z    |
2020-04-09T21:02:57.1304706Z    |
2020-04-09T21:02:57.1304945Z LL |   async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1305203Z    |  __________________^
2020-04-09T21:02:57.1305397Z LL | |     rec_2().await;
2020-04-09T21:02:57.1305719Z    | |_^
2020-04-09T21:02:57.1305719Z    | |_^
2020-04-09T21:02:57.1305978Z note: ...which requires const checking `rec_1::{{closure}}#0`...
2020-04-09T21:02:57.1307108Z    |
2020-04-09T21:02:57.1307108Z    |
2020-04-09T21:02:57.1307333Z LL |   async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1307749Z    |  __________________^
2020-04-09T21:02:57.1307965Z LL | |     rec_2().await;
2020-04-09T21:02:57.1308280Z    | |_^
2020-04-09T21:02:57.1308280Z    | |_^
2020-04-09T21:02:57.1309528Z    = note: ...which requires computing whether `[static generator@/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:5:18: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]` is freeze...
2020-04-09T21:02:57.1311210Z    = note: ...which requires evaluating trait selection obligation `[static generator@/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:5:18: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1311973Z note: ...which requires processing `rec_2::{{opaque}}#0`...
2020-04-09T21:02:57.1312917Z    |
2020-04-09T21:02:57.1312917Z    |
2020-04-09T21:02:57.1313159Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1313422Z    |                  ^
2020-04-09T21:02:57.1313867Z note: ...which requires borrow-checking `rec_2`...
2020-04-09T21:02:57.1314764Z    |
2020-04-09T21:02:57.1314764Z    |
2020-04-09T21:02:57.1314990Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1315263Z    | ^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1315756Z note: ...which requires borrow-checking `rec_2::{{closure}}#0`...
2020-04-09T21:02:57.1316714Z    |
2020-04-09T21:02:57.1316714Z    |
2020-04-09T21:02:57.1316942Z LL |   async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1317327Z    |  __________________^
2020-04-09T21:02:57.1317541Z LL | |     rec_1().await;
2020-04-09T21:02:57.1317849Z    | |_^
2020-04-09T21:02:57.1317849Z    | |_^
2020-04-09T21:02:57.1318123Z note: ...which requires processing `rec_2::{{closure}}#0`...
2020-04-09T21:02:57.1319088Z    |
2020-04-09T21:02:57.1319088Z    |
2020-04-09T21:02:57.1319337Z LL |   async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1319596Z    |  __________________^
2020-04-09T21:02:57.1319793Z LL | |     rec_1().await;
2020-04-09T21:02:57.1320120Z    | |_^
2020-04-09T21:02:57.1320120Z    | |_^
2020-04-09T21:02:57.1320380Z note: ...which requires const checking `rec_2::{{closure}}#0`...
2020-04-09T21:02:57.1321373Z    |
2020-04-09T21:02:57.1321373Z    |
2020-04-09T21:02:57.1321604Z LL |   async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1321879Z    |  __________________^
2020-04-09T21:02:57.1322098Z LL | |     rec_1().await;
2020-04-09T21:02:57.1322405Z    | |_^
2020-04-09T21:02:57.1322405Z    | |_^
2020-04-09T21:02:57.1323351Z    = note: ...which requires computing whether `[static generator@/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18: 11:2 {std::future::ResumeTy, impl std::future::Future, ()}]` is freeze...
2020-04-09T21:02:57.1324773Z    = note: ...which requires evaluating trait selection obligation `[static generator@/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18: 11:2 {std::future::ResumeTy, impl std::future::Future, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1325593Z    = note: ...which again requires processing `rec_1::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1326188Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1327099Z    |
2020-04-09T21:02:57.1327099Z    |
2020-04-09T21:02:57.1327324Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1327710Z 
2020-04-09T21:02:57.1328026Z error[E0733]: recursion in an `async fn` requires boxing
2020-04-09T21:02:57.1328665Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:5:18
2020-04-09T21:02:57.1328955Z    |
2020-04-09T21:02:57.1328955Z    |
2020-04-09T21:02:57.1329196Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1329483Z    |                  ^ recursive `async fn`
2020-04-09T21:02:57.1329963Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-04-09T21:02:57.1330201Z 
2020-04-09T21:02:57.1330414Z error[E0733]: recursion in an `async fn` requires boxing
2020-04-09T21:02:57.1331031Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18
2020-04-09T21:02:57.1331031Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18
2020-04-09T21:02:57.1331345Z    |
2020-04-09T21:02:57.1331575Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-04-09T21:02:57.1331873Z    |                  ^ recursive `async fn`
2020-04-09T21:02:57.1332359Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-04-09T21:02:57.1332606Z 
2020-04-09T21:02:57.1332810Z error: aborting due to 3 previous errors
2020-04-09T21:02:57.1332978Z 
---
2020-04-09T21:02:57.1334599Z 
2020-04-09T21:02:57.1335051Z ---- [ui] ui/async-await/recursive-async-impl-trait-type.rs stdout ----
2020-04-09T21:02:57.1335311Z diff of stderr:
2020-04-09T21:02:57.1335513Z 
2020-04-09T21:02:57.1335910Z + error[E0391]: cycle detected when processing `recursive_async_function::{{opaque}}#0`
2020-04-09T21:02:57.1336889Z +    |
2020-04-09T21:02:57.1337290Z + LL | async fn recursive_async_function() -> () {
2020-04-09T21:02:57.1337587Z +    |                                        ^^
2020-04-09T21:02:57.1337787Z +    |
2020-04-09T21:02:57.1337787Z +    |
2020-04-09T21:02:57.1338242Z + note: ...which requires borrow-checking `recursive_async_function`...
2020-04-09T21:02:57.1339023Z +    |
2020-04-09T21:02:57.1339414Z + LL | async fn recursive_async_function() -> () {
2020-04-09T21:02:57.1339718Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1339718Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1340311Z + note: ...which requires borrow-checking `recursive_async_function::{{closure}}#0`...
2020-04-09T21:02:57.1341265Z +    |
2020-04-09T21:02:57.1341654Z + LL |   async fn recursive_async_function() -> () {
2020-04-09T21:02:57.1341930Z +    |  ___________________________________________^
2020-04-09T21:02:57.1341930Z +    |  ___________________________________________^
2020-04-09T21:02:57.1342224Z + LL | |     recursive_async_function().await;
2020-04-09T21:02:57.1342429Z + LL | | }
2020-04-09T21:02:57.1342570Z +    | |_^
2020-04-09T21:02:57.1342880Z + note: ...which requires processing `recursive_async_function::{{closure}}#0`...
2020-04-09T21:02:57.1343673Z +    |
2020-04-09T21:02:57.1344074Z + LL |   async fn recursive_async_function() -> () {
2020-04-09T21:02:57.1344357Z +    |  ___________________________________________^
2020-04-09T21:02:57.1344357Z +    |  ___________________________________________^
2020-04-09T21:02:57.1344632Z + LL | |     recursive_async_function().await;
2020-04-09T21:02:57.1344838Z + LL | | }
2020-04-09T21:02:57.1344995Z +    | |_^
2020-04-09T21:02:57.1345293Z + note: ...which requires const checking `recursive_async_function::{{closure}}#0`...
2020-04-09T21:02:57.1346228Z +    |
2020-04-09T21:02:57.1346629Z + LL |   async fn recursive_async_function() -> () {
2020-04-09T21:02:57.1346913Z +    |  ___________________________________________^
2020-04-09T21:02:57.1346913Z +    |  ___________________________________________^
2020-04-09T21:02:57.1347280Z + LL | |     recursive_async_function().await;
2020-04-09T21:02:57.1347497Z + LL | | }
2020-04-09T21:02:57.1347737Z +    | |_^
2020-04-09T21:02:57.1351993Z +    = note: ...which requires computing whether `[static generator@$DIR/recursive-async-impl-trait-type.rs:5:43: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]` is freeze...
2020-04-09T21:02:57.1353619Z +    = note: ...which requires evaluating trait selection obligation `[static generator@$DIR/recursive-async-impl-trait-type.rs:5:43: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1354432Z +    = note: ...which again requires processing `recursive_async_function::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1355111Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1355877Z +    |
2020-04-09T21:02:57.1356542Z + LL | async fn recursive_async_function() -> () {
2020-04-09T21:02:57.1356843Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1357064Z + 
---
2020-04-09T21:02:57.1361508Z 12 
2020-04-09T21:02:57.1361616Z 
2020-04-09T21:02:57.1361714Z 
2020-04-09T21:02:57.1361943Z The actual stderr differed from the expected stderr.
2020-04-09T21:02:57.1362700Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2020-04-09T21:02:57.1363388Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T21:02:57.1364256Z To only update this specific test, also pass `--test-args async-await/recursive-async-impl-trait-type.rs`
2020-04-09T21:02:57.1364723Z error: 1 errors occurred comparing output.
2020-04-09T21:02:57.1364955Z status: exit code: 1
2020-04-09T21:02:57.1364955Z status: exit code: 1
2020-04-09T21:02:57.1367127Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/auxiliary"
2020-04-09T21:02:57.1369078Z ------------------------------------------
2020-04-09T21:02:57.1369251Z 
2020-04-09T21:02:57.1369644Z ------------------------------------------
2020-04-09T21:02:57.1369844Z stderr:
2020-04-09T21:02:57.1369844Z stderr:
2020-04-09T21:02:57.1370217Z ------------------------------------------
2020-04-09T21:02:57.1370592Z error[E0391]: cycle detected when processing `recursive_async_function::{{opaque}}#0`
2020-04-09T21:02:57.1371550Z    |
2020-04-09T21:02:57.1371550Z    |
2020-04-09T21:02:57.1372090Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1372597Z    |
2020-04-09T21:02:57.1372597Z    |
2020-04-09T21:02:57.1373077Z note: ...which requires borrow-checking `recursive_async_function`...
2020-04-09T21:02:57.1374191Z    |
2020-04-09T21:02:57.1374191Z    |
2020-04-09T21:02:57.1374605Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1374919Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1375504Z note: ...which requires borrow-checking `recursive_async_function::{{closure}}#0`...
2020-04-09T21:02:57.1376589Z    |
2020-04-09T21:02:57.1376589Z    |
2020-04-09T21:02:57.1377055Z LL |   async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1377360Z    |  ___________________________________________^
2020-04-09T21:02:57.1377659Z LL | |     recursive_async_function().await;
2020-04-09T21:02:57.1378007Z    | |_^
2020-04-09T21:02:57.1378007Z    | |_^
2020-04-09T21:02:57.1378330Z note: ...which requires processing `recursive_async_function::{{closure}}#0`...
2020-04-09T21:02:57.1379265Z    |
2020-04-09T21:02:57.1379265Z    |
2020-04-09T21:02:57.1379713Z LL |   async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1380026Z    |  ___________________________________________^
2020-04-09T21:02:57.1380307Z LL | |     recursive_async_function().await;
2020-04-09T21:02:57.1380820Z    | |_^
2020-04-09T21:02:57.1380820Z    | |_^
2020-04-09T21:02:57.1381129Z note: ...which requires const checking `recursive_async_function::{{closure}}#0`...
2020-04-09T21:02:57.1382309Z    |
2020-04-09T21:02:57.1382309Z    |
2020-04-09T21:02:57.1382740Z LL |   async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1383083Z    |  ___________________________________________^
2020-04-09T21:02:57.1383351Z LL | |     recursive_async_function().await;
2020-04-09T21:02:57.1383870Z    | |_^
2020-04-09T21:02:57.1383870Z    | |_^
2020-04-09T21:02:57.1384781Z    = note: ...which requires computing whether `[static generator@/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs:5:43: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]` is freeze...
2020-04-09T21:02:57.1386148Z    = note: ...which requires evaluating trait selection obligation `[static generator@/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs:5:43: 7:2 {std::future::ResumeTy, impl std::future::Future, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1386976Z    = note: ...which again requires processing `recursive_async_function::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1387614Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1388500Z    |
2020-04-09T21:02:57.1388500Z    |
2020-04-09T21:02:57.1389573Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1390825Z 
2020-04-09T21:02:57.1391057Z error[E0733]: recursion in an `async fn` requires boxing
2020-04-09T21:02:57.1391735Z   --> /checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs:5:40
2020-04-09T21:02:57.1392217Z    |
2020-04-09T21:02:57.1392217Z    |
2020-04-09T21:02:57.1392675Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-04-09T21:02:57.1393013Z    |                                        ^^ recursive `async fn`
2020-04-09T21:02:57.1393548Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-04-09T21:02:57.1393792Z 
2020-04-09T21:02:57.1394004Z error: aborting due to 2 previous errors
2020-04-09T21:02:57.1394171Z 
---
2020-04-09T21:02:57.1396969Z 
2020-04-09T21:02:57.1397093Z 14    |
2020-04-09T21:02:57.1397480Z 15 LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1397718Z 16    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1397987Z + note: ...which requires const checking `cycle1`...
2020-04-09T21:02:57.1398690Z +    |
2020-04-09T21:02:57.1399044Z + LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1399294Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1399294Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1399550Z 17 note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1400206Z 19    |
2020-04-09T21:02:57.1400337Z 
2020-04-09T21:02:57.1400460Z 50    |
2020-04-09T21:02:57.1400821Z 51 LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1400821Z 51 LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1401077Z 52    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1401336Z + note: ...which requires const checking `cycle2`...
2020-04-09T21:02:57.1402191Z +    |
2020-04-09T21:02:57.1402553Z + LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1402782Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1402782Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1403032Z 53 note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1403826Z 55    |
2020-04-09T21:02:57.1403935Z 
2020-04-09T21:02:57.1404086Z 100    |
2020-04-09T21:02:57.1404489Z 101 LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1404489Z 101 LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1404721Z 102    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1404998Z + note: ...which requires const checking `cycle1`...
2020-04-09T21:02:57.1405673Z +    |
2020-04-09T21:02:57.1406024Z + LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1406274Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1406274Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1406534Z 103 note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1407226Z 105    |
2020-04-09T21:02:57.1407336Z 
2020-04-09T21:02:57.1407460Z 136    |
2020-04-09T21:02:57.1407824Z 137 LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1407824Z 137 LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1408082Z 138    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1409462Z + note: ...which requires const checking `cycle2`...
2020-04-09T21:02:57.1410234Z +    |
2020-04-09T21:02:57.1410591Z + LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1410824Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1410824Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1411114Z 139 note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1411780Z 141    |
2020-04-09T21:02:57.1411890Z 
2020-04-09T21:02:57.1412039Z 185    |
2020-04-09T21:02:57.1412401Z 186 LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1412401Z 186 LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1412635Z 187    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1412922Z + note: ...which requires const checking `cycle1`...
2020-04-09T21:02:57.1413572Z +    |
2020-04-09T21:02:57.1413947Z + LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1414177Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1414177Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1414443Z 188 note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1415137Z 190    |
2020-04-09T21:02:57.1415249Z 
2020-04-09T21:02:57.1415599Z 217 LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1415858Z 218    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1415858Z 218    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1416234Z 219 note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1416959Z +    |
2020-04-09T21:02:57.1417313Z + LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1417539Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1417539Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1417822Z + note: ...which requires const checking `cycle2`...
2020-04-09T21:02:57.1418485Z 221    |
2020-04-09T21:02:57.1418867Z 222 LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1419030Z 
2020-04-09T21:02:57.1419124Z 
2020-04-09T21:02:57.1419124Z 
2020-04-09T21:02:57.1419331Z The actual stderr differed from the expected stderr.
2020-04-09T21:02:57.1420037Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
2020-04-09T21:02:57.1420685Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T21:02:57.1421283Z To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`
2020-04-09T21:02:57.1421755Z error: 1 errors occurred comparing output.
2020-04-09T21:02:57.1421990Z status: exit code: 1
2020-04-09T21:02:57.1421990Z status: exit code: 1
2020-04-09T21:02:57.1424135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
2020-04-09T21:02:57.1425927Z ------------------------------------------
2020-04-09T21:02:57.1426101Z 
2020-04-09T21:02:57.1426473Z ------------------------------------------
2020-04-09T21:02:57.1426677Z stderr:
2020-04-09T21:02:57.1426677Z stderr:
2020-04-09T21:02:57.1427077Z ------------------------------------------
2020-04-09T21:02:57.1427402Z error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
2020-04-09T21:02:57.1428244Z    |
2020-04-09T21:02:57.1428789Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1429022Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1429218Z    |
2020-04-09T21:02:57.1429218Z    |
2020-04-09T21:02:57.1429663Z note: ...which requires borrow-checking `cycle1`...
2020-04-09T21:02:57.1430444Z    |
2020-04-09T21:02:57.1430790Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1431019Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1431019Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1431285Z note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1432037Z    |
2020-04-09T21:02:57.1432408Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1432630Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1432630Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1432878Z note: ...which requires const checking `cycle1`...
2020-04-09T21:02:57.1433658Z    |
2020-04-09T21:02:57.1434003Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1434222Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1434222Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1434486Z note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1435239Z    |
2020-04-09T21:02:57.1435609Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1435828Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1435828Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1436278Z note: ...which requires unsafety-checking `cycle1`...
2020-04-09T21:02:57.1437063Z    |
2020-04-09T21:02:57.1437511Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1437767Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1438005Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1438005Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1438542Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:12:1
2020-04-09T21:02:57.1438800Z    |
2020-04-09T21:02:57.1439148Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1439366Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1439829Z note: ...which requires type-checking `cycle1`...
2020-04-09T21:02:57.1440581Z    |
2020-04-09T21:02:57.1440920Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1441164Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1441578Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1441578Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1442095Z note: ...which requires processing `cycle2::{{opaque}}#0`...
2020-04-09T21:02:57.1442902Z    |
2020-04-09T21:02:57.1443271Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1443491Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1443491Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1443928Z note: ...which requires borrow-checking `cycle2`...
2020-04-09T21:02:57.1444700Z    |
2020-04-09T21:02:57.1445042Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1445261Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1445261Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1445526Z note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1446270Z    |
2020-04-09T21:02:57.1446765Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1446984Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1446984Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1447237Z note: ...which requires const checking `cycle2`...
2020-04-09T21:02:57.1448024Z    |
2020-04-09T21:02:57.1448368Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1448618Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1448618Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1448860Z note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1449645Z    |
2020-04-09T21:02:57.1449991Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1450211Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1450211Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1450676Z note: ...which requires unsafety-checking `cycle2`...
2020-04-09T21:02:57.1451448Z    |
2020-04-09T21:02:57.1451789Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1452027Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1480814Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1480814Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1481544Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:22:1
2020-04-09T21:02:57.1481789Z    |
2020-04-09T21:02:57.1482192Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1482418Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1482879Z note: ...which requires type-checking `cycle2`...
2020-04-09T21:02:57.1483913Z    |
2020-04-09T21:02:57.1484319Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1484562Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1484986Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1484986Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1485537Z    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1486229Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1487015Z    |
2020-04-09T21:02:57.1487205Z LL | / use std::cell::Cell;
2020-04-09T21:02:57.1487432Z LL | | use std::rc::Rc;
2020-04-09T21:02:57.1487766Z LL | |
2020-04-09T21:02:57.1487766Z LL | |
2020-04-09T21:02:57.1487981Z LL | | fn send<T: Send>(_: T) {}
2020-04-09T21:02:57.1488406Z LL | |     Rc::new(String::from("foo"))
2020-04-09T21:02:57.1488620Z LL | | }
2020-04-09T21:02:57.1488828Z    | |_^
2020-04-09T21:02:57.1488944Z 
2020-04-09T21:02:57.1488944Z 
2020-04-09T21:02:57.1489205Z error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
2020-04-09T21:02:57.1490089Z    |
2020-04-09T21:02:57.1490435Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1490975Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1491185Z    |
2020-04-09T21:02:57.1491185Z    |
2020-04-09T21:02:57.1491645Z note: ...which requires borrow-checking `cycle1`...
2020-04-09T21:02:57.1492779Z    |
2020-04-09T21:02:57.1493141Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1493352Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1493352Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1493606Z note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1494490Z    |
2020-04-09T21:02:57.1494843Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1495066Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1495066Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1495324Z note: ...which requires const checking `cycle1`...
2020-04-09T21:02:57.1496090Z    |
2020-04-09T21:02:57.1496427Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1496646Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1496646Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1497009Z note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1497851Z    |
2020-04-09T21:02:57.1498359Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1498578Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1498578Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1499016Z note: ...which requires unsafety-checking `cycle1`...
2020-04-09T21:02:57.1499792Z    |
2020-04-09T21:02:57.1500128Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1500366Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1500613Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1500613Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1501117Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:12:1
2020-04-09T21:02:57.1501365Z    |
2020-04-09T21:02:57.1501706Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1501925Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1502362Z note: ...which requires type-checking `cycle1`...
2020-04-09T21:02:57.1503110Z    |
2020-04-09T21:02:57.1503445Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1503679Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1504091Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1504091Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1504584Z note: ...which requires processing `cycle2::{{opaque}}#0`...
2020-04-09T21:02:57.1505412Z    |
2020-04-09T21:02:57.1505750Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1505994Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1505994Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1506433Z note: ...which requires borrow-checking `cycle2`...
2020-04-09T21:02:57.1507198Z    |
2020-04-09T21:02:57.1507540Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1507757Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1507757Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1508014Z note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1508978Z    |
2020-04-09T21:02:57.1509374Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1509593Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1509593Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1509959Z note: ...which requires const checking `cycle2`...
2020-04-09T21:02:57.1510771Z    |
2020-04-09T21:02:57.1511110Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1511344Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1511344Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1511587Z note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1512329Z    |
2020-04-09T21:02:57.1512783Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1512993Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1512993Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1513416Z note: ...which requires unsafety-checking `cycle2`...
2020-04-09T21:02:57.1514164Z    |
2020-04-09T21:02:57.1514484Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1514709Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1514942Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1514942Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1515432Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:22:1
2020-04-09T21:02:57.1515679Z    |
2020-04-09T21:02:57.1516008Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1516344Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1516788Z note: ...which requires type-checking `cycle2`...
2020-04-09T21:02:57.1517535Z    |
2020-04-09T21:02:57.1517881Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1518100Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1518100Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1518453Z    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1519179Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1519947Z    |
2020-04-09T21:02:57.1520126Z LL | / use std::cell::Cell;
2020-04-09T21:02:57.1520374Z LL | | use std::rc::Rc;
2020-04-09T21:02:57.1520553Z LL | |
2020-04-09T21:02:57.1520553Z LL | |
2020-04-09T21:02:57.1520750Z LL | | fn send<T: Send>(_: T) {}
2020-04-09T21:02:57.1521173Z LL | |     Rc::new(String::from("foo"))
2020-04-09T21:02:57.1521387Z LL | | }
2020-04-09T21:02:57.1521545Z    | |_^
2020-04-09T21:02:57.1521660Z 
2020-04-09T21:02:57.1521660Z 
2020-04-09T21:02:57.1521919Z error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
2020-04-09T21:02:57.1522768Z    |
2020-04-09T21:02:57.1523112Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1523334Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1523523Z    |
2020-04-09T21:02:57.1523523Z    |
2020-04-09T21:02:57.1523925Z note: ...which requires borrow-checking `cycle1`...
2020-04-09T21:02:57.1524683Z    |
2020-04-09T21:02:57.1525020Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1525244Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1525244Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1525501Z note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1526263Z    |
2020-04-09T21:02:57.1526612Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1526831Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1526831Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1527089Z note: ...which requires const checking `cycle1`...
2020-04-09T21:02:57.1527863Z    |
2020-04-09T21:02:57.1528202Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1528428Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1528428Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1528694Z note: ...which requires processing `cycle1`...
2020-04-09T21:02:57.1529456Z    |
2020-04-09T21:02:57.1529812Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1530030Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1530030Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1530544Z note: ...which requires unsafety-checking `cycle1`...
2020-04-09T21:02:57.1531341Z    |
2020-04-09T21:02:57.1531678Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1531914Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1532162Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1532162Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1532670Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:12:1
2020-04-09T21:02:57.1532926Z    |
2020-04-09T21:02:57.1533270Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1533488Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1533911Z note: ...which requires type-checking `cycle1`...
2020-04-09T21:02:57.1534676Z    |
2020-04-09T21:02:57.1535009Z LL | fn cycle1() -> impl Clone {
2020-04-09T21:02:57.1535249Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1535671Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1535671Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-04-09T21:02:57.1536164Z note: ...which requires processing `cycle2::{{opaque}}#0`...
2020-04-09T21:02:57.1536976Z    |
2020-04-09T21:02:57.1537308Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1537541Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1537541Z    |                ^^^^^^^^^^
2020-04-09T21:02:57.1537981Z note: ...which requires borrow-checking `cycle2`...
2020-04-09T21:02:57.1538747Z    |
2020-04-09T21:02:57.1539086Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1539303Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1539303Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1539651Z note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1540434Z    |
2020-04-09T21:02:57.1540786Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1541020Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1541020Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1541279Z note: ...which requires const checking `cycle2`...
2020-04-09T21:02:57.1542082Z    |
2020-04-09T21:02:57.1542419Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1542637Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1542637Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1542894Z note: ...which requires processing `cycle2`...
2020-04-09T21:02:57.1543627Z    |
2020-04-09T21:02:57.1543981Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1544199Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1544199Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1544639Z note: ...which requires unsafety-checking `cycle2`...
2020-04-09T21:02:57.1545414Z    |
2020-04-09T21:02:57.1545747Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1545980Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1546730Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1546730Z note: ...which requires building MIR for...
2020-04-09T21:02:57.1547541Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:22:1
2020-04-09T21:02:57.1547802Z    |
2020-04-09T21:02:57.1548151Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1548369Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1548998Z note: ...which requires type-checking `cycle2`...
2020-04-09T21:02:57.1549766Z    |
2020-04-09T21:02:57.1550102Z LL | fn cycle2() -> impl Clone {
2020-04-09T21:02:57.1550337Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1550337Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1550681Z    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1551290Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1552072Z    |
2020-04-09T21:02:57.1552253Z LL | / use std::cell::Cell;
2020-04-09T21:02:57.1552613Z LL | | use std::rc::Rc;
2020-04-09T21:02:57.1552802Z LL | |
2020-04-09T21:02:57.1552802Z LL | |
2020-04-09T21:02:57.1552999Z LL | | fn send<T: Send>(_: T) {}
2020-04-09T21:02:57.1553422Z LL | |     Rc::new(String::from("foo"))
2020-04-09T21:02:57.1553637Z LL | | }
2020-04-09T21:02:57.1553796Z    | |_^
2020-04-09T21:02:57.1553912Z 
2020-04-09T21:02:57.1553912Z 
2020-04-09T21:02:57.1554198Z error[E0277]: `std::rc::Rc<std::string::String>` cannot be sent between threads safely
2020-04-09T21:02:57.1554832Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:16:5
2020-04-09T21:02:57.1555094Z    |
2020-04-09T21:02:57.1555282Z LL | fn send<T: Send>(_: T) {}
2020-04-09T21:02:57.1555969Z ...
2020-04-09T21:02:57.1556143Z LL |     send(cycle2().clone());
2020-04-09T21:02:57.1556483Z    |     ^^^^ `std::rc::Rc<std::string::String>` cannot be sent between threads safely
2020-04-09T21:02:57.1556789Z ...
---
2020-04-09T21:02:57.1561303Z 
2020-04-09T21:02:57.1561776Z ---- [ui] ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs stdout ----
2020-04-09T21:02:57.1562065Z diff of stderr:
2020-04-09T21:02:57.1562192Z 
2020-04-09T21:02:57.1562465Z + error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-04-09T21:02:57.1563259Z +    |
2020-04-09T21:02:57.1563259Z +    |
2020-04-09T21:02:57.1563611Z + LL | fn foo() -> impl Quux {
2020-04-09T21:02:57.1564006Z +    |
2020-04-09T21:02:57.1564006Z +    |
2020-04-09T21:02:57.1564415Z + note: ...which requires borrow-checking `foo`...
2020-04-09T21:02:57.1565137Z +    |
2020-04-09T21:02:57.1565137Z +    |
2020-04-09T21:02:57.1565469Z + LL | fn foo() -> impl Quux {
2020-04-09T21:02:57.1566085Z + note: ...which requires processing `foo`...
2020-04-09T21:02:57.1566592Z +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:1
2020-04-09T21:02:57.1566836Z +    |
2020-04-09T21:02:57.1566836Z +    |
2020-04-09T21:02:57.1567173Z + LL | fn foo() -> impl Quux {
2020-04-09T21:02:57.1567655Z + note: ...which requires const checking `foo`...
2020-04-09T21:02:57.1568155Z +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:1
2020-04-09T21:02:57.1568390Z +    |
2020-04-09T21:02:57.1568390Z +    |
2020-04-09T21:02:57.1568738Z + LL | fn foo() -> impl Quux {
2020-04-09T21:02:57.1568950Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1569252Z +    = note: ...which requires computing whether `impl Quux` is freeze...
2020-04-09T21:02:57.1569749Z +    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-04-09T21:02:57.1570219Z + note: ...which requires processing `bar::{{opaque}}#0`...
2020-04-09T21:02:57.1571010Z +    |
2020-04-09T21:02:57.1571355Z + LL | fn bar() -> impl Quux {
2020-04-09T21:02:57.1571568Z +    |             ^^^^^^^^^
2020-04-09T21:02:57.1571568Z +    |             ^^^^^^^^^
2020-04-09T21:02:57.1572004Z + note: ...which requires borrow-checking `bar`...
2020-04-09T21:02:57.1572870Z +    |
2020-04-09T21:02:57.1573245Z + LL | fn bar() -> impl Quux {
2020-04-09T21:02:57.1573485Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1573485Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1573734Z + note: ...which requires processing `bar`...
2020-04-09T21:02:57.1574471Z +    |
2020-04-09T21:02:57.1574809Z + LL | fn bar() -> impl Quux {
2020-04-09T21:02:57.1575020Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1575020Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1575282Z + note: ...which requires const checking `bar`...
2020-04-09T21:02:57.1576011Z +    |
2020-04-09T21:02:57.1576373Z + LL | fn bar() -> impl Quux {
2020-04-09T21:02:57.1576587Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1576587Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1576888Z +    = note: ...which requires computing whether `impl Quux` is freeze...
2020-04-09T21:02:57.1577386Z +    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-04-09T21:02:57.1577913Z +    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1578514Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1579276Z +    |
2020-04-09T21:02:57.1579276Z +    |
2020-04-09T21:02:57.1579432Z + LL | trait Quux {}
2020-04-09T21:02:57.1579780Z + 
2020-04-09T21:02:57.1580005Z 1 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1580534Z 2   --> $DIR/infinite-impl-trait-issue-38064.rs:8:13
2020-04-09T21:02:57.1580769Z 3    |
2020-04-09T21:02:57.1580769Z 3    |
2020-04-09T21:02:57.1580879Z 
2020-04-09T21:02:57.1581105Z 14    |
2020-04-09T21:02:57.1581382Z 15    = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`
2020-04-09T21:02:57.1582035Z - error: aborting due to 2 previous errors
2020-04-09T21:02:57.1582323Z + error: aborting due to 3 previous errors
2020-04-09T21:02:57.1582521Z 18 
2020-04-09T21:02:57.1582963Z - For more information about this error, try `rustc --explain E0720`.
2020-04-09T21:02:57.1582963Z - For more information about this error, try `rustc --explain E0720`.
2020-04-09T21:02:57.1583333Z + Some errors have detailed explanations: E0391, E0720.
2020-04-09T21:02:57.1583876Z + For more information about an error, try `rustc --explain E0391`.
2020-04-09T21:02:57.1584107Z 20 
2020-04-09T21:02:57.1584207Z 
2020-04-09T21:02:57.1584318Z 
2020-04-09T21:02:57.1584527Z The actual stderr differed from the expected stderr.
2020-04-09T21:02:57.1585296Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
2020-04-09T21:02:57.1586008Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T21:02:57.1586680Z To only update this specific test, also pass `--test-args impl-trait/issues/infinite-impl-trait-issue-38064.rs`
2020-04-09T21:02:57.1587189Z error: 1 errors occurred comparing output.
2020-04-09T21:02:57.1587428Z status: exit code: 1
2020-04-09T21:02:57.1587428Z status: exit code: 1
2020-04-09T21:02:57.1589878Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064/auxiliary"
2020-04-09T21:02:57.1591688Z ------------------------------------------
2020-04-09T21:02:57.1591877Z 
2020-04-09T21:02:57.1592240Z ------------------------------------------
2020-04-09T21:02:57.1592545Z stderr:
2020-04-09T21:02:57.1592545Z stderr:
2020-04-09T21:02:57.1592967Z ------------------------------------------
2020-04-09T21:02:57.1593287Z error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-04-09T21:02:57.1594225Z    |
2020-04-09T21:02:57.1594225Z    |
2020-04-09T21:02:57.1594679Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1595155Z    |
2020-04-09T21:02:57.1595545Z note: ...which requires borrow-checking `foo`...
2020-04-09T21:02:57.1596121Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-04-09T21:02:57.1596422Z    |
2020-04-09T21:02:57.1596422Z    |
2020-04-09T21:02:57.1596867Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1597403Z note: ...which requires processing `foo`...
2020-04-09T21:02:57.1597976Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-04-09T21:02:57.1598255Z    |
2020-04-09T21:02:57.1598255Z    |
2020-04-09T21:02:57.1598708Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1599230Z note: ...which requires const checking `foo`...
2020-04-09T21:02:57.1599828Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-04-09T21:02:57.1600119Z    |
2020-04-09T21:02:57.1600119Z    |
2020-04-09T21:02:57.1600565Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1600867Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1601161Z    = note: ...which requires computing whether `impl Quux` is freeze...
2020-04-09T21:02:57.1601740Z    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-04-09T21:02:57.1602215Z note: ...which requires processing `bar::{{opaque}}#0`...
2020-04-09T21:02:57.1603152Z    |
2020-04-09T21:02:57.1603152Z    |
2020-04-09T21:02:57.1603607Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1603888Z    |             ^^^^^^^^^
2020-04-09T21:02:57.1604308Z note: ...which requires borrow-checking `bar`...
2020-04-09T21:02:57.1605182Z    |
2020-04-09T21:02:57.1605182Z    |
2020-04-09T21:02:57.1605624Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1606152Z note: ...which requires processing `bar`...
2020-04-09T21:02:57.1606722Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:1
2020-04-09T21:02:57.1607034Z    |
2020-04-09T21:02:57.1607034Z    |
2020-04-09T21:02:57.1607486Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1608026Z note: ...which requires const checking `bar`...
2020-04-09T21:02:57.1608629Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:1
2020-04-09T21:02:57.1608919Z    |
2020-04-09T21:02:57.1608919Z    |
2020-04-09T21:02:57.1609356Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1609661Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1609953Z    = note: ...which requires computing whether `impl Quux` is freeze...
2020-04-09T21:02:57.1610420Z    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-04-09T21:02:57.1610951Z    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1611552Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1612455Z    |
2020-04-09T21:02:57.1612455Z    |
2020-04-09T21:02:57.1612610Z LL | trait Quux {}
2020-04-09T21:02:57.1613016Z 
2020-04-09T21:02:57.1613234Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1613852Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:13
2020-04-09T21:02:57.1614148Z    |
2020-04-09T21:02:57.1614148Z    |
2020-04-09T21:02:57.1614589Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1615143Z    |
2020-04-09T21:02:57.1615143Z    |
2020-04-09T21:02:57.1615394Z    = note: expanded type is `foo::Foo<bar::Bar<impl Quux>>`
2020-04-09T21:02:57.1615831Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1616817Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:13
2020-04-09T21:02:57.1617110Z    |
2020-04-09T21:02:57.1617110Z    |
2020-04-09T21:02:57.1617589Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-04-09T21:02:57.1618155Z    |
2020-04-09T21:02:57.1618155Z    |
2020-04-09T21:02:57.1618401Z    = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`
2020-04-09T21:02:57.1618831Z error: aborting due to 3 previous errors
2020-04-09T21:02:57.1619002Z 
2020-04-09T21:02:57.1619226Z Some errors have detailed explanations: E0391, E0720.
2020-04-09T21:02:57.1619791Z For more information about an error, try `rustc --explain E0391`.
2020-04-09T21:02:57.1619791Z For more information about an error, try `rustc --explain E0391`.
2020-04-09T21:02:57.1620008Z 
2020-04-09T21:02:57.1620366Z ------------------------------------------
2020-04-09T21:02:57.1620533Z 
2020-04-09T21:02:57.1620644Z 
2020-04-09T21:02:57.1621089Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-indirect.rs stdout ----
2020-04-09T21:02:57.1621477Z diff of stderr:
2020-04-09T21:02:57.1621603Z 
2020-04-09T21:02:57.1621881Z + error[E0391]: cycle detected when processing `option::{{opaque}}#0`
2020-04-09T21:02:57.1622715Z +    |
2020-04-09T21:02:57.1622715Z +    |
2020-04-09T21:02:57.1623633Z + LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1624088Z +    |
2020-04-09T21:02:57.1624774Z + note: ...which requires borrow-checking `option`...
2020-04-09T21:02:57.1625347Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-04-09T21:02:57.1625582Z +    |
2020-04-09T21:02:57.1625582Z +    |
2020-04-09T21:02:57.1625969Z + LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1626495Z + note: ...which requires processing `option`...
2020-04-09T21:02:57.1627000Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-04-09T21:02:57.1627258Z +    |
2020-04-09T21:02:57.1627258Z +    |
2020-04-09T21:02:57.1627636Z + LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1628196Z + note: ...which requires const checking `option`...
2020-04-09T21:02:57.1628864Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-04-09T21:02:57.1629108Z +    |
2020-04-09T21:02:57.1629108Z +    |
2020-04-09T21:02:57.1629526Z + LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1630088Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1630088Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1630587Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1631121Z +    = note: ...which again requires processing `option::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1631729Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1632498Z +    |
2020-04-09T21:02:57.1632498Z +    |
2020-04-09T21:02:57.1632689Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1632955Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1633160Z + LL | |
2020-04-09T21:02:57.1633554Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1633787Z + ...  |
2020-04-09T21:02:57.1634050Z + LL | |
2020-04-09T21:02:57.1634226Z + LL | | fn main() {}
2020-04-09T21:02:57.1634588Z + 
2020-04-09T21:02:57.1634814Z 1 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1635371Z 2   --> $DIR/recursive-impl-trait-type-indirect.rs:7:22
2020-04-09T21:02:57.1635624Z 3    |
2020-04-09T21:02:57.1635624Z 3    |
2020-04-09T21:02:57.1635732Z 
2020-04-09T21:02:57.1635857Z 6    |
2020-04-09T21:02:57.1636146Z 7    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-04-09T21:02:57.1636411Z 8 
2020-04-09T21:02:57.1636677Z + error[E0391]: cycle detected when processing `tuple::{{opaque}}#0`
2020-04-09T21:02:57.1637506Z +    |
2020-04-09T21:02:57.1637850Z + LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1638074Z +    |               ^^^^^^^^^^
2020-04-09T21:02:57.1638267Z +    |
2020-04-09T21:02:57.1638267Z +    |
2020-04-09T21:02:57.1638663Z + note: ...which requires borrow-checking `tuple`...
2020-04-09T21:02:57.1639419Z +    |
2020-04-09T21:02:57.1639768Z + LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1640000Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1640000Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1640273Z + note: ...which requires processing `tuple`...
2020-04-09T21:02:57.1641008Z +    |
2020-04-09T21:02:57.1641359Z + LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1641584Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1641584Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1641838Z + note: ...which requires const checking `tuple`...
2020-04-09T21:02:57.1642681Z +    |
2020-04-09T21:02:57.1643048Z + LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1643285Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1643594Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1643594Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1644081Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1644626Z +    = note: ...which again requires processing `tuple::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1645235Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1646002Z +    |
2020-04-09T21:02:57.1646002Z +    |
2020-04-09T21:02:57.1646185Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1646433Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1646638Z + LL | |
2020-04-09T21:02:57.1647046Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1647264Z + ...  |
2020-04-09T21:02:57.1647413Z + LL | |
2020-04-09T21:02:57.1647599Z + LL | | fn main() {}
2020-04-09T21:02:57.1647945Z + 
2020-04-09T21:02:57.1648184Z 9 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1648728Z 10   --> $DIR/recursive-impl-trait-type-indirect.rs:12:15
2020-04-09T21:02:57.1648980Z 11    |
2020-04-09T21:02:57.1648980Z 11    |
2020-04-09T21:02:57.1649092Z 
2020-04-09T21:02:57.1649235Z 14    |
2020-04-09T21:02:57.1649452Z 15    = note: expanded type is `(impl Sized,)`
2020-04-09T21:02:57.1649660Z 16 
2020-04-09T21:02:57.1649946Z + error[E0391]: cycle detected when processing `array::{{opaque}}#0`
2020-04-09T21:02:57.1650747Z +    |
2020-04-09T21:02:57.1651101Z + LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1651334Z +    |               ^^^^^^^^^^
2020-04-09T21:02:57.1651513Z +    |
2020-04-09T21:02:57.1651513Z +    |
2020-04-09T21:02:57.1651928Z + note: ...which requires borrow-checking `array`...
2020-04-09T21:02:57.1652689Z +    |
2020-04-09T21:02:57.1653044Z + LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1653269Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1653269Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1653590Z + note: ...which requires processing `array`...
2020-04-09T21:02:57.1654375Z +    |
2020-04-09T21:02:57.1654721Z + LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1654944Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1654944Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1655214Z + note: ...which requires const checking `array`...
2020-04-09T21:02:57.1655950Z +    |
2020-04-09T21:02:57.1656309Z + LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1656637Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1656928Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1656928Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1657407Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1657924Z +    = note: ...which again requires processing `array::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1658515Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1659271Z +    |
2020-04-09T21:02:57.1659271Z +    |
2020-04-09T21:02:57.1659449Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1659704Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1659901Z + LL | |
2020-04-09T21:02:57.1660275Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1660492Z + ...  |
2020-04-09T21:02:57.1660631Z + LL | |
2020-04-09T21:02:57.1660792Z + LL | | fn main() {}
2020-04-09T21:02:57.1661140Z + 
2020-04-09T21:02:57.1661359Z 17 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1661873Z 18   --> $DIR/recursive-impl-trait-type-indirect.rs:17:15
2020-04-09T21:02:57.1662215Z 19    |
2020-04-09T21:02:57.1662215Z 19    |
2020-04-09T21:02:57.1662325Z 
2020-04-09T21:02:57.1662448Z 22    |
2020-04-09T21:02:57.1662675Z 23    = note: expanded type is `[impl Sized; 1]`
2020-04-09T21:02:57.1662880Z 24 
2020-04-09T21:02:57.1663138Z + error[E0391]: cycle detected when processing `ptr::{{opaque}}#0`
2020-04-09T21:02:57.1663952Z +    |
2020-04-09T21:02:57.1664412Z + LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1664631Z +    |             ^^^^^^^^^^
2020-04-09T21:02:57.1664822Z +    |
2020-04-09T21:02:57.1664822Z +    |
2020-04-09T21:02:57.1665216Z + note: ...which requires borrow-checking `ptr`...
2020-04-09T21:02:57.1665968Z +    |
2020-04-09T21:02:57.1666308Z + LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1666524Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1666524Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1666784Z + note: ...which requires processing `ptr`...
2020-04-09T21:02:57.1667525Z +    |
2020-04-09T21:02:57.1667920Z + LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1668141Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1668141Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1668392Z + note: ...which requires const checking `ptr`...
2020-04-09T21:02:57.1669390Z +    |
2020-04-09T21:02:57.1669730Z + LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1669945Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1670263Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1670263Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1670743Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1671285Z +    = note: ...which again requires processing `ptr::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1671889Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1672663Z +    |
2020-04-09T21:02:57.1672663Z +    |
2020-04-09T21:02:57.1672847Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1673096Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1673301Z + LL | |
2020-04-09T21:02:57.1673823Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1674051Z + ...  |
2020-04-09T21:02:57.1674195Z + LL | |
2020-04-09T21:02:57.1674379Z + LL | | fn main() {}
2020-04-09T21:02:57.1674724Z + 
2020-04-09T21:02:57.1674968Z 25 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1675535Z 26   --> $DIR/recursive-impl-trait-type-indirect.rs:22:13
2020-04-09T21:02:57.1675779Z 27    |
2020-04-09T21:02:57.1675779Z 27    |
2020-04-09T21:02:57.1675888Z 
2020-04-09T21:02:57.1676027Z 38    |
2020-04-09T21:02:57.1676428Z 39    = note: expanded type is `fn() -> impl Sized`
2020-04-09T21:02:57.1676641Z 40 
2020-04-09T21:02:57.1676934Z + error[E0391]: cycle detected when processing `closure_capture::{{opaque}}#0`
2020-04-09T21:02:57.1677759Z +    |
2020-04-09T21:02:57.1678137Z + LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1678399Z +    |                         ^^^^^^^^^^
2020-04-09T21:02:57.1678590Z +    |
2020-04-09T21:02:57.1678590Z +    |
2020-04-09T21:02:57.1679031Z + note: ...which requires borrow-checking `closure_capture`...
2020-04-09T21:02:57.1679805Z +    |
2020-04-09T21:02:57.1680172Z + LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1680443Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1680443Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1680725Z + note: ...which requires processing `closure_capture`...
2020-04-09T21:02:57.1681507Z +    |
2020-04-09T21:02:57.1681877Z + LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1682136Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1682136Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1682543Z + note: ...which requires const checking `closure_capture`...
2020-04-09T21:02:57.1683331Z +    |
2020-04-09T21:02:57.1683727Z + LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1683983Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1684294Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1684294Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1684772Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1685320Z +    = note: ...which again requires processing `closure_capture::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1685949Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1686718Z +    |
2020-04-09T21:02:57.1686718Z +    |
2020-04-09T21:02:57.1686896Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1687166Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1687370Z + LL | |
2020-04-09T21:02:57.1687759Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1687984Z + ...  |
2020-04-09T21:02:57.1688123Z + LL | |
2020-04-09T21:02:57.1688290Z + LL | | fn main() {}
2020-04-09T21:02:57.1688652Z + 
2020-04-09T21:02:57.1688880Z 41 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1689410Z 42   --> $DIR/recursive-impl-trait-type-indirect.rs:32:25
2020-04-09T21:02:57.1689671Z 43    |
2020-04-09T21:02:57.1689671Z 43    |
2020-04-09T21:02:57.1689785Z 
2020-04-09T21:02:57.1689913Z 46    |
2020-04-09T21:02:57.1690519Z 47    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:35:5: 37:6 x:impl Sized]`
2020-04-09T21:02:57.1690868Z 48 
2020-04-09T21:02:57.1691172Z + error[E0391]: cycle detected when processing `closure_ref_capture::{{opaque}}#0`
2020-04-09T21:02:57.1692043Z +    |
2020-04-09T21:02:57.1692424Z + LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1692708Z +    |                             ^^^^^^^^^^
2020-04-09T21:02:57.1692905Z +    |
2020-04-09T21:02:57.1692905Z +    |
2020-04-09T21:02:57.1693417Z + note: ...which requires borrow-checking `closure_ref_capture`...
2020-04-09T21:02:57.1694226Z +    |
2020-04-09T21:02:57.1694608Z + LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1694891Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1694891Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1695197Z + note: ...which requires processing `closure_ref_capture`...
2020-04-09T21:02:57.1695978Z +    |
2020-04-09T21:02:57.1696368Z + LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1696645Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1696645Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1696957Z + note: ...which requires const checking `closure_ref_capture`...
2020-04-09T21:02:57.1697762Z +    |
2020-04-09T21:02:57.1698137Z + LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1698425Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1698748Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1698748Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1699228Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1699803Z +    = note: ...which again requires processing `closure_ref_capture::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1700438Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1701211Z +    |
2020-04-09T21:02:57.1701211Z +    |
2020-04-09T21:02:57.1701393Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1701640Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1701947Z + LL | |
2020-04-09T21:02:57.1702358Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1707817Z + ...  |
2020-04-09T21:02:57.1707994Z + LL | |
2020-04-09T21:02:57.1708164Z + LL | | fn main() {}
2020-04-09T21:02:57.1708966Z + 
2020-04-09T21:02:57.1709198Z 49 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1709861Z 50   --> $DIR/recursive-impl-trait-type-indirect.rs:40:29
2020-04-09T21:02:57.1710129Z 51    |
2020-04-09T21:02:57.1710129Z 51    |
2020-04-09T21:02:57.1710245Z 
2020-04-09T21:02:57.1710373Z 70    |
2020-04-09T21:02:57.1710936Z 71    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:55:5: 55:23]`
2020-04-09T21:02:57.1711273Z 72 
2020-04-09T21:02:57.1711568Z + error[E0391]: cycle detected when processing `generator_capture::{{opaque}}#0`
2020-04-09T21:02:57.1712413Z +    |
2020-04-09T21:02:57.1712802Z + LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1713064Z +    |                           ^^^^^^^^^^
2020-04-09T21:02:57.1713271Z +    |
2020-04-09T21:02:57.1713271Z +    |
2020-04-09T21:02:57.1713712Z + note: ...which requires borrow-checking `generator_capture`...
2020-04-09T21:02:57.1714496Z +    |
2020-04-09T21:02:57.1714873Z + LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1715135Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1715135Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1715440Z + note: ...which requires processing `generator_capture`...
2020-04-09T21:02:57.1716213Z +    |
2020-04-09T21:02:57.1716583Z + LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1716865Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1716865Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1717170Z + note: ...which requires const checking `generator_capture`...
2020-04-09T21:02:57.1717960Z +    |
2020-04-09T21:02:57.1718334Z + LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1718598Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1719064Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1719064Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1719558Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1720115Z +    = note: ...which again requires processing `generator_capture::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1720801Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1724375Z +    |
2020-04-09T21:02:57.1724375Z +    |
2020-04-09T21:02:57.1724579Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1724827Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1725031Z + LL | |
2020-04-09T21:02:57.1725447Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1725668Z + ...  |
2020-04-09T21:02:57.1725807Z + LL | |
2020-04-09T21:02:57.1725988Z + LL | | fn main() {}
2020-04-09T21:02:57.1726333Z + 
2020-04-09T21:02:57.1726581Z 73 error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1727221Z 74   --> $DIR/recursive-impl-trait-type-indirect.rs:58:27
2020-04-09T21:02:57.1727460Z 75    |
2020-04-09T21:02:57.1727460Z 75    |
2020-04-09T21:02:57.1727569Z 
2020-04-09T21:02:57.1727705Z 78    |
2020-04-09T21:02:57.1728280Z 79    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:61:5: 64:6 x:impl Sized {()}]`
2020-04-09T21:02:57.1728612Z 80 
2020-04-09T21:02:57.1728901Z + error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-04-09T21:02:57.1729923Z +    |
2020-04-09T21:02:57.1729923Z +    |
2020-04-09T21:02:57.1730333Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1730941Z +    |
2020-04-09T21:02:57.1730941Z +    |
2020-04-09T21:02:57.1731394Z + note: ...which requires borrow-checking `substs_change`...
2020-04-09T21:02:57.1732138Z +    |
2020-04-09T21:02:57.1732138Z +    |
2020-04-09T21:02:57.1732664Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1733075Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1733490Z + note: ...which requires processing `substs_change`...
2020-04-09T21:02:57.1734269Z +    |
2020-04-09T21:02:57.1734269Z +    |
2020-04-09T21:02:57.1734677Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1734991Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1735317Z + note: ...which requires const checking `substs_change`...
2020-04-09T21:02:57.1736205Z +    |
2020-04-09T21:02:57.1736205Z +    |
2020-04-09T21:02:57.1736843Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1737464Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1737464Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1738073Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1738621Z +    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1739248Z + note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1740031Z +    |
2020-04-09T21:02:57.1740031Z +    |
2020-04-09T21:02:57.1740326Z + LL | / #![feature(generators)]
2020-04-09T21:02:57.1740582Z + LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1740778Z + LL | |
2020-04-09T21:02:57.1741153Z + LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1741376Z + ...  |
2020-04-09T21:02:57.1741510Z + LL | |
2020-04-09T21:02:57.1741665Z + LL | | fn main() {}
---
2020-04-09T21:02:57.1782001Z 
2020-04-09T21:02:57.1782096Z 
2020-04-09T21:02:57.1782297Z The actual stderr differed from the expected stderr.
2020-04-09T21:02:57.1783050Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/recursive-impl-trait-type-indirect.stderr
2020-04-09T21:02:57.1783714Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T21:02:57.1784320Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-indirect.rs`
2020-04-09T21:02:57.1784832Z error: 1 errors occurred comparing output.
2020-04-09T21:02:57.1785062Z status: exit code: 1
2020-04-09T21:02:57.1785062Z status: exit code: 1
2020-04-09T21:02:57.1787097Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/auxiliary"
2020-04-09T21:02:57.1789017Z ------------------------------------------
2020-04-09T21:02:57.1789192Z 
2020-04-09T21:02:57.1789545Z ------------------------------------------
2020-04-09T21:02:57.1789749Z stderr:
2020-04-09T21:02:57.1789749Z stderr:
2020-04-09T21:02:57.1790105Z ------------------------------------------
2020-04-09T21:02:57.1790525Z error[E0391]: cycle detected when processing `option::{{opaque}}#0`
2020-04-09T21:02:57.1791580Z    |
2020-04-09T21:02:57.1791580Z    |
2020-04-09T21:02:57.1791954Z LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1792416Z    |
2020-04-09T21:02:57.1792809Z note: ...which requires borrow-checking `option`...
2020-04-09T21:02:57.1793378Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-04-09T21:02:57.1793668Z    |
2020-04-09T21:02:57.1793668Z    |
2020-04-09T21:02:57.1794031Z LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1794557Z note: ...which requires processing `option`...
2020-04-09T21:02:57.1795194Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-04-09T21:02:57.1795471Z    |
2020-04-09T21:02:57.1795471Z    |
2020-04-09T21:02:57.1795967Z LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1796465Z note: ...which requires const checking `option`...
2020-04-09T21:02:57.1797032Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-04-09T21:02:57.1797305Z    |
2020-04-09T21:02:57.1797305Z    |
2020-04-09T21:02:57.1797656Z LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1798216Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1798216Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1798805Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1799351Z    = note: ...which again requires processing `option::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1800083Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1801829Z    |
2020-04-09T21:02:57.1802013Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1802249Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1802464Z LL | |
2020-04-09T21:02:57.1802464Z LL | |
2020-04-09T21:02:57.1802925Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1803286Z LL | |
2020-04-09T21:02:57.1803448Z LL | | fn main() {}
2020-04-09T21:02:57.1803631Z    | |____________^
2020-04-09T21:02:57.1803764Z 
2020-04-09T21:02:57.1803764Z 
2020-04-09T21:02:57.1803997Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1804683Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:22
2020-04-09T21:02:57.1804949Z    |
2020-04-09T21:02:57.1805317Z LL | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1805824Z    |
2020-04-09T21:02:57.1805824Z    |
2020-04-09T21:02:57.1806086Z    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-04-09T21:02:57.1806311Z 
2020-04-09T21:02:57.1806561Z error[E0391]: cycle detected when processing `tuple::{{opaque}}#0`
2020-04-09T21:02:57.1807471Z    |
2020-04-09T21:02:57.1807803Z LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1808012Z    |               ^^^^^^^^^^
2020-04-09T21:02:57.1808188Z    |
2020-04-09T21:02:57.1808188Z    |
2020-04-09T21:02:57.1808565Z note: ...which requires borrow-checking `tuple`...
2020-04-09T21:02:57.1809392Z    |
2020-04-09T21:02:57.1809719Z LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1809934Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1809934Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1810186Z note: ...which requires processing `tuple`...
2020-04-09T21:02:57.1811006Z    |
2020-04-09T21:02:57.1811346Z LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1811672Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1811672Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1811926Z note: ...which requires const checking `tuple`...
2020-04-09T21:02:57.1812805Z    |
2020-04-09T21:02:57.1813135Z LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1813366Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1813657Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1813657Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1814116Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1814636Z    = note: ...which again requires processing `tuple::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1815228Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1816084Z    |
2020-04-09T21:02:57.1816257Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1816495Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1816684Z LL | |
2020-04-09T21:02:57.1816684Z LL | |
2020-04-09T21:02:57.1817072Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1817405Z LL | |
2020-04-09T21:02:57.1817575Z LL | | fn main() {}
2020-04-09T21:02:57.1817751Z    | |____________^
2020-04-09T21:02:57.1817880Z 
2020-04-09T21:02:57.1817880Z 
2020-04-09T21:02:57.1818089Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1818808Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:12:15
2020-04-09T21:02:57.1819087Z    |
2020-04-09T21:02:57.1819421Z LL | fn tuple() -> impl Sized {
2020-04-09T21:02:57.1819700Z    |               ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1820004Z    |
2020-04-09T21:02:57.1820208Z    = note: expanded type is `(impl Sized,)`
2020-04-09T21:02:57.1820405Z 
2020-04-09T21:02:57.1820666Z error[E0391]: cycle detected when processing `array::{{opaque}}#0`
2020-04-09T21:02:57.1821629Z    |
2020-04-09T21:02:57.1821976Z LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1822202Z    |               ^^^^^^^^^^
2020-04-09T21:02:57.1822390Z    |
2020-04-09T21:02:57.1822390Z    |
2020-04-09T21:02:57.1822784Z note: ...which requires borrow-checking `array`...
2020-04-09T21:02:57.1823638Z    |
2020-04-09T21:02:57.1823994Z LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1824210Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1824451Z note: ...which requires processing `array`...
---
2020-04-09T21:02:57.1826772Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:17:1
2020-04-09T21:02:57.1827053Z    |
2020-04-09T21:02:57.1827377Z LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1827585Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1827890Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1828347Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1829114Z    = note: ...which again requires processing `array::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1829756Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1830631Z    |
2020-04-09T21:02:57.1830884Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1831125Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1831322Z LL | |
2020-04-09T21:02:57.1831322Z LL | |
2020-04-09T21:02:57.1831933Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1832279Z LL | |
2020-04-09T21:02:57.1832434Z LL | | fn main() {}
2020-04-09T21:02:57.1832627Z    | |____________^
2020-04-09T21:02:57.1832757Z 
2020-04-09T21:02:57.1832757Z 
2020-04-09T21:02:57.1832964Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1833575Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:17:15
2020-04-09T21:02:57.1833843Z    |
2020-04-09T21:02:57.1834169Z LL | fn array() -> impl Sized {
2020-04-09T21:02:57.1834436Z    |               ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1834642Z    |
2020-04-09T21:02:57.1834841Z    = note: expanded type is `[impl Sized; 1]`
2020-04-09T21:02:57.1835023Z 
2020-04-09T21:02:57.1835283Z error[E0391]: cycle detected when processing `ptr::{{opaque}}#0`
2020-04-09T21:02:57.1836151Z    |
2020-04-09T21:02:57.1836497Z LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1836700Z    |             ^^^^^^^^^^
2020-04-09T21:02:57.1836861Z    |
2020-04-09T21:02:57.1836861Z    |
2020-04-09T21:02:57.1837252Z note: ...which requires borrow-checking `ptr`...
2020-04-09T21:02:57.1838063Z    |
2020-04-09T21:02:57.1838395Z LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1839083Z    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1839330Z note: ...which requires processing `ptr`...
---
2020-04-09T21:02:57.1841877Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:22:1
2020-04-09T21:02:57.1842151Z    |
2020-04-09T21:02:57.1842504Z LL | fn ptr() -> impl Sized {
2020-04-09T21:02:57.1842714Z    | ^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1843002Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1843495Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1844013Z    = note: ...which again requires processing `ptr::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1844605Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1845478Z    |
2020-04-09T21:02:57.1845656Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1845913Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1846110Z LL | |
2020-04-09T21:02:57.1846110Z LL | |
2020-04-09T21:02:57.1846495Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1846847Z LL | |
2020-04-09T21:02:57.1847007Z LL | | fn main() {}
2020-04-09T21:02:57.1847208Z    | |____________^
2020-04-09T21:02:57.1847344Z 
---
2020-04-09T21:02:57.1849730Z 
2020-04-09T21:02:57.1849945Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1850621Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:27:16
2020-04-09T21:02:57.1850908Z    |
2020-04-09T21:02:57.1851249Z LL | fn fn_ptr() -> impl Sized {
2020-04-09T21:02:57.1851853Z    |
2020-04-09T21:02:57.1852275Z    = note: expanded type is `fn() -> impl Sized`
2020-04-09T21:02:57.1852476Z 
2020-04-09T21:02:57.1852476Z 
2020-04-09T21:02:57.1852750Z error[E0391]: cycle detected when processing `closure_capture::{{opaque}}#0`
2020-04-09T21:02:57.1853701Z    |
2020-04-09T21:02:57.1854070Z LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1854330Z    |                         ^^^^^^^^^^
2020-04-09T21:02:57.1854522Z    |
2020-04-09T21:02:57.1854522Z    |
2020-04-09T21:02:57.1854955Z note: ...which requires borrow-checking `closure_capture`...
2020-04-09T21:02:57.1855814Z    |
2020-04-09T21:02:57.1856192Z LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1856440Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1856712Z note: ...which requires processing `closure_capture`...
---
2020-04-09T21:02:57.1860188Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:1
2020-04-09T21:02:57.1860497Z    |
2020-04-09T21:02:57.1860871Z LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1861118Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1861451Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1862061Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1862606Z    = note: ...which again requires processing `closure_capture::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1863335Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1864209Z    |
2020-04-09T21:02:57.1864403Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1864644Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1864839Z LL | |
2020-04-09T21:02:57.1864839Z LL | |
2020-04-09T21:02:57.1865235Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1865573Z LL | |
2020-04-09T21:02:57.1865747Z LL | | fn main() {}
2020-04-09T21:02:57.1865926Z    | |____________^
2020-04-09T21:02:57.1866335Z 
2020-04-09T21:02:57.1866335Z 
2020-04-09T21:02:57.1866559Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1867293Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:25
2020-04-09T21:02:57.1867583Z    |
2020-04-09T21:02:57.1867950Z LL | fn closure_capture() -> impl Sized {
2020-04-09T21:02:57.1868279Z    |                         ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1868519Z    |
2020-04-09T21:02:57.1869454Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:35:5: 37:6 x:impl Sized]`
2020-04-09T21:02:57.1869842Z 
2020-04-09T21:02:57.1870138Z error[E0391]: cycle detected when processing `closure_ref_capture::{{opaque}}#0`
2020-04-09T21:02:57.1871089Z    |
2020-04-09T21:02:57.1871466Z LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1871798Z    |                             ^^^^^^^^^^
2020-04-09T21:02:57.1872004Z    |
2020-04-09T21:02:57.1872004Z    |
2020-04-09T21:02:57.1872502Z note: ...which requires borrow-checking `closure_ref_capture`...
2020-04-09T21:02:57.1873388Z    |
2020-04-09T21:02:57.1873781Z LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1874172Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1874172Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1874480Z note: ...which requires processing `closure_ref_capture`...
2020-04-09T21:02:57.1875398Z    |
2020-04-09T21:02:57.1875772Z LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1876053Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1876053Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1876357Z note: ...which requires const checking `closure_ref_capture`...
2020-04-09T21:02:57.1877243Z    |
2020-04-09T21:02:57.1877621Z LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1877895Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1878235Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1878235Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1878716Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1879271Z    = note: ...which again requires processing `closure_ref_capture::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1879919Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1880772Z    |
2020-04-09T21:02:57.1880965Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1881206Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1881403Z LL | |
2020-04-09T21:02:57.1881403Z LL | |
2020-04-09T21:02:57.1881801Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1882139Z LL | |
2020-04-09T21:02:57.1882403Z LL | | fn main() {}
2020-04-09T21:02:57.1882581Z    | |____________^
2020-04-09T21:02:57.1882710Z 
2020-04-09T21:02:57.1882710Z 
2020-04-09T21:02:57.1882924Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1883560Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:40:29
2020-04-09T21:02:57.1883957Z    |
2020-04-09T21:02:57.1884324Z LL | fn closure_ref_capture() -> impl Sized {
2020-04-09T21:02:57.1884652Z    |                             ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1885008Z    |
2020-04-09T21:02:57.1885648Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:43:5: 45:6 x:impl Sized]`
2020-04-09T21:02:57.1886237Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1886820Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:48:21
2020-04-09T21:02:57.1887112Z    |
2020-04-09T21:02:57.1887473Z LL | fn closure_sig() -> impl Sized {
2020-04-09T21:02:57.1887473Z LL | fn closure_sig() -> impl Sized {
2020-04-09T21:02:57.1887776Z    |                     ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1888007Z    |
2020-04-09T21:02:57.1888733Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:50:5: 50:21]`
2020-04-09T21:02:57.1889268Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1889845Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:53:23
2020-04-09T21:02:57.1890122Z    |
2020-04-09T21:02:57.1890468Z LL | fn generator_sig() -> impl Sized {
2020-04-09T21:02:57.1890468Z LL | fn generator_sig() -> impl Sized {
2020-04-09T21:02:57.1890770Z    |                       ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1890997Z    |
2020-04-09T21:02:57.1891576Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:55:5: 55:23]`
2020-04-09T21:02:57.1891914Z 
2020-04-09T21:02:57.1892190Z error[E0391]: cycle detected when processing `generator_capture::{{opaque}}#0`
2020-04-09T21:02:57.1893105Z    |
2020-04-09T21:02:57.1893462Z LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1893788Z    |                           ^^^^^^^^^^
2020-04-09T21:02:57.1893975Z    |
2020-04-09T21:02:57.1893975Z    |
2020-04-09T21:02:57.1894427Z note: ...which requires borrow-checking `generator_capture`...
2020-04-09T21:02:57.1895256Z    |
2020-04-09T21:02:57.1895627Z LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1895873Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1895873Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1896150Z note: ...which requires processing `generator_capture`...
2020-04-09T21:02:57.1897149Z    |
2020-04-09T21:02:57.1897520Z LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1897842Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1897842Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1898140Z note: ...which requires const checking `generator_capture`...
2020-04-09T21:02:57.1899033Z    |
2020-04-09T21:02:57.1899410Z LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1899785Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1900106Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1900106Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1900562Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1901092Z    = note: ...which again requires processing `generator_capture::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1901713Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1902654Z    |
2020-04-09T21:02:57.1902843Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1903075Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1903263Z LL | |
2020-04-09T21:02:57.1903263Z LL | |
2020-04-09T21:02:57.1903673Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1904013Z LL | |
2020-04-09T21:02:57.1904183Z LL | | fn main() {}
2020-04-09T21:02:57.1904361Z    | |____________^
2020-04-09T21:02:57.1904489Z 
2020-04-09T21:02:57.1904489Z 
2020-04-09T21:02:57.1904698Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1905285Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:58:27
2020-04-09T21:02:57.1905553Z    |
2020-04-09T21:02:57.1905907Z LL | fn generator_capture() -> impl Sized {
2020-04-09T21:02:57.1906217Z    |                           ^^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1906452Z    |
2020-04-09T21:02:57.1907087Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:61:5: 64:6 x:impl Sized {()}]`
2020-04-09T21:02:57.1907453Z 
2020-04-09T21:02:57.1907715Z error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-04-09T21:02:57.1908880Z    |
2020-04-09T21:02:57.1908880Z    |
2020-04-09T21:02:57.1909326Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1909884Z    |
2020-04-09T21:02:57.1909884Z    |
2020-04-09T21:02:57.1910317Z note: ...which requires borrow-checking `substs_change`...
2020-04-09T21:02:57.1911165Z    |
2020-04-09T21:02:57.1911165Z    |
2020-04-09T21:02:57.1911581Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1912163Z note: ...which requires processing `substs_change`...
2020-04-09T21:02:57.1912766Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-04-09T21:02:57.1913052Z    |
2020-04-09T21:02:57.1913052Z    |
2020-04-09T21:02:57.1913455Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1914248Z note: ...which requires const checking `substs_change`...
2020-04-09T21:02:57.1914849Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-04-09T21:02:57.1915139Z    |
2020-04-09T21:02:57.1915139Z    |
2020-04-09T21:02:57.1915535Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1916164Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1916164Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1916622Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1917143Z    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1917886Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1918740Z    |
2020-04-09T21:02:57.1918939Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1919181Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1919378Z LL | |
2020-04-09T21:02:57.1919378Z LL | |
2020-04-09T21:02:57.1919778Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1920126Z LL | |
2020-04-09T21:02:57.1920300Z LL | | fn main() {}
2020-04-09T21:02:57.1920483Z    | |____________^
2020-04-09T21:02:57.1920616Z 
2020-04-09T21:02:57.1920616Z 
2020-04-09T21:02:57.1920890Z error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-04-09T21:02:57.1921832Z    |
2020-04-09T21:02:57.1921832Z    |
2020-04-09T21:02:57.1922237Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1922837Z    |
2020-04-09T21:02:57.1922837Z    |
2020-04-09T21:02:57.1923271Z note: ...which requires borrow-checking `substs_change`...
2020-04-09T21:02:57.1924151Z    |
2020-04-09T21:02:57.1924151Z    |
2020-04-09T21:02:57.1924847Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1925484Z note: ...which requires processing `substs_change`...
2020-04-09T21:02:57.1926127Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-04-09T21:02:57.1926497Z    |
2020-04-09T21:02:57.1926497Z    |
2020-04-09T21:02:57.1926919Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1927535Z note: ...which requires const checking `substs_change`...
2020-04-09T21:02:57.1928122Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-04-09T21:02:57.1928407Z    |
2020-04-09T21:02:57.1928407Z    |
2020-04-09T21:02:57.1928825Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1929120Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1929462Z    = note: ...which requires computing whether `(impl Sized,)` is freeze...
2020-04-09T21:02:57.1930020Z    = note: ...which requires evaluating trait selection obligation `(impl Sized,): std::marker::Freeze`...
2020-04-09T21:02:57.1930574Z    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1931203Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1932090Z    |
2020-04-09T21:02:57.1932267Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1932523Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1932722Z LL | |
2020-04-09T21:02:57.1932722Z LL | |
2020-04-09T21:02:57.1933109Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1933462Z LL | |
2020-04-09T21:02:57.1933621Z LL | | fn main() {}
2020-04-09T21:02:57.1933804Z    | |____________^
2020-04-09T21:02:57.1933954Z 
2020-04-09T21:02:57.1933954Z 
2020-04-09T21:02:57.1934448Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1935125Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:35
2020-04-09T21:02:57.1935427Z    |
2020-04-09T21:02:57.1935843Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-04-09T21:02:57.1936452Z    |
2020-04-09T21:02:57.1936452Z    |
2020-04-09T21:02:57.1936674Z    = note: expanded type is `(impl Sized,)`
2020-04-09T21:02:57.1936855Z 
2020-04-09T21:02:57.1937132Z error[E0391]: cycle detected when processing `generator_hold::{{opaque}}#0`
2020-04-09T21:02:57.1938099Z    |
2020-04-09T21:02:57.1938099Z    |
2020-04-09T21:02:57.1938465Z LL | fn generator_hold() -> impl Sized {
2020-04-09T21:02:57.1938923Z    |
2020-04-09T21:02:57.1938923Z    |
2020-04-09T21:02:57.1939427Z note: ...which requires borrow-checking `generator_hold`...
2020-04-09T21:02:57.1940309Z    |
2020-04-09T21:02:57.1940309Z    |
2020-04-09T21:02:57.1940671Z LL | fn generator_hold() -> impl Sized {
2020-04-09T21:02:57.1941021Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1941297Z note: ...which requires processing `generator_hold`...
2020-04-09T21:02:57.1942248Z    |
2020-04-09T21:02:57.1942248Z    |
2020-04-09T21:02:57.1942639Z LL | fn generator_hold() -> impl Sized {
2020-04-09T21:02:57.1943083Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1943372Z note: ...which requires const checking `generator_hold`...
2020-04-09T21:02:57.1944415Z    |
2020-04-09T21:02:57.1944415Z    |
2020-04-09T21:02:57.1945171Z LL | fn generator_hold() -> impl Sized {
2020-04-09T21:02:57.1945458Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-09T21:02:57.1946326Z    = note: ...which requires computing whether `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]` is freeze...
2020-04-09T21:02:57.1947564Z    = note: ...which requires evaluating trait selection obligation `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]: std::marker::Freeze`...
2020-04-09T21:02:57.1948290Z    = note: ...which again requires processing `generator_hold::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1949120Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1950016Z    |
2020-04-09T21:02:57.1950190Z LL | / #![feature(generators)]
2020-04-09T21:02:57.1950448Z LL | | #![allow(unconditional_recursion)]
2020-04-09T21:02:57.1950646Z LL | |
2020-04-09T21:02:57.1950646Z LL | |
2020-04-09T21:02:57.1951033Z LL | | fn option(i: i32) -> impl Sized {
2020-04-09T21:02:57.1951383Z LL | |
2020-04-09T21:02:57.1951541Z LL | | fn main() {}
2020-04-09T21:02:57.1951847Z    | |____________^
2020-04-09T21:02:57.1951977Z 
2020-04-09T21:02:57.1951977Z 
2020-04-09T21:02:57.1952185Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1952887Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:72:24
2020-04-09T21:02:57.1953189Z    |
2020-04-09T21:02:57.1953553Z LL | fn generator_hold() -> impl Sized {
2020-04-09T21:02:57.1954104Z    |
2020-04-09T21:02:57.1954104Z    |
2020-04-09T21:02:57.1954753Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]`
2020-04-09T21:02:57.1955354Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1956059Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:86:26
2020-04-09T21:02:57.1956349Z    |
2020-04-09T21:02:57.1956757Z LL | fn mutual_recursion() -> impl Sync {
2020-04-09T21:02:57.1956757Z LL | fn mutual_recursion() -> impl Sync {
2020-04-09T21:02:57.1957062Z    |                          ^^^^^^^^^ expands to a recursive type
2020-04-09T21:02:57.1957300Z    |
2020-04-09T21:02:57.1957504Z    = note: type resolves to itself
2020-04-09T21:02:57.1957670Z 
2020-04-09T21:02:57.1957887Z error[E0720]: opaque type expands to a recursive type
2020-04-09T21:02:57.1958575Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:91:28
2020-04-09T21:02:57.1958986Z    |
2020-04-09T21:02:57.1959359Z LL | fn mutual_recursion_b() -> impl Sized {
2020-04-09T21:02:57.1959938Z    |
2020-04-09T21:02:57.1960126Z    = note: type resolves to itself
2020-04-09T21:02:57.1960290Z 
2020-04-09T21:02:57.1960481Z error: aborting due to 24 previous errors
---
2020-04-09T21:02:57.1962817Z ---- [ui] ui/impl-trait/unsafety-checking-cycle.rs stdout ----
2020-04-09T21:02:57.1963030Z 
2020-04-09T21:02:57.1963408Z error: test compilation failed although it shouldn't!
2020-04-09T21:02:57.1963649Z status: exit code: 1
2020-04-09T21:02:57.1965558Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/unsafety-checking-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/unsafety-checking-cycle/auxiliary"
2020-04-09T21:02:57.1967276Z ------------------------------------------
2020-04-09T21:02:57.1967440Z 
2020-04-09T21:02:57.1967788Z ------------------------------------------
2020-04-09T21:02:57.1967994Z stderr:
2020-04-09T21:02:57.1967994Z stderr:
2020-04-09T21:02:57.1968351Z ------------------------------------------
2020-04-09T21:02:57.1968677Z error[E0391]: cycle detected when processing `not_restricted::{{opaque}}#0`
2020-04-09T21:02:57.1969542Z    |
2020-04-09T21:02:57.1969542Z    |
2020-04-09T21:02:57.1969922Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-04-09T21:02:57.1970406Z    |
2020-04-09T21:02:57.1970818Z note: ...which requires borrow-checking `not_restricted`...
2020-04-09T21:02:57.1971372Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-04-09T21:02:57.1971664Z    |
2020-04-09T21:02:57.1971664Z    |
2020-04-09T21:02:57.1972057Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-04-09T21:02:57.1972754Z note: ...which requires processing `not_restricted`...
2020-04-09T21:02:57.1973308Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-04-09T21:02:57.1973584Z    |
2020-04-09T21:02:57.1973584Z    |
2020-04-09T21:02:57.1973979Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-04-09T21:02:57.1974567Z note: ...which requires const checking `not_restricted`...
2020-04-09T21:02:57.1975132Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-04-09T21:02:57.1975381Z    |
2020-04-09T21:02:57.1975381Z    |
2020-04-09T21:02:57.1975766Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-04-09T21:02:57.1976556Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1976556Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1977015Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1977559Z    = note: ...which again requires processing `not_restricted::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1978183Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1979092Z    |
2020-04-09T21:02:57.1979092Z    |
2020-04-09T21:02:57.1979271Z LL | / #![feature(rustc_attrs)]
2020-04-09T21:02:57.1979467Z LL | |
2020-04-09T21:02:57.1979652Z LL | | struct AnyValue<T>(T);
2020-04-09T21:02:57.1979964Z ...  |
2020-04-09T21:02:57.1980116Z LL | |
2020-04-09T21:02:57.1980276Z LL | | fn main() {}
2020-04-09T21:02:57.1980458Z    | |____________^
2020-04-09T21:02:57.1980458Z    | |____________^
2020-04-09T21:02:57.1980608Z 
2020-04-09T21:02:57.1980879Z error[E0391]: cycle detected when processing `not_field::{{opaque}}#0`
2020-04-09T21:02:57.1981772Z    |
2020-04-09T21:02:57.1981772Z    |
2020-04-09T21:02:57.1982154Z LL | fn not_field(c: bool) -> impl Sized {
2020-04-09T21:02:57.1982961Z    |
2020-04-09T21:02:57.1983449Z note: ...which requires borrow-checking `not_field`...
2020-04-09T21:02:57.1983990Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-04-09T21:02:57.1984301Z    |
2020-04-09T21:02:57.1984301Z    |
2020-04-09T21:02:57.1984716Z LL | fn not_field(c: bool) -> impl Sized {
2020-04-09T21:02:57.1985377Z note: ...which requires processing `not_field`...
2020-04-09T21:02:57.1985969Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-04-09T21:02:57.1986223Z    |
2020-04-09T21:02:57.1986223Z    |
2020-04-09T21:02:57.1986603Z LL | fn not_field(c: bool) -> impl Sized {
2020-04-09T21:02:57.1987162Z note: ...which requires const checking `not_field`...
2020-04-09T21:02:57.1987714Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-04-09T21:02:57.1987990Z    |
2020-04-09T21:02:57.1987990Z    |
2020-04-09T21:02:57.1988367Z LL | fn not_field(c: bool) -> impl Sized {
2020-04-09T21:02:57.1989105Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1989105Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-04-09T21:02:57.1989579Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-04-09T21:02:57.1990110Z    = note: ...which again requires processing `not_field::{{opaque}}#0`, completing the cycle
2020-04-09T21:02:57.1990767Z note: cycle used when checking item types in top-level module
2020-04-09T21:02:57.1991570Z    |
2020-04-09T21:02:57.1991570Z    |
2020-04-09T21:02:57.1991760Z LL | / #![feature(rustc_attrs)]
2020-04-09T21:02:57.1991942Z LL | |
2020-04-09T21:02:57.1992119Z LL | | struct AnyValue<T>(T);
2020-04-09T21:02:57.1992508Z ...  |
2020-04-09T21:02:57.1992643Z LL | |
2020-04-09T21:02:57.1992802Z LL | | fn main() {}
2020-04-09T21:02:57.1993001Z    | |____________^
---
2020-04-09T21:02:57.2000927Z +   --> $DIR/issue-52843-closure-constrain.rs:7:1
2020-04-09T21:02:57.2001151Z +    |
2020-04-09T21:02:57.2001319Z + LL | fn main() {
2020-04-09T21:02:57.2001487Z +    | ^^^^^^^^^
2020-04-09T21:02:57.2001754Z +    = note: ...which requires computing whether `Opaque` is freeze...
2020-04-09T21:02:57.2002221Z +    = note: ...which requires evaluating trait selection obligation `Opaque: std::marker::Freeze`...
2020-04-09T21:02:57.2002716Z +    = note: ...which again requires processing `main::Opaque`, completing the cycle
2020-04-09T21:02:57.2003300Z + note: cycle used when collecting item types in top-level module
2020-04-09T21:02:57.2004032Z +    |
2020-04-09T21:02:57.2004228Z + LL | / #![feature(type_alias_impl_trait)]
2020-04-09T21:02:57.2004530Z + LL | |
2020-04-09T21:02:57.2004719Z + LL | | use std::fmt::Debug;
---
2020-04-09T21:02:57.2006020Z 1 error: concrete type differs from previous defining opaque type use
2020-04-09T21:02:57.2006563Z 2   --> $DIR/issue-52843-closure-constrain.rs:10:16
2020-04-09T21:02:57.2006804Z 3    |
2020-04-09T21:02:57.2006910Z 
2020-04-09T21:02:57.2007655Z 10 LL |     fn _unused() -> Opaque { String::new() }
2020-04-09T21:02:57.2008220Z 12 
2020-04-09T21:02:57.2008639Z - error: aborting due to previous error
2020-04-09T21:02:57.2008898Z + error: aborting due to 2 previous errors
2020-04-09T21:02:57.2009107Z 14 
2020-04-09T21:02:57.2009107Z 14 
2020-04-09T21:02:57.2009566Z + For more information about this error, try `rustc --explain E0391`.
2020-04-09T21:02:57.2009809Z 15 
2020-04-09T21:02:57.2009930Z 
2020-04-09T21:02:57.2010029Z 
2020-04-09T21:02:57.2010237Z The actual stderr differed from the expected stderr.
2020-04-09T21:02:57.2011015Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain/issue-52843-closure-constrain.stderr
2020-04-09T21:02:57.2011815Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T21:02:57.2012476Z To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-52843-closure-constrain.rs`
2020-04-09T21:02:57.2012977Z error: 1 errors occurred comparing output.
2020-04-09T21:02:57.2013215Z status: exit code: 1
2020-04-09T21:02:57.2013215Z status: exit code: 1
2020-04-09T21:02:57.2016298Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain/auxiliary"
2020-04-09T21:02:57.2018204Z ------------------------------------------
2020-04-09T21:02:57.2018385Z 
2020-04-09T21:02:57.2018751Z ------------------------------------------
2020-04-09T21:02:57.2018967Z stderr:
---
2020-04-09T21:02:57.2025155Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:7:1
2020-04-09T21:02:57.2025543Z    |
2020-04-09T21:02:57.2025696Z LL | fn main() {
2020-04-09T21:02:57.2025880Z    | ^^^^^^^^^
2020-04-09T21:02:57.2026152Z    = note: ...which requires computing whether `Opaque` is freeze...
2020-04-09T21:02:57.2026619Z    = note: ...which requires evaluating trait selection obligation `Opaque: std::marker::Freeze`...
2020-04-09T21:02:57.2027425Z    = note: ...which again requires processing `main::Opaque`, completing the cycle
2020-04-09T21:02:57.2028092Z note: cycle used when collecting item types in top-level module
2020-04-09T21:02:57.2029246Z    |
2020-04-09T21:02:57.2029445Z LL | / #![feature(type_alias_impl_trait)]
2020-04-09T21:02:57.2029642Z LL | |
2020-04-09T21:02:57.2029845Z LL | | use std::fmt::Debug;
---
2020-04-09T21:02:57.2030863Z 
2020-04-09T21:02:57.2031116Z error: concrete type differs from previous defining opaque type use
2020-04-09T21:02:57.2035361Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:10:16
2020-04-09T21:02:57.2035669Z    |
2020-04-09T21:02:57.2036060Z LL |     let null = || -> Opaque { 0 };
2020-04-09T21:02:57.2036423Z    |                ^^^^^^^^^^^^^^^^^^ expected `std::string::String`, got `i32`
2020-04-09T21:02:57.2036898Z note: previous use here
2020-04-09T21:02:57.2038081Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:9:5
2020-04-09T21:02:57.2038390Z    |
2020-04-09T21:02:57.2038390Z    |
2020-04-09T21:02:57.2038869Z LL |     fn _unused() -> Opaque { String::new() }
2020-04-09T21:02:57.2039387Z 
2020-04-09T21:02:57.2040267Z error: aborting due to 2 previous errors
2020-04-09T21:02:57.2040463Z 
2020-04-09T21:02:57.2041013Z For more information about this error, try `rustc --explain E0391`.
---
2020-04-09T21:02:57.2047708Z test result: FAILED. 9814 passed; 7 failed; 60 ignored; 0 measured; 0 filtered out
2020-04-09T21:02:57.2047981Z 
2020-04-09T21:02:57.2048082Z 
2020-04-09T21:02:57.2048179Z 
2020-04-09T21:02:57.2051921Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-09T21:02:57.2054817Z 
2020-04-09T21:02:57.2054937Z 
2020-04-09T21:02:57.2055472Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-09T21:02:57.2055849Z Build completed unsuccessfully in 1:12:35
2020-04-09T21:02:57.2055849Z Build completed unsuccessfully in 1:12:35
2020-04-09T21:02:57.2056116Z == clock drift check ==
2020-04-09T21:02:57.2056444Z   local time: Thu Apr  9 21:02:57 UTC 2020
2020-04-09T21:02:57.3012476Z   network time: Thu, 09 Apr 2020 21:02:57 GMT
2020-04-09T21:02:57.9987904Z 
2020-04-09T21:02:57.9987904Z 
2020-04-09T21:02:58.0098782Z ##[error]Bash exited with code '1'.
2020-04-09T21:02:58.0114699Z ##[section]Finishing: Run build
2020-04-09T21:02:58.0169363Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-04-09T21:02:58.0174849Z Task         : Get sources
2020-04-09T21:02:58.0175240Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T21:02:58.0175621Z Version      : 1.0.0
2020-04-09T21:02:58.0175875Z Author       : Microsoft
2020-04-09T21:02:58.0175875Z Author       : Microsoft
2020-04-09T21:02:58.0176292Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T21:02:58.0176788Z ==============================================================================
2020-04-09T21:02:58.3956513Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T21:02:58.4000539Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-04-09T21:02:58.4106651Z Cleaning up task key
2020-04-09T21:02:58.4107873Z Start cleaning up orphan processes.
2020-04-09T21:02:58.4305461Z Terminate orphan process: pid (3676) (python)
2020-04-09T21:02:58.4501348Z ##[section]Finishing: Finalize Job
