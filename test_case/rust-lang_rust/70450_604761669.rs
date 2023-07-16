plain
2020-03-26T23:52:06.7638070Z ========================== Starting Command Output ===========================
2020-03-26T23:52:06.7641758Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a622cf56-5131-4fb8-a69d-9451025feb25.sh
2020-03-26T23:52:06.7642163Z 
2020-03-26T23:52:06.7646023Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-26T23:52:06.7666345Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-03-26T23:52:06.7671299Z Task         : Get sources
2020-03-26T23:52:06.7671853Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T23:52:06.7672292Z Version      : 1.0.0
2020-03-26T23:52:06.7672475Z Author       : Microsoft
---
2020-03-26T23:52:07.9555861Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-26T23:52:07.9566228Z ##[command]git config gc.auto 0
2020-03-26T23:52:07.9571613Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-26T23:52:07.9577600Z ##[command]git config --get-all http.proxy
2020-03-26T23:52:07.9587162Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70450/merge:refs/remotes/pull/70450/merge
---
2020-03-26T23:59:32.2042557Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T23:59:43.1664939Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T23:59:49.6120418Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T00:00:03.3560385Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-27T00:00:06.4414490Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T00:00:07.9275697Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T00:00:43.8290724Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-27T00:00:52.8582474Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-27T00:01:45.8866723Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-27T00:23:28.1032926Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T00:23:29.8990175Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T00:23:41.1129133Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T00:23:54.3970656Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-27T00:24:00.2415080Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T00:24:01.9486436Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T00:24:48.3900416Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-27T00:24:59.1090387Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-27T00:26:07.6820371Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-27T00:50:22.7334097Z .................................................................................................... 1700/9846
2020-03-27T00:50:26.5752945Z .................................................................................................... 1800/9846
2020-03-27T00:50:36.4171955Z .........................................................................................i.......... 1900/9846
2020-03-27T00:50:43.0798522Z .................................................................................................... 2000/9846
2020-03-27T00:50:49.3283751Z ...............................................................................iiiii................ 2100/9846
2020-03-27T00:51:10.3564101Z .................................................................................................... 2300/9846
2020-03-27T00:51:12.4405759Z .................................................................................................... 2400/9846
2020-03-27T00:51:14.7904066Z .................................................................................................... 2500/9846
2020-03-27T00:51:23.8127573Z .................................................................................................... 2600/9846
---
2020-03-27T00:54:12.7764109Z .....................................................i...............i.............................. 5000/9846
2020-03-27T00:54:20.2418997Z .................................................................................................... 5100/9846
2020-03-27T00:54:27.6810095Z ..................................................................................................i. 5200/9846
2020-03-27T00:54:32.7108878Z .................................................................................................... 5300/9846
2020-03-27T00:54:43.3009302Z .................................................................................ii.ii........i...i. 5400/9846
2020-03-27T00:54:50.3657036Z .....................i.............................................................................. 5600/9846
2020-03-27T00:54:57.8181442Z ..........................i......................................................................... 5700/9846
2020-03-27T00:55:05.5574636Z ...........................................ii....................................i.................. 5800/9846
2020-03-27T00:55:12.6503999Z .................................................................................................... 5900/9846
2020-03-27T00:55:12.6503999Z .................................................................................................... 5900/9846
2020-03-27T00:55:17.8968870Z .................................................................................................... 6000/9846
2020-03-27T00:55:27.1703338Z ...........................................................................ii...i..ii...........i... 6100/9846
2020-03-27T00:55:47.3299623Z .................................................................................................... 6300/9846
2020-03-27T00:55:54.3131791Z .................................................................................................... 6400/9846
2020-03-27T00:56:01.2189479Z .................................................................................................... 6500/9846
2020-03-27T00:56:01.2189479Z .................................................................................................... 6500/9846
2020-03-27T00:56:13.8877696Z .....i..ii.......................................................................................... 6600/9846
2020-03-27T00:56:33.9597622Z .................................................................................................... 6800/9846
2020-03-27T00:56:36.0273006Z ....i............................................................................................... 6900/9846
2020-03-27T00:56:38.0451942Z .................................................................................................... 7000/9846
2020-03-27T00:56:40.2824498Z ........................................i........................................................... 7100/9846
---
2020-03-27T00:58:21.3310288Z .................................................................................................... 7800/9846
2020-03-27T00:58:26.1706321Z .................................................................................................... 7900/9846
2020-03-27T00:58:33.3440761Z .................................................................................................i.. 8000/9846
2020-03-27T00:58:41.0771704Z .................................................................................................... 8100/9846
2020-03-27T00:58:48.6604375Z ..............................................iiiiiiiiii.i.......................................... 8200/9846
2020-03-27T00:58:58.1895688Z ..........................................................................................i......i.. 8300/9846
2020-03-27T00:59:07.7075547Z .................................................................................................... 8500/9846
2020-03-27T00:59:20.9049015Z .................................................................................................... 8600/9846
2020-03-27T00:59:29.9599881Z .................................................................................................... 8700/9846
2020-03-27T00:59:35.5232297Z .................................................................................................... 8800/9846
---
2020-03-27T01:01:24.7962628Z 
2020-03-27T01:01:24.7963820Z ---- [ui] ui/async-await/mutually-recursive-async-impl-trait-type.rs stdout ----
2020-03-27T01:01:24.7964418Z diff of stderr:
2020-03-27T01:01:24.7964717Z 
2020-03-27T01:01:24.7965019Z + error[E0391]: cycle detected when processing `rec_1::{{opaque}}#0`
2020-03-27T01:01:24.7970032Z +    |
2020-03-27T01:01:24.7970032Z +    |
2020-03-27T01:01:24.7970237Z + LL | async fn rec_1() {
2020-03-27T01:01:24.7970633Z +    |
2020-03-27T01:01:24.7970633Z +    |
2020-03-27T01:01:24.7971197Z + note: ...which requires borrow-checking `rec_1`...
2020-03-27T01:01:24.7972047Z +    |
2020-03-27T01:01:24.7972047Z +    |
2020-03-27T01:01:24.7972241Z + LL | async fn rec_1() {
2020-03-27T01:01:24.7972447Z +    | ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.7972698Z + note: ...which requires processing `rec_1`...
2020-03-27T01:01:24.7974192Z +    |
2020-03-27T01:01:24.7974192Z +    |
2020-03-27T01:01:24.7974351Z + LL | async fn rec_1() {
2020-03-27T01:01:24.7974884Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.7974884Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.7975841Z +    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T01:01:24.7976372Z + note: ...which requires processing `rec_2::{{opaque}}#0`...
2020-03-27T01:01:24.7985102Z +    |
2020-03-27T01:01:24.7985102Z +    |
2020-03-27T01:01:24.7985285Z + LL | async fn rec_2() {
2020-03-27T01:01:24.7985480Z +    |                  ^
2020-03-27T01:01:24.7985941Z + note: ...which requires borrow-checking `rec_2`...
2020-03-27T01:01:24.7986743Z +    |
2020-03-27T01:01:24.7986743Z +    |
2020-03-27T01:01:24.7986906Z + LL | async fn rec_2() {
2020-03-27T01:01:24.7987114Z +    | ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.7987346Z + note: ...which requires processing `rec_2`...
2020-03-27T01:01:24.8002558Z +    |
2020-03-27T01:01:24.8002558Z +    |
2020-03-27T01:01:24.8002758Z + LL | async fn rec_2() {
2020-03-27T01:01:24.8003459Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8003459Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8004013Z +    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T01:01:24.8004746Z +    = note: ...which again requires processing `rec_1::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8006570Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8007839Z +    |
2020-03-27T01:01:24.8007839Z +    |
2020-03-27T01:01:24.8008023Z + LL | async fn rec_1() {
2020-03-27T01:01:24.8008366Z + 
2020-03-27T01:01:24.8008805Z 1 error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T01:01:24.8010485Z 2   --> $DIR/mutually-recursive-async-impl-trait-type.rs:5:18
2020-03-27T01:01:24.8010745Z 3    |
---
2020-03-27T01:01:24.8014954Z 20 
2020-03-27T01:01:24.8015074Z 
2020-03-27T01:01:24.8015167Z 
2020-03-27T01:01:24.8015364Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8016185Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/mutually-recursive-async-impl-trait-type.stderr
2020-03-27T01:01:24.8016935Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8017605Z To only update this specific test, also pass `--test-args async-await/mutually-recursive-async-impl-trait-type.rs`
2020-03-27T01:01:24.8018106Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8018332Z status: exit code: 1
2020-03-27T01:01:24.8018332Z status: exit code: 1
2020-03-27T01:01:24.8020501Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/auxiliary"
2020-03-27T01:01:24.8022256Z ------------------------------------------
2020-03-27T01:01:24.8022425Z 
2020-03-27T01:01:24.8022790Z ------------------------------------------
2020-03-27T01:01:24.8023003Z stderr:
2020-03-27T01:01:24.8023003Z stderr:
2020-03-27T01:01:24.8023377Z ------------------------------------------
2020-03-27T01:01:24.8023700Z error[E0391]: cycle detected when processing `rec_1::{{opaque}}#0`
2020-03-27T01:01:24.8024656Z    |
2020-03-27T01:01:24.8024656Z    |
2020-03-27T01:01:24.8024877Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8025309Z    |
2020-03-27T01:01:24.8025309Z    |
2020-03-27T01:01:24.8025721Z note: ...which requires borrow-checking `rec_1`...
2020-03-27T01:01:24.8026623Z    |
2020-03-27T01:01:24.8026623Z    |
2020-03-27T01:01:24.8026843Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8027114Z    | ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8027339Z note: ...which requires processing `rec_1`...
2020-03-27T01:01:24.8028236Z    |
2020-03-27T01:01:24.8028236Z    |
2020-03-27T01:01:24.8028455Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8029044Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8029044Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8029579Z    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T01:01:24.8030184Z note: ...which requires processing `rec_2::{{opaque}}#0`...
2020-03-27T01:01:24.8031172Z    |
2020-03-27T01:01:24.8031172Z    |
2020-03-27T01:01:24.8031391Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8031750Z    |                  ^
2020-03-27T01:01:24.8032385Z note: ...which requires borrow-checking `rec_2`...
2020-03-27T01:01:24.8033254Z    |
2020-03-27T01:01:24.8033254Z    |
2020-03-27T01:01:24.8033467Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8033703Z    | ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8033937Z note: ...which requires processing `rec_2`...
2020-03-27T01:01:24.8035015Z    |
2020-03-27T01:01:24.8035015Z    |
2020-03-27T01:01:24.8035251Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8035818Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8035818Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8036549Z    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T01:01:24.8037105Z    = note: ...which again requires processing `rec_1::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8038224Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8039114Z    |
2020-03-27T01:01:24.8039114Z    |
2020-03-27T01:01:24.8039324Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8039710Z 
2020-03-27T01:01:24.8039913Z error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T01:01:24.8040538Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:5:18
2020-03-27T01:01:24.8040816Z    |
2020-03-27T01:01:24.8040816Z    |
2020-03-27T01:01:24.8041028Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8041320Z    |                  ^ recursive `async fn`
2020-03-27T01:01:24.8041943Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-03-27T01:01:24.8042179Z 
2020-03-27T01:01:24.8042407Z error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T01:01:24.8043025Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18
2020-03-27T01:01:24.8043025Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18
2020-03-27T01:01:24.8043315Z    |
2020-03-27T01:01:24.8043552Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T01:01:24.8043836Z    |                  ^ recursive `async fn`
2020-03-27T01:01:24.8044300Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-03-27T01:01:24.8044865Z 
2020-03-27T01:01:24.8045227Z error: aborting due to 3 previous errors
2020-03-27T01:01:24.8045565Z 
---
2020-03-27T01:01:24.8047345Z 
2020-03-27T01:01:24.8048149Z ---- [ui] ui/async-await/recursive-async-impl-trait-type.rs stdout ----
2020-03-27T01:01:24.8048412Z diff of stderr:
2020-03-27T01:01:24.8048549Z 
2020-03-27T01:01:24.8048839Z + error[E0391]: cycle detected when processing `recursive_async_function::{{opaque}}#0`
2020-03-27T01:01:24.8049683Z +    |
2020-03-27T01:01:24.8050081Z + LL | async fn recursive_async_function() -> () {
2020-03-27T01:01:24.8050360Z +    |                                        ^^
2020-03-27T01:01:24.8050670Z +    |
2020-03-27T01:01:24.8050670Z +    |
2020-03-27T01:01:24.8051157Z + note: ...which requires borrow-checking `recursive_async_function`...
2020-03-27T01:01:24.8051931Z +    |
2020-03-27T01:01:24.8052327Z + LL | async fn recursive_async_function() -> () {
2020-03-27T01:01:24.8052679Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8052679Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8053014Z + note: ...which requires processing `recursive_async_function`...
2020-03-27T01:01:24.8054009Z +    |
2020-03-27T01:01:24.8054428Z + LL | async fn recursive_async_function() -> () {
2020-03-27T01:01:24.8054729Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8055102Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8055102Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8055645Z +    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T01:01:24.8056252Z +    = note: ...which again requires processing `recursive_async_function::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8057088Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8058028Z +    |
2020-03-27T01:01:24.8058432Z + LL | async fn recursive_async_function() -> () {
2020-03-27T01:01:24.8058734Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8058926Z + 
---
2020-03-27T01:01:24.8064756Z 12 
2020-03-27T01:01:24.8064859Z 
2020-03-27T01:01:24.8064994Z 
2020-03-27T01:01:24.8065222Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8066023Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2020-03-27T01:01:24.8066755Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8067423Z To only update this specific test, also pass `--test-args async-await/recursive-async-impl-trait-type.rs`
2020-03-27T01:01:24.8067924Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8068160Z status: exit code: 1
2020-03-27T01:01:24.8068160Z status: exit code: 1
2020-03-27T01:01:24.8070320Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/auxiliary"
2020-03-27T01:01:24.8072233Z ------------------------------------------
2020-03-27T01:01:24.8072539Z 
2020-03-27T01:01:24.8072943Z ------------------------------------------
2020-03-27T01:01:24.8073139Z stderr:
2020-03-27T01:01:24.8073139Z stderr:
2020-03-27T01:01:24.8073527Z ------------------------------------------
2020-03-27T01:01:24.8073889Z error[E0391]: cycle detected when processing `recursive_async_function::{{opaque}}#0`
2020-03-27T01:01:24.8074920Z    |
2020-03-27T01:01:24.8074920Z    |
2020-03-27T01:01:24.8075376Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T01:01:24.8075879Z    |
2020-03-27T01:01:24.8075879Z    |
2020-03-27T01:01:24.8076332Z note: ...which requires borrow-checking `recursive_async_function`...
2020-03-27T01:01:24.8077216Z    |
2020-03-27T01:01:24.8077216Z    |
2020-03-27T01:01:24.8077632Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T01:01:24.8077922Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8078244Z note: ...which requires processing `recursive_async_function`...
2020-03-27T01:01:24.8079108Z    |
2020-03-27T01:01:24.8079108Z    |
2020-03-27T01:01:24.8079537Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T01:01:24.8080198Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8080198Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T01:01:24.8080751Z    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T01:01:24.8081328Z    = note: ...which again requires processing `recursive_async_function::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8081971Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8082851Z    |
2020-03-27T01:01:24.8082851Z    |
2020-03-27T01:01:24.8083263Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T01:01:24.8083739Z 
2020-03-27T01:01:24.8083949Z error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T01:01:24.8084730Z   --> /checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs:5:40
2020-03-27T01:01:24.8085192Z    |
2020-03-27T01:01:24.8085192Z    |
2020-03-27T01:01:24.8085642Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T01:01:24.8086320Z    |                                        ^^ recursive `async fn`
2020-03-27T01:01:24.8086823Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-03-27T01:01:24.8087059Z 
2020-03-27T01:01:24.8087257Z error: aborting due to 2 previous errors
2020-03-27T01:01:24.8087421Z 
---
2020-03-27T01:01:24.8089479Z ---- [ui] ui/impl-trait/closure-calling-parent-fn.rs stdout ----
2020-03-27T01:01:24.8089686Z 
2020-03-27T01:01:24.8090083Z error: test compilation failed although it shouldn't!
2020-03-27T01:01:24.8090333Z status: exit code: 1
2020-03-27T01:01:24.8092330Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/closure-calling-parent-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-calling-parent-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-calling-parent-fn/auxiliary"
2020-03-27T01:01:24.8146727Z ------------------------------------------
2020-03-27T01:01:24.8146914Z 
2020-03-27T01:01:24.8147321Z ------------------------------------------
2020-03-27T01:01:24.8147521Z stderr:
2020-03-27T01:01:24.8147521Z stderr:
2020-03-27T01:01:24.8148053Z ------------------------------------------
2020-03-27T01:01:24.8148576Z error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-03-27T01:01:24.8149536Z    |
2020-03-27T01:01:24.8149536Z    |
2020-03-27T01:01:24.8150215Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T01:01:24.8150608Z    |
2020-03-27T01:01:24.8151024Z note: ...which requires borrow-checking `foo`...
2020-03-27T01:01:24.8151576Z   --> /checkout/src/test/ui/impl-trait/closure-calling-parent-fn.rs:10:1
2020-03-27T01:01:24.8152708Z    |
2020-03-27T01:01:24.8152708Z    |
2020-03-27T01:01:24.8153340Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T01:01:24.8153588Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8154072Z note: ...which requires borrow-checking `foo::{{closure}}#0`...
2020-03-27T01:01:24.8155296Z    |
2020-03-27T01:01:24.8155296Z    |
2020-03-27T01:01:24.8155705Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T01:01:24.8155971Z    |                         ^^^^^^^^
2020-03-27T01:01:24.8157177Z note: ...which requires processing `foo::{{closure}}#0`...
2020-03-27T01:01:24.8158522Z    |
2020-03-27T01:01:24.8158522Z    |
2020-03-27T01:01:24.8158911Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T01:01:24.8159486Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T01:01:24.8159486Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T01:01:24.8160031Z    = note: ...which requires evaluating trait selection obligation `impl std::marker::Copy: std::marker::Freeze`...
2020-03-27T01:01:24.8160542Z    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8161147Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8162126Z    |
2020-03-27T01:01:24.8162126Z    |
2020-03-27T01:01:24.8162515Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T01:01:24.8162883Z 
2020-03-27T01:01:24.8162883Z 
2020-03-27T01:01:24.8163127Z error[E0391]: cycle detected when processing `bar::{{opaque}}#0`
2020-03-27T01:01:24.8164244Z    |
2020-03-27T01:01:24.8164244Z    |
2020-03-27T01:01:24.8164784Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T01:01:24.8165211Z    |
2020-03-27T01:01:24.8165211Z    |
2020-03-27T01:01:24.8166356Z note: ...which requires borrow-checking `bar`...
2020-03-27T01:01:24.8182058Z    |
2020-03-27T01:01:24.8182058Z    |
2020-03-27T01:01:24.8182458Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T01:01:24.8182699Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8183198Z note: ...which requires borrow-checking `bar::{{closure}}#0`...
2020-03-27T01:01:24.8184034Z    |
2020-03-27T01:01:24.8184034Z    |
2020-03-27T01:01:24.8184411Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T01:01:24.8184649Z    |                         ^^^^^^^^
2020-03-27T01:01:24.8184932Z note: ...which requires processing `bar::{{closure}}#0`...
2020-03-27T01:01:24.8186507Z    |
2020-03-27T01:01:24.8186507Z    |
2020-03-27T01:01:24.8186902Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T01:01:24.8187749Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T01:01:24.8187749Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T01:01:24.8188469Z    = note: ...which requires evaluating trait selection obligation `impl std::marker::Copy: std::marker::Freeze`...
2020-03-27T01:01:24.8189388Z    = note: ...which again requires processing `bar::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8190020Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8190844Z    |
2020-03-27T01:01:24.8190844Z    |
2020-03-27T01:01:24.8191202Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T01:01:24.8191575Z 
2020-03-27T01:01:24.8191750Z error: aborting due to 2 previous errors
2020-03-27T01:01:24.8191908Z 
2020-03-27T01:01:24.8192329Z For more information about this error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8192329Z For more information about this error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8192560Z 
2020-03-27T01:01:24.8192921Z ------------------------------------------
2020-03-27T01:01:24.8193086Z 
2020-03-27T01:01:24.8193176Z 
2020-03-27T01:01:24.8193633Z ---- [ui] ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs stdout ----
2020-03-27T01:01:24.8193896Z diff of stderr:
2020-03-27T01:01:24.8194011Z 
2020-03-27T01:01:24.8194255Z + error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-03-27T01:01:24.8195039Z +    |
2020-03-27T01:01:24.8195039Z +    |
2020-03-27T01:01:24.8195372Z + LL | fn foo() -> impl Quux {
2020-03-27T01:01:24.8195759Z +    |
2020-03-27T01:01:24.8195759Z +    |
2020-03-27T01:01:24.8196148Z + note: ...which requires borrow-checking `foo`...
2020-03-27T01:01:24.8196872Z +    |
2020-03-27T01:01:24.8196872Z +    |
2020-03-27T01:01:24.8197209Z + LL | fn foo() -> impl Quux {
2020-03-27T01:01:24.8197669Z + note: ...which requires processing `foo`...
2020-03-27T01:01:24.8198152Z +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T01:01:24.8198387Z +    |
2020-03-27T01:01:24.8198387Z +    |
2020-03-27T01:01:24.8198725Z + LL | fn foo() -> impl Quux {
2020-03-27T01:01:24.8198931Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8199208Z +    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T01:01:24.8199675Z +    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T01:01:24.8200111Z + note: ...which requires processing `bar::{{opaque}}#0`...
2020-03-27T01:01:24.8200888Z +    |
2020-03-27T01:01:24.8201225Z + LL | fn bar() -> impl Quux {
2020-03-27T01:01:24.8201432Z +    |             ^^^^^^^^^
2020-03-27T01:01:24.8201432Z +    |             ^^^^^^^^^
2020-03-27T01:01:24.8201878Z + note: ...which requires borrow-checking `bar`...
2020-03-27T01:01:24.8202590Z +    |
2020-03-27T01:01:24.8202948Z + LL | fn bar() -> impl Quux {
2020-03-27T01:01:24.8203155Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8203155Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8203384Z + note: ...which requires processing `bar`...
2020-03-27T01:01:24.8204097Z +    |
2020-03-27T01:01:24.8204433Z + LL | fn bar() -> impl Quux {
2020-03-27T01:01:24.8204662Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8204662Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8204943Z +    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T01:01:24.8205385Z +    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T01:01:24.8205886Z +    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8206477Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8207315Z +    |
2020-03-27T01:01:24.8207315Z +    |
2020-03-27T01:01:24.8207466Z + LL | trait Quux {}
2020-03-27T01:01:24.8207786Z + 
2020-03-27T01:01:24.8207993Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8208524Z 2   --> $DIR/infinite-impl-trait-issue-38064.rs:8:13
2020-03-27T01:01:24.8208746Z 3    |
2020-03-27T01:01:24.8208746Z 3    |
2020-03-27T01:01:24.8208865Z 
2020-03-27T01:01:24.8208983Z 14    |
2020-03-27T01:01:24.8209294Z 15    = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`
2020-03-27T01:01:24.8209952Z - error: aborting due to 2 previous errors
2020-03-27T01:01:24.8210209Z + error: aborting due to 3 previous errors
2020-03-27T01:01:24.8210406Z 18 
2020-03-27T01:01:24.8210834Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T01:01:24.8210834Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T01:01:24.8211166Z + Some errors have detailed explanations: E0391, E0720.
2020-03-27T01:01:24.8211684Z + For more information about an error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8211934Z 20 
2020-03-27T01:01:24.8212028Z 
2020-03-27T01:01:24.8212117Z 
2020-03-27T01:01:24.8212307Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8213227Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
2020-03-27T01:01:24.8214086Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8214934Z To only update this specific test, also pass `--test-args impl-trait/issues/infinite-impl-trait-issue-38064.rs`
2020-03-27T01:01:24.8215538Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8215765Z status: exit code: 1
2020-03-27T01:01:24.8215765Z status: exit code: 1
2020-03-27T01:01:24.8218466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064/auxiliary"
2020-03-27T01:01:24.8220312Z ------------------------------------------
2020-03-27T01:01:24.8220478Z 
2020-03-27T01:01:24.8220994Z ------------------------------------------
2020-03-27T01:01:24.8221178Z stderr:
2020-03-27T01:01:24.8221178Z stderr:
2020-03-27T01:01:24.8221527Z ------------------------------------------
2020-03-27T01:01:24.8221843Z error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-03-27T01:01:24.8222701Z    |
2020-03-27T01:01:24.8222701Z    |
2020-03-27T01:01:24.8223141Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8223560Z    |
2020-03-27T01:01:24.8224126Z note: ...which requires borrow-checking `foo`...
2020-03-27T01:01:24.8224691Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T01:01:24.8224960Z    |
2020-03-27T01:01:24.8224960Z    |
2020-03-27T01:01:24.8225413Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8225909Z note: ...which requires processing `foo`...
2020-03-27T01:01:24.8226487Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T01:01:24.8226756Z    |
2020-03-27T01:01:24.8226756Z    |
2020-03-27T01:01:24.8227190Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8227478Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8227748Z    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T01:01:24.8228288Z    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T01:01:24.8228730Z note: ...which requires processing `bar::{{opaque}}#0`...
2020-03-27T01:01:24.8229641Z    |
2020-03-27T01:01:24.8229641Z    |
2020-03-27T01:01:24.8230162Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8230445Z    |             ^^^^^^^^^
2020-03-27T01:01:24.8230892Z note: ...which requires borrow-checking `bar`...
2020-03-27T01:01:24.8231751Z    |
2020-03-27T01:01:24.8231751Z    |
2020-03-27T01:01:24.8232187Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8232696Z note: ...which requires processing `bar`...
2020-03-27T01:01:24.8233251Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:1
2020-03-27T01:01:24.8233547Z    |
2020-03-27T01:01:24.8233547Z    |
2020-03-27T01:01:24.8234154Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8234414Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8234675Z    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T01:01:24.8235119Z    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T01:01:24.8235582Z    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8236161Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8237315Z    |
2020-03-27T01:01:24.8237315Z    |
2020-03-27T01:01:24.8237470Z LL | trait Quux {}
2020-03-27T01:01:24.8237736Z 
2020-03-27T01:01:24.8237927Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8238516Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:13
2020-03-27T01:01:24.8238779Z    |
2020-03-27T01:01:24.8238779Z    |
2020-03-27T01:01:24.8239201Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8239716Z    |
2020-03-27T01:01:24.8239716Z    |
2020-03-27T01:01:24.8239943Z    = note: expanded type is `foo::Foo<bar::Bar<impl Quux>>`
2020-03-27T01:01:24.8240350Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8240914Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:13
2020-03-27T01:01:24.8241177Z    |
2020-03-27T01:01:24.8241177Z    |
2020-03-27T01:01:24.8241613Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T01:01:24.8242110Z    |
2020-03-27T01:01:24.8242110Z    |
2020-03-27T01:01:24.8242348Z    = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`
2020-03-27T01:01:24.8242720Z error: aborting due to 3 previous errors
2020-03-27T01:01:24.8242872Z 
2020-03-27T01:01:24.8243089Z Some errors have detailed explanations: E0391, E0720.
2020-03-27T01:01:24.8243597Z For more information about an error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8243597Z For more information about an error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8243792Z 
2020-03-27T01:01:24.8244151Z ------------------------------------------
2020-03-27T01:01:24.8244310Z 
2020-03-27T01:01:24.8244397Z 
2020-03-27T01:01:24.8244809Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-direct.rs stdout ----
2020-03-27T01:01:24.8245069Z diff of stderr:
2020-03-27T01:01:24.8245181Z 
2020-03-27T01:01:24.8245600Z + error[E0391]: cycle detected when processing `test::{{opaque}}#0`
2020-03-27T01:01:24.8246401Z +    |
2020-03-27T01:01:24.8246757Z + LL | fn test() -> impl Sized {
2020-03-27T01:01:24.8246979Z +    |              ^^^^^^^^^^
2020-03-27T01:01:24.8247251Z +    |
---
2020-03-27T01:01:24.8250384Z +   --> $DIR/recursive-impl-trait-type-direct.rs:5:1
2020-03-27T01:01:24.8250599Z +    |
2020-03-27T01:01:24.8250923Z + LL | fn test() -> impl Sized {
2020-03-27T01:01:24.8251150Z +    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8251424Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8251859Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8252351Z +    = note: ...which again requires processing `test::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8252934Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8254229Z +    |
2020-03-27T01:01:24.8254229Z +    |
2020-03-27T01:01:24.8254424Z + LL | / #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8254920Z + LL | |
2020-03-27T01:01:24.8255707Z + LL | | fn test() -> impl Sized {
2020-03-27T01:01:24.8255904Z + LL | |
2020-03-27T01:01:24.8256039Z + ...  |
2020-03-27T01:01:24.8256191Z + LL | |
2020-03-27T01:01:24.8256522Z + LL | | fn main() {}
2020-03-27T01:01:24.8257055Z + 
2020-03-27T01:01:24.8257472Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8258159Z 2   --> $DIR/recursive-impl-trait-type-direct.rs:5:14
2020-03-27T01:01:24.8258391Z 3    |
---
2020-03-27T01:01:24.8261749Z 
2020-03-27T01:01:24.8261835Z 
2020-03-27T01:01:24.8262019Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8262942Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-direct/recursive-impl-trait-type-direct.stderr
2020-03-27T01:01:24.8263611Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8264254Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-direct.rs`
2020-03-27T01:01:24.8264943Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8265185Z status: exit code: 1
2020-03-27T01:01:24.8265185Z status: exit code: 1
2020-03-27T01:01:24.8268138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-direct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-direct/auxiliary"
2020-03-27T01:01:24.8269754Z ------------------------------------------
2020-03-27T01:01:24.8269920Z 
2020-03-27T01:01:24.8270443Z ------------------------------------------
2020-03-27T01:01:24.8270637Z stderr:
2020-03-27T01:01:24.8270637Z stderr:
2020-03-27T01:01:24.8271004Z ------------------------------------------
2020-03-27T01:01:24.8271329Z error[E0391]: cycle detected when processing `test::{{opaque}}#0`
2020-03-27T01:01:24.8272196Z    |
2020-03-27T01:01:24.8272638Z LL | fn test() -> impl Sized {
2020-03-27T01:01:24.8272859Z    |              ^^^^^^^^^^
2020-03-27T01:01:24.8273017Z    |
---
2020-03-27T01:01:24.8276049Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-direct.rs:5:1
2020-03-27T01:01:24.8276353Z    |
2020-03-27T01:01:24.8276717Z LL | fn test() -> impl Sized {
2020-03-27T01:01:24.8276938Z    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8277253Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8277899Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8278385Z    = note: ...which again requires processing `test::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8278988Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8279812Z    |
2020-03-27T01:01:24.8280010Z LL | / #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8280191Z LL | |
2020-03-27T01:01:24.8280532Z LL | | fn test() -> impl Sized {
2020-03-27T01:01:24.8280532Z LL | | fn test() -> impl Sized {
2020-03-27T01:01:24.8281441Z LL | |     //~^ ERROR E0720
2020-03-27T01:01:24.8282271Z LL | |
2020-03-27T01:01:24.8282700Z LL | | fn main() {}
2020-03-27T01:01:24.8282897Z    | |____________^
2020-03-27T01:01:24.8283033Z 
---
2020-03-27T01:01:24.8288731Z 
2020-03-27T01:01:24.8289213Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-indirect.rs stdout ----
2020-03-27T01:01:24.8289497Z diff of stderr:
2020-03-27T01:01:24.8289625Z 
2020-03-27T01:01:24.8289914Z + error[E0391]: cycle detected when processing `option::{{opaque}}#0`
2020-03-27T01:01:24.8290789Z +    |
2020-03-27T01:01:24.8290789Z +    |
2020-03-27T01:01:24.8291649Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8292210Z +    |
2020-03-27T01:01:24.8292744Z + note: ...which requires borrow-checking `option`...
2020-03-27T01:01:24.8293306Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T01:01:24.8294111Z +    |
2020-03-27T01:01:24.8294111Z +    |
2020-03-27T01:01:24.8295056Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8295884Z + note: ...which requires processing `option`...
2020-03-27T01:01:24.8296949Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T01:01:24.8297214Z +    |
2020-03-27T01:01:24.8297214Z +    |
2020-03-27T01:01:24.8297802Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8299030Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8299030Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8299507Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8300167Z +    = note: ...which again requires processing `option::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8305013Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8305865Z +    |
2020-03-27T01:01:24.8305865Z +    |
2020-03-27T01:01:24.8306083Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8306339Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8306564Z + LL | |
2020-03-27T01:01:24.8307421Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8307648Z + ...  |
2020-03-27T01:01:24.8307793Z + LL | |
2020-03-27T01:01:24.8307981Z + LL | | fn main() {}
2020-03-27T01:01:24.8308478Z + 
2020-03-27T01:01:24.8308866Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8309404Z 2   --> $DIR/recursive-impl-trait-type-indirect.rs:7:22
2020-03-27T01:01:24.8309642Z 3    |
2020-03-27T01:01:24.8309642Z 3    |
2020-03-27T01:01:24.8309747Z 
2020-03-27T01:01:24.8309884Z 6    |
2020-03-27T01:01:24.8310148Z 7    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-03-27T01:01:24.8310399Z 8 
2020-03-27T01:01:24.8310672Z + error[E0391]: cycle detected when processing `tuple::{{opaque}}#0`
2020-03-27T01:01:24.8311487Z +    |
2020-03-27T01:01:24.8311856Z + LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8312090Z +    |               ^^^^^^^^^^
2020-03-27T01:01:24.8312261Z +    |
2020-03-27T01:01:24.8312261Z +    |
2020-03-27T01:01:24.8312674Z + note: ...which requires borrow-checking `tuple`...
2020-03-27T01:01:24.8313450Z +    |
2020-03-27T01:01:24.8313800Z + LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8314049Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8314049Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8314295Z + note: ...which requires processing `tuple`...
2020-03-27T01:01:24.8315835Z +    |
2020-03-27T01:01:24.8316243Z + LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8316468Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8316782Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8316782Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8317895Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8318415Z +    = note: ...which again requires processing `tuple::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8319143Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8319914Z +    |
2020-03-27T01:01:24.8319914Z +    |
2020-03-27T01:01:24.8320109Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8320351Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8320547Z + LL | |
2020-03-27T01:01:24.8320972Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8321184Z + ...  |
2020-03-27T01:01:24.8321318Z + LL | |
2020-03-27T01:01:24.8321494Z + LL | | fn main() {}
2020-03-27T01:01:24.8321823Z + 
2020-03-27T01:01:24.8322038Z 9 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8322593Z 10   --> $DIR/recursive-impl-trait-type-indirect.rs:12:15
2020-03-27T01:01:24.8322833Z 11    |
2020-03-27T01:01:24.8322833Z 11    |
2020-03-27T01:01:24.8322941Z 
2020-03-27T01:01:24.8323253Z 14    |
2020-03-27T01:01:24.8323461Z 15    = note: expanded type is `(impl Sized,)`
2020-03-27T01:01:24.8323659Z 16 
2020-03-27T01:01:24.8323915Z + error[E0391]: cycle detected when processing `array::{{opaque}}#0`
2020-03-27T01:01:24.8325198Z +    |
2020-03-27T01:01:24.8325578Z + LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8325937Z +    |               ^^^^^^^^^^
2020-03-27T01:01:24.8326132Z +    |
2020-03-27T01:01:24.8326132Z +    |
2020-03-27T01:01:24.8326605Z + note: ...which requires borrow-checking `array`...
2020-03-27T01:01:24.8327441Z +    |
2020-03-27T01:01:24.8327819Z + LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8328079Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8328079Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8328344Z + note: ...which requires processing `array`...
2020-03-27T01:01:24.8329157Z +    |
2020-03-27T01:01:24.8329551Z + LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8329948Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8330422Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8330422Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8331256Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8331770Z +    = note: ...which again requires processing `array::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8332402Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8333959Z +    |
2020-03-27T01:01:24.8333959Z +    |
2020-03-27T01:01:24.8334170Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8334612Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8334994Z + LL | |
2020-03-27T01:01:24.8335532Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8335761Z + ...  |
2020-03-27T01:01:24.8335906Z + LL | |
2020-03-27T01:01:24.8336089Z + LL | | fn main() {}
2020-03-27T01:01:24.8336938Z + 
2020-03-27T01:01:24.8337161Z 17 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8338252Z 18   --> $DIR/recursive-impl-trait-type-indirect.rs:17:15
2020-03-27T01:01:24.8338495Z 19    |
2020-03-27T01:01:24.8338495Z 19    |
2020-03-27T01:01:24.8338601Z 
2020-03-27T01:01:24.8338721Z 22    |
2020-03-27T01:01:24.8338955Z 23    = note: expanded type is `[impl Sized; 1]`
2020-03-27T01:01:24.8339156Z 24 
2020-03-27T01:01:24.8339409Z + error[E0391]: cycle detected when processing `ptr::{{opaque}}#0`
2020-03-27T01:01:24.8340239Z +    |
2020-03-27T01:01:24.8340585Z + LL | fn ptr() -> impl Sized {
2020-03-27T01:01:24.8340820Z +    |             ^^^^^^^^^^
2020-03-27T01:01:24.8340989Z +    |
2020-03-27T01:01:24.8340989Z +    |
2020-03-27T01:01:24.8341391Z + note: ...which requires borrow-checking `ptr`...
2020-03-27T01:01:24.8342161Z +    |
2020-03-27T01:01:24.8342682Z + LL | fn ptr() -> impl Sized {
2020-03-27T01:01:24.8342927Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8342927Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8343175Z + note: ...which requires processing `ptr`...
2020-03-27T01:01:24.8343944Z +    |
2020-03-27T01:01:24.8344330Z + LL | fn ptr() -> impl Sized {
2020-03-27T01:01:24.8344554Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8344857Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8344857Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8345354Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8345881Z +    = note: ...which again requires processing `ptr::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8346510Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8347476Z +    |
2020-03-27T01:01:24.8347476Z +    |
2020-03-27T01:01:24.8347658Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8347924Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8348126Z + LL | |
2020-03-27T01:01:24.8348579Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8348817Z + ...  |
2020-03-27T01:01:24.8348957Z + LL | |
2020-03-27T01:01:24.8349121Z + LL | | fn main() {}
2020-03-27T01:01:24.8349565Z + 
2020-03-27T01:01:24.8349789Z 25 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8350393Z 26   --> $DIR/recursive-impl-trait-type-indirect.rs:22:13
2020-03-27T01:01:24.8350644Z 27    |
2020-03-27T01:01:24.8350644Z 27    |
2020-03-27T01:01:24.8350755Z 
2020-03-27T01:01:24.8350880Z 38    |
2020-03-27T01:01:24.8351330Z 39    = note: expanded type is `fn() -> impl Sized`
2020-03-27T01:01:24.8351551Z 40 
2020-03-27T01:01:24.8351839Z + error[E0391]: cycle detected when processing `closure_capture::{{opaque}}#0`
2020-03-27T01:01:24.8352867Z +    |
2020-03-27T01:01:24.8353248Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8353522Z +    |                         ^^^^^^^^^^
2020-03-27T01:01:24.8353709Z +    |
2020-03-27T01:01:24.8353709Z +    |
2020-03-27T01:01:24.8354142Z + note: ...which requires borrow-checking `closure_capture`...
2020-03-27T01:01:24.8354932Z +    |
2020-03-27T01:01:24.8355317Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8355575Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8355575Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8355872Z + note: ...which requires processing `closure_capture`...
2020-03-27T01:01:24.8356804Z +    |
2020-03-27T01:01:24.8357214Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8358053Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8358363Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8358363Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8358852Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8359382Z +    = note: ...which again requires processing `closure_capture::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8360017Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8360815Z +    |
2020-03-27T01:01:24.8360815Z +    |
2020-03-27T01:01:24.8360989Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8361242Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8361437Z + LL | |
2020-03-27T01:01:24.8361842Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8362114Z + ...  |
2020-03-27T01:01:24.8362250Z + LL | |
2020-03-27T01:01:24.8362408Z + LL | | fn main() {}
2020-03-27T01:01:24.8362922Z + 
2020-03-27T01:01:24.8363151Z 41 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8364052Z 42   --> $DIR/recursive-impl-trait-type-indirect.rs:32:25
2020-03-27T01:01:24.8364311Z 43    |
2020-03-27T01:01:24.8364311Z 43    |
2020-03-27T01:01:24.8364421Z 
2020-03-27T01:01:24.8364545Z 46    |
2020-03-27T01:01:24.8365218Z 47    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:35:5: 37:6 x:impl Sized]`
2020-03-27T01:01:24.8365563Z 48 
2020-03-27T01:01:24.8365866Z + error[E0391]: cycle detected when processing `closure_ref_capture::{{opaque}}#0`
2020-03-27T01:01:24.8366762Z +    |
2020-03-27T01:01:24.8367302Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8367602Z +    |                             ^^^^^^^^^^
2020-03-27T01:01:24.8367803Z +    |
2020-03-27T01:01:24.8367803Z +    |
2020-03-27T01:01:24.8368266Z + note: ...which requires borrow-checking `closure_ref_capture`...
2020-03-27T01:01:24.8369208Z +    |
2020-03-27T01:01:24.8369650Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8369952Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8369952Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8370257Z + note: ...which requires processing `closure_ref_capture`...
2020-03-27T01:01:24.8371053Z +    |
2020-03-27T01:01:24.8371547Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8371836Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8372165Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8372165Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8373955Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8374551Z +    = note: ...which again requires processing `closure_ref_capture::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8375343Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8376334Z +    |
2020-03-27T01:01:24.8376334Z +    |
2020-03-27T01:01:24.8376516Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8376781Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8376983Z + LL | |
2020-03-27T01:01:24.8377395Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8377630Z + ...  |
2020-03-27T01:01:24.8377778Z + LL | |
2020-03-27T01:01:24.8377946Z + LL | | fn main() {}
2020-03-27T01:01:24.8378298Z + 
2020-03-27T01:01:24.8378522Z 49 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8379107Z 50   --> $DIR/recursive-impl-trait-type-indirect.rs:40:29
2020-03-27T01:01:24.8379358Z 51    |
2020-03-27T01:01:24.8379358Z 51    |
2020-03-27T01:01:24.8379469Z 
2020-03-27T01:01:24.8379594Z 54    |
2020-03-27T01:01:24.8380219Z 55    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:43:5: 45:6 x:impl Sized]`
2020-03-27T01:01:24.8380562Z 56 
2020-03-27T01:01:24.8380848Z + error[E0391]: cycle detected when processing `closure_sig::{{opaque}}#0`
2020-03-27T01:01:24.8381719Z +    |
2020-03-27T01:01:24.8382098Z + LL | fn closure_sig() -> impl Sized {
2020-03-27T01:01:24.8382371Z +    |                     ^^^^^^^^^^
2020-03-27T01:01:24.8382558Z +    |
2020-03-27T01:01:24.8382558Z +    |
2020-03-27T01:01:24.8383002Z + note: ...which requires borrow-checking `closure_sig`...
2020-03-27T01:01:24.8383992Z +    |
2020-03-27T01:01:24.8384390Z + LL | fn closure_sig() -> impl Sized {
2020-03-27T01:01:24.8384668Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8384668Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8385245Z + note: ...which requires borrow-checking `closure_sig::{{closure}}#0`...
2020-03-27T01:01:24.8386129Z +    |
2020-03-27T01:01:24.8386308Z + LL |     || closure_sig()
2020-03-27T01:01:24.8386532Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8386532Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8386851Z + note: ...which requires processing `closure_sig::{{closure}}#0`...
2020-03-27T01:01:24.8387737Z +    |
2020-03-27T01:01:24.8387915Z + LL |     || closure_sig()
2020-03-27T01:01:24.8388147Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8388464Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8388464Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8388961Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8389540Z +    = note: ...which again requires processing `closure_sig::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8390215Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8391053Z +    |
2020-03-27T01:01:24.8391053Z +    |
2020-03-27T01:01:24.8391242Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8391627Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8391853Z + LL | |
2020-03-27T01:01:24.8392322Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8392549Z + ...  |
2020-03-27T01:01:24.8392713Z + LL | |
2020-03-27T01:01:24.8392884Z + LL | | fn main() {}
2020-03-27T01:01:24.8393253Z + 
2020-03-27T01:01:24.8393726Z 57 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8394321Z 58   --> $DIR/recursive-impl-trait-type-indirect.rs:48:21
2020-03-27T01:01:24.8394572Z 59    |
2020-03-27T01:01:24.8394572Z 59    |
2020-03-27T01:01:24.8394702Z 
2020-03-27T01:01:24.8394826Z 62    |
2020-03-27T01:01:24.8395405Z 63    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:50:5: 50:21]`
2020-03-27T01:01:24.8395742Z 64 
2020-03-27T01:01:24.8396025Z + error[E0391]: cycle detected when processing `generator_sig::{{opaque}}#0`
2020-03-27T01:01:24.8396904Z +    |
2020-03-27T01:01:24.8397299Z + LL | fn generator_sig() -> impl Sized {
2020-03-27T01:01:24.8397559Z +    |                       ^^^^^^^^^^
2020-03-27T01:01:24.8397767Z +    |
2020-03-27T01:01:24.8397767Z +    |
2020-03-27T01:01:24.8398212Z + note: ...which requires borrow-checking `generator_sig`...
2020-03-27T01:01:24.8399007Z +    |
2020-03-27T01:01:24.8399418Z + LL | fn generator_sig() -> impl Sized {
2020-03-27T01:01:24.8399678Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8399678Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8400240Z + note: ...which requires borrow-checking `generator_sig::{{closure}}#0`...
2020-03-27T01:01:24.8401104Z +    |
2020-03-27T01:01:24.8401279Z + LL |     || generator_sig()
2020-03-27T01:01:24.8401510Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8401510Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8401824Z + note: ...which requires processing `generator_sig::{{closure}}#0`...
2020-03-27T01:01:24.8402686Z +    |
2020-03-27T01:01:24.8402863Z + LL |     || generator_sig()
2020-03-27T01:01:24.8403076Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8403394Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8403394Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8403877Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8404425Z +    = note: ...which again requires processing `generator_sig::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8405093Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8405883Z +    |
2020-03-27T01:01:24.8405883Z +    |
2020-03-27T01:01:24.8406080Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8406326Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8406527Z + LL | |
2020-03-27T01:01:24.8406959Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8407185Z + ...  |
2020-03-27T01:01:24.8407325Z + LL | |
2020-03-27T01:01:24.8407489Z + LL | | fn main() {}
2020-03-27T01:01:24.8407843Z + 
2020-03-27T01:01:24.8408066Z 65 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8408644Z 66   --> $DIR/recursive-impl-trait-type-indirect.rs:53:23
2020-03-27T01:01:24.8408901Z 67    |
2020-03-27T01:01:24.8408901Z 67    |
2020-03-27T01:01:24.8409012Z 
2020-03-27T01:01:24.8409151Z 70    |
2020-03-27T01:01:24.8409732Z 71    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:55:5: 55:23]`
2020-03-27T01:01:24.8410051Z 72 
2020-03-27T01:01:24.8410359Z + error[E0391]: cycle detected when processing `generator_capture::{{opaque}}#0`
2020-03-27T01:01:24.8411218Z +    |
2020-03-27T01:01:24.8411610Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8411902Z +    |                           ^^^^^^^^^^
2020-03-27T01:01:24.8412192Z +    |
2020-03-27T01:01:24.8412192Z +    |
2020-03-27T01:01:24.8412681Z + note: ...which requires borrow-checking `generator_capture`...
2020-03-27T01:01:24.8413964Z +    |
2020-03-27T01:01:24.8414395Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8414784Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8414784Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8415079Z + note: ...which requires processing `generator_capture`...
2020-03-27T01:01:24.8415896Z +    |
2020-03-27T01:01:24.8416282Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8416547Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8416877Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8416877Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8417338Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8417880Z +    = note: ...which again requires processing `generator_capture::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8418534Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8419299Z +    |
2020-03-27T01:01:24.8419299Z +    |
2020-03-27T01:01:24.8419493Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8419735Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8419930Z + LL | |
2020-03-27T01:01:24.8420350Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8420562Z + ...  |
2020-03-27T01:01:24.8420697Z + LL | |
2020-03-27T01:01:24.8420871Z + LL | | fn main() {}
2020-03-27T01:01:24.8421198Z + 
2020-03-27T01:01:24.8421414Z 73 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8421969Z 74   --> $DIR/recursive-impl-trait-type-indirect.rs:58:27
2020-03-27T01:01:24.8422211Z 75    |
2020-03-27T01:01:24.8422211Z 75    |
2020-03-27T01:01:24.8422324Z 
2020-03-27T01:01:24.8422460Z 78    |
2020-03-27T01:01:24.8423069Z 79    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:61:5: 64:6 x:impl Sized {()}]`
2020-03-27T01:01:24.8423412Z 80 
2020-03-27T01:01:24.8423702Z + error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T01:01:24.8424530Z +    |
2020-03-27T01:01:24.8424530Z +    |
2020-03-27T01:01:24.8424948Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8425474Z +    |
2020-03-27T01:01:24.8425474Z +    |
2020-03-27T01:01:24.8425905Z + note: ...which requires borrow-checking `substs_change`...
2020-03-27T01:01:24.8426688Z +    |
2020-03-27T01:01:24.8426688Z +    |
2020-03-27T01:01:24.8427101Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8427421Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8427718Z + note: ...which requires processing `substs_change`...
2020-03-27T01:01:24.8428494Z +    |
2020-03-27T01:01:24.8428494Z +    |
2020-03-27T01:01:24.8428912Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8429566Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8429566Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8430027Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8430554Z +    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8431202Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8431963Z +    |
2020-03-27T01:01:24.8431963Z +    |
2020-03-27T01:01:24.8432156Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8432483Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8432677Z + LL | |
2020-03-27T01:01:24.8433285Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8433490Z + ...  |
2020-03-27T01:01:24.8433619Z + LL | |
2020-03-27T01:01:24.8433787Z + LL | | fn main() {}
2020-03-27T01:01:24.8434102Z + 
2020-03-27T01:01:24.8434102Z + 
2020-03-27T01:01:24.8434445Z + error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T01:01:24.8435490Z +    |
2020-03-27T01:01:24.8435490Z +    |
2020-03-27T01:01:24.8435906Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8437127Z +    |
2020-03-27T01:01:24.8437127Z +    |
2020-03-27T01:01:24.8438270Z + note: ...which requires borrow-checking `substs_change`...
2020-03-27T01:01:24.8439613Z +    |
2020-03-27T01:01:24.8439613Z +    |
2020-03-27T01:01:24.8440070Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8440414Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8440727Z + note: ...which requires processing `substs_change`...
2020-03-27T01:01:24.8441559Z +    |
2020-03-27T01:01:24.8441559Z +    |
2020-03-27T01:01:24.8442019Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8442343Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8442718Z +    = note: ...which requires computing whether `(impl Sized,)` is freeze...
2020-03-27T01:01:24.8443232Z +    = note: ...which requires evaluating trait selection obligation `(impl Sized,): std::marker::Freeze`...
2020-03-27T01:01:24.8443807Z +    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8444501Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8445333Z +    |
2020-03-27T01:01:24.8445333Z +    |
2020-03-27T01:01:24.8445541Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8445950Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8446307Z + LL | |
2020-03-27T01:01:24.8447378Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8447589Z + ...  |
2020-03-27T01:01:24.8447730Z + LL | |
2020-03-27T01:01:24.8447905Z + LL | | fn main() {}
2020-03-27T01:01:24.8448233Z + 
2020-03-27T01:01:24.8448449Z 81 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8449892Z 82   --> $DIR/recursive-impl-trait-type-indirect.rs:67:35
2020-03-27T01:01:24.8450472Z 83    |
2020-03-27T01:01:24.8450472Z 83    |
2020-03-27T01:01:24.8450583Z 
2020-03-27T01:01:24.8450725Z 86    |
2020-03-27T01:01:24.8450931Z 87    = note: expanded type is `(impl Sized,)`
2020-03-27T01:01:24.8451128Z 88 
2020-03-27T01:01:24.8451406Z + error[E0391]: cycle detected when processing `generator_hold::{{opaque}}#0`
2020-03-27T01:01:24.8452345Z +    |
2020-03-27T01:01:24.8452345Z +    |
2020-03-27T01:01:24.8452909Z + LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8453354Z +    |
2020-03-27T01:01:24.8453354Z +    |
2020-03-27T01:01:24.8453977Z + note: ...which requires borrow-checking `generator_hold`...
2020-03-27T01:01:24.8455161Z +    |
2020-03-27T01:01:24.8455161Z +    |
2020-03-27T01:01:24.8455869Z + LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8456309Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8456731Z + note: ...which requires processing `generator_hold`...
2020-03-27T01:01:24.8457875Z +    |
2020-03-27T01:01:24.8457875Z +    |
2020-03-27T01:01:24.8458242Z + LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8458489Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8459787Z +    = note: ...which requires computing whether `[generator@$DIR/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]` is freeze...
2020-03-27T01:01:24.8460808Z +    = note: ...which requires evaluating trait selection obligation `[generator@$DIR/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]: std::marker::Freeze`...
2020-03-27T01:01:24.8461548Z +    = note: ...which again requires processing `generator_hold::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8462430Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8463226Z +    |
2020-03-27T01:01:24.8463226Z +    |
2020-03-27T01:01:24.8463402Z + LL | / #![feature(generators)]
2020-03-27T01:01:24.8463639Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8463834Z + LL | |
2020-03-27T01:01:24.8464437Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8464655Z + ...  |
2020-03-27T01:01:24.8464974Z + LL | |
2020-03-27T01:01:24.8465163Z + LL | | fn main() {}
2020-03-27T01:01:24.8465515Z + 
2020-03-27T01:01:24.8465763Z 89 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8466346Z 90   --> $DIR/recursive-impl-trait-type-indirect.rs:72:24
2020-03-27T01:01:24.8466607Z 91    |
2020-03-27T01:01:24.8466607Z 91    |
2020-03-27T01:01:24.8466721Z 
2020-03-27T01:01:24.8466875Z 94    |
2020-03-27T01:01:24.8467844Z 95    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]`
2020-03-27T01:01:24.8468508Z 96 
2020-03-27T01:01:24.8468808Z + error[E0391]: cycle detected when processing `mutual_recursion::{{opaque}}#0`
2020-03-27T01:01:24.8469635Z +    |
2020-03-27T01:01:24.8470035Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8470293Z +    |                          ^^^^^^^^^
2020-03-27T01:01:24.8470486Z +    |
2020-03-27T01:01:24.8470486Z +    |
2020-03-27T01:01:24.8470940Z + note: ...which requires borrow-checking `mutual_recursion`...
2020-03-27T01:01:24.8471716Z +    |
2020-03-27T01:01:24.8472108Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8472367Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8472367Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8472654Z + note: ...which requires processing `mutual_recursion`...
2020-03-27T01:01:24.8473436Z +    |
2020-03-27T01:01:24.8473816Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8474090Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8474090Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8474370Z + note: ...which requires processing `mutual_recursion`...
2020-03-27T01:01:24.8475127Z +    |
2020-03-27T01:01:24.8475523Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8475787Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8475787Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8476286Z + note: ...which requires unsafety-checking `mutual_recursion`...
2020-03-27T01:01:24.8477080Z +    |
2020-03-27T01:01:24.8477453Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8477726Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8477990Z + note: ...which requires building MIR for...
2020-03-27T01:01:24.8477990Z + note: ...which requires building MIR for...
2020-03-27T01:01:24.8478498Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:86:1
2020-03-27T01:01:24.8478750Z +    |
2020-03-27T01:01:24.8479129Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8479385Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8479884Z + note: ...which requires type-checking `mutual_recursion`...
2020-03-27T01:01:24.8480650Z +    |
2020-03-27T01:01:24.8481039Z + LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8481390Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8481390Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8481788Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Sync`...
---
2020-03-27T01:01:24.8497932Z 
2020-03-27T01:01:24.8498038Z 
2020-03-27T01:01:24.8498230Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8498971Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/recursive-impl-trait-type-indirect.stderr
2020-03-27T01:01:24.8499665Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8500291Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-indirect.rs`
2020-03-27T01:01:24.8500754Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8500974Z status: exit code: 1
2020-03-27T01:01:24.8500974Z status: exit code: 1
2020-03-27T01:01:24.8503071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/auxiliary"
2020-03-27T01:01:24.8505630Z ------------------------------------------
2020-03-27T01:01:24.8505818Z 
2020-03-27T01:01:24.8506180Z ------------------------------------------
2020-03-27T01:01:24.8506370Z stderr:
2020-03-27T01:01:24.8506370Z stderr:
2020-03-27T01:01:24.8506743Z ------------------------------------------
2020-03-27T01:01:24.8507058Z error[E0391]: cycle detected when processing `option::{{opaque}}#0`
2020-03-27T01:01:24.8508296Z    |
2020-03-27T01:01:24.8508296Z    |
2020-03-27T01:01:24.8508667Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8509094Z    |
2020-03-27T01:01:24.8509485Z note: ...which requires borrow-checking `option`...
2020-03-27T01:01:24.8510046Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T01:01:24.8510327Z    |
2020-03-27T01:01:24.8510327Z    |
2020-03-27T01:01:24.8510694Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8511175Z note: ...which requires processing `option`...
2020-03-27T01:01:24.8511741Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T01:01:24.8512004Z    |
2020-03-27T01:01:24.8512004Z    |
2020-03-27T01:01:24.8512361Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8512912Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8512912Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8513497Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8513985Z    = note: ...which again requires processing `option::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8514559Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8515392Z    |
2020-03-27T01:01:24.8515550Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8515764Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8515956Z LL | |
2020-03-27T01:01:24.8515956Z LL | |
2020-03-27T01:01:24.8516317Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8516641Z LL | |
2020-03-27T01:01:24.8516783Z LL | | fn main() {}
2020-03-27T01:01:24.8516945Z    | |____________^
2020-03-27T01:01:24.8517078Z 
2020-03-27T01:01:24.8517078Z 
2020-03-27T01:01:24.8517275Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8517829Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:22
2020-03-27T01:01:24.8518086Z    |
2020-03-27T01:01:24.8518454Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8518937Z    |
2020-03-27T01:01:24.8518937Z    |
2020-03-27T01:01:24.8519195Z    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-03-27T01:01:24.8519410Z 
2020-03-27T01:01:24.8519638Z error[E0391]: cycle detected when processing `tuple::{{opaque}}#0`
2020-03-27T01:01:24.8520512Z    |
2020-03-27T01:01:24.8520839Z LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8521058Z    |               ^^^^^^^^^^
2020-03-27T01:01:24.8521212Z    |
2020-03-27T01:01:24.8521212Z    |
2020-03-27T01:01:24.8521584Z note: ...which requires borrow-checking `tuple`...
2020-03-27T01:01:24.8522527Z    |
2020-03-27T01:01:24.8522857Z LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8523059Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8523059Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8523296Z note: ...which requires processing `tuple`...
2020-03-27T01:01:24.8524164Z    |
2020-03-27T01:01:24.8524535Z LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8524739Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8525007Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8525007Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8525446Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8525914Z    = note: ...which again requires processing `tuple::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8526475Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8527313Z    |
2020-03-27T01:01:24.8527471Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8527702Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8527876Z LL | |
2020-03-27T01:01:24.8527876Z LL | |
2020-03-27T01:01:24.8528239Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8528569Z LL | |
2020-03-27T01:01:24.8528711Z LL | | fn main() {}
2020-03-27T01:01:24.8529084Z    | |____________^
2020-03-27T01:01:24.8529381Z 
2020-03-27T01:01:24.8529381Z 
2020-03-27T01:01:24.8529586Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8530587Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:12:15
2020-03-27T01:01:24.8530881Z    |
2020-03-27T01:01:24.8531227Z LL | fn tuple() -> impl Sized {
2020-03-27T01:01:24.8531646Z    |               ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8531865Z    |
2020-03-27T01:01:24.8532216Z    = note: expanded type is `(impl Sized,)`
2020-03-27T01:01:24.8532377Z 
2020-03-27T01:01:24.8532609Z error[E0391]: cycle detected when processing `array::{{opaque}}#0`
2020-03-27T01:01:24.8533621Z    |
2020-03-27T01:01:24.8533978Z LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8534206Z    |               ^^^^^^^^^^
2020-03-27T01:01:24.8534359Z    |
2020-03-27T01:01:24.8534359Z    |
2020-03-27T01:01:24.8534737Z note: ...which requires borrow-checking `array`...
2020-03-27T01:01:24.8535554Z    |
2020-03-27T01:01:24.8535871Z LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8536089Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8536311Z note: ...which requires processing `array`...
2020-03-27T01:01:24.8536311Z note: ...which requires processing `array`...
2020-03-27T01:01:24.8536840Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:17:1
2020-03-27T01:01:24.8537117Z    |
2020-03-27T01:01:24.8537445Z LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8537648Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8537933Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8538359Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8538832Z    = note: ...which again requires processing `array::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8539613Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8540603Z    |
2020-03-27T01:01:24.8540775Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8540990Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8541164Z LL | |
2020-03-27T01:01:24.8541164Z LL | |
2020-03-27T01:01:24.8541529Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8541953Z LL | |
2020-03-27T01:01:24.8542094Z LL | | fn main() {}
2020-03-27T01:01:24.8542272Z    | |____________^
2020-03-27T01:01:24.8542390Z 
2020-03-27T01:01:24.8542390Z 
2020-03-27T01:01:24.8542582Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8543184Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:17:15
2020-03-27T01:01:24.8543443Z    |
2020-03-27T01:01:24.8543844Z LL | fn array() -> impl Sized {
2020-03-27T01:01:24.8544103Z    |               ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8544319Z    |
2020-03-27T01:01:24.8544503Z    = note: expanded type is `[impl Sized; 1]`
2020-03-27T01:01:24.8544668Z 
2020-03-27T01:01:24.8544912Z error[E0391]: cycle detected when processing `ptr::{{opaque}}#0`
2020-03-27T01:01:24.8545783Z    |
2020-03-27T01:01:24.8546120Z LL | fn ptr() -> impl Sized {
2020-03-27T01:01:24.8546317Z    |             ^^^^^^^^^^
2020-03-27T01:01:24.8546474Z    |
2020-03-27T01:01:24.8546474Z    |
2020-03-27T01:01:24.8546864Z note: ...which requires borrow-checking `ptr`...
2020-03-27T01:01:24.8547660Z    |
2020-03-27T01:01:24.8547973Z LL | fn ptr() -> impl Sized {
2020-03-27T01:01:24.8548186Z    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8548408Z note: ...which requires processing `ptr`...
2020-03-27T01:01:24.8548408Z note: ...which requires processing `ptr`...
2020-03-27T01:01:24.8548941Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:22:1
2020-03-27T01:01:24.8549214Z    |
2020-03-27T01:01:24.8549532Z LL | fn ptr() -> impl Sized {
2020-03-27T01:01:24.8549728Z    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8550010Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8550435Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8550897Z    = note: ...which again requires processing `ptr::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8551473Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8552279Z    |
2020-03-27T01:01:24.8552452Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8552665Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8552845Z LL | |
2020-03-27T01:01:24.8552845Z LL | |
2020-03-27T01:01:24.8553230Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8553540Z LL | |
2020-03-27T01:01:24.8553697Z LL | | fn main() {}
2020-03-27T01:01:24.8553858Z    | |____________^
2020-03-27T01:01:24.8553975Z 
---
2020-03-27T01:01:24.8556236Z 
2020-03-27T01:01:24.8556437Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8556973Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:27:16
2020-03-27T01:01:24.8557223Z    |
2020-03-27T01:01:24.8557563Z LL | fn fn_ptr() -> impl Sized {
2020-03-27T01:01:24.8557997Z    |
2020-03-27T01:01:24.8558377Z    = note: expanded type is `fn() -> impl Sized`
2020-03-27T01:01:24.8558545Z 
2020-03-27T01:01:24.8558545Z 
2020-03-27T01:01:24.8558787Z error[E0391]: cycle detected when processing `closure_capture::{{opaque}}#0`
2020-03-27T01:01:24.8559639Z    |
2020-03-27T01:01:24.8559977Z LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8560287Z    |                         ^^^^^^^^^^
2020-03-27T01:01:24.8560468Z    |
2020-03-27T01:01:24.8560468Z    |
2020-03-27T01:01:24.8561108Z note: ...which requires borrow-checking `closure_capture`...
2020-03-27T01:01:24.8561942Z    |
2020-03-27T01:01:24.8562460Z LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8562757Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8563032Z note: ...which requires processing `closure_capture`...
2020-03-27T01:01:24.8563032Z note: ...which requires processing `closure_capture`...
2020-03-27T01:01:24.8563590Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:1
2020-03-27T01:01:24.8563838Z    |
2020-03-27T01:01:24.8564195Z LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8564421Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8564694Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8565121Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8565599Z    = note: ...which again requires processing `closure_capture::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8566344Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8567662Z    |
2020-03-27T01:01:24.8567842Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8568104Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8568304Z LL | |
2020-03-27T01:01:24.8568304Z LL | |
2020-03-27T01:01:24.8568723Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8569095Z LL | |
2020-03-27T01:01:24.8569258Z LL | | fn main() {}
2020-03-27T01:01:24.8569460Z    | |____________^
2020-03-27T01:01:24.8569748Z 
2020-03-27T01:01:24.8569748Z 
2020-03-27T01:01:24.8570106Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8570823Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:25
2020-03-27T01:01:24.8571266Z    |
2020-03-27T01:01:24.8571607Z LL | fn closure_capture() -> impl Sized {
2020-03-27T01:01:24.8571877Z    |                         ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8572099Z    |
2020-03-27T01:01:24.8572687Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:35:5: 37:6 x:impl Sized]`
2020-03-27T01:01:24.8573009Z 
2020-03-27T01:01:24.8573278Z error[E0391]: cycle detected when processing `closure_ref_capture::{{opaque}}#0`
2020-03-27T01:01:24.8574271Z    |
2020-03-27T01:01:24.8574634Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8574875Z    |                             ^^^^^^^^^^
2020-03-27T01:01:24.8575044Z    |
2020-03-27T01:01:24.8575044Z    |
2020-03-27T01:01:24.8575457Z note: ...which requires borrow-checking `closure_ref_capture`...
2020-03-27T01:01:24.8576264Z    |
2020-03-27T01:01:24.8576622Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8576860Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8576860Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8577117Z note: ...which requires processing `closure_ref_capture`...
2020-03-27T01:01:24.8577922Z    |
2020-03-27T01:01:24.8578273Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8578512Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8578808Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8578808Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8579687Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8580353Z    = note: ...which again requires processing `closure_ref_capture::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8581225Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8582026Z    |
2020-03-27T01:01:24.8582180Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8582387Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8582555Z LL | |
2020-03-27T01:01:24.8582555Z LL | |
2020-03-27T01:01:24.8583009Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8583315Z LL | |
2020-03-27T01:01:24.8583472Z LL | | fn main() {}
2020-03-27T01:01:24.8583629Z    | |____________^
2020-03-27T01:01:24.8583742Z 
2020-03-27T01:01:24.8583742Z 
2020-03-27T01:01:24.8583926Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8584507Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:40:29
2020-03-27T01:01:24.8584758Z    |
2020-03-27T01:01:24.8585100Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T01:01:24.8585398Z    |                             ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8585615Z    |
2020-03-27T01:01:24.8586395Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:43:5: 45:6 x:impl Sized]`
2020-03-27T01:01:24.8586737Z 
2020-03-27T01:01:24.8586979Z error[E0391]: cycle detected when processing `closure_sig::{{opaque}}#0`
2020-03-27T01:01:24.8588008Z    |
2020-03-27T01:01:24.8588343Z LL | fn closure_sig() -> impl Sized {
2020-03-27T01:01:24.8588558Z    |                     ^^^^^^^^^^
2020-03-27T01:01:24.8588731Z    |
2020-03-27T01:01:24.8588731Z    |
2020-03-27T01:01:24.8589108Z note: ...which requires borrow-checking `closure_sig`...
2020-03-27T01:01:24.8590321Z    |
2020-03-27T01:01:24.8590816Z LL | fn closure_sig() -> impl Sized {
2020-03-27T01:01:24.8591044Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8591044Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8591538Z note: ...which requires borrow-checking `closure_sig::{{closure}}#0`...
2020-03-27T01:01:24.8592587Z    |
2020-03-27T01:01:24.8592895Z LL |     || closure_sig()
2020-03-27T01:01:24.8593093Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8593093Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8594112Z note: ...which requires processing `closure_sig::{{closure}}#0`...
2020-03-27T01:01:24.8595165Z    |
2020-03-27T01:01:24.8595336Z LL |     || closure_sig()
2020-03-27T01:01:24.8595544Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8595861Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8595861Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8596347Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8597049Z    = note: ...which again requires processing `closure_sig::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8597635Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8598417Z    |
2020-03-27T01:01:24.8598587Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8598799Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8598969Z LL | |
2020-03-27T01:01:24.8598969Z LL | |
2020-03-27T01:01:24.8599741Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8600074Z LL | |
2020-03-27T01:01:24.8600242Z LL | | fn main() {}
2020-03-27T01:01:24.8600417Z    | |____________^
2020-03-27T01:01:24.8600543Z 
2020-03-27T01:01:24.8600543Z 
2020-03-27T01:01:24.8600749Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8601524Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:48:21
2020-03-27T01:01:24.8601811Z    |
2020-03-27T01:01:24.8602368Z LL | fn closure_sig() -> impl Sized {
2020-03-27T01:01:24.8603121Z    |                     ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8603364Z    |
2020-03-27T01:01:24.8604086Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:50:5: 50:21]`
2020-03-27T01:01:24.8604460Z 
2020-03-27T01:01:24.8604843Z error[E0391]: cycle detected when processing `generator_sig::{{opaque}}#0`
2020-03-27T01:01:24.8605896Z    |
2020-03-27T01:01:24.8606444Z LL | fn generator_sig() -> impl Sized {
2020-03-27T01:01:24.8606833Z    |                       ^^^^^^^^^^
2020-03-27T01:01:24.8606997Z    |
2020-03-27T01:01:24.8606997Z    |
2020-03-27T01:01:24.8607408Z note: ...which requires borrow-checking `generator_sig`...
2020-03-27T01:01:24.8608215Z    |
2020-03-27T01:01:24.8608573Z LL | fn generator_sig() -> impl Sized {
2020-03-27T01:01:24.8608985Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8608985Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8609977Z note: ...which requires borrow-checking `generator_sig::{{closure}}#0`...
2020-03-27T01:01:24.8611667Z    |
2020-03-27T01:01:24.8611986Z LL |     || generator_sig()
2020-03-27T01:01:24.8612208Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8612208Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8612504Z note: ...which requires processing `generator_sig::{{closure}}#0`...
2020-03-27T01:01:24.8613851Z    |
2020-03-27T01:01:24.8614005Z LL |     || generator_sig()
2020-03-27T01:01:24.8614190Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8614469Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8614469Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8614891Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8615381Z    = note: ...which again requires processing `generator_sig::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8616008Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8616817Z    |
2020-03-27T01:01:24.8616995Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8617210Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8617384Z LL | |
2020-03-27T01:01:24.8617384Z LL | |
2020-03-27T01:01:24.8617770Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8618080Z LL | |
2020-03-27T01:01:24.8618223Z LL | | fn main() {}
2020-03-27T01:01:24.8618399Z    | |____________^
2020-03-27T01:01:24.8618517Z 
2020-03-27T01:01:24.8618517Z 
2020-03-27T01:01:24.8618708Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8619276Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:53:23
2020-03-27T01:01:24.8619542Z    |
2020-03-27T01:01:24.8619890Z LL | fn generator_sig() -> impl Sized {
2020-03-27T01:01:24.8620178Z    |                       ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8620387Z    |
2020-03-27T01:01:24.8620956Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:55:5: 55:23]`
2020-03-27T01:01:24.8621263Z 
2020-03-27T01:01:24.8621539Z error[E0391]: cycle detected when processing `generator_capture::{{opaque}}#0`
2020-03-27T01:01:24.8622411Z    |
2020-03-27T01:01:24.8622784Z LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8623025Z    |                           ^^^^^^^^^^
2020-03-27T01:01:24.8623197Z    |
2020-03-27T01:01:24.8623197Z    |
2020-03-27T01:01:24.8623612Z note: ...which requires borrow-checking `generator_capture`...
2020-03-27T01:01:24.8624523Z    |
2020-03-27T01:01:24.8624925Z LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8625166Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8625166Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8625425Z note: ...which requires processing `generator_capture`...
2020-03-27T01:01:24.8626326Z    |
2020-03-27T01:01:24.8626710Z LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8626967Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8627253Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8627253Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8627676Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8628184Z    = note: ...which again requires processing `generator_capture::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8628772Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8630246Z    |
2020-03-27T01:01:24.8630600Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8630846Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8631062Z LL | |
2020-03-27T01:01:24.8631062Z LL | |
2020-03-27T01:01:24.8631482Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8631860Z LL | |
2020-03-27T01:01:24.8632023Z LL | | fn main() {}
2020-03-27T01:01:24.8632522Z    | |____________^
2020-03-27T01:01:24.8632648Z 
2020-03-27T01:01:24.8632648Z 
2020-03-27T01:01:24.8633026Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8633776Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:58:27
2020-03-27T01:01:24.8634035Z    |
2020-03-27T01:01:24.8634398Z LL | fn generator_capture() -> impl Sized {
2020-03-27T01:01:24.8634683Z    |                           ^^^^^^^^^^ expands to a recursive type
2020-03-27T01:01:24.8634903Z    |
2020-03-27T01:01:24.8635540Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:61:5: 64:6 x:impl Sized {()}]`
2020-03-27T01:01:24.8635878Z 
2020-03-27T01:01:24.8636123Z error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T01:01:24.8637009Z    |
2020-03-27T01:01:24.8637009Z    |
2020-03-27T01:01:24.8637399Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8637876Z    |
2020-03-27T01:01:24.8637876Z    |
2020-03-27T01:01:24.8638270Z note: ...which requires borrow-checking `substs_change`...
2020-03-27T01:01:24.8639090Z    |
2020-03-27T01:01:24.8639090Z    |
2020-03-27T01:01:24.8640054Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8640640Z note: ...which requires processing `substs_change`...
2020-03-27T01:01:24.8641259Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-03-27T01:01:24.8641518Z    |
2020-03-27T01:01:24.8641518Z    |
2020-03-27T01:01:24.8641927Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8642505Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8642505Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8642944Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8643426Z    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8644006Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8644832Z    |
2020-03-27T01:01:24.8645113Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8645344Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8645519Z LL | |
2020-03-27T01:01:24.8645519Z LL | |
2020-03-27T01:01:24.8645920Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8646244Z LL | |
2020-03-27T01:01:24.8646385Z LL | | fn main() {}
2020-03-27T01:01:24.8646562Z    | |____________^
2020-03-27T01:01:24.8646680Z 
2020-03-27T01:01:24.8646680Z 
2020-03-27T01:01:24.8646998Z error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T01:01:24.8647915Z    |
2020-03-27T01:01:24.8647915Z    |
2020-03-27T01:01:24.8648300Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8648775Z    |
2020-03-27T01:01:24.8648775Z    |
2020-03-27T01:01:24.8649169Z note: ...which requires borrow-checking `substs_change`...
2020-03-27T01:01:24.8649999Z    |
2020-03-27T01:01:24.8649999Z    |
2020-03-27T01:01:24.8650387Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8650943Z note: ...which requires processing `substs_change`...
2020-03-27T01:01:24.8651488Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-03-27T01:01:24.8651749Z    |
2020-03-27T01:01:24.8651749Z    |
2020-03-27T01:01:24.8652153Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8652429Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8652731Z    = note: ...which requires computing whether `(impl Sized,)` is freeze...
2020-03-27T01:01:24.8653184Z    = note: ...which requires evaluating trait selection obligation `(impl Sized,): std::marker::Freeze`...
2020-03-27T01:01:24.8653806Z    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8654412Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8655255Z    |
2020-03-27T01:01:24.8655412Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8655646Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8655821Z LL | |
2020-03-27T01:01:24.8655821Z LL | |
2020-03-27T01:01:24.8656194Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8656521Z LL | |
2020-03-27T01:01:24.8656661Z LL | | fn main() {}
2020-03-27T01:01:24.8656823Z    | |____________^
2020-03-27T01:01:24.8656956Z 
2020-03-27T01:01:24.8656956Z 
2020-03-27T01:01:24.8657147Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8657703Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:35
2020-03-27T01:01:24.8657977Z    |
2020-03-27T01:01:24.8658402Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T01:01:24.8659132Z    |
2020-03-27T01:01:24.8659132Z    |
2020-03-27T01:01:24.8659307Z    = note: expanded type is `(impl Sized,)`
2020-03-27T01:01:24.8659462Z 
2020-03-27T01:01:24.8659700Z error[E0391]: cycle detected when processing `generator_hold::{{opaque}}#0`
2020-03-27T01:01:24.8660570Z    |
2020-03-27T01:01:24.8660570Z    |
2020-03-27T01:01:24.8660907Z LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8661310Z    |
2020-03-27T01:01:24.8661310Z    |
2020-03-27T01:01:24.8661697Z note: ...which requires borrow-checking `generator_hold`...
2020-03-27T01:01:24.8662495Z    |
2020-03-27T01:01:24.8662495Z    |
2020-03-27T01:01:24.8662829Z LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8663069Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8663311Z note: ...which requires processing `generator_hold`...
2020-03-27T01:01:24.8664256Z    |
2020-03-27T01:01:24.8664256Z    |
2020-03-27T01:01:24.8664591Z LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8664813Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8665599Z    = note: ...which requires computing whether `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]` is freeze...
2020-03-27T01:01:24.8666706Z    = note: ...which requires evaluating trait selection obligation `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]: std::marker::Freeze`...
2020-03-27T01:01:24.8667336Z    = note: ...which again requires processing `generator_hold::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8667917Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8668722Z    |
2020-03-27T01:01:24.8668875Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8669082Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8669250Z LL | |
2020-03-27T01:01:24.8669250Z LL | |
2020-03-27T01:01:24.8669625Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8669922Z LL | |
2020-03-27T01:01:24.8670081Z LL | | fn main() {}
2020-03-27T01:01:24.8670238Z    | |____________^
2020-03-27T01:01:24.8670351Z 
2020-03-27T01:01:24.8670351Z 
2020-03-27T01:01:24.8670535Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8671094Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:72:24
2020-03-27T01:01:24.8671344Z    |
2020-03-27T01:01:24.8671676Z LL | fn generator_hold() -> impl Sized {
2020-03-27T01:01:24.8672162Z    |
2020-03-27T01:01:24.8672162Z    |
2020-03-27T01:01:24.8672754Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]`
2020-03-27T01:01:24.8673592Z 
2020-03-27T01:01:24.8674000Z error[E0391]: cycle detected when processing `mutual_recursion::{{opaque}}#0`
2020-03-27T01:01:24.8674908Z    |
2020-03-27T01:01:24.8675249Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8675471Z    |                          ^^^^^^^^^
2020-03-27T01:01:24.8675644Z    |
2020-03-27T01:01:24.8675644Z    |
2020-03-27T01:01:24.8676210Z note: ...which requires borrow-checking `mutual_recursion`...
2020-03-27T01:01:24.8677011Z    |
2020-03-27T01:01:24.8677350Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8677577Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8677577Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8677839Z note: ...which requires processing `mutual_recursion`...
2020-03-27T01:01:24.8678631Z    |
2020-03-27T01:01:24.8679459Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8679901Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8679901Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8680317Z note: ...which requires processing `mutual_recursion`...
2020-03-27T01:01:24.8681337Z    |
2020-03-27T01:01:24.8681700Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8681941Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8681941Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8682434Z note: ...which requires unsafety-checking `mutual_recursion`...
2020-03-27T01:01:24.8683413Z    |
2020-03-27T01:01:24.8683776Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8684167Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8684520Z note: ...which requires building MIR for...
2020-03-27T01:01:24.8684520Z note: ...which requires building MIR for...
2020-03-27T01:01:24.8685089Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:86:1
2020-03-27T01:01:24.8685337Z    |
2020-03-27T01:01:24.8685670Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8685912Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8686416Z note: ...which requires type-checking `mutual_recursion`...
2020-03-27T01:01:24.8687259Z    |
2020-03-27T01:01:24.8687599Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T01:01:24.8687825Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8687825Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8688193Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Sync`...
2020-03-27T01:01:24.8688618Z note: ...which requires processing `mutual_recursion_b::{{opaque}}#0`...
2020-03-27T01:01:24.8689469Z    |
2020-03-27T01:01:24.8689469Z    |
2020-03-27T01:01:24.8689821Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T01:01:24.8690058Z    |                            ^^^^^^^^^^
2020-03-27T01:01:24.8690514Z note: ...which requires borrow-checking `mutual_recursion_b`...
2020-03-27T01:01:24.8691314Z    |
2020-03-27T01:01:24.8691314Z    |
2020-03-27T01:01:24.8691671Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T01:01:24.8691908Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8692161Z note: ...which requires processing `mutual_recursion_b`...
2020-03-27T01:01:24.8692957Z    |
2020-03-27T01:01:24.8692957Z    |
2020-03-27T01:01:24.8693300Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T01:01:24.8694083Z    = note: ...which requires computing whether `impl std::marker::Sync` is freeze...
2020-03-27T01:01:24.8694083Z    = note: ...which requires computing whether `impl std::marker::Sync` is freeze...
2020-03-27T01:01:24.8694718Z    = note: ...which requires evaluating trait selection obligation `impl std::marker::Sync: std::marker::Freeze`...
2020-03-27T01:01:24.8695221Z    = note: ...which again requires processing `mutual_recursion::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8695802Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8696582Z    |
2020-03-27T01:01:24.8696730Z LL | / #![feature(generators)]
2020-03-27T01:01:24.8696930Z LL | | #![allow(unconditional_recursion)]
2020-03-27T01:01:24.8697108Z LL | |
2020-03-27T01:01:24.8697108Z LL | |
2020-03-27T01:01:24.8697450Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T01:01:24.8697738Z LL | |
2020-03-27T01:01:24.8697886Z LL | | fn main() {}
2020-03-27T01:01:24.8698036Z    | |____________^
2020-03-27T01:01:24.8698152Z 
---
2020-03-27T01:01:24.8700220Z 
2020-03-27T01:01:24.8700400Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8701098Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:91:28
2020-03-27T01:01:24.8701365Z    |
2020-03-27T01:01:24.8701711Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T01:01:24.8702367Z    |
2020-03-27T01:01:24.8702703Z    = note: type resolves to itself
2020-03-27T01:01:24.8702939Z 
2020-03-27T01:01:24.8703102Z error: aborting due to 27 previous errors
---
2020-03-27T01:01:24.8704964Z 
2020-03-27T01:01:24.8705408Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs stdout ----
2020-03-27T01:01:24.8705679Z diff of stderr:
2020-03-27T01:01:24.8705783Z 
2020-03-27T01:01:24.8706015Z + error[E0391]: cycle detected when processing `recursive_id::{{opaque}}#0`
2020-03-27T01:01:24.8706810Z +    |
2020-03-27T01:01:24.8706810Z +    |
2020-03-27T01:01:24.8707138Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T01:01:24.8707540Z +    |
2020-03-27T01:01:24.8707540Z +    |
2020-03-27T01:01:24.8707922Z + note: ...which requires borrow-checking `recursive_id`...
2020-03-27T01:01:24.8708659Z +    |
2020-03-27T01:01:24.8708659Z +    |
2020-03-27T01:01:24.8709157Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T01:01:24.8709618Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8709890Z + note: ...which requires processing `recursive_id`...
2020-03-27T01:01:24.8710848Z +    |
2020-03-27T01:01:24.8710848Z +    |
2020-03-27T01:01:24.8711226Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T01:01:24.8712325Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8712325Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8712805Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8713952Z + note: ...which requires processing `recursive_id2::{{opaque}}#0`...
2020-03-27T01:01:24.8714792Z +    |
2020-03-27T01:01:24.8714792Z +    |
2020-03-27T01:01:24.8715145Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T01:01:24.8715380Z +    |                       ^^^^^^^^^^
2020-03-27T01:01:24.8716013Z + note: ...which requires borrow-checking `recursive_id2`...
2020-03-27T01:01:24.8716908Z +    |
2020-03-27T01:01:24.8716908Z +    |
2020-03-27T01:01:24.8717255Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T01:01:24.8717474Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8717713Z + note: ...which requires processing `recursive_id2`...
2020-03-27T01:01:24.8718444Z +    |
2020-03-27T01:01:24.8718444Z +    |
2020-03-27T01:01:24.8718772Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T01:01:24.8719511Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8719511Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8719928Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8720416Z +    = note: ...which again requires processing `recursive_id::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8720989Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8721764Z +    |
2020-03-27T01:01:24.8721764Z +    |
2020-03-27T01:01:24.8722320Z + LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T01:01:24.8722730Z + 
2020-03-27T01:01:24.8722930Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8723841Z 2   --> $DIR/recursive-impl-trait-type-through-non-recursive.rs:7:22
2020-03-27T01:01:24.8724211Z 3    |
2020-03-27T01:01:24.8724211Z 3    |
2020-03-27T01:01:24.8724316Z 
2020-03-27T01:01:24.8724436Z 14    |
2020-03-27T01:01:24.8724626Z 15    = note: type resolves to itself
2020-03-27T01:01:24.8724825Z 16 
2020-03-27T01:01:24.8725262Z + error[E0391]: cycle detected when processing `recursive_wrap::{{opaque}}#0`
2020-03-27T01:01:24.8726485Z +    |
2020-03-27T01:01:24.8727239Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T01:01:24.8727480Z +    |                        ^^^^^^^^^^
2020-03-27T01:01:24.8727665Z +    |
2020-03-27T01:01:24.8727665Z +    |
2020-03-27T01:01:24.8728084Z + note: ...which requires borrow-checking `recursive_wrap`...
2020-03-27T01:01:24.8729409Z +    |
2020-03-27T01:01:24.8730146Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T01:01:24.8730586Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8730586Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8730860Z + note: ...which requires processing `recursive_wrap`...
2020-03-27T01:01:24.8731832Z +    |
2020-03-27T01:01:24.8732368Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T01:01:24.8732638Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8733096Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8733096Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8733786Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8734237Z + note: ...which requires processing `recursive_wrap2::{{opaque}}#0`...
2020-03-27T01:01:24.8735063Z +    |
2020-03-27T01:01:24.8735425Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T01:01:24.8735659Z +    |                         ^^^^^^^^^^
2020-03-27T01:01:24.8735659Z +    |                         ^^^^^^^^^^
2020-03-27T01:01:24.8736101Z + note: ...which requires borrow-checking `recursive_wrap2`...
2020-03-27T01:01:24.8736885Z +    |
2020-03-27T01:01:24.8737227Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T01:01:24.8737476Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8737476Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8737725Z + note: ...which requires processing `recursive_wrap2`...
2020-03-27T01:01:24.8738496Z +    |
2020-03-27T01:01:24.8738847Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T01:01:24.8739079Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8739373Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8739373Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8739790Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8740267Z +    = note: ...which again requires processing `recursive_wrap::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8740862Z + note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8741611Z +    |
2020-03-27T01:01:24.8741611Z +    |
2020-03-27T01:01:24.8741979Z + LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T01:01:24.8742512Z + 
2020-03-27T01:01:24.8742729Z 17 error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8743283Z 18   --> $DIR/recursive-impl-trait-type-through-non-recursive.rs:17:24
2020-03-27T01:01:24.8743525Z 19    |
2020-03-27T01:01:24.8743525Z 19    |
2020-03-27T01:01:24.8743637Z 
2020-03-27T01:01:24.8743747Z 30    |
2020-03-27T01:01:24.8743939Z 31    = note: expanded type is `((impl Sized,),)`
2020-03-27T01:01:24.8744490Z - error: aborting due to 4 previous errors
2020-03-27T01:01:24.8744728Z + error: aborting due to 6 previous errors
2020-03-27T01:01:24.8744897Z 34 
2020-03-27T01:01:24.8745320Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T01:01:24.8745320Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T01:01:24.8745736Z + Some errors have detailed explanations: E0391, E0720.
2020-03-27T01:01:24.8746257Z + For more information about an error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8746488Z 36 
2020-03-27T01:01:24.8746579Z 
2020-03-27T01:01:24.8746662Z 
2020-03-27T01:01:24.8746840Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8747680Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive/recursive-impl-trait-type-through-non-recursive.stderr
2020-03-27T01:01:24.8748389Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8748997Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-through-non-recursive.rs`
2020-03-27T01:01:24.8750098Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8750342Z status: exit code: 1
2020-03-27T01:01:24.8750342Z status: exit code: 1
2020-03-27T01:01:24.8752735Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive/auxiliary"
2020-03-27T01:01:24.8756048Z ------------------------------------------
2020-03-27T01:01:24.8756402Z 
2020-03-27T01:01:24.8756821Z ------------------------------------------
2020-03-27T01:01:24.8757044Z stderr:
2020-03-27T01:01:24.8757044Z stderr:
2020-03-27T01:01:24.8757434Z ------------------------------------------
2020-03-27T01:01:24.8758031Z error[E0391]: cycle detected when processing `recursive_id::{{opaque}}#0`
2020-03-27T01:01:24.8760017Z    |
2020-03-27T01:01:24.8760017Z    |
2020-03-27T01:01:24.8760475Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8761118Z    |
2020-03-27T01:01:24.8761118Z    |
2020-03-27T01:01:24.8761679Z note: ...which requires borrow-checking `recursive_id`...
2020-03-27T01:01:24.8762566Z    |
2020-03-27T01:01:24.8762566Z    |
2020-03-27T01:01:24.8763018Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8763341Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8763584Z note: ...which requires processing `recursive_id`...
2020-03-27T01:01:24.8764467Z    |
2020-03-27T01:01:24.8764467Z    |
2020-03-27T01:01:24.8764925Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8765528Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8765528Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8765956Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8766391Z note: ...which requires processing `recursive_id2::{{opaque}}#0`...
2020-03-27T01:01:24.8767327Z    |
2020-03-27T01:01:24.8767327Z    |
2020-03-27T01:01:24.8767786Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8768113Z    |                       ^^^^^^^^^^
2020-03-27T01:01:24.8768709Z note: ...which requires borrow-checking `recursive_id2`...
2020-03-27T01:01:24.8769603Z    |
2020-03-27T01:01:24.8769603Z    |
2020-03-27T01:01:24.8770070Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8770376Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8770959Z note: ...which requires processing `recursive_id2`...
2020-03-27T01:01:24.8771885Z    |
2020-03-27T01:01:24.8771885Z    |
2020-03-27T01:01:24.8772387Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8773151Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8773151Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8773709Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8774199Z    = note: ...which again requires processing `recursive_id::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8774823Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8775852Z    |
2020-03-27T01:01:24.8775852Z    |
2020-03-27T01:01:24.8776216Z LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T01:01:24.8776574Z 
2020-03-27T01:01:24.8776759Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8782832Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:7:22
2020-03-27T01:01:24.8783216Z    |
2020-03-27T01:01:24.8783216Z    |
2020-03-27T01:01:24.8783727Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8784297Z    |
2020-03-27T01:01:24.8784618Z    = note: type resolves to itself
2020-03-27T01:01:24.8785000Z 
2020-03-27T01:01:24.8785209Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8785209Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8786190Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:11:23
2020-03-27T01:01:24.8786623Z    |
2020-03-27T01:01:24.8787182Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8788445Z    |
2020-03-27T01:01:24.8789020Z    = note: type resolves to itself
2020-03-27T01:01:24.8789373Z 
2020-03-27T01:01:24.8789373Z 
2020-03-27T01:01:24.8789898Z error[E0391]: cycle detected when processing `recursive_wrap::{{opaque}}#0`
2020-03-27T01:01:24.8791792Z    |
2020-03-27T01:01:24.8791792Z    |
2020-03-27T01:01:24.8792568Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8793992Z    |
2020-03-27T01:01:24.8793992Z    |
2020-03-27T01:01:24.8794485Z note: ...which requires borrow-checking `recursive_wrap`...
2020-03-27T01:01:24.8795670Z    |
2020-03-27T01:01:24.8795670Z    |
2020-03-27T01:01:24.8796191Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8796643Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8796982Z note: ...which requires processing `recursive_wrap`...
2020-03-27T01:01:24.8798106Z    |
2020-03-27T01:01:24.8798106Z    |
2020-03-27T01:01:24.8798623Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8799497Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8799497Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8800008Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8800547Z note: ...which requires processing `recursive_wrap2::{{opaque}}#0`...
2020-03-27T01:01:24.8802240Z    |
2020-03-27T01:01:24.8802240Z    |
2020-03-27T01:01:24.8802976Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8803291Z    |                         ^^^^^^^^^^
2020-03-27T01:01:24.8803769Z note: ...which requires borrow-checking `recursive_wrap2`...
2020-03-27T01:01:24.8804965Z    |
2020-03-27T01:01:24.8804965Z    |
2020-03-27T01:01:24.8805523Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8805975Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T01:01:24.8806351Z note: ...which requires processing `recursive_wrap2`...
2020-03-27T01:01:24.8807493Z    |
2020-03-27T01:01:24.8807493Z    |
2020-03-27T01:01:24.8808062Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8808816Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8808816Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8809338Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8809925Z    = note: ...which again requires processing `recursive_wrap::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8810665Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8812062Z    |
2020-03-27T01:01:24.8812062Z    |
2020-03-27T01:01:24.8812754Z LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T01:01:24.8813651Z 
2020-03-27T01:01:24.8813948Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8815002Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:17:24
2020-03-27T01:01:24.8816623Z    |
2020-03-27T01:01:24.8816623Z    |
2020-03-27T01:01:24.8817207Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8818027Z    |
2020-03-27T01:01:24.8818027Z    |
2020-03-27T01:01:24.8818241Z    = note: expanded type is `((impl Sized,),)`
2020-03-27T01:01:24.8818711Z error[E0720]: opaque type expands to a recursive type
2020-03-27T01:01:24.8819457Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:21:25
2020-03-27T01:01:24.8819878Z    |
2020-03-27T01:01:24.8819878Z    |
2020-03-27T01:01:24.8820437Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T01:01:24.8821257Z    |
2020-03-27T01:01:24.8821257Z    |
2020-03-27T01:01:24.8821473Z    = note: expanded type is `((impl Sized,),)`
2020-03-27T01:01:24.8821926Z error: aborting due to 6 previous errors
2020-03-27T01:01:24.8822383Z 
2020-03-27T01:01:24.8822695Z Some errors have detailed explanations: E0391, E0720.
2020-03-27T01:01:24.8823479Z For more information about an error, try `rustc --explain E0391`.
---
2020-03-27T01:01:24.8825382Z ---- [ui] ui/impl-trait/unsafety-checking-cycle.rs stdout ----
2020-03-27T01:01:24.8825716Z 
2020-03-27T01:01:24.8826321Z error: test compilation failed although it shouldn't!
2020-03-27T01:01:24.8826725Z status: exit code: 1
2020-03-27T01:01:24.8828790Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/unsafety-checking-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/unsafety-checking-cycle/auxiliary"
2020-03-27T01:01:24.8831363Z ------------------------------------------
2020-03-27T01:01:24.8831581Z 
2020-03-27T01:01:24.8832240Z ------------------------------------------
2020-03-27T01:01:24.8832476Z stderr:
2020-03-27T01:01:24.8832476Z stderr:
2020-03-27T01:01:24.8832875Z ------------------------------------------
2020-03-27T01:01:24.8834120Z error[E0391]: cycle detected when processing `not_restricted::{{opaque}}#0`
2020-03-27T01:01:24.8835675Z    |
2020-03-27T01:01:24.8835675Z    |
2020-03-27T01:01:24.8836135Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T01:01:24.8836736Z    |
2020-03-27T01:01:24.8837235Z note: ...which requires borrow-checking `not_restricted`...
2020-03-27T01:01:24.8837768Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-03-27T01:01:24.8838004Z    |
2020-03-27T01:01:24.8838004Z    |
2020-03-27T01:01:24.8838548Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T01:01:24.8839516Z note: ...which requires processing `not_restricted`...
2020-03-27T01:01:24.8840209Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-03-27T01:01:24.8840585Z    |
2020-03-27T01:01:24.8840585Z    |
2020-03-27T01:01:24.8841026Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T01:01:24.8842275Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8842275Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8842882Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8843575Z    = note: ...which again requires processing `not_restricted::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8844401Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8846121Z    |
2020-03-27T01:01:24.8846121Z    |
2020-03-27T01:01:24.8846334Z LL | / #![feature(rustc_attrs)]
2020-03-27T01:01:24.8846500Z LL | |
2020-03-27T01:01:24.8846763Z LL | | struct AnyValue<T>(T);
2020-03-27T01:01:24.8847195Z ...  |
2020-03-27T01:01:24.8847345Z LL | |
2020-03-27T01:01:24.8847510Z LL | | fn main() {}
2020-03-27T01:01:24.8847771Z    | |____________^
2020-03-27T01:01:24.8847771Z    | |____________^
2020-03-27T01:01:24.8847915Z 
2020-03-27T01:01:24.8848198Z error[E0391]: cycle detected when processing `not_field::{{opaque}}#0`
2020-03-27T01:01:24.8849318Z    |
2020-03-27T01:01:24.8849318Z    |
2020-03-27T01:01:24.8849779Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T01:01:24.8850440Z    |
2020-03-27T01:01:24.8850921Z note: ...which requires borrow-checking `not_field`...
2020-03-27T01:01:24.8851621Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-03-27T01:01:24.8852022Z    |
2020-03-27T01:01:24.8852022Z    |
2020-03-27T01:01:24.8852468Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T01:01:24.8853383Z note: ...which requires processing `not_field`...
2020-03-27T01:01:24.8854264Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-03-27T01:01:24.8854880Z    |
2020-03-27T01:01:24.8854880Z    |
2020-03-27T01:01:24.8855340Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T01:01:24.8855975Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8855975Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T01:01:24.8856585Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T01:01:24.8857156Z    = note: ...which again requires processing `not_field::{{opaque}}#0`, completing the cycle
2020-03-27T01:01:24.8857895Z note: cycle used when checking item types in top-level module
2020-03-27T01:01:24.8858972Z    |
2020-03-27T01:01:24.8858972Z    |
2020-03-27T01:01:24.8859284Z LL | / #![feature(rustc_attrs)]
2020-03-27T01:01:24.8859540Z LL | |
2020-03-27T01:01:24.8859736Z LL | | struct AnyValue<T>(T);
2020-03-27T01:01:24.8860029Z ...  |
2020-03-27T01:01:24.8860147Z LL | |
2020-03-27T01:01:24.8860408Z LL | | fn main() {}
2020-03-27T01:01:24.8860595Z    | |____________^
---
2020-03-27T01:01:24.8868210Z +   --> $DIR/issue-52843-closure-constrain.rs:7:1
2020-03-27T01:01:24.8868420Z +    |
2020-03-27T01:01:24.8868561Z + LL | fn main() {
2020-03-27T01:01:24.8868733Z +    | ^^^^^^^^^
2020-03-27T01:01:24.8868981Z +    = note: ...which requires computing whether `Opaque` is freeze...
2020-03-27T01:01:24.8869562Z +    = note: ...which requires evaluating trait selection obligation `Opaque: std::marker::Freeze`...
2020-03-27T01:01:24.8870065Z +    = note: ...which again requires processing `main::Opaque`, completing the cycle
2020-03-27T01:01:24.8870783Z + note: cycle used when collecting item types in top-level module
2020-03-27T01:01:24.8871989Z +    |
2020-03-27T01:01:24.8872211Z + LL | / #![feature(type_alias_impl_trait)]
2020-03-27T01:01:24.8872777Z + LL | |
2020-03-27T01:01:24.8873113Z + LL | | use std::fmt::Debug;
---
2020-03-27T01:01:24.8875073Z 1 error: concrete type differs from previous defining opaque type use
2020-03-27T01:01:24.8876043Z 2   --> $DIR/issue-52843-closure-constrain.rs:10:16
2020-03-27T01:01:24.8876392Z 3    |
2020-03-27T01:01:24.8876513Z 
2020-03-27T01:01:24.8876992Z 10 LL |     fn _unused() -> Opaque { String::new() }
2020-03-27T01:01:24.8877747Z 12 
2020-03-27T01:01:24.8878202Z - error: aborting due to previous error
2020-03-27T01:01:24.8878448Z + error: aborting due to 2 previous errors
2020-03-27T01:01:24.8878755Z 14 
2020-03-27T01:01:24.8878755Z 14 
2020-03-27T01:01:24.8879267Z + For more information about this error, try `rustc --explain E0391`.
2020-03-27T01:01:24.8879580Z 15 
2020-03-27T01:01:24.8879681Z 
2020-03-27T01:01:24.8879910Z 
2020-03-27T01:01:24.8880147Z The actual stderr differed from the expected stderr.
2020-03-27T01:01:24.8881021Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain/issue-52843-closure-constrain.stderr
2020-03-27T01:01:24.8882071Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T01:01:24.8883079Z To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-52843-closure-constrain.rs`
2020-03-27T01:01:24.8884757Z error: 1 errors occurred comparing output.
2020-03-27T01:01:24.8885293Z status: exit code: 1
2020-03-27T01:01:24.8885293Z status: exit code: 1
2020-03-27T01:01:24.8887853Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain/auxiliary"
2020-03-27T01:01:24.8890068Z ------------------------------------------
2020-03-27T01:01:24.8890238Z 
2020-03-27T01:01:24.8890793Z ------------------------------------------
2020-03-27T01:01:24.8891148Z stderr:
---
2020-03-27T01:01:24.8897240Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:7:1
2020-03-27T01:01:24.8897645Z    |
2020-03-27T01:01:24.8897829Z LL | fn main() {
2020-03-27T01:01:24.8898070Z    | ^^^^^^^^^
2020-03-27T01:01:24.8898353Z    = note: ...which requires computing whether `Opaque` is freeze...
2020-03-27T01:01:24.8898857Z    = note: ...which requires evaluating trait selection obligation `Opaque: std::marker::Freeze`...
2020-03-27T01:01:24.8899402Z    = note: ...which again requires processing `main::Opaque`, completing the cycle
2020-03-27T01:01:24.8900119Z note: cycle used when collecting item types in top-level module
2020-03-27T01:01:24.8901281Z    |
2020-03-27T01:01:24.8901488Z LL | / #![feature(type_alias_impl_trait)]
2020-03-27T01:01:24.8901663Z LL | |
2020-03-27T01:01:24.8901828Z LL | | use std::fmt::Debug;
---
2020-03-27T01:01:24.8903211Z 
2020-03-27T01:01:24.8903533Z error: concrete type differs from previous defining opaque type use
2020-03-27T01:01:24.8904277Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:10:16
2020-03-27T01:01:24.8904693Z    |
2020-03-27T01:01:24.8905225Z LL |     let null = || -> Opaque { 0 };
2020-03-27T01:01:24.8905565Z    |                ^^^^^^^^^^^^^^^^^^ expected `std::string::String`, got `i32`
2020-03-27T01:01:24.8906566Z note: previous use here
2020-03-27T01:01:24.8908216Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:9:5
2020-03-27T01:01:24.8908554Z    |
2020-03-27T01:01:24.8908554Z    |
2020-03-27T01:01:24.8909225Z LL |     fn _unused() -> Opaque { String::new() }
2020-03-27T01:01:24.8910305Z 
2020-03-27T01:01:24.8910515Z error: aborting due to 2 previous errors
2020-03-27T01:01:24.8910670Z 
2020-03-27T01:01:24.8911244Z For more information about this error, try `rustc --explain E0391`.
---
2020-03-27T01:01:24.8924228Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T01:01:24.8924816Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T01:01:24.8925166Z 
2020-03-27T01:01:24.8925325Z 
2020-03-27T01:01:24.8929436Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T01:01:24.8934139Z 
2020-03-27T01:01:24.8934284Z 
2020-03-27T01:01:24.8935048Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T01:01:24.8935538Z Build completed unsuccessfully in 1:05:28
2020-03-27T01:01:24.8935538Z Build completed unsuccessfully in 1:05:28
2020-03-27T01:01:24.8935788Z == clock drift check ==
2020-03-27T01:01:24.8936030Z   local time: Fri Mar 27 01:01:24 UTC 2020
2020-03-27T01:01:25.1135222Z   network time: Fri, 27 Mar 2020 01:01:25 GMT
2020-03-27T01:01:25.1135720Z == end clock drift check ==
2020-03-27T01:01:25.5947489Z 
2020-03-27T01:01:25.6020615Z ##[error]Bash exited with code '1'.
2020-03-27T01:01:25.6035392Z ##[section]Finishing: Run build
2020-03-27T01:01:25.6089400Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-03-27T01:01:25.6094814Z Task         : Get sources
2020-03-27T01:01:25.6095168Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T01:01:25.6095510Z Version      : 1.0.0
2020-03-27T01:01:25.6095761Z Author       : Microsoft
2020-03-27T01:01:25.6095761Z Author       : Microsoft
2020-03-27T01:01:25.6096125Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T01:01:25.6096545Z ==============================================================================
2020-03-27T01:01:25.9440162Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T01:01:25.9469110Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-03-27T01:01:25.9567161Z Cleaning up task key
2020-03-27T01:01:25.9568307Z Start cleaning up orphan processes.
2020-03-27T01:01:25.9746312Z Terminate orphan process: pid (4110) (python)
2020-03-27T01:01:25.9963880Z ##[section]Finishing: Finalize Job
