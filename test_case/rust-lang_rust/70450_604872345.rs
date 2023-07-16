plain
2020-03-27T07:02:33.3491597Z ========================== Starting Command Output ===========================
2020-03-27T07:02:33.3495921Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2cbbd860-8a23-441e-ba1e-59310ef55632.sh
2020-03-27T07:02:33.3496162Z 
2020-03-27T07:02:33.3501430Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T07:02:33.3519846Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-03-27T07:02:33.3523056Z Task         : Get sources
2020-03-27T07:02:33.3523377Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T07:02:33.3523627Z Version      : 1.0.0
2020-03-27T07:02:33.3523798Z Author       : Microsoft
---
2020-03-27T07:02:34.5341818Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T07:02:34.5352134Z ##[command]git config gc.auto 0
2020-03-27T07:02:34.5356614Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T07:02:34.5360051Z ##[command]git config --get-all http.proxy
2020-03-27T07:02:34.5367554Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70450/merge:refs/remotes/pull/70450/merge
---
2020-03-27T07:10:18.0515484Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T07:10:19.7896766Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T07:10:22.4672566Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T07:10:30.8577807Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T07:10:35.2243284Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T07:10:36.7139885Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T07:10:45.8324310Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T07:11:21.1742243Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T07:11:57.8920542Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T07:13:56.7601898Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T07:35:05.9155792Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T07:35:08.0005704Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T07:35:09.5694205Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T07:35:21.1554587Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T07:35:25.1295542Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T07:35:26.9206087Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T07:35:39.3088828Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T07:36:23.5967839Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T07:37:12.4592318Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T07:39:35.8829688Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T08:03:13.4261689Z .................................................................................................... 1700/9849
2020-03-27T08:03:17.5397310Z .................................................................................................... 1800/9849
2020-03-27T08:03:28.0118311Z .........................................................................................i.......... 1900/9849
2020-03-27T08:03:34.9549890Z .................................................................................................... 2000/9849
2020-03-27T08:03:41.5766265Z ...............................................................................iiiii................ 2100/9849
2020-03-27T08:04:03.9062084Z .................................................................................................... 2300/9849
2020-03-27T08:04:06.1712799Z .................................................................................................... 2400/9849
2020-03-27T08:04:08.6379148Z .................................................................................................... 2500/9849
2020-03-27T08:04:17.8872958Z .................................................................................................... 2600/9849
---
2020-03-27T08:07:10.9676461Z .....................................................i...............i.............................. 5000/9849
2020-03-27T08:07:18.6829370Z .................................................................................................... 5100/9849
2020-03-27T08:07:26.3413399Z ..................................................................................................i. 5200/9849
2020-03-27T08:07:31.7100633Z .................................................................................................... 5300/9849
2020-03-27T08:07:42.8702154Z ...................................................................................ii.ii........i... 5400/9849
2020-03-27T08:07:46.6319588Z i................................................................................................... 5500/9849
2020-03-27T08:07:56.8727171Z ............................i....................................................................... 5700/9849
2020-03-27T08:08:07.0490297Z .............................................ii....................................i................ 5800/9849
2020-03-27T08:08:14.9653305Z .................................................................................................... 5900/9849
2020-03-27T08:08:20.5706277Z .................................................................................................... 6000/9849
2020-03-27T08:08:20.5706277Z .................................................................................................... 6000/9849
2020-03-27T08:08:30.0290711Z .............................................................................ii...i..ii...........i. 6100/9849
2020-03-27T08:08:51.3300903Z .................................................................................................... 6300/9849
2020-03-27T08:08:58.7812886Z .................................................................................................... 6400/9849
2020-03-27T08:09:06.1997197Z .................................................................................................... 6500/9849
2020-03-27T08:09:06.1997197Z .................................................................................................... 6500/9849
2020-03-27T08:09:19.3978292Z .......i..ii........................................................................................ 6600/9849
2020-03-27T08:09:40.0608939Z .................................................................................................... 6800/9849
2020-03-27T08:09:42.1487608Z .......i............................................................................................ 6900/9849
2020-03-27T08:09:44.2670921Z .................................................................................................... 7000/9849
2020-03-27T08:09:46.5546617Z ...........................................i........................................................ 7100/9849
---
2020-03-27T08:11:25.8687252Z .................................................................................................... 7700/9849
2020-03-27T08:11:30.3320117Z .................................................................................................... 7800/9849
2020-03-27T08:11:35.3853549Z .................................................................................................... 7900/9849
2020-03-27T08:11:42.7022293Z .................................................................................................... 8000/9849
2020-03-27T08:11:50.2947522Z i................................................................................................... 8100/9849
2020-03-27T08:11:58.2675712Z .................................................iiiiiiiiii.i....................................... 8200/9849
2020-03-27T08:12:12.6825890Z i................................................................................................... 8400/9849
2020-03-27T08:12:17.8771976Z .................................................................................................... 8500/9849
2020-03-27T08:12:31.8793906Z .................................................................................................... 8600/9849
2020-03-27T08:12:41.6004328Z .................................................................................................... 8700/9849
---
2020-03-27T08:14:40.6797565Z 
2020-03-27T08:14:40.6798385Z ---- [ui] ui/async-await/mutually-recursive-async-impl-trait-type.rs stdout ----
2020-03-27T08:14:40.6798796Z diff of stderr:
2020-03-27T08:14:40.6798992Z 
2020-03-27T08:14:40.6799635Z + error[E0391]: cycle detected when processing `rec_1::{{opaque}}#0`
2020-03-27T08:14:40.6802282Z +    |
2020-03-27T08:14:40.6802282Z +    |
2020-03-27T08:14:40.6802580Z + LL | async fn rec_1() {
2020-03-27T08:14:40.6803207Z +    |
2020-03-27T08:14:40.6803207Z +    |
2020-03-27T08:14:40.6803842Z + note: ...which requires borrow-checking `rec_1`...
2020-03-27T08:14:40.6805262Z +    |
2020-03-27T08:14:40.6805262Z +    |
2020-03-27T08:14:40.6805901Z + LL | async fn rec_1() {
2020-03-27T08:14:40.6806178Z +    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6806469Z + note: ...which requires processing `rec_1`...
2020-03-27T08:14:40.6807413Z +    |
2020-03-27T08:14:40.6807413Z +    |
2020-03-27T08:14:40.6807644Z + LL | async fn rec_1() {
2020-03-27T08:14:40.6807913Z +    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6808210Z + note: ...which requires const checking `rec_1`...
2020-03-27T08:14:40.6809203Z +    |
2020-03-27T08:14:40.6809203Z +    |
2020-03-27T08:14:40.6809432Z + LL | async fn rec_1() {
2020-03-27T08:14:40.6810425Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6810425Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6810991Z +    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T08:14:40.6811523Z + note: ...which requires processing `rec_2::{{opaque}}#0`...
2020-03-27T08:14:40.6812871Z +    |
2020-03-27T08:14:40.6812871Z +    |
2020-03-27T08:14:40.6813375Z + LL | async fn rec_2() {
2020-03-27T08:14:40.6813658Z +    |                  ^
2020-03-27T08:14:40.6814223Z + note: ...which requires borrow-checking `rec_2`...
2020-03-27T08:14:40.6815401Z +    |
2020-03-27T08:14:40.6815401Z +    |
2020-03-27T08:14:40.6816000Z + LL | async fn rec_2() {
2020-03-27T08:14:40.6816430Z +    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6816893Z + note: ...which requires processing `rec_2`...
2020-03-27T08:14:40.6817814Z +    |
2020-03-27T08:14:40.6817814Z +    |
2020-03-27T08:14:40.6818069Z + LL | async fn rec_2() {
2020-03-27T08:14:40.6818325Z +    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6818640Z + note: ...which requires const checking `rec_2`...
2020-03-27T08:14:40.6819577Z +    |
2020-03-27T08:14:40.6819577Z +    |
2020-03-27T08:14:40.6819810Z + LL | async fn rec_2() {
2020-03-27T08:14:40.6821700Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6821700Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6822287Z +    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T08:14:40.6823036Z +    = note: ...which again requires processing `rec_1::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.6824129Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.6825827Z +    |
2020-03-27T08:14:40.6825827Z +    |
2020-03-27T08:14:40.6826098Z + LL | async fn rec_1() {
2020-03-27T08:14:40.6826640Z + 
2020-03-27T08:14:40.6826976Z 1 error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T08:14:40.6828310Z 2   --> $DIR/mutually-recursive-async-impl-trait-type.rs:5:18
2020-03-27T08:14:40.6833444Z 3    |
---
2020-03-27T08:14:40.6871968Z 20 
2020-03-27T08:14:40.6872068Z 
2020-03-27T08:14:40.6872157Z 
2020-03-27T08:14:40.6872369Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.6873279Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/mutually-recursive-async-impl-trait-type.stderr
2020-03-27T08:14:40.6873952Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.6874581Z To only update this specific test, also pass `--test-args async-await/mutually-recursive-async-impl-trait-type.rs`
2020-03-27T08:14:40.6875034Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.6875263Z status: exit code: 1
2020-03-27T08:14:40.6875263Z status: exit code: 1
2020-03-27T08:14:40.6877259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/auxiliary"
2020-03-27T08:14:40.6878881Z ------------------------------------------
2020-03-27T08:14:40.6879053Z 
2020-03-27T08:14:40.6879387Z ------------------------------------------
2020-03-27T08:14:40.6879570Z stderr:
2020-03-27T08:14:40.6879570Z stderr:
2020-03-27T08:14:40.6879905Z ------------------------------------------
2020-03-27T08:14:40.6880222Z error[E0391]: cycle detected when processing `rec_1::{{opaque}}#0`
2020-03-27T08:14:40.6881284Z    |
2020-03-27T08:14:40.6881284Z    |
2020-03-27T08:14:40.6881500Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6882069Z    |
2020-03-27T08:14:40.6882069Z    |
2020-03-27T08:14:40.6883078Z note: ...which requires borrow-checking `rec_1`...
2020-03-27T08:14:40.6884184Z    |
2020-03-27T08:14:40.6884184Z    |
2020-03-27T08:14:40.6884386Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6884609Z    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6884813Z note: ...which requires processing `rec_1`...
2020-03-27T08:14:40.6885611Z    |
2020-03-27T08:14:40.6885611Z    |
2020-03-27T08:14:40.6885813Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6886055Z    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6886263Z note: ...which requires const checking `rec_1`...
2020-03-27T08:14:40.6887059Z    |
2020-03-27T08:14:40.6887059Z    |
2020-03-27T08:14:40.6887258Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6887793Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6887793Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6888416Z    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T08:14:40.6888853Z note: ...which requires processing `rec_2::{{opaque}}#0`...
2020-03-27T08:14:40.6889797Z    |
2020-03-27T08:14:40.6889797Z    |
2020-03-27T08:14:40.6889998Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6890243Z    |                  ^
2020-03-27T08:14:40.6890641Z note: ...which requires borrow-checking `rec_2`...
2020-03-27T08:14:40.6891434Z    |
2020-03-27T08:14:40.6891434Z    |
2020-03-27T08:14:40.6891633Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6891857Z    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6892323Z note: ...which requires processing `rec_2`...
2020-03-27T08:14:40.6893309Z    |
2020-03-27T08:14:40.6893309Z    |
2020-03-27T08:14:40.6893531Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6893761Z    | ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6893982Z note: ...which requires const checking `rec_2`...
2020-03-27T08:14:40.6895326Z    |
2020-03-27T08:14:40.6895326Z    |
2020-03-27T08:14:40.6895735Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6896699Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6896699Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6897453Z    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T08:14:40.6898002Z    = note: ...which again requires processing `rec_1::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.6898741Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.6900079Z    |
2020-03-27T08:14:40.6900079Z    |
2020-03-27T08:14:40.6901275Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6901636Z 
2020-03-27T08:14:40.6902013Z error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T08:14:40.6902953Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:5:18
2020-03-27T08:14:40.6903409Z    |
2020-03-27T08:14:40.6903409Z    |
2020-03-27T08:14:40.6903622Z LL | async fn rec_1() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6903898Z    |                  ^ recursive `async fn`
2020-03-27T08:14:40.6904683Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-03-27T08:14:40.6904899Z 
2020-03-27T08:14:40.6905089Z error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T08:14:40.6905649Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18
2020-03-27T08:14:40.6905649Z   --> /checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs:9:18
2020-03-27T08:14:40.6905909Z    |
2020-03-27T08:14:40.6906111Z LL | async fn rec_2() { //~ ERROR recursion in an `async fn`
2020-03-27T08:14:40.6906384Z    |                  ^ recursive `async fn`
2020-03-27T08:14:40.6906791Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-03-27T08:14:40.6907004Z 
2020-03-27T08:14:40.6907183Z error: aborting due to 3 previous errors
2020-03-27T08:14:40.6907332Z 
---
2020-03-27T08:14:40.6908802Z 
2020-03-27T08:14:40.6909201Z ---- [ui] ui/async-await/recursive-async-impl-trait-type.rs stdout ----
2020-03-27T08:14:40.6909535Z diff of stderr:
2020-03-27T08:14:40.6909643Z 
2020-03-27T08:14:40.6909923Z + error[E0391]: cycle detected when processing `recursive_async_function::{{opaque}}#0`
2020-03-27T08:14:40.6910745Z +    |
2020-03-27T08:14:40.6911124Z + LL | async fn recursive_async_function() -> () {
2020-03-27T08:14:40.6911377Z +    |                                        ^^
2020-03-27T08:14:40.6911557Z +    |
2020-03-27T08:14:40.6911557Z +    |
2020-03-27T08:14:40.6912140Z + note: ...which requires borrow-checking `recursive_async_function`...
2020-03-27T08:14:40.6913143Z +    |
2020-03-27T08:14:40.6913549Z + LL | async fn recursive_async_function() -> () {
2020-03-27T08:14:40.6913829Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6913829Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6914115Z + note: ...which requires processing `recursive_async_function`...
2020-03-27T08:14:40.6915016Z +    |
2020-03-27T08:14:40.6915538Z + LL | async fn recursive_async_function() -> () {
2020-03-27T08:14:40.6915797Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6915797Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6916111Z + note: ...which requires const checking `recursive_async_function`...
2020-03-27T08:14:40.6916801Z +    |
2020-03-27T08:14:40.6917171Z + LL | async fn recursive_async_function() -> () {
2020-03-27T08:14:40.6917430Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6917780Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6917780Z +    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6918303Z +    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T08:14:40.6918845Z +    = note: ...which again requires processing `recursive_async_function::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.6919440Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.6920138Z +    |
2020-03-27T08:14:40.6920492Z + LL | async fn recursive_async_function() -> () {
2020-03-27T08:14:40.6920768Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6920947Z + 
---
2020-03-27T08:14:40.6924886Z 12 
2020-03-27T08:14:40.6924993Z 
2020-03-27T08:14:40.6925078Z 
2020-03-27T08:14:40.6925255Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.6925927Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2020-03-27T08:14:40.6926557Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.6927120Z To only update this specific test, also pass `--test-args async-await/recursive-async-impl-trait-type.rs`
2020-03-27T08:14:40.6927547Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.6927754Z status: exit code: 1
2020-03-27T08:14:40.6927754Z status: exit code: 1
2020-03-27T08:14:40.6938741Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/auxiliary"
2020-03-27T08:14:40.6941691Z ------------------------------------------
2020-03-27T08:14:40.6941857Z 
2020-03-27T08:14:40.6942191Z ------------------------------------------
2020-03-27T08:14:40.6942392Z stderr:
2020-03-27T08:14:40.6942392Z stderr:
2020-03-27T08:14:40.6942720Z ------------------------------------------
2020-03-27T08:14:40.6943060Z error[E0391]: cycle detected when processing `recursive_async_function::{{opaque}}#0`
2020-03-27T08:14:40.6943919Z    |
2020-03-27T08:14:40.6943919Z    |
2020-03-27T08:14:40.6944286Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T08:14:40.6944737Z    |
2020-03-27T08:14:40.6944737Z    |
2020-03-27T08:14:40.6945123Z note: ...which requires borrow-checking `recursive_async_function`...
2020-03-27T08:14:40.6945905Z    |
2020-03-27T08:14:40.6945905Z    |
2020-03-27T08:14:40.6946687Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T08:14:40.6947142Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6947414Z note: ...which requires processing `recursive_async_function`...
2020-03-27T08:14:40.6948202Z    |
2020-03-27T08:14:40.6948202Z    |
2020-03-27T08:14:40.6948562Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T08:14:40.6949965Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6950448Z note: ...which requires const checking `recursive_async_function`...
2020-03-27T08:14:40.6951468Z    |
2020-03-27T08:14:40.6951468Z    |
2020-03-27T08:14:40.6956808Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T08:14:40.6964363Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6964363Z    = note: ...which requires computing whether `impl std::future::Future` is freeze...
2020-03-27T08:14:40.6964907Z    = note: ...which requires evaluating trait selection obligation `impl std::future::Future: std::marker::Freeze`...
2020-03-27T08:14:40.6965836Z    = note: ...which again requires processing `recursive_async_function::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.6966721Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.6967532Z    |
2020-03-27T08:14:40.6967532Z    |
2020-03-27T08:14:40.6969936Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T08:14:40.6970618Z 
2020-03-27T08:14:40.6970831Z error[E0733]: recursion in an `async fn` requires boxing
2020-03-27T08:14:40.6972302Z   --> /checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs:5:40
2020-03-27T08:14:40.6972577Z    |
2020-03-27T08:14:40.6972577Z    |
2020-03-27T08:14:40.6973189Z LL | async fn recursive_async_function() -> () { //~ ERROR
2020-03-27T08:14:40.6973513Z    |                                        ^^ recursive `async fn`
2020-03-27T08:14:40.6973971Z    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
2020-03-27T08:14:40.6974343Z 
2020-03-27T08:14:40.6974664Z error: aborting due to 2 previous errors
2020-03-27T08:14:40.6974821Z 
---
2020-03-27T08:14:40.6977197Z 
2020-03-27T08:14:40.6977325Z 14    |
2020-03-27T08:14:40.6977654Z 15 LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6977868Z 16    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6978110Z + note: ...which requires const checking `cycle1`...
2020-03-27T08:14:40.6978721Z +    |
2020-03-27T08:14:40.6979045Z + LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6979273Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6979273Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6979509Z 17 note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.6980118Z 19    |
2020-03-27T08:14:40.6980222Z 
2020-03-27T08:14:40.6980839Z 50    |
2020-03-27T08:14:40.6981231Z 51 LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6981231Z 51 LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6981467Z 52    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6981708Z + note: ...which requires const checking `cycle2`...
2020-03-27T08:14:40.6982319Z +    |
2020-03-27T08:14:40.6982634Z + LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6982844Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6982844Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6983094Z 53 note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.6984186Z 55    |
2020-03-27T08:14:40.6984283Z 
2020-03-27T08:14:40.6984416Z 100    |
2020-03-27T08:14:40.6984737Z 101 LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6984737Z 101 LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6984946Z 102    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6985199Z + note: ...which requires const checking `cycle1`...
2020-03-27T08:14:40.6985776Z +    |
2020-03-27T08:14:40.6986101Z + LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6986303Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6986303Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6986531Z 103 note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.6987324Z 105    |
2020-03-27T08:14:40.6987426Z 
2020-03-27T08:14:40.6987535Z 136    |
2020-03-27T08:14:40.6987922Z 137 LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6987922Z 137 LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6988133Z 138    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6988368Z + note: ...which requires const checking `cycle2`...
2020-03-27T08:14:40.6988960Z +    |
2020-03-27T08:14:40.6989274Z + LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6989492Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6989492Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6989722Z 139 note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.6990307Z 141    |
2020-03-27T08:14:40.6990422Z 
2020-03-27T08:14:40.6990532Z 185    |
2020-03-27T08:14:40.6990851Z 186 LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6990851Z 186 LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6991076Z 187    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6991311Z + note: ...which requires const checking `cycle1`...
2020-03-27T08:14:40.6991900Z +    |
2020-03-27T08:14:40.6992207Z + LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.6992409Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6992409Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6992639Z 188 note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.6993681Z 190    |
2020-03-27T08:14:40.6993898Z 
2020-03-27T08:14:40.6994269Z 217 LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6994487Z 218    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6994487Z 218    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6994726Z 219 note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.6995552Z +    |
2020-03-27T08:14:40.6995890Z + LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6996093Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6996093Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.6996342Z + note: ...which requires const checking `cycle2`...
2020-03-27T08:14:40.6996930Z 221    |
2020-03-27T08:14:40.6997261Z 222 LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.6997404Z 
2020-03-27T08:14:40.6997488Z 
2020-03-27T08:14:40.6997488Z 
2020-03-27T08:14:40.6997667Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.6998276Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
2020-03-27T08:14:40.6998845Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.6999366Z To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`
2020-03-27T08:14:40.6999934Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.7000147Z status: exit code: 1
2020-03-27T08:14:40.7000147Z status: exit code: 1
2020-03-27T08:14:40.7001927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
2020-03-27T08:14:40.7003846Z ------------------------------------------
2020-03-27T08:14:40.7003999Z 
2020-03-27T08:14:40.7004317Z ------------------------------------------
2020-03-27T08:14:40.7004509Z stderr:
2020-03-27T08:14:40.7004509Z stderr:
2020-03-27T08:14:40.7004833Z ------------------------------------------
2020-03-27T08:14:40.7005309Z error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
2020-03-27T08:14:40.7006205Z    |
2020-03-27T08:14:40.7006496Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7006704Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7006851Z    |
2020-03-27T08:14:40.7006851Z    |
2020-03-27T08:14:40.7007187Z note: ...which requires borrow-checking `cycle1`...
2020-03-27T08:14:40.7007845Z    |
2020-03-27T08:14:40.7008313Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7008525Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7008525Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7008750Z note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.7009720Z    |
2020-03-27T08:14:40.7010033Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7010227Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7010227Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7010480Z note: ...which requires const checking `cycle1`...
2020-03-27T08:14:40.7011156Z    |
2020-03-27T08:14:40.7011445Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7012040Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7012040Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7012257Z note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.7013116Z    |
2020-03-27T08:14:40.7013419Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7013614Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7013614Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7014356Z note: ...which requires unsafety-checking `cycle1`...
2020-03-27T08:14:40.7015161Z    |
2020-03-27T08:14:40.7015480Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7015675Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7015941Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7015941Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7016413Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:12:1
2020-03-27T08:14:40.7016644Z    |
2020-03-27T08:14:40.7016943Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7017139Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7017538Z note: ...which requires type-checking `cycle1`...
2020-03-27T08:14:40.7018199Z    |
2020-03-27T08:14:40.7018690Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7018894Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7019257Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7019257Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7019716Z note: ...which requires processing `cycle2::{{opaque}}#0`...
2020-03-27T08:14:40.7020537Z    |
2020-03-27T08:14:40.7020889Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7021088Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7021088Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7021474Z note: ...which requires borrow-checking `cycle2`...
2020-03-27T08:14:40.7022366Z    |
2020-03-27T08:14:40.7022667Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7023045Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7023045Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7023257Z note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.7023912Z    |
2020-03-27T08:14:40.7024201Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7024391Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7024391Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7024630Z note: ...which requires const checking `cycle2`...
2020-03-27T08:14:40.7025275Z    |
2020-03-27T08:14:40.7025581Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7025774Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7025774Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7025985Z note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.7026645Z    |
2020-03-27T08:14:40.7026938Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7027127Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7027127Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7027870Z note: ...which requires unsafety-checking `cycle2`...
2020-03-27T08:14:40.7028745Z    |
2020-03-27T08:14:40.7029055Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7029244Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7029450Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7029450Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7029905Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:22:1
2020-03-27T08:14:40.7030107Z    |
2020-03-27T08:14:40.7030400Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7030604Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7030976Z note: ...which requires type-checking `cycle2`...
2020-03-27T08:14:40.7031635Z    |
2020-03-27T08:14:40.7031925Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7032114Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7032465Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7032465Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7032945Z    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7033454Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7034463Z    |
2020-03-27T08:14:40.7034619Z LL | / use std::cell::Cell;
2020-03-27T08:14:40.7036089Z LL | | use std::rc::Rc;
2020-03-27T08:14:40.7036525Z LL | |
2020-03-27T08:14:40.7036525Z LL | |
2020-03-27T08:14:40.7036694Z LL | | fn send<T: Send>(_: T) {}
2020-03-27T08:14:40.7037330Z LL | |     Rc::new(String::from("foo"))
2020-03-27T08:14:40.7037510Z LL | | }
2020-03-27T08:14:40.7037629Z    | |_^
2020-03-27T08:14:40.7037723Z 
2020-03-27T08:14:40.7037723Z 
2020-03-27T08:14:40.7037959Z error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
2020-03-27T08:14:40.7038747Z    |
2020-03-27T08:14:40.7039061Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7039251Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7039397Z    |
2020-03-27T08:14:40.7039397Z    |
2020-03-27T08:14:40.7039748Z note: ...which requires borrow-checking `cycle1`...
2020-03-27T08:14:40.7040400Z    |
2020-03-27T08:14:40.7040709Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7040899Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7040899Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7041111Z note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.7041770Z    |
2020-03-27T08:14:40.7042062Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7042267Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7042267Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7042485Z note: ...which requires const checking `cycle1`...
2020-03-27T08:14:40.7043131Z    |
2020-03-27T08:14:40.7043436Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7043627Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7043627Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7043839Z note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.7044681Z    |
2020-03-27T08:14:40.7044969Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7045170Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7045170Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7045538Z note: ...which requires unsafety-checking `cycle1`...
2020-03-27T08:14:40.7046185Z    |
2020-03-27T08:14:40.7046473Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7046657Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7046874Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7046874Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7047289Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:12:1
2020-03-27T08:14:40.7047770Z    |
2020-03-27T08:14:40.7048075Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7048265Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7048787Z note: ...which requires type-checking `cycle1`...
2020-03-27T08:14:40.7049425Z    |
2020-03-27T08:14:40.7049705Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7049892Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7050248Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7050248Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7050653Z note: ...which requires processing `cycle2::{{opaque}}#0`...
2020-03-27T08:14:40.7051334Z    |
2020-03-27T08:14:40.7051615Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7051798Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7051798Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7052173Z note: ...which requires borrow-checking `cycle2`...
2020-03-27T08:14:40.7052977Z    |
2020-03-27T08:14:40.7053287Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7053475Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7053475Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7053685Z note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.7054566Z    |
2020-03-27T08:14:40.7055221Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7055435Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7055435Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7055659Z note: ...which requires const checking `cycle2`...
2020-03-27T08:14:40.7057376Z    |
2020-03-27T08:14:40.7057705Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7058089Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7058089Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7058299Z note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.7059323Z    |
2020-03-27T08:14:40.7059791Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7060004Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7060004Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7060809Z note: ...which requires unsafety-checking `cycle2`...
2020-03-27T08:14:40.7061828Z    |
2020-03-27T08:14:40.7062118Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7062302Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7062517Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7062517Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7062939Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:22:1
2020-03-27T08:14:40.7063138Z    |
2020-03-27T08:14:40.7063437Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7063621Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7063981Z note: ...which requires type-checking `cycle2`...
2020-03-27T08:14:40.7064618Z    |
2020-03-27T08:14:40.7064899Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7065083Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7065083Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7065391Z    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7065884Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7066546Z    |
2020-03-27T08:14:40.7066694Z LL | / use std::cell::Cell;
2020-03-27T08:14:40.7066876Z LL | | use std::rc::Rc;
2020-03-27T08:14:40.7067037Z LL | |
2020-03-27T08:14:40.7067037Z LL | |
2020-03-27T08:14:40.7067200Z LL | | fn send<T: Send>(_: T) {}
2020-03-27T08:14:40.7067545Z LL | |     Rc::new(String::from("foo"))
2020-03-27T08:14:40.7067718Z LL | | }
2020-03-27T08:14:40.7067834Z    | |_^
2020-03-27T08:14:40.7067926Z 
2020-03-27T08:14:40.7067926Z 
2020-03-27T08:14:40.7068157Z error[E0391]: cycle detected when processing `cycle1::{{opaque}}#0`
2020-03-27T08:14:40.7068839Z    |
2020-03-27T08:14:40.7069140Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7069326Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7069466Z    |
2020-03-27T08:14:40.7069466Z    |
2020-03-27T08:14:40.7069806Z note: ...which requires borrow-checking `cycle1`...
2020-03-27T08:14:40.7070440Z    |
2020-03-27T08:14:40.7070738Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7070922Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7070922Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7071125Z note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.7071764Z    |
2020-03-27T08:14:40.7072050Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7072234Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7072234Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7072460Z note: ...which requires const checking `cycle1`...
2020-03-27T08:14:40.7073086Z    |
2020-03-27T08:14:40.7073385Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7073568Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7073568Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7073773Z note: ...which requires processing `cycle1`...
2020-03-27T08:14:40.7074503Z    |
2020-03-27T08:14:40.7074809Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7075010Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7075010Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7075379Z note: ...which requires unsafety-checking `cycle1`...
2020-03-27T08:14:40.7076090Z    |
2020-03-27T08:14:40.7076393Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7076576Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7076792Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7076792Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7077209Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:12:1
2020-03-27T08:14:40.7077407Z    |
2020-03-27T08:14:40.7077688Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7077887Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7078242Z note: ...which requires type-checking `cycle1`...
2020-03-27T08:14:40.7078880Z    |
2020-03-27T08:14:40.7079168Z LL | fn cycle1() -> impl Clone {
2020-03-27T08:14:40.7079351Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7079709Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7079709Z    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
2020-03-27T08:14:40.7080115Z note: ...which requires processing `cycle2::{{opaque}}#0`...
2020-03-27T08:14:40.7080996Z    |
2020-03-27T08:14:40.7081286Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7081476Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7081476Z    |                ^^^^^^^^^^
2020-03-27T08:14:40.7081862Z note: ...which requires borrow-checking `cycle2`...
2020-03-27T08:14:40.7082507Z    |
2020-03-27T08:14:40.7082810Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7083148Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7083148Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7083352Z note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.7084159Z    |
2020-03-27T08:14:40.7084770Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7084963Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7084963Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7085171Z note: ...which requires const checking `cycle2`...
2020-03-27T08:14:40.7085778Z    |
2020-03-27T08:14:40.7086067Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7086246Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7086246Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7086445Z note: ...which requires processing `cycle2`...
2020-03-27T08:14:40.7087059Z    |
2020-03-27T08:14:40.7087330Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7087858Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7087858Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7088642Z note: ...which requires unsafety-checking `cycle2`...
2020-03-27T08:14:40.7089497Z    |
2020-03-27T08:14:40.7089794Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7089988Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7090217Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7090217Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7090848Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:22:1
2020-03-27T08:14:40.7091067Z    |
2020-03-27T08:14:40.7091423Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7091619Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7092374Z note: ...which requires type-checking `cycle2`...
2020-03-27T08:14:40.7093011Z    |
2020-03-27T08:14:40.7093292Z LL | fn cycle2() -> impl Clone {
2020-03-27T08:14:40.7093474Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7093474Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7093779Z    = note: ...which again requires processing `cycle1::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7094271Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7095051Z    |
2020-03-27T08:14:40.7095197Z LL | / use std::cell::Cell;
2020-03-27T08:14:40.7095379Z LL | | use std::rc::Rc;
2020-03-27T08:14:40.7095540Z LL | |
2020-03-27T08:14:40.7095540Z LL | |
2020-03-27T08:14:40.7095751Z LL | | fn send<T: Send>(_: T) {}
2020-03-27T08:14:40.7096101Z LL | |     Rc::new(String::from("foo"))
2020-03-27T08:14:40.7096274Z LL | | }
2020-03-27T08:14:40.7096389Z    | |_^
2020-03-27T08:14:40.7096481Z 
2020-03-27T08:14:40.7096481Z 
2020-03-27T08:14:40.7096731Z error[E0277]: `std::rc::Rc<std::string::String>` cannot be sent between threads safely
2020-03-27T08:14:40.7097247Z   --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:16:5
2020-03-27T08:14:40.7097446Z    |
2020-03-27T08:14:40.7097618Z LL | fn send<T: Send>(_: T) {}
2020-03-27T08:14:40.7098169Z ...
2020-03-27T08:14:40.7098330Z LL |     send(cycle2().clone());
2020-03-27T08:14:40.7098786Z    |     ^^^^ `std::rc::Rc<std::string::String>` cannot be sent between threads safely
2020-03-27T08:14:40.7099025Z ...
---
2020-03-27T08:14:40.7104532Z ---- [ui] ui/impl-trait/closure-calling-parent-fn.rs stdout ----
2020-03-27T08:14:40.7104728Z 
2020-03-27T08:14:40.7105218Z error: test compilation failed although it shouldn't!
2020-03-27T08:14:40.7105615Z status: exit code: 1
2020-03-27T08:14:40.7107490Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/closure-calling-parent-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-calling-parent-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-calling-parent-fn/auxiliary"
2020-03-27T08:14:40.7109005Z ------------------------------------------
2020-03-27T08:14:40.7109144Z 
2020-03-27T08:14:40.7109431Z ------------------------------------------
2020-03-27T08:14:40.7109607Z stderr:
2020-03-27T08:14:40.7109607Z stderr:
2020-03-27T08:14:40.7109932Z ------------------------------------------
2020-03-27T08:14:40.7110197Z error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-03-27T08:14:40.7111847Z    |
2020-03-27T08:14:40.7111847Z    |
2020-03-27T08:14:40.7112186Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7112679Z    |
2020-03-27T08:14:40.7112991Z note: ...which requires borrow-checking `foo`...
2020-03-27T08:14:40.7113443Z   --> /checkout/src/test/ui/impl-trait/closure-calling-parent-fn.rs:10:1
2020-03-27T08:14:40.7114027Z    |
2020-03-27T08:14:40.7114027Z    |
2020-03-27T08:14:40.7114621Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7114889Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7115573Z note: ...which requires borrow-checking `foo::{{closure}}#0`...
2020-03-27T08:14:40.7116487Z    |
2020-03-27T08:14:40.7116487Z    |
2020-03-27T08:14:40.7116938Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7117207Z    |                         ^^^^^^^^
2020-03-27T08:14:40.7117878Z note: ...which requires processing `foo::{{closure}}#0`...
2020-03-27T08:14:40.7118791Z    |
2020-03-27T08:14:40.7118791Z    |
2020-03-27T08:14:40.7119256Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7119481Z    |                         ^^^^^^^^
2020-03-27T08:14:40.7119738Z note: ...which requires const checking `foo::{{closure}}#0`...
2020-03-27T08:14:40.7120456Z    |
2020-03-27T08:14:40.7120456Z    |
2020-03-27T08:14:40.7120752Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7121276Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T08:14:40.7121276Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T08:14:40.7121731Z    = note: ...which requires evaluating trait selection obligation `impl std::marker::Copy: std::marker::Freeze`...
2020-03-27T08:14:40.7122377Z    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7123064Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7123808Z    |
2020-03-27T08:14:40.7123808Z    |
2020-03-27T08:14:40.7124125Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7124607Z 
2020-03-27T08:14:40.7124607Z 
2020-03-27T08:14:40.7124836Z error[E0391]: cycle detected when processing `bar::{{opaque}}#0`
2020-03-27T08:14:40.7125960Z    |
2020-03-27T08:14:40.7125960Z    |
2020-03-27T08:14:40.7126433Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T08:14:40.7126759Z    |
2020-03-27T08:14:40.7126759Z    |
2020-03-27T08:14:40.7127090Z note: ...which requires borrow-checking `bar`...
2020-03-27T08:14:40.7127759Z    |
2020-03-27T08:14:40.7127759Z    |
2020-03-27T08:14:40.7128076Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T08:14:40.7128438Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7128850Z note: ...which requires borrow-checking `bar::{{closure}}#0`...
2020-03-27T08:14:40.7129730Z    |
2020-03-27T08:14:40.7129730Z    |
2020-03-27T08:14:40.7130026Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T08:14:40.7130233Z    |                         ^^^^^^^^
2020-03-27T08:14:40.7130496Z note: ...which requires processing `bar::{{closure}}#0`...
2020-03-27T08:14:40.7131196Z    |
2020-03-27T08:14:40.7131196Z    |
2020-03-27T08:14:40.7131509Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T08:14:40.7131716Z    |                         ^^^^^^^^
2020-03-27T08:14:40.7131976Z note: ...which requires const checking `bar::{{closure}}#0`...
2020-03-27T08:14:40.7132895Z    |
2020-03-27T08:14:40.7132895Z    |
2020-03-27T08:14:40.7133540Z LL | fn bar() -> impl Copy { || bar(); }
2020-03-27T08:14:40.7134073Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T08:14:40.7134073Z    = note: ...which requires computing whether `impl std::marker::Copy` is freeze...
2020-03-27T08:14:40.7134535Z    = note: ...which requires evaluating trait selection obligation `impl std::marker::Copy: std::marker::Freeze`...
2020-03-27T08:14:40.7135160Z    = note: ...which again requires processing `bar::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7135735Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7136433Z    |
2020-03-27T08:14:40.7136433Z    |
2020-03-27T08:14:40.7136730Z LL | fn foo() -> impl Copy { || foo(); }
2020-03-27T08:14:40.7137118Z 
2020-03-27T08:14:40.7137271Z error: aborting due to 2 previous errors
2020-03-27T08:14:40.7137411Z 
2020-03-27T08:14:40.7137785Z For more information about this error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7137785Z For more information about this error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7137979Z 
2020-03-27T08:14:40.7138277Z ------------------------------------------
2020-03-27T08:14:40.7138419Z 
2020-03-27T08:14:40.7138498Z 
2020-03-27T08:14:40.7138885Z ---- [ui] ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs stdout ----
2020-03-27T08:14:40.7139113Z diff of stderr:
2020-03-27T08:14:40.7139215Z 
2020-03-27T08:14:40.7139424Z + error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-03-27T08:14:40.7140091Z +    |
2020-03-27T08:14:40.7140091Z +    |
2020-03-27T08:14:40.7140556Z + LL | fn foo() -> impl Quux {
2020-03-27T08:14:40.7140993Z +    |
2020-03-27T08:14:40.7140993Z +    |
2020-03-27T08:14:40.7141346Z + note: ...which requires borrow-checking `foo`...
2020-03-27T08:14:40.7141968Z +    |
2020-03-27T08:14:40.7141968Z +    |
2020-03-27T08:14:40.7142247Z + LL | fn foo() -> impl Quux {
2020-03-27T08:14:40.7142644Z + note: ...which requires processing `foo`...
2020-03-27T08:14:40.7143046Z +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T08:14:40.7143252Z +    |
2020-03-27T08:14:40.7143252Z +    |
2020-03-27T08:14:40.7143532Z + LL | fn foo() -> impl Quux {
2020-03-27T08:14:40.7144328Z + note: ...which requires const checking `foo`...
2020-03-27T08:14:40.7144957Z +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T08:14:40.7145158Z +    |
2020-03-27T08:14:40.7145158Z +    |
2020-03-27T08:14:40.7145446Z + LL | fn foo() -> impl Quux {
2020-03-27T08:14:40.7145649Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7145903Z +    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T08:14:40.7146310Z +    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T08:14:40.7146883Z + note: ...which requires processing `bar::{{opaque}}#0`...
2020-03-27T08:14:40.7147675Z +    |
2020-03-27T08:14:40.7147961Z + LL | fn bar() -> impl Quux {
2020-03-27T08:14:40.7148135Z +    |             ^^^^^^^^^
2020-03-27T08:14:40.7148135Z +    |             ^^^^^^^^^
2020-03-27T08:14:40.7148479Z + note: ...which requires borrow-checking `bar`...
2020-03-27T08:14:40.7149080Z +    |
2020-03-27T08:14:40.7149350Z + LL | fn bar() -> impl Quux {
2020-03-27T08:14:40.7149542Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7149542Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7149741Z + note: ...which requires processing `bar`...
2020-03-27T08:14:40.7150338Z +    |
2020-03-27T08:14:40.7150611Z + LL | fn bar() -> impl Quux {
2020-03-27T08:14:40.7150788Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7150788Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7150989Z + note: ...which requires const checking `bar`...
2020-03-27T08:14:40.7151594Z +    |
2020-03-27T08:14:40.7151864Z + LL | fn bar() -> impl Quux {
2020-03-27T08:14:40.7152055Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7152055Z +    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7152293Z +    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T08:14:40.7152890Z +    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T08:14:40.7153351Z +    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7154175Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7154854Z +    |
2020-03-27T08:14:40.7154854Z +    |
2020-03-27T08:14:40.7154994Z + LL | trait Quux {}
2020-03-27T08:14:40.7155359Z + 
2020-03-27T08:14:40.7155560Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7156189Z 2   --> $DIR/infinite-impl-trait-issue-38064.rs:8:13
2020-03-27T08:14:40.7156404Z 3    |
2020-03-27T08:14:40.7156404Z 3    |
2020-03-27T08:14:40.7156496Z 
2020-03-27T08:14:40.7156781Z 14    |
2020-03-27T08:14:40.7157008Z 15    = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`
2020-03-27T08:14:40.7157570Z - error: aborting due to 2 previous errors
2020-03-27T08:14:40.7157807Z + error: aborting due to 3 previous errors
2020-03-27T08:14:40.7157993Z 18 
2020-03-27T08:14:40.7158378Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T08:14:40.7158378Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T08:14:40.7158688Z + Some errors have detailed explanations: E0391, E0720.
2020-03-27T08:14:40.7159185Z + For more information about an error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7159391Z 20 
2020-03-27T08:14:40.7159480Z 
2020-03-27T08:14:40.7159564Z 
2020-03-27T08:14:40.7159763Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.7160451Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
2020-03-27T08:14:40.7161069Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.7161822Z To only update this specific test, also pass `--test-args impl-trait/issues/infinite-impl-trait-issue-38064.rs`
2020-03-27T08:14:40.7162403Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.7162624Z status: exit code: 1
2020-03-27T08:14:40.7162624Z status: exit code: 1
2020-03-27T08:14:40.7164892Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064/auxiliary"
2020-03-27T08:14:40.7166784Z ------------------------------------------
2020-03-27T08:14:40.7166927Z 
2020-03-27T08:14:40.7167239Z ------------------------------------------
2020-03-27T08:14:40.7167404Z stderr:
2020-03-27T08:14:40.7167404Z stderr:
2020-03-27T08:14:40.7167705Z ------------------------------------------
2020-03-27T08:14:40.7167988Z error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
2020-03-27T08:14:40.7168901Z    |
2020-03-27T08:14:40.7168901Z    |
2020-03-27T08:14:40.7169275Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7169641Z    |
2020-03-27T08:14:40.7169968Z note: ...which requires borrow-checking `foo`...
2020-03-27T08:14:40.7170429Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T08:14:40.7170656Z    |
2020-03-27T08:14:40.7170656Z    |
2020-03-27T08:14:40.7171031Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7171447Z note: ...which requires processing `foo`...
2020-03-27T08:14:40.7171918Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T08:14:40.7172147Z    |
2020-03-27T08:14:40.7172147Z    |
2020-03-27T08:14:40.7172500Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7173010Z note: ...which requires const checking `foo`...
2020-03-27T08:14:40.7173491Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:1
2020-03-27T08:14:40.7173734Z    |
2020-03-27T08:14:40.7173734Z    |
2020-03-27T08:14:40.7174145Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7174380Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7174628Z    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T08:14:40.7175001Z    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T08:14:40.7175364Z note: ...which requires processing `bar::{{opaque}}#0`...
2020-03-27T08:14:40.7176130Z    |
2020-03-27T08:14:40.7176130Z    |
2020-03-27T08:14:40.7176483Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7176733Z    |             ^^^^^^^^^
2020-03-27T08:14:40.7177075Z note: ...which requires borrow-checking `bar`...
2020-03-27T08:14:40.7177976Z    |
2020-03-27T08:14:40.7177976Z    |
2020-03-27T08:14:40.7178348Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7178795Z note: ...which requires processing `bar`...
2020-03-27T08:14:40.7179776Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:1
2020-03-27T08:14:40.7180019Z    |
2020-03-27T08:14:40.7180019Z    |
2020-03-27T08:14:40.7180865Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7181337Z note: ...which requires const checking `bar`...
2020-03-27T08:14:40.7181909Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:1
2020-03-27T08:14:40.7182169Z    |
2020-03-27T08:14:40.7182169Z    |
2020-03-27T08:14:40.7182737Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7183141Z    | ^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7183540Z    = note: ...which requires computing whether `impl Quux` is freeze...
2020-03-27T08:14:40.7183906Z    = note: ...which requires evaluating trait selection obligation `impl Quux: std::marker::Freeze`...
2020-03-27T08:14:40.7184321Z    = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7184784Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7185488Z    |
2020-03-27T08:14:40.7185488Z    |
2020-03-27T08:14:40.7185607Z LL | trait Quux {}
2020-03-27T08:14:40.7185836Z 
2020-03-27T08:14:40.7186018Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7186480Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:8:13
2020-03-27T08:14:40.7186706Z    |
2020-03-27T08:14:40.7186706Z    |
2020-03-27T08:14:40.7187256Z LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7187697Z    |
2020-03-27T08:14:40.7187697Z    |
2020-03-27T08:14:40.7187910Z    = note: expanded type is `foo::Foo<bar::Bar<impl Quux>>`
2020-03-27T08:14:40.7188255Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7188917Z   --> /checkout/src/test/ui/impl-trait/issues/infinite-impl-trait-issue-38064.rs:14:13
2020-03-27T08:14:40.7189353Z    |
2020-03-27T08:14:40.7189353Z    |
2020-03-27T08:14:40.7189732Z LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
2020-03-27T08:14:40.7190358Z    |
2020-03-27T08:14:40.7190358Z    |
2020-03-27T08:14:40.7190560Z    = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`
2020-03-27T08:14:40.7191010Z error: aborting due to 3 previous errors
2020-03-27T08:14:40.7191151Z 
2020-03-27T08:14:40.7191335Z Some errors have detailed explanations: E0391, E0720.
2020-03-27T08:14:40.7191804Z For more information about an error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7191804Z For more information about an error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7192058Z 
2020-03-27T08:14:40.7192535Z ------------------------------------------
2020-03-27T08:14:40.7192674Z 
2020-03-27T08:14:40.7192910Z 
2020-03-27T08:14:40.7193260Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-direct.rs stdout ----
2020-03-27T08:14:40.7193466Z diff of stderr:
2020-03-27T08:14:40.7193561Z 
2020-03-27T08:14:40.7193760Z + error[E0391]: cycle detected when processing `test::{{opaque}}#0`
2020-03-27T08:14:40.7194394Z +    |
2020-03-27T08:14:40.7194663Z + LL | fn test() -> impl Sized {
2020-03-27T08:14:40.7194854Z +    |              ^^^^^^^^^^
2020-03-27T08:14:40.7194990Z +    |
---
2020-03-27T08:14:40.7198172Z +   --> $DIR/recursive-impl-trait-type-direct.rs:5:1
2020-03-27T08:14:40.7198353Z +    |
2020-03-27T08:14:40.7198620Z + LL | fn test() -> impl Sized {
2020-03-27T08:14:40.7198811Z +    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7199047Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7199421Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7200235Z +    = note: ...which again requires processing `test::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7200735Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7201529Z +    |
2020-03-27T08:14:40.7201529Z +    |
2020-03-27T08:14:40.7201690Z + LL | / #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7201851Z + LL | |
2020-03-27T08:14:40.7202154Z + LL | | fn test() -> impl Sized {
2020-03-27T08:14:40.7202307Z + LL | |
2020-03-27T08:14:40.7202417Z + ...  |
2020-03-27T08:14:40.7202545Z + LL | |
2020-03-27T08:14:40.7202676Z + LL | | fn main() {}
2020-03-27T08:14:40.7202966Z + 
2020-03-27T08:14:40.7203144Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7203712Z 2   --> $DIR/recursive-impl-trait-type-direct.rs:5:14
2020-03-27T08:14:40.7203893Z 3    |
---
2020-03-27T08:14:40.7206309Z 
2020-03-27T08:14:40.7206381Z 
2020-03-27T08:14:40.7206535Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.7207127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-direct/recursive-impl-trait-type-direct.stderr
2020-03-27T08:14:40.7207739Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.7208241Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-direct.rs`
2020-03-27T08:14:40.7208831Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.7209420Z status: exit code: 1
2020-03-27T08:14:40.7209420Z status: exit code: 1
2020-03-27T08:14:40.7211839Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-direct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-direct/auxiliary"
2020-03-27T08:14:40.7214127Z ------------------------------------------
2020-03-27T08:14:40.7214280Z 
2020-03-27T08:14:40.7214616Z ------------------------------------------
2020-03-27T08:14:40.7214954Z stderr:
2020-03-27T08:14:40.7214954Z stderr:
2020-03-27T08:14:40.7215272Z ------------------------------------------
2020-03-27T08:14:40.7215722Z error[E0391]: cycle detected when processing `test::{{opaque}}#0`
2020-03-27T08:14:40.7216463Z    |
2020-03-27T08:14:40.7216909Z LL | fn test() -> impl Sized {
2020-03-27T08:14:40.7217085Z    |              ^^^^^^^^^^
2020-03-27T08:14:40.7217219Z    |
---
2020-03-27T08:14:40.7221301Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-direct.rs:5:1
2020-03-27T08:14:40.7221527Z    |
2020-03-27T08:14:40.7221819Z LL | fn test() -> impl Sized {
2020-03-27T08:14:40.7221997Z    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7222241Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7223257Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7223898Z    = note: ...which again requires processing `test::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7224638Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7225388Z    |
2020-03-27T08:14:40.7225552Z LL | / #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7225733Z LL | |
2020-03-27T08:14:40.7226178Z LL | | fn test() -> impl Sized {
2020-03-27T08:14:40.7226178Z LL | | fn test() -> impl Sized {
2020-03-27T08:14:40.7226673Z LL | |     //~^ ERROR E0720
2020-03-27T08:14:40.7226925Z LL | |
2020-03-27T08:14:40.7227047Z LL | | fn main() {}
2020-03-27T08:14:40.7227186Z    | |____________^
2020-03-27T08:14:40.7227304Z 
---
2020-03-27T08:14:40.7230832Z 
2020-03-27T08:14:40.7231172Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-indirect.rs stdout ----
2020-03-27T08:14:40.7231566Z diff of stderr:
2020-03-27T08:14:40.7231829Z 
2020-03-27T08:14:40.7232032Z + error[E0391]: cycle detected when processing `option::{{opaque}}#0`
2020-03-27T08:14:40.7232679Z +    |
2020-03-27T08:14:40.7232679Z +    |
2020-03-27T08:14:40.7232977Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7233344Z +    |
2020-03-27T08:14:40.7233830Z + note: ...which requires borrow-checking `option`...
2020-03-27T08:14:40.7234673Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T08:14:40.7234892Z +    |
2020-03-27T08:14:40.7234892Z +    |
2020-03-27T08:14:40.7235213Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7235680Z + note: ...which requires processing `option`...
2020-03-27T08:14:40.7236104Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T08:14:40.7236305Z +    |
2020-03-27T08:14:40.7236305Z +    |
2020-03-27T08:14:40.7236819Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7237445Z + note: ...which requires const checking `option`...
2020-03-27T08:14:40.7238263Z +   --> $DIR/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T08:14:40.7238464Z +    |
2020-03-27T08:14:40.7238464Z +    |
2020-03-27T08:14:40.7238776Z + LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7239271Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7239271Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7239666Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7240119Z +    = note: ...which again requires processing `option::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7240939Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7241552Z +    |
2020-03-27T08:14:40.7241552Z +    |
2020-03-27T08:14:40.7241865Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7242061Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7242239Z + LL | |
2020-03-27T08:14:40.7242776Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7243146Z + ...  |
2020-03-27T08:14:40.7243494Z + LL | |
2020-03-27T08:14:40.7243655Z + LL | | fn main() {}
2020-03-27T08:14:40.7244129Z + 
2020-03-27T08:14:40.7244524Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7245007Z 2   --> $DIR/recursive-impl-trait-type-indirect.rs:7:22
2020-03-27T08:14:40.7245437Z 3    |
2020-03-27T08:14:40.7245437Z 3    |
2020-03-27T08:14:40.7245737Z 
2020-03-27T08:14:40.7246022Z 6    |
2020-03-27T08:14:40.7246643Z 7    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-03-27T08:14:40.7246878Z 8 
2020-03-27T08:14:40.7247134Z + error[E0391]: cycle detected when processing `tuple::{{opaque}}#0`
2020-03-27T08:14:40.7248173Z +    |
2020-03-27T08:14:40.7248482Z + LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7248829Z +    |               ^^^^^^^^^^
2020-03-27T08:14:40.7249129Z +    |
2020-03-27T08:14:40.7249129Z +    |
2020-03-27T08:14:40.7249729Z + note: ...which requires borrow-checking `tuple`...
2020-03-27T08:14:40.7250728Z +    |
2020-03-27T08:14:40.7251282Z + LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7251547Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7251547Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7251961Z + note: ...which requires processing `tuple`...
2020-03-27T08:14:40.7252906Z +    |
2020-03-27T08:14:40.7253217Z + LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7253417Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7253417Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7253673Z + note: ...which requires const checking `tuple`...
2020-03-27T08:14:40.7255077Z +    |
2020-03-27T08:14:40.7255407Z + LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7255614Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7255887Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7255887Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7256343Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7257196Z +    = note: ...which again requires processing `tuple::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7258104Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7259172Z +    |
2020-03-27T08:14:40.7259172Z +    |
2020-03-27T08:14:40.7259335Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7259574Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7259757Z + LL | |
2020-03-27T08:14:40.7260414Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7260629Z + ...  |
2020-03-27T08:14:40.7260753Z + LL | |
2020-03-27T08:14:40.7260895Z + LL | | fn main() {}
2020-03-27T08:14:40.7261207Z + 
2020-03-27T08:14:40.7261401Z 9 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7262245Z 10   --> $DIR/recursive-impl-trait-type-indirect.rs:12:15
2020-03-27T08:14:40.7262451Z 11    |
2020-03-27T08:14:40.7262451Z 11    |
2020-03-27T08:14:40.7262542Z 
2020-03-27T08:14:40.7262643Z 14    |
2020-03-27T08:14:40.7262879Z 15    = note: expanded type is `(impl Sized,)`
2020-03-27T08:14:40.7263048Z 16 
2020-03-27T08:14:40.7263267Z + error[E0391]: cycle detected when processing `array::{{opaque}}#0`
2020-03-27T08:14:40.7264097Z +    |
2020-03-27T08:14:40.7264378Z + LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7264561Z +    |               ^^^^^^^^^^
2020-03-27T08:14:40.7264720Z +    |
2020-03-27T08:14:40.7264720Z +    |
2020-03-27T08:14:40.7265040Z + note: ...which requires borrow-checking `array`...
2020-03-27T08:14:40.7265847Z +    |
2020-03-27T08:14:40.7266133Z + LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7266321Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7266321Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7266706Z + note: ...which requires processing `array`...
2020-03-27T08:14:40.7267485Z +    |
2020-03-27T08:14:40.7267952Z + LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7268399Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7268399Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7268774Z + note: ...which requires const checking `array`...
2020-03-27T08:14:40.7269432Z +    |
2020-03-27T08:14:40.7276034Z + LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7276256Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7276673Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7276673Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7277067Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7277518Z +    = note: ...which again requires processing `array::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7278164Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7278813Z +    |
2020-03-27T08:14:40.7278813Z +    |
2020-03-27T08:14:40.7278962Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7279228Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7279416Z + LL | |
2020-03-27T08:14:40.7279760Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7279937Z + ...  |
2020-03-27T08:14:40.7280053Z + LL | |
2020-03-27T08:14:40.7280206Z + LL | | fn main() {}
2020-03-27T08:14:40.7280484Z + 
2020-03-27T08:14:40.7280687Z 17 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7281527Z 18   --> $DIR/recursive-impl-trait-type-indirect.rs:17:15
2020-03-27T08:14:40.7281895Z 19    |
2020-03-27T08:14:40.7281895Z 19    |
2020-03-27T08:14:40.7281989Z 
2020-03-27T08:14:40.7282120Z 22    |
2020-03-27T08:14:40.7282305Z 23    = note: expanded type is `[impl Sized; 1]`
2020-03-27T08:14:40.7282647Z 24 
2020-03-27T08:14:40.7282878Z + error[E0391]: cycle detected when processing `ptr::{{opaque}}#0`
2020-03-27T08:14:40.7283544Z +    |
2020-03-27T08:14:40.7283846Z + LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7284030Z +    |             ^^^^^^^^^^
2020-03-27T08:14:40.7284173Z +    |
2020-03-27T08:14:40.7284173Z +    |
2020-03-27T08:14:40.7284517Z + note: ...which requires borrow-checking `ptr`...
2020-03-27T08:14:40.7285288Z +    |
2020-03-27T08:14:40.7285578Z + LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7285755Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7285755Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7285951Z + note: ...which requires processing `ptr`...
2020-03-27T08:14:40.7286558Z +    |
2020-03-27T08:14:40.7286829Z + LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7287006Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7287006Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7287228Z + note: ...which requires const checking `ptr`...
2020-03-27T08:14:40.7287824Z +    |
2020-03-27T08:14:40.7288112Z + LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7288293Z +    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7288537Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7288537Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7288935Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7289354Z +    = note: ...which again requires processing `ptr::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7289835Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7290944Z +    |
2020-03-27T08:14:40.7290944Z +    |
2020-03-27T08:14:40.7291098Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7291492Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7291673Z + LL | |
2020-03-27T08:14:40.7292191Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7292561Z + ...  |
2020-03-27T08:14:40.7292863Z + LL | |
2020-03-27T08:14:40.7293012Z + LL | | fn main() {}
2020-03-27T08:14:40.7293339Z + 
2020-03-27T08:14:40.7293542Z 25 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7294180Z 26   --> $DIR/recursive-impl-trait-type-indirect.rs:22:13
2020-03-27T08:14:40.7294412Z 27    |
2020-03-27T08:14:40.7294412Z 27    |
2020-03-27T08:14:40.7294666Z 
2020-03-27T08:14:40.7294771Z 38    |
2020-03-27T08:14:40.7295288Z 39    = note: expanded type is `fn() -> impl Sized`
2020-03-27T08:14:40.7295467Z 40 
2020-03-27T08:14:40.7295703Z + error[E0391]: cycle detected when processing `closure_capture::{{opaque}}#0`
2020-03-27T08:14:40.7296399Z +    |
2020-03-27T08:14:40.7296705Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7296992Z +    |                         ^^^^^^^^^^
2020-03-27T08:14:40.7297170Z +    |
2020-03-27T08:14:40.7297170Z +    |
2020-03-27T08:14:40.7297542Z + note: ...which requires borrow-checking `closure_capture`...
2020-03-27T08:14:40.7298249Z +    |
2020-03-27T08:14:40.7298581Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7298799Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7298799Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7299053Z + note: ...which requires processing `closure_capture`...
2020-03-27T08:14:40.7300242Z +    |
2020-03-27T08:14:40.7301486Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7301925Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7301925Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7302388Z + note: ...which requires const checking `closure_capture`...
2020-03-27T08:14:40.7303408Z +    |
2020-03-27T08:14:40.7303730Z + LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7304142Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7304622Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7304622Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7305433Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7306101Z +    = note: ...which again requires processing `closure_capture::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7306623Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7307271Z +    |
2020-03-27T08:14:40.7307271Z +    |
2020-03-27T08:14:40.7307420Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7307621Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7307804Z + LL | |
2020-03-27T08:14:40.7308124Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7308300Z + ...  |
2020-03-27T08:14:40.7308420Z + LL | |
2020-03-27T08:14:40.7308573Z + LL | | fn main() {}
2020-03-27T08:14:40.7308852Z + 
2020-03-27T08:14:40.7309053Z 41 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7309499Z 42   --> $DIR/recursive-impl-trait-type-indirect.rs:32:25
2020-03-27T08:14:40.7309940Z 43    |
2020-03-27T08:14:40.7309940Z 43    |
2020-03-27T08:14:40.7310049Z 
2020-03-27T08:14:40.7310156Z 46    |
2020-03-27T08:14:40.7310656Z 47    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:35:5: 37:6 x:impl Sized]`
2020-03-27T08:14:40.7310943Z 48 
2020-03-27T08:14:40.7311384Z + error[E0391]: cycle detected when processing `closure_ref_capture::{{opaque}}#0`
2020-03-27T08:14:40.7312830Z +    |
2020-03-27T08:14:40.7313362Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7313605Z +    |                             ^^^^^^^^^^
2020-03-27T08:14:40.7313785Z +    |
2020-03-27T08:14:40.7313785Z +    |
2020-03-27T08:14:40.7314185Z + note: ...which requires borrow-checking `closure_ref_capture`...
2020-03-27T08:14:40.7314905Z +    |
2020-03-27T08:14:40.7315272Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7315515Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7315515Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7315779Z + note: ...which requires processing `closure_ref_capture`...
2020-03-27T08:14:40.7316610Z +    |
2020-03-27T08:14:40.7316936Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7317186Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7317186Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7317448Z + note: ...which requires const checking `closure_ref_capture`...
2020-03-27T08:14:40.7318118Z +    |
2020-03-27T08:14:40.7318443Z + LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7318768Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7319061Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7319061Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7319512Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7320356Z +    = note: ...which again requires processing `closure_ref_capture::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7320974Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7321823Z +    |
2020-03-27T08:14:40.7321823Z +    |
2020-03-27T08:14:40.7321998Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7322212Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7322569Z + LL | |
2020-03-27T08:14:40.7322942Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7323295Z + ...  |
2020-03-27T08:14:40.7323416Z + LL | |
2020-03-27T08:14:40.7323745Z + LL | | fn main() {}
2020-03-27T08:14:40.7324247Z + 
2020-03-27T08:14:40.7324850Z 49 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7325580Z 50   --> $DIR/recursive-impl-trait-type-indirect.rs:40:29
2020-03-27T08:14:40.7325842Z 51    |
2020-03-27T08:14:40.7325842Z 51    |
2020-03-27T08:14:40.7325957Z 
2020-03-27T08:14:40.7326243Z 54    |
2020-03-27T08:14:40.7327440Z 55    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:43:5: 45:6 x:impl Sized]`
2020-03-27T08:14:40.7328107Z 56 
2020-03-27T08:14:40.7328922Z + error[E0391]: cycle detected when processing `closure_sig::{{opaque}}#0`
2020-03-27T08:14:40.7329836Z +    |
2020-03-27T08:14:40.7330153Z + LL | fn closure_sig() -> impl Sized {
2020-03-27T08:14:40.7330389Z +    |                     ^^^^^^^^^^
2020-03-27T08:14:40.7330553Z +    |
2020-03-27T08:14:40.7330553Z +    |
2020-03-27T08:14:40.7330916Z + note: ...which requires borrow-checking `closure_sig`...
2020-03-27T08:14:40.7331604Z +    |
2020-03-27T08:14:40.7331920Z + LL | fn closure_sig() -> impl Sized {
2020-03-27T08:14:40.7332155Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7332155Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7332627Z + note: ...which requires borrow-checking `closure_sig::{{closure}}#0`...
2020-03-27T08:14:40.7333353Z +    |
2020-03-27T08:14:40.7333505Z + LL |     || closure_sig()
2020-03-27T08:14:40.7333687Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7333687Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7333971Z + note: ...which requires processing `closure_sig::{{closure}}#0`...
2020-03-27T08:14:40.7334672Z +    |
2020-03-27T08:14:40.7334822Z + LL |     || closure_sig()
2020-03-27T08:14:40.7335020Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7335020Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7335295Z + note: ...which requires const checking `closure_sig::{{closure}}#0`...
2020-03-27T08:14:40.7336231Z +    |
2020-03-27T08:14:40.7336385Z + LL |     || closure_sig()
2020-03-27T08:14:40.7336748Z +    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7337049Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7337049Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7337651Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7338294Z +    = note: ...which again requires processing `closure_sig::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7338857Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7339526Z +    |
2020-03-27T08:14:40.7339526Z +    |
2020-03-27T08:14:40.7339701Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7339916Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7340092Z + LL | |
2020-03-27T08:14:40.7340743Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7340935Z + ...  |
2020-03-27T08:14:40.7341057Z + LL | |
2020-03-27T08:14:40.7341217Z + LL | | fn main() {}
2020-03-27T08:14:40.7341514Z + 
2020-03-27T08:14:40.7341812Z 57 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7342313Z 58   --> $DIR/recursive-impl-trait-type-indirect.rs:48:21
2020-03-27T08:14:40.7342528Z 59    |
2020-03-27T08:14:40.7342528Z 59    |
2020-03-27T08:14:40.7342625Z 
2020-03-27T08:14:40.7342751Z 62    |
2020-03-27T08:14:40.7343241Z 63    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:50:5: 50:21]`
2020-03-27T08:14:40.7343741Z 64 
2020-03-27T08:14:40.7343999Z + error[E0391]: cycle detected when processing `generator_sig::{{opaque}}#0`
2020-03-27T08:14:40.7344699Z +    |
2020-03-27T08:14:40.7345027Z + LL | fn generator_sig() -> impl Sized {
2020-03-27T08:14:40.7345253Z +    |                       ^^^^^^^^^^
2020-03-27T08:14:40.7345414Z +    |
2020-03-27T08:14:40.7345414Z +    |
2020-03-27T08:14:40.7345977Z + note: ...which requires borrow-checking `generator_sig`...
2020-03-27T08:14:40.7346801Z +    |
2020-03-27T08:14:40.7347698Z + LL | fn generator_sig() -> impl Sized {
2020-03-27T08:14:40.7347931Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7347931Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7348391Z + note: ...which requires borrow-checking `generator_sig::{{closure}}#0`...
2020-03-27T08:14:40.7349097Z +    |
2020-03-27T08:14:40.7349247Z + LL |     || generator_sig()
2020-03-27T08:14:40.7349429Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7349429Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7349714Z + note: ...which requires processing `generator_sig::{{closure}}#0`...
2020-03-27T08:14:40.7350396Z +    |
2020-03-27T08:14:40.7350564Z + LL |     || generator_sig()
2020-03-27T08:14:40.7350746Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7350746Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7351020Z + note: ...which requires const checking `generator_sig::{{closure}}#0`...
2020-03-27T08:14:40.7351955Z +    |
2020-03-27T08:14:40.7352263Z + LL |     || generator_sig()
2020-03-27T08:14:40.7352461Z +    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7352715Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7352715Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7353304Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7353798Z +    = note: ...which again requires processing `generator_sig::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7354349Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7355033Z +    |
2020-03-27T08:14:40.7355033Z +    |
2020-03-27T08:14:40.7355197Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7355412Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7355606Z + LL | |
2020-03-27T08:14:40.7355950Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7356138Z + ...  |
2020-03-27T08:14:40.7356264Z + LL | |
2020-03-27T08:14:40.7356424Z + LL | | fn main() {}
2020-03-27T08:14:40.7356720Z + 
2020-03-27T08:14:40.7356937Z 65 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7357408Z 66   --> $DIR/recursive-impl-trait-type-indirect.rs:53:23
2020-03-27T08:14:40.7357624Z 67    |
2020-03-27T08:14:40.7357624Z 67    |
2020-03-27T08:14:40.7357736Z 
2020-03-27T08:14:40.7357846Z 70    |
2020-03-27T08:14:40.7358332Z 71    = note: expanded type is `[closure@$DIR/recursive-impl-trait-type-indirect.rs:55:5: 55:23]`
2020-03-27T08:14:40.7358789Z 72 
2020-03-27T08:14:40.7359342Z + error[E0391]: cycle detected when processing `generator_capture::{{opaque}}#0`
2020-03-27T08:14:40.7360389Z +    |
2020-03-27T08:14:40.7360762Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7361325Z +    |                           ^^^^^^^^^^
2020-03-27T08:14:40.7361498Z +    |
2020-03-27T08:14:40.7361498Z +    |
2020-03-27T08:14:40.7361951Z + note: ...which requires borrow-checking `generator_capture`...
2020-03-27T08:14:40.7362651Z +    |
2020-03-27T08:14:40.7362999Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7363234Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7363234Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7363492Z + note: ...which requires processing `generator_capture`...
2020-03-27T08:14:40.7364175Z +    |
2020-03-27T08:14:40.7364507Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7364758Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7364758Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7365030Z + note: ...which requires const checking `generator_capture`...
2020-03-27T08:14:40.7366293Z +    |
2020-03-27T08:14:40.7366810Z + LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7367223Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7367717Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7367717Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7368165Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7368836Z +    = note: ...which again requires processing `generator_capture::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7369578Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7370250Z +    |
2020-03-27T08:14:40.7370250Z +    |
2020-03-27T08:14:40.7370425Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7370640Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7370820Z + LL | |
2020-03-27T08:14:40.7371164Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7371369Z + ...  |
2020-03-27T08:14:40.7371492Z + LL | |
2020-03-27T08:14:40.7371637Z + LL | | fn main() {}
2020-03-27T08:14:40.7371956Z + 
2020-03-27T08:14:40.7372150Z 73 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7372634Z 74   --> $DIR/recursive-impl-trait-type-indirect.rs:58:27
2020-03-27T08:14:40.7372849Z 75    |
2020-03-27T08:14:40.7372849Z 75    |
2020-03-27T08:14:40.7372946Z 
2020-03-27T08:14:40.7373054Z 78    |
2020-03-27T08:14:40.7373607Z 79    = note: expanded type is `[generator@$DIR/recursive-impl-trait-type-indirect.rs:61:5: 64:6 x:impl Sized {()}]`
2020-03-27T08:14:40.7373914Z 80 
2020-03-27T08:14:40.7374346Z + error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T08:14:40.7375290Z +    |
2020-03-27T08:14:40.7375290Z +    |
2020-03-27T08:14:40.7376280Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7376939Z +    |
2020-03-27T08:14:40.7376939Z +    |
2020-03-27T08:14:40.7377327Z + note: ...which requires borrow-checking `substs_change`...
2020-03-27T08:14:40.7378036Z +    |
2020-03-27T08:14:40.7378036Z +    |
2020-03-27T08:14:40.7378407Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7378708Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7378981Z + note: ...which requires processing `substs_change`...
2020-03-27T08:14:40.7379679Z +    |
2020-03-27T08:14:40.7379679Z +    |
2020-03-27T08:14:40.7380050Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7380538Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7380844Z + note: ...which requires const checking `substs_change`...
2020-03-27T08:14:40.7381932Z +    |
2020-03-27T08:14:40.7381932Z +    |
2020-03-27T08:14:40.7382493Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7383327Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7383327Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7383989Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7384485Z +    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7385073Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7385782Z +    |
2020-03-27T08:14:40.7385782Z +    |
2020-03-27T08:14:40.7385945Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7386184Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7386372Z + LL | |
2020-03-27T08:14:40.7386725Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7386920Z + ...  |
2020-03-27T08:14:40.7387064Z + LL | |
2020-03-27T08:14:40.7387213Z + LL | | fn main() {}
2020-03-27T08:14:40.7387541Z + 
2020-03-27T08:14:40.7387541Z + 
2020-03-27T08:14:40.7387796Z + error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T08:14:40.7388563Z +    |
2020-03-27T08:14:40.7388563Z +    |
2020-03-27T08:14:40.7388936Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7389424Z +    |
2020-03-27T08:14:40.7389424Z +    |
2020-03-27T08:14:40.7389806Z + note: ...which requires borrow-checking `substs_change`...
2020-03-27T08:14:40.7390513Z +    |
2020-03-27T08:14:40.7390513Z +    |
2020-03-27T08:14:40.7390886Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7391169Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7391460Z + note: ...which requires processing `substs_change`...
2020-03-27T08:14:40.7392937Z +    |
2020-03-27T08:14:40.7392937Z +    |
2020-03-27T08:14:40.7393489Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7393796Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7394084Z + note: ...which requires const checking `substs_change`...
2020-03-27T08:14:40.7394813Z +    |
2020-03-27T08:14:40.7394813Z +    |
2020-03-27T08:14:40.7395198Z + LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7395487Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7395826Z +    = note: ...which requires computing whether `(impl Sized,)` is freeze...
2020-03-27T08:14:40.7396287Z +    = note: ...which requires evaluating trait selection obligation `(impl Sized,): std::marker::Freeze`...
2020-03-27T08:14:40.7396808Z +    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7397409Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7398146Z +    |
2020-03-27T08:14:40.7398146Z +    |
2020-03-27T08:14:40.7398317Z + LL | / #![feature(generators)]
2020-03-27T08:14:40.7398547Z + LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7398736Z + LL | |
2020-03-27T08:14:40.7399121Z + LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7399322Z + ...  |
2020-03-27T08:14:40.7399453Z + LL | |
2020-03-27T08:14:40.7399623Z + LL | | fn main() {}
2020-03-27T08:14:40.7399939Z + 
2020-03-27T08:14:40.7400166Z 81 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7400664Z 82   --> $DIR/recursive-impl-trait-type-indirect.rs:67:35
2020-03-27T08:14:40.7400965Z 83    |
2020-03-27T08:14:40.7400965Z 83    |
2020-03-27T08:14:40.7401069Z 
2020-03-27T08:14:40.7401203Z 86    |
2020-03-27T08:14:40.7401403Z 87    = note: expanded type is `(impl Sized,)`
2020-03-27T08:14:40.7401593Z 88 
2020-03-27T08:14:40.7401919Z + error[E0391]: cycle detected when processing `generator_hold::{{opaque}}#0`
2020-03-27T08:14:40.7402726Z +    |
2020-03-27T08:14:40.7402726Z +    |
2020-03-27T08:14:40.7403090Z + LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7403514Z +    |
2020-03-27T08:14:40.7403514Z +    |
2020-03-27T08:14:40.7404114Z + note: ...which requires borrow-checking `generator_hold`...
2020-03-27T08:14:40.7404859Z +    |
2020-03-27T08:14:40.7404859Z +    |
2020-03-27T08:14:40.7405216Z + LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7405485Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7406216Z + note: ...which requires processing `generator_hold`...
2020-03-27T08:14:40.7407183Z +    |
2020-03-27T08:14:40.7407183Z +    |
2020-03-27T08:14:40.7407531Z + LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7407934Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7408397Z + note: ...which requires const checking `generator_hold`...
2020-03-27T08:14:40.7409252Z +    |
2020-03-27T08:14:40.7409252Z +    |
2020-03-27T08:14:40.7409603Z + LL | fn generator_hold() -> impl Sized {
---
2020-03-27T08:14:40.7449707Z 
2020-03-27T08:14:40.7449810Z 
2020-03-27T08:14:40.7449996Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.7450693Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/recursive-impl-trait-type-indirect.stderr
2020-03-27T08:14:40.7451343Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.7451922Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-indirect.rs`
2020-03-27T08:14:40.7452353Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.7452582Z status: exit code: 1
2020-03-27T08:14:40.7452582Z status: exit code: 1
2020-03-27T08:14:40.7454481Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-indirect/auxiliary"
2020-03-27T08:14:40.7456036Z ------------------------------------------
2020-03-27T08:14:40.7456213Z 
2020-03-27T08:14:40.7456541Z ------------------------------------------
2020-03-27T08:14:40.7456723Z stderr:
2020-03-27T08:14:40.7456723Z stderr:
2020-03-27T08:14:40.7457069Z ------------------------------------------
2020-03-27T08:14:40.7457375Z error[E0391]: cycle detected when processing `option::{{opaque}}#0`
2020-03-27T08:14:40.7458222Z    |
2020-03-27T08:14:40.7458222Z    |
2020-03-27T08:14:40.7458556Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7458953Z    |
2020-03-27T08:14:40.7459325Z note: ...which requires borrow-checking `option`...
2020-03-27T08:14:40.7459844Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T08:14:40.7460098Z    |
2020-03-27T08:14:40.7460098Z    |
2020-03-27T08:14:40.7460589Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7461151Z note: ...which requires processing `option`...
2020-03-27T08:14:40.7461707Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T08:14:40.7461961Z    |
2020-03-27T08:14:40.7461961Z    |
2020-03-27T08:14:40.7462290Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7462846Z note: ...which requires const checking `option`...
2020-03-27T08:14:40.7463383Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:1
2020-03-27T08:14:40.7463653Z    |
2020-03-27T08:14:40.7463653Z    |
2020-03-27T08:14:40.7464139Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7464646Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7464646Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7465059Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7465519Z    = note: ...which again requires processing `option::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7466071Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7466833Z    |
2020-03-27T08:14:40.7467009Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7467218Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7467388Z LL | |
2020-03-27T08:14:40.7467388Z LL | |
2020-03-27T08:14:40.7467740Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7468038Z LL | |
2020-03-27T08:14:40.7468192Z LL | | fn main() {}
2020-03-27T08:14:40.7468350Z    | |____________^
2020-03-27T08:14:40.7468463Z 
2020-03-27T08:14:40.7468463Z 
2020-03-27T08:14:40.7468649Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7469178Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:7:22
2020-03-27T08:14:40.7469425Z    |
2020-03-27T08:14:40.7469743Z LL | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7470232Z    |
2020-03-27T08:14:40.7470232Z    |
2020-03-27T08:14:40.7470462Z    = note: expanded type is `std::option::Option<(impl Sized, i32)>`
2020-03-27T08:14:40.7470687Z 
2020-03-27T08:14:40.7470914Z error[E0391]: cycle detected when processing `tuple::{{opaque}}#0`
2020-03-27T08:14:40.7471718Z    |
2020-03-27T08:14:40.7472034Z LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7472229Z    |               ^^^^^^^^^^
2020-03-27T08:14:40.7472377Z    |
2020-03-27T08:14:40.7472377Z    |
2020-03-27T08:14:40.7472734Z note: ...which requires borrow-checking `tuple`...
2020-03-27T08:14:40.7473484Z    |
2020-03-27T08:14:40.7473794Z LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7473987Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7473987Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7474201Z note: ...which requires processing `tuple`...
2020-03-27T08:14:40.7474965Z    |
2020-03-27T08:14:40.7475263Z LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7475473Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7475473Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7475700Z note: ...which requires const checking `tuple`...
2020-03-27T08:14:40.7476465Z    |
2020-03-27T08:14:40.7476761Z LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7476953Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7477214Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7477214Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7477643Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7478098Z    = note: ...which again requires processing `tuple::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7478635Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7479495Z    |
2020-03-27T08:14:40.7479665Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7479928Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7480104Z LL | |
2020-03-27T08:14:40.7480104Z LL | |
2020-03-27T08:14:40.7480455Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7480771Z LL | |
2020-03-27T08:14:40.7480909Z LL | | fn main() {}
2020-03-27T08:14:40.7481084Z    | |____________^
2020-03-27T08:14:40.7481198Z 
2020-03-27T08:14:40.7481198Z 
2020-03-27T08:14:40.7481386Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7481923Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:12:15
2020-03-27T08:14:40.7482172Z    |
2020-03-27T08:14:40.7482468Z LL | fn tuple() -> impl Sized {
2020-03-27T08:14:40.7482707Z    |               ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7482919Z    |
2020-03-27T08:14:40.7483096Z    = note: expanded type is `(impl Sized,)`
2020-03-27T08:14:40.7483251Z 
2020-03-27T08:14:40.7483491Z error[E0391]: cycle detected when processing `array::{{opaque}}#0`
2020-03-27T08:14:40.7484297Z    |
2020-03-27T08:14:40.7484610Z LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7484804Z    |               ^^^^^^^^^^
2020-03-27T08:14:40.7485127Z    |
2020-03-27T08:14:40.7485127Z    |
2020-03-27T08:14:40.7485460Z note: ...which requires borrow-checking `array`...
2020-03-27T08:14:40.7486204Z    |
2020-03-27T08:14:40.7486488Z LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7486694Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7486903Z note: ...which requires processing `array`...
---
2020-03-27T08:14:40.7488843Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:17:1
2020-03-27T08:14:40.7489080Z    |
2020-03-27T08:14:40.7489384Z LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7489571Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7489823Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7490238Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7490678Z    = note: ...which again requires processing `array::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7491185Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7492285Z    |
2020-03-27T08:14:40.7492433Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7492651Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7492816Z LL | |
2020-03-27T08:14:40.7492816Z LL | |
2020-03-27T08:14:40.7493143Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7493582Z LL | |
2020-03-27T08:14:40.7493715Z LL | | fn main() {}
2020-03-27T08:14:40.7493869Z    | |____________^
2020-03-27T08:14:40.7493994Z 
2020-03-27T08:14:40.7493994Z 
2020-03-27T08:14:40.7494178Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7494699Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:17:15
2020-03-27T08:14:40.7494954Z    |
2020-03-27T08:14:40.7495243Z LL | fn array() -> impl Sized {
2020-03-27T08:14:40.7495475Z    |               ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7495674Z    |
2020-03-27T08:14:40.7495848Z    = note: expanded type is `[impl Sized; 1]`
2020-03-27T08:14:40.7496074Z 
2020-03-27T08:14:40.7496287Z error[E0391]: cycle detected when processing `ptr::{{opaque}}#0`
2020-03-27T08:14:40.7497094Z    |
2020-03-27T08:14:40.7497432Z LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7497638Z    |             ^^^^^^^^^^
2020-03-27T08:14:40.7497779Z    |
2020-03-27T08:14:40.7497779Z    |
2020-03-27T08:14:40.7498125Z note: ...which requires borrow-checking `ptr`...
2020-03-27T08:14:40.7498866Z    |
2020-03-27T08:14:40.7499149Z LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7499345Z    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7499547Z note: ...which requires processing `ptr`...
---
2020-03-27T08:14:40.7501595Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:22:1
2020-03-27T08:14:40.7501834Z    |
2020-03-27T08:14:40.7502118Z LL | fn ptr() -> impl Sized {
2020-03-27T08:14:40.7502316Z    | ^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7502565Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7502964Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7503415Z    = note: ...which again requires processing `ptr::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7503918Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7504670Z    |
2020-03-27T08:14:40.7504988Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7505202Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7505391Z LL | |
2020-03-27T08:14:40.7505391Z LL | |
2020-03-27T08:14:40.7505943Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7506818Z LL | |
2020-03-27T08:14:40.7506960Z LL | | fn main() {}
2020-03-27T08:14:40.7507297Z    | |____________^
2020-03-27T08:14:40.7507594Z 
---
2020-03-27T08:14:40.7510255Z 
2020-03-27T08:14:40.7510429Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7511255Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:27:16
2020-03-27T08:14:40.7511505Z    |
2020-03-27T08:14:40.7511788Z LL | fn fn_ptr() -> impl Sized {
2020-03-27T08:14:40.7512214Z    |
2020-03-27T08:14:40.7512536Z    = note: expanded type is `fn() -> impl Sized`
2020-03-27T08:14:40.7512691Z 
2020-03-27T08:14:40.7512691Z 
2020-03-27T08:14:40.7513108Z error[E0391]: cycle detected when processing `closure_capture::{{opaque}}#0`
2020-03-27T08:14:40.7514275Z    |
2020-03-27T08:14:40.7514596Z LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7514805Z    |                         ^^^^^^^^^^
2020-03-27T08:14:40.7514958Z    |
2020-03-27T08:14:40.7514958Z    |
2020-03-27T08:14:40.7515307Z note: ...which requires borrow-checking `closure_capture`...
2020-03-27T08:14:40.7516149Z    |
2020-03-27T08:14:40.7516482Z LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7516719Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7516957Z note: ...which requires processing `closure_capture`...
---
2020-03-27T08:14:40.7519084Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:1
2020-03-27T08:14:40.7519322Z    |
2020-03-27T08:14:40.7519645Z LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7519861Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7520128Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7520549Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7521007Z    = note: ...which again requires processing `closure_capture::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7522155Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7523632Z    |
2020-03-27T08:14:40.7523796Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7524038Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7524384Z LL | |
2020-03-27T08:14:40.7524384Z LL | |
2020-03-27T08:14:40.7524740Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7525228Z LL | |
2020-03-27T08:14:40.7525553Z LL | | fn main() {}
2020-03-27T08:14:40.7525908Z    | |____________^
2020-03-27T08:14:40.7526035Z 
2020-03-27T08:14:40.7526035Z 
2020-03-27T08:14:40.7526241Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7527127Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:32:25
2020-03-27T08:14:40.7527392Z    |
2020-03-27T08:14:40.7527712Z LL | fn closure_capture() -> impl Sized {
2020-03-27T08:14:40.7527984Z    |                         ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7528204Z    |
2020-03-27T08:14:40.7528924Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:35:5: 37:6 x:impl Sized]`
2020-03-27T08:14:40.7529232Z 
2020-03-27T08:14:40.7529492Z error[E0391]: cycle detected when processing `closure_ref_capture::{{opaque}}#0`
2020-03-27T08:14:40.7530292Z    |
2020-03-27T08:14:40.7530622Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7530852Z    |                             ^^^^^^^^^^
2020-03-27T08:14:40.7531016Z    |
2020-03-27T08:14:40.7531016Z    |
2020-03-27T08:14:40.7531393Z note: ...which requires borrow-checking `closure_ref_capture`...
2020-03-27T08:14:40.7532143Z    |
2020-03-27T08:14:40.7532472Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7532704Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7532704Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7532954Z note: ...which requires processing `closure_ref_capture`...
2020-03-27T08:14:40.7533714Z    |
2020-03-27T08:14:40.7534032Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7534261Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7534261Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7534708Z note: ...which requires const checking `closure_ref_capture`...
2020-03-27T08:14:40.7535477Z    |
2020-03-27T08:14:40.7535820Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7536125Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7536406Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7536406Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7536877Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7537360Z    = note: ...which again requires processing `closure_ref_capture::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7537930Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7538714Z    |
2020-03-27T08:14:40.7538867Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7539093Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7539263Z LL | |
2020-03-27T08:14:40.7539263Z LL | |
2020-03-27T08:14:40.7539596Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7539909Z LL | |
2020-03-27T08:14:40.7540049Z LL | | fn main() {}
2020-03-27T08:14:40.7540224Z    | |____________^
2020-03-27T08:14:40.7540430Z 
2020-03-27T08:14:40.7540430Z 
2020-03-27T08:14:40.7540618Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7541167Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:40:29
2020-03-27T08:14:40.7541434Z    |
2020-03-27T08:14:40.7541767Z LL | fn closure_ref_capture() -> impl Sized {
2020-03-27T08:14:40.7542046Z    |                             ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7542275Z    |
2020-03-27T08:14:40.7542834Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:43:5: 45:6 x:impl Sized]`
2020-03-27T08:14:40.7543149Z 
2020-03-27T08:14:40.7543400Z error[E0391]: cycle detected when processing `closure_sig::{{opaque}}#0`
2020-03-27T08:14:40.7544207Z    |
2020-03-27T08:14:40.7544687Z LL | fn closure_sig() -> impl Sized {
2020-03-27T08:14:40.7544897Z    |                     ^^^^^^^^^^
2020-03-27T08:14:40.7545050Z    |
2020-03-27T08:14:40.7545050Z    |
2020-03-27T08:14:40.7545398Z note: ...which requires borrow-checking `closure_sig`...
2020-03-27T08:14:40.7546154Z    |
2020-03-27T08:14:40.7546456Z LL | fn closure_sig() -> impl Sized {
2020-03-27T08:14:40.7546676Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7546676Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7547122Z note: ...which requires borrow-checking `closure_sig::{{closure}}#0`...
2020-03-27T08:14:40.7547914Z    |
2020-03-27T08:14:40.7548054Z LL |     || closure_sig()
2020-03-27T08:14:40.7548226Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7548226Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7548492Z note: ...which requires processing `closure_sig::{{closure}}#0`...
2020-03-27T08:14:40.7549267Z    |
2020-03-27T08:14:40.7549423Z LL |     || closure_sig()
2020-03-27T08:14:40.7549593Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7549593Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7549852Z note: ...which requires const checking `closure_sig::{{closure}}#0`...
2020-03-27T08:14:40.7550650Z    |
2020-03-27T08:14:40.7550791Z LL |     || closure_sig()
2020-03-27T08:14:40.7550976Z    |     ^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7551221Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7551221Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7551619Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7552086Z    = note: ...which again requires processing `closure_sig::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7552608Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7553444Z    |
2020-03-27T08:14:40.7553592Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7553793Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7553974Z LL | |
2020-03-27T08:14:40.7553974Z LL | |
2020-03-27T08:14:40.7554375Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7554685Z LL | |
2020-03-27T08:14:40.7554818Z LL | | fn main() {}
2020-03-27T08:14:40.7554972Z    | |____________^
2020-03-27T08:14:40.7555082Z 
2020-03-27T08:14:40.7555082Z 
2020-03-27T08:14:40.7555278Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7555800Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:48:21
2020-03-27T08:14:40.7556041Z    |
2020-03-27T08:14:40.7556358Z LL | fn closure_sig() -> impl Sized {
2020-03-27T08:14:40.7556605Z    |                     ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7556799Z    |
2020-03-27T08:14:40.7557330Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:50:5: 50:21]`
2020-03-27T08:14:40.7557622Z 
2020-03-27T08:14:40.7557854Z error[E0391]: cycle detected when processing `generator_sig::{{opaque}}#0`
2020-03-27T08:14:40.7558665Z    |
2020-03-27T08:14:40.7558972Z LL | fn generator_sig() -> impl Sized {
2020-03-27T08:14:40.7559184Z    |                       ^^^^^^^^^^
2020-03-27T08:14:40.7559354Z    |
2020-03-27T08:14:40.7559354Z    |
2020-03-27T08:14:40.7559700Z note: ...which requires borrow-checking `generator_sig`...
2020-03-27T08:14:40.7560451Z    |
2020-03-27T08:14:40.7560753Z LL | fn generator_sig() -> impl Sized {
2020-03-27T08:14:40.7560964Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7560964Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7561429Z note: ...which requires borrow-checking `generator_sig::{{closure}}#0`...
2020-03-27T08:14:40.7562216Z    |
2020-03-27T08:14:40.7562374Z LL |     || generator_sig()
2020-03-27T08:14:40.7562552Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7562552Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7562816Z note: ...which requires processing `generator_sig::{{closure}}#0`...
2020-03-27T08:14:40.7563606Z    |
2020-03-27T08:14:40.7563748Z LL |     || generator_sig()
2020-03-27T08:14:40.7563938Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7563938Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7564204Z note: ...which requires const checking `generator_sig::{{closure}}#0`...
2020-03-27T08:14:40.7564996Z    |
2020-03-27T08:14:40.7565139Z LL |     || generator_sig()
2020-03-27T08:14:40.7565315Z    |     ^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7565709Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7565709Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7566116Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7566559Z    = note: ...which again requires processing `generator_sig::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7567084Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7567801Z    |
2020-03-27T08:14:40.7567944Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7568155Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7568315Z LL | |
2020-03-27T08:14:40.7568315Z LL | |
2020-03-27T08:14:40.7568629Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7568922Z LL | |
2020-03-27T08:14:40.7569051Z LL | | fn main() {}
2020-03-27T08:14:40.7569215Z    | |____________^
2020-03-27T08:14:40.7569322Z 
2020-03-27T08:14:40.7569322Z 
2020-03-27T08:14:40.7569497Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7570069Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:53:23
2020-03-27T08:14:40.7570319Z    |
2020-03-27T08:14:40.7570615Z LL | fn generator_sig() -> impl Sized {
2020-03-27T08:14:40.7570917Z    |                       ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7571131Z    |
2020-03-27T08:14:40.7571652Z    = note: expanded type is `[closure@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:55:5: 55:23]`
2020-03-27T08:14:40.7571930Z 
2020-03-27T08:14:40.7572177Z error[E0391]: cycle detected when processing `generator_capture::{{opaque}}#0`
2020-03-27T08:14:40.7572947Z    |
2020-03-27T08:14:40.7573264Z LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7573481Z    |                           ^^^^^^^^^^
2020-03-27T08:14:40.7573637Z    |
2020-03-27T08:14:40.7573637Z    |
2020-03-27T08:14:40.7574000Z note: ...which requires borrow-checking `generator_capture`...
2020-03-27T08:14:40.7574728Z    |
2020-03-27T08:14:40.7575032Z LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7575266Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7575266Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7575503Z note: ...which requires processing `generator_capture`...
2020-03-27T08:14:40.7576235Z    |
2020-03-27T08:14:40.7576540Z LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7576755Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7576755Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7577011Z note: ...which requires const checking `generator_capture`...
2020-03-27T08:14:40.7577728Z    |
2020-03-27T08:14:40.7578045Z LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7578264Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7578524Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7578524Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7578927Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7579374Z    = note: ...which again requires processing `generator_capture::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7579888Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7580710Z    |
2020-03-27T08:14:40.7580854Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7581066Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7581225Z LL | |
2020-03-27T08:14:40.7581225Z LL | |
2020-03-27T08:14:40.7581566Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7581861Z LL | |
2020-03-27T08:14:40.7581990Z LL | | fn main() {}
2020-03-27T08:14:40.7582158Z    | |____________^
2020-03-27T08:14:40.7582265Z 
2020-03-27T08:14:40.7582265Z 
2020-03-27T08:14:40.7582888Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7583812Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:58:27
2020-03-27T08:14:40.7584145Z    |
2020-03-27T08:14:40.7584541Z LL | fn generator_capture() -> impl Sized {
2020-03-27T08:14:40.7584867Z    |                           ^^^^^^^^^^ expands to a recursive type
2020-03-27T08:14:40.7585132Z    |
2020-03-27T08:14:40.7585822Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:61:5: 64:6 x:impl Sized {()}]`
2020-03-27T08:14:40.7586211Z 
2020-03-27T08:14:40.7586512Z error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T08:14:40.7587481Z    |
2020-03-27T08:14:40.7587481Z    |
2020-03-27T08:14:40.7588419Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7589466Z    |
2020-03-27T08:14:40.7589466Z    |
2020-03-27T08:14:40.7590269Z note: ...which requires borrow-checking `substs_change`...
2020-03-27T08:14:40.7591110Z    |
2020-03-27T08:14:40.7591110Z    |
2020-03-27T08:14:40.7591484Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7592358Z note: ...which requires processing `substs_change`...
2020-03-27T08:14:40.7593035Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-03-27T08:14:40.7593289Z    |
2020-03-27T08:14:40.7593289Z    |
2020-03-27T08:14:40.7593892Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7594436Z note: ...which requires const checking `substs_change`...
2020-03-27T08:14:40.7594955Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-03-27T08:14:40.7595573Z    |
2020-03-27T08:14:40.7595573Z    |
2020-03-27T08:14:40.7595942Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7596850Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7596850Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7597748Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7598219Z    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7598758Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7599880Z    |
2020-03-27T08:14:40.7600033Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7600406Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7600575Z LL | |
2020-03-27T08:14:40.7600575Z LL | |
2020-03-27T08:14:40.7601076Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7601388Z LL | |
2020-03-27T08:14:40.7601702Z LL | | fn main() {}
2020-03-27T08:14:40.7601881Z    | |____________^
2020-03-27T08:14:40.7602167Z 
2020-03-27T08:14:40.7602167Z 
2020-03-27T08:14:40.7602404Z error[E0391]: cycle detected when processing `substs_change::{{opaque}}#0`
2020-03-27T08:14:40.7603369Z    |
2020-03-27T08:14:40.7603369Z    |
2020-03-27T08:14:40.7603712Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7604307Z    |
2020-03-27T08:14:40.7604307Z    |
2020-03-27T08:14:40.7604827Z note: ...which requires borrow-checking `substs_change`...
2020-03-27T08:14:40.7606017Z    |
2020-03-27T08:14:40.7606017Z    |
2020-03-27T08:14:40.7606383Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7606940Z note: ...which requires processing `substs_change`...
2020-03-27T08:14:40.7607652Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-03-27T08:14:40.7608267Z    |
2020-03-27T08:14:40.7608267Z    |
2020-03-27T08:14:40.7608992Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7609765Z note: ...which requires const checking `substs_change`...
2020-03-27T08:14:40.7610702Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:1
2020-03-27T08:14:40.7610965Z    |
2020-03-27T08:14:40.7610965Z    |
2020-03-27T08:14:40.7611650Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7612094Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7612617Z    = note: ...which requires computing whether `(impl Sized,)` is freeze...
2020-03-27T08:14:40.7613297Z    = note: ...which requires evaluating trait selection obligation `(impl Sized,): std::marker::Freeze`...
2020-03-27T08:14:40.7614112Z    = note: ...which again requires processing `substs_change::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7615132Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7616174Z    |
2020-03-27T08:14:40.7616326Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7616533Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7616719Z LL | |
2020-03-27T08:14:40.7616719Z LL | |
2020-03-27T08:14:40.7617451Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7617746Z LL | |
2020-03-27T08:14:40.7617899Z LL | | fn main() {}
2020-03-27T08:14:40.7618056Z    | |____________^
2020-03-27T08:14:40.7618169Z 
2020-03-27T08:14:40.7618169Z 
2020-03-27T08:14:40.7618372Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7618895Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:67:35
2020-03-27T08:14:40.7619144Z    |
2020-03-27T08:14:40.7619511Z LL | fn substs_change<T: 'static>() -> impl Sized {
2020-03-27T08:14:40.7620043Z    |
2020-03-27T08:14:40.7620043Z    |
2020-03-27T08:14:40.7620234Z    = note: expanded type is `(impl Sized,)`
2020-03-27T08:14:40.7620485Z 
2020-03-27T08:14:40.7620726Z error[E0391]: cycle detected when processing `generator_hold::{{opaque}}#0`
2020-03-27T08:14:40.7621590Z    |
2020-03-27T08:14:40.7621590Z    |
2020-03-27T08:14:40.7621910Z LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7622308Z    |
2020-03-27T08:14:40.7622308Z    |
2020-03-27T08:14:40.7622671Z note: ...which requires borrow-checking `generator_hold`...
2020-03-27T08:14:40.7623457Z    |
2020-03-27T08:14:40.7623457Z    |
2020-03-27T08:14:40.7623773Z LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7623995Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7624257Z note: ...which requires processing `generator_hold`...
2020-03-27T08:14:40.7625360Z    |
2020-03-27T08:14:40.7625360Z    |
2020-03-27T08:14:40.7625844Z LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7626057Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7626299Z note: ...which requires const checking `generator_hold`...
2020-03-27T08:14:40.7627051Z    |
2020-03-27T08:14:40.7627051Z    |
2020-03-27T08:14:40.7627357Z LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7627569Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7628269Z    = note: ...which requires computing whether `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]` is freeze...
2020-03-27T08:14:40.7629265Z    = note: ...which requires evaluating trait selection obligation `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]: std::marker::Freeze`...
2020-03-27T08:14:40.7629891Z    = note: ...which again requires processing `generator_hold::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7630419Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7631170Z    |
2020-03-27T08:14:40.7631318Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7631519Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7631700Z LL | |
2020-03-27T08:14:40.7631700Z LL | |
2020-03-27T08:14:40.7632270Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7632691Z LL | |
2020-03-27T08:14:40.7632824Z LL | | fn main() {}
2020-03-27T08:14:40.7632977Z    | |____________^
2020-03-27T08:14:40.7633103Z 
2020-03-27T08:14:40.7633103Z 
2020-03-27T08:14:40.7633284Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7633880Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:72:24
2020-03-27T08:14:40.7634130Z    |
2020-03-27T08:14:40.7634473Z LL | fn generator_hold() -> impl Sized {
2020-03-27T08:14:40.7634927Z    |
2020-03-27T08:14:40.7634927Z    |
2020-03-27T08:14:40.7635501Z    = note: expanded type is `[generator@/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:74:5: 78:6 {impl Sized, ()}]`
2020-03-27T08:14:40.7635814Z 
2020-03-27T08:14:40.7636050Z error[E0391]: cycle detected when processing `mutual_recursion::{{opaque}}#0`
2020-03-27T08:14:40.7636855Z    |
2020-03-27T08:14:40.7637171Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7637404Z    |                          ^^^^^^^^^
2020-03-27T08:14:40.7637562Z    |
2020-03-27T08:14:40.7637562Z    |
2020-03-27T08:14:40.7637918Z note: ...which requires borrow-checking `mutual_recursion`...
2020-03-27T08:14:40.7638678Z    |
2020-03-27T08:14:40.7638990Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7639221Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7639221Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7639462Z note: ...which requires processing `mutual_recursion`...
2020-03-27T08:14:40.7640212Z    |
2020-03-27T08:14:40.7640519Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7640736Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7640736Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7640980Z note: ...which requires const checking `mutual_recursion`...
2020-03-27T08:14:40.7641956Z    |
2020-03-27T08:14:40.7642274Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7642688Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7642688Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7642947Z note: ...which requires processing `mutual_recursion`...
2020-03-27T08:14:40.7643752Z    |
2020-03-27T08:14:40.7644081Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7644791Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7644791Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7645740Z note: ...which requires unsafety-checking `mutual_recursion`...
2020-03-27T08:14:40.7646484Z    |
2020-03-27T08:14:40.7646810Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7647026Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7647436Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7647436Z note: ...which requires building MIR for...
2020-03-27T08:14:40.7647945Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:86:1
2020-03-27T08:14:40.7648191Z    |
2020-03-27T08:14:40.7648709Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7649395Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7650206Z note: ...which requires type-checking `mutual_recursion`...
2020-03-27T08:14:40.7651348Z    |
2020-03-27T08:14:40.7651676Z LL | fn mutual_recursion() -> impl Sync {
2020-03-27T08:14:40.7651905Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7651905Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7652615Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Sync`...
2020-03-27T08:14:40.7653205Z note: ...which requires processing `mutual_recursion_b::{{opaque}}#0`...
2020-03-27T08:14:40.7654401Z    |
2020-03-27T08:14:40.7654401Z    |
2020-03-27T08:14:40.7654897Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T08:14:40.7655124Z    |                            ^^^^^^^^^^
2020-03-27T08:14:40.7655613Z note: ...which requires borrow-checking `mutual_recursion_b`...
2020-03-27T08:14:40.7656382Z    |
2020-03-27T08:14:40.7656382Z    |
2020-03-27T08:14:40.7656711Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T08:14:40.7656937Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7657184Z note: ...which requires processing `mutual_recursion_b`...
2020-03-27T08:14:40.7657938Z    |
2020-03-27T08:14:40.7657938Z    |
2020-03-27T08:14:40.7658254Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T08:14:40.7658477Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7658753Z note: ...which requires const checking `mutual_recursion_b`...
2020-03-27T08:14:40.7659502Z    |
2020-03-27T08:14:40.7659502Z    |
2020-03-27T08:14:40.7659833Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T08:14:40.7660480Z    = note: ...which requires computing whether `impl std::marker::Sync` is freeze...
2020-03-27T08:14:40.7660480Z    = note: ...which requires computing whether `impl std::marker::Sync` is freeze...
2020-03-27T08:14:40.7660963Z    = note: ...which requires evaluating trait selection obligation `impl std::marker::Sync: std::marker::Freeze`...
2020-03-27T08:14:40.7661447Z    = note: ...which again requires processing `mutual_recursion::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7662020Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7663204Z    |
2020-03-27T08:14:40.7663352Z LL | / #![feature(generators)]
2020-03-27T08:14:40.7663582Z LL | | #![allow(unconditional_recursion)]
2020-03-27T08:14:40.7663746Z LL | |
2020-03-27T08:14:40.7663746Z LL | |
2020-03-27T08:14:40.7664113Z LL | | fn option(i: i32) -> impl Sized {
2020-03-27T08:14:40.7664573Z LL | |
2020-03-27T08:14:40.7664707Z LL | | fn main() {}
2020-03-27T08:14:40.7664871Z    | |____________^
2020-03-27T08:14:40.7664978Z 
---
2020-03-27T08:14:40.7666941Z 
2020-03-27T08:14:40.7667131Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7667623Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-indirect.rs:91:28
2020-03-27T08:14:40.7667856Z    |
2020-03-27T08:14:40.7668177Z LL | fn mutual_recursion_b() -> impl Sized {
2020-03-27T08:14:40.7668639Z    |
2020-03-27T08:14:40.7668791Z    = note: type resolves to itself
2020-03-27T08:14:40.7668939Z 
2020-03-27T08:14:40.7669093Z error: aborting due to 27 previous errors
---
2020-03-27T08:14:40.7670581Z 
2020-03-27T08:14:40.7670972Z ---- [ui] ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs stdout ----
2020-03-27T08:14:40.7671212Z diff of stderr:
2020-03-27T08:14:40.7671436Z 
2020-03-27T08:14:40.7671664Z + error[E0391]: cycle detected when processing `recursive_id::{{opaque}}#0`
2020-03-27T08:14:40.7672424Z +    |
2020-03-27T08:14:40.7672424Z +    |
2020-03-27T08:14:40.7672805Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T08:14:40.7673175Z +    |
2020-03-27T08:14:40.7673175Z +    |
2020-03-27T08:14:40.7673560Z + note: ...which requires borrow-checking `recursive_id`...
2020-03-27T08:14:40.7674240Z +    |
2020-03-27T08:14:40.7674240Z +    |
2020-03-27T08:14:40.7674559Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T08:14:40.7674765Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7674994Z + note: ...which requires processing `recursive_id`...
2020-03-27T08:14:40.7675682Z +    |
2020-03-27T08:14:40.7675682Z +    |
2020-03-27T08:14:40.7675988Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T08:14:40.7676211Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7676446Z + note: ...which requires const checking `recursive_id`...
2020-03-27T08:14:40.7677143Z +    |
2020-03-27T08:14:40.7677143Z +    |
2020-03-27T08:14:40.7677446Z + LL | fn recursive_id() -> impl Sized {
2020-03-27T08:14:40.7677928Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7677928Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7678322Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7678908Z + note: ...which requires processing `recursive_id2::{{opaque}}#0`...
2020-03-27T08:14:40.7679822Z +    |
2020-03-27T08:14:40.7679822Z +    |
2020-03-27T08:14:40.7680127Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T08:14:40.7680539Z +    |                       ^^^^^^^^^^
2020-03-27T08:14:40.7680951Z + note: ...which requires borrow-checking `recursive_id2`...
2020-03-27T08:14:40.7681858Z +    |
2020-03-27T08:14:40.7681858Z +    |
2020-03-27T08:14:40.7682165Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T08:14:40.7682607Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7682856Z + note: ...which requires processing `recursive_id2`...
2020-03-27T08:14:40.7683557Z +    |
2020-03-27T08:14:40.7683557Z +    |
2020-03-27T08:14:40.7683875Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T08:14:40.7684085Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7684323Z + note: ...which requires const checking `recursive_id2`...
2020-03-27T08:14:40.7685026Z +    |
2020-03-27T08:14:40.7685026Z +    |
2020-03-27T08:14:40.7685338Z + LL | fn recursive_id2() -> impl Sized {
2020-03-27T08:14:40.7685987Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7685987Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7686372Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7686980Z +    = note: ...which again requires processing `recursive_id::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7688250Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7688963Z +    |
2020-03-27T08:14:40.7688963Z +    |
2020-03-27T08:14:40.7689277Z + LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T08:14:40.7690152Z + 
2020-03-27T08:14:40.7690514Z 1 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7691153Z 2   --> $DIR/recursive-impl-trait-type-through-non-recursive.rs:7:22
2020-03-27T08:14:40.7691651Z 3    |
2020-03-27T08:14:40.7691651Z 3    |
2020-03-27T08:14:40.7691757Z 
2020-03-27T08:14:40.7691860Z 14    |
2020-03-27T08:14:40.7692202Z 15    = note: type resolves to itself
2020-03-27T08:14:40.7692362Z 16 
2020-03-27T08:14:40.7692665Z + error[E0391]: cycle detected when processing `recursive_wrap::{{opaque}}#0`
2020-03-27T08:14:40.7693682Z +    |
2020-03-27T08:14:40.7694010Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T08:14:40.7694223Z +    |                        ^^^^^^^^^^
2020-03-27T08:14:40.7694382Z +    |
2020-03-27T08:14:40.7694382Z +    |
2020-03-27T08:14:40.7694747Z + note: ...which requires borrow-checking `recursive_wrap`...
2020-03-27T08:14:40.7695435Z +    |
2020-03-27T08:14:40.7695758Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T08:14:40.7695971Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7695971Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7696441Z + note: ...which requires processing `recursive_wrap`...
2020-03-27T08:14:40.7697306Z +    |
2020-03-27T08:14:40.7697613Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T08:14:40.7697844Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7697844Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7698084Z + note: ...which requires const checking `recursive_wrap`...
2020-03-27T08:14:40.7699001Z +    |
2020-03-27T08:14:40.7699317Z + LL | fn recursive_wrap() -> impl Sized {
2020-03-27T08:14:40.7699537Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7699967Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7699967Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7700469Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7700883Z + note: ...which requires processing `recursive_wrap2::{{opaque}}#0`...
2020-03-27T08:14:40.7701721Z +    |
2020-03-27T08:14:40.7702033Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T08:14:40.7702270Z +    |                         ^^^^^^^^^^
2020-03-27T08:14:40.7702270Z +    |                         ^^^^^^^^^^
2020-03-27T08:14:40.7702676Z + note: ...which requires borrow-checking `recursive_wrap2`...
2020-03-27T08:14:40.7703377Z +    |
2020-03-27T08:14:40.7703684Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T08:14:40.7703901Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7703901Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7704305Z + note: ...which requires processing `recursive_wrap2`...
2020-03-27T08:14:40.7704968Z +    |
2020-03-27T08:14:40.7705266Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T08:14:40.7705669Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7705669Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7705911Z + note: ...which requires const checking `recursive_wrap2`...
2020-03-27T08:14:40.7706616Z +    |
2020-03-27T08:14:40.7706930Z + LL | fn recursive_wrap2() -> impl Sized {
2020-03-27T08:14:40.7707323Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7707610Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7707610Z +    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7708171Z +    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7708770Z +    = note: ...which again requires processing `recursive_wrap::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7709291Z + note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7709958Z +    |
2020-03-27T08:14:40.7709958Z +    |
2020-03-27T08:14:40.7710394Z + LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T08:14:40.7710742Z + 
2020-03-27T08:14:40.7710937Z 17 error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7711454Z 18   --> $DIR/recursive-impl-trait-type-through-non-recursive.rs:17:24
2020-03-27T08:14:40.7711680Z 19    |
2020-03-27T08:14:40.7711680Z 19    |
2020-03-27T08:14:40.7711960Z 
2020-03-27T08:14:40.7712063Z 30    |
2020-03-27T08:14:40.7712243Z 31    = note: expanded type is `((impl Sized,),)`
2020-03-27T08:14:40.7713176Z - error: aborting due to 4 previous errors
2020-03-27T08:14:40.7713723Z + error: aborting due to 6 previous errors
2020-03-27T08:14:40.7714078Z 34 
2020-03-27T08:14:40.7714507Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T08:14:40.7714507Z - For more information about this error, try `rustc --explain E0720`.
2020-03-27T08:14:40.7714838Z + Some errors have detailed explanations: E0391, E0720.
2020-03-27T08:14:40.7715719Z + For more information about an error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7715962Z 36 
2020-03-27T08:14:40.7716058Z 
2020-03-27T08:14:40.7716148Z 
2020-03-27T08:14:40.7716340Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.7717591Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive/recursive-impl-trait-type-through-non-recursive.stderr
2020-03-27T08:14:40.7718211Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.7718768Z To only update this specific test, also pass `--test-args impl-trait/recursive-impl-trait-type-through-non-recursive.rs`
2020-03-27T08:14:40.7719193Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.7719386Z status: exit code: 1
2020-03-27T08:14:40.7719386Z status: exit code: 1
2020-03-27T08:14:40.7721214Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive/auxiliary"
2020-03-27T08:14:40.7723221Z ------------------------------------------
2020-03-27T08:14:40.7723379Z 
2020-03-27T08:14:40.7723704Z ------------------------------------------
2020-03-27T08:14:40.7724320Z stderr:
2020-03-27T08:14:40.7724320Z stderr:
2020-03-27T08:14:40.7724677Z ------------------------------------------
2020-03-27T08:14:40.7725402Z error[E0391]: cycle detected when processing `recursive_id::{{opaque}}#0`
2020-03-27T08:14:40.7727020Z    |
2020-03-27T08:14:40.7727020Z    |
2020-03-27T08:14:40.7727436Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7727896Z    |
2020-03-27T08:14:40.7727896Z    |
2020-03-27T08:14:40.7728590Z note: ...which requires borrow-checking `recursive_id`...
2020-03-27T08:14:40.7729785Z    |
2020-03-27T08:14:40.7729785Z    |
2020-03-27T08:14:40.7730195Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7730492Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7730722Z note: ...which requires processing `recursive_id`...
2020-03-27T08:14:40.7731673Z    |
2020-03-27T08:14:40.7731673Z    |
2020-03-27T08:14:40.7732418Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7732765Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7733008Z note: ...which requires const checking `recursive_id`...
2020-03-27T08:14:40.7733852Z    |
2020-03-27T08:14:40.7733852Z    |
2020-03-27T08:14:40.7734472Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7735013Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7735013Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7735426Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7735834Z note: ...which requires processing `recursive_id2::{{opaque}}#0`...
2020-03-27T08:14:40.7736693Z    |
2020-03-27T08:14:40.7736693Z    |
2020-03-27T08:14:40.7737112Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7737413Z    |                       ^^^^^^^^^^
2020-03-27T08:14:40.7737818Z note: ...which requires borrow-checking `recursive_id2`...
2020-03-27T08:14:40.7739691Z    |
2020-03-27T08:14:40.7739691Z    |
2020-03-27T08:14:40.7740386Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7740746Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7741051Z note: ...which requires processing `recursive_id2`...
2020-03-27T08:14:40.7742206Z    |
2020-03-27T08:14:40.7742206Z    |
2020-03-27T08:14:40.7743036Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7743328Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7743568Z note: ...which requires const checking `recursive_id2`...
2020-03-27T08:14:40.7744380Z    |
2020-03-27T08:14:40.7744380Z    |
2020-03-27T08:14:40.7744801Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7745365Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7745365Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7745764Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7746233Z    = note: ...which again requires processing `recursive_id::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7746755Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7747573Z    |
2020-03-27T08:14:40.7747573Z    |
2020-03-27T08:14:40.7747892Z LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T08:14:40.7748235Z 
2020-03-27T08:14:40.7748436Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7748968Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:7:22
2020-03-27T08:14:40.7749229Z    |
2020-03-27T08:14:40.7749229Z    |
2020-03-27T08:14:40.7749659Z LL | fn recursive_id() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7750175Z    |
2020-03-27T08:14:40.7750347Z    = note: type resolves to itself
2020-03-27T08:14:40.7750484Z 
2020-03-27T08:14:40.7750665Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7750665Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7751213Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:11:23
2020-03-27T08:14:40.7751587Z    |
2020-03-27T08:14:40.7752035Z LL | fn recursive_id2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7752574Z    |
2020-03-27T08:14:40.7752794Z    = note: type resolves to itself
2020-03-27T08:14:40.7752937Z 
2020-03-27T08:14:40.7752937Z 
2020-03-27T08:14:40.7753186Z error[E0391]: cycle detected when processing `recursive_wrap::{{opaque}}#0`
2020-03-27T08:14:40.7754056Z    |
2020-03-27T08:14:40.7754056Z    |
2020-03-27T08:14:40.7754489Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7754935Z    |
2020-03-27T08:14:40.7754935Z    |
2020-03-27T08:14:40.7755303Z note: ...which requires borrow-checking `recursive_wrap`...
2020-03-27T08:14:40.7756105Z    |
2020-03-27T08:14:40.7756105Z    |
2020-03-27T08:14:40.7756539Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7756825Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7757064Z note: ...which requires processing `recursive_wrap`...
2020-03-27T08:14:40.7757873Z    |
2020-03-27T08:14:40.7757873Z    |
2020-03-27T08:14:40.7758289Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7758590Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7758831Z note: ...which requires const checking `recursive_wrap`...
2020-03-27T08:14:40.7759640Z    |
2020-03-27T08:14:40.7759640Z    |
2020-03-27T08:14:40.7760054Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7760623Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7760623Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7761024Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7761435Z note: ...which requires processing `recursive_wrap2::{{opaque}}#0`...
2020-03-27T08:14:40.7762288Z    |
2020-03-27T08:14:40.7762288Z    |
2020-03-27T08:14:40.7762709Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7763014Z    |                         ^^^^^^^^^^
2020-03-27T08:14:40.7763421Z note: ...which requires borrow-checking `recursive_wrap2`...
2020-03-27T08:14:40.7764241Z    |
2020-03-27T08:14:40.7764241Z    |
2020-03-27T08:14:40.7764836Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7765119Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7765366Z note: ...which requires processing `recursive_wrap2`...
2020-03-27T08:14:40.7766136Z    |
2020-03-27T08:14:40.7766136Z    |
2020-03-27T08:14:40.7766559Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7766840Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-27T08:14:40.7767076Z note: ...which requires const checking `recursive_wrap2`...
2020-03-27T08:14:40.7767864Z    |
2020-03-27T08:14:40.7767864Z    |
2020-03-27T08:14:40.7768272Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7768895Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7768895Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7769281Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7769779Z    = note: ...which again requires processing `recursive_wrap::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7770312Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7771099Z    |
2020-03-27T08:14:40.7771099Z    |
2020-03-27T08:14:40.7771403Z LL | fn id<T>(t: T) -> impl Sized { t }
2020-03-27T08:14:40.7771751Z 
2020-03-27T08:14:40.7771926Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7772441Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:17:24
2020-03-27T08:14:40.7772718Z    |
2020-03-27T08:14:40.7772718Z    |
2020-03-27T08:14:40.7773127Z LL | fn recursive_wrap() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7773639Z    |
2020-03-27T08:14:40.7773639Z    |
2020-03-27T08:14:40.7773829Z    = note: expanded type is `((impl Sized,),)`
2020-03-27T08:14:40.7774155Z error[E0720]: opaque type expands to a recursive type
2020-03-27T08:14:40.7774692Z   --> /checkout/src/test/ui/impl-trait/recursive-impl-trait-type-through-non-recursive.rs:21:25
2020-03-27T08:14:40.7774948Z    |
2020-03-27T08:14:40.7774948Z    |
2020-03-27T08:14:40.7775357Z LL | fn recursive_wrap2() -> impl Sized { //~ ERROR opaque type expands to a recursive type
2020-03-27T08:14:40.7775887Z    |
2020-03-27T08:14:40.7775887Z    |
2020-03-27T08:14:40.7776057Z    = note: expanded type is `((impl Sized,),)`
2020-03-27T08:14:40.7776381Z error: aborting due to 6 previous errors
2020-03-27T08:14:40.7776525Z 
2020-03-27T08:14:40.7776708Z Some errors have detailed explanations: E0391, E0720.
2020-03-27T08:14:40.7777170Z For more information about an error, try `rustc --explain E0391`.
---
2020-03-27T08:14:40.7778220Z ---- [ui] ui/impl-trait/unsafety-checking-cycle.rs stdout ----
2020-03-27T08:14:40.7778390Z 
2020-03-27T08:14:40.7778709Z error: test compilation failed although it shouldn't!
2020-03-27T08:14:40.7778932Z status: exit code: 1
2020-03-27T08:14:40.7780685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/unsafety-checking-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/unsafety-checking-cycle/auxiliary"
2020-03-27T08:14:40.7782093Z ------------------------------------------
2020-03-27T08:14:40.7782237Z 
2020-03-27T08:14:40.7782550Z ------------------------------------------
2020-03-27T08:14:40.7782716Z stderr:
2020-03-27T08:14:40.7782716Z stderr:
2020-03-27T08:14:40.7783018Z ------------------------------------------
2020-03-27T08:14:40.7783321Z error[E0391]: cycle detected when processing `not_restricted::{{opaque}}#0`
2020-03-27T08:14:40.7784044Z    |
2020-03-27T08:14:40.7784044Z    |
2020-03-27T08:14:40.7784382Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T08:14:40.7785128Z    |
2020-03-27T08:14:40.7785523Z note: ...which requires borrow-checking `not_restricted`...
2020-03-27T08:14:40.7786387Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-03-27T08:14:40.7786620Z    |
2020-03-27T08:14:40.7786620Z    |
2020-03-27T08:14:40.7787052Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T08:14:40.7787580Z note: ...which requires processing `not_restricted`...
2020-03-27T08:14:40.7788277Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-03-27T08:14:40.7788656Z    |
2020-03-27T08:14:40.7788656Z    |
2020-03-27T08:14:40.7789154Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T08:14:40.7789647Z note: ...which requires const checking `not_restricted`...
2020-03-27T08:14:40.7790105Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:12:1
2020-03-27T08:14:40.7790333Z    |
2020-03-27T08:14:40.7790333Z    |
2020-03-27T08:14:40.7790661Z LL | fn not_restricted(c: bool) -> impl Sized {
2020-03-27T08:14:40.7791165Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7791165Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7791764Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7792417Z    = note: ...which again requires processing `not_restricted::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7792950Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7794077Z    |
2020-03-27T08:14:40.7794077Z    |
2020-03-27T08:14:40.7794406Z LL | / #![feature(rustc_attrs)]
2020-03-27T08:14:40.7794557Z LL | |
2020-03-27T08:14:40.7794705Z LL | | struct AnyValue<T>(T);
2020-03-27T08:14:40.7795136Z ...  |
2020-03-27T08:14:40.7795244Z LL | |
2020-03-27T08:14:40.7795373Z LL | | fn main() {}
2020-03-27T08:14:40.7795541Z    | |____________^
2020-03-27T08:14:40.7795541Z    | |____________^
2020-03-27T08:14:40.7795648Z 
2020-03-27T08:14:40.7795865Z error[E0391]: cycle detected when processing `not_field::{{opaque}}#0`
2020-03-27T08:14:40.7796601Z    |
2020-03-27T08:14:40.7796601Z    |
2020-03-27T08:14:40.7796915Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T08:14:40.7797308Z    |
2020-03-27T08:14:40.7797638Z note: ...which requires borrow-checking `not_field`...
2020-03-27T08:14:40.7798091Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-03-27T08:14:40.7798655Z    |
2020-03-27T08:14:40.7798655Z    |
2020-03-27T08:14:40.7798971Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T08:14:40.7799440Z note: ...which requires processing `not_field`...
2020-03-27T08:14:40.7799897Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-03-27T08:14:40.7800120Z    |
2020-03-27T08:14:40.7800120Z    |
2020-03-27T08:14:40.7800457Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T08:14:40.7800921Z note: ...which requires const checking `not_field`...
2020-03-27T08:14:40.7801562Z   --> /checkout/src/test/ui/impl-trait/unsafety-checking-cycle.rs:24:1
2020-03-27T08:14:40.7801774Z    |
2020-03-27T08:14:40.7801774Z    |
2020-03-27T08:14:40.7802085Z LL | fn not_field(c: bool) -> impl Sized {
2020-03-27T08:14:40.7802580Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7802580Z    = note: ...which requires computing whether `impl Sized` is freeze...
2020-03-27T08:14:40.7802965Z    = note: ...which requires evaluating trait selection obligation `impl Sized: std::marker::Freeze`...
2020-03-27T08:14:40.7803413Z    = note: ...which again requires processing `not_field::{{opaque}}#0`, completing the cycle
2020-03-27T08:14:40.7803912Z note: cycle used when checking item types in top-level module
2020-03-27T08:14:40.7804942Z    |
2020-03-27T08:14:40.7804942Z    |
2020-03-27T08:14:40.7805082Z LL | / #![feature(rustc_attrs)]
2020-03-27T08:14:40.7805224Z LL | |
2020-03-27T08:14:40.7805378Z LL | | struct AnyValue<T>(T);
2020-03-27T08:14:40.7805684Z ...  |
2020-03-27T08:14:40.7805964Z LL | |
2020-03-27T08:14:40.7806108Z LL | | fn main() {}
2020-03-27T08:14:40.7806256Z    | |____________^
---
2020-03-27T08:14:40.7815474Z +   --> $DIR/issue-52843-closure-constrain.rs:7:1
2020-03-27T08:14:40.7815679Z +    |
2020-03-27T08:14:40.7815808Z + LL | fn main() {
2020-03-27T08:14:40.7815949Z +    | ^^^^^^^^^
2020-03-27T08:14:40.7816195Z +    = note: ...which requires computing whether `Opaque` is freeze...
2020-03-27T08:14:40.7816579Z +    = note: ...which requires evaluating trait selection obligation `Opaque: std::marker::Freeze`...
2020-03-27T08:14:40.7816993Z +    = note: ...which again requires processing `main::Opaque`, completing the cycle
2020-03-27T08:14:40.7817504Z + note: cycle used when collecting item types in top-level module
2020-03-27T08:14:40.7818119Z +    |
2020-03-27T08:14:40.7818300Z + LL | / #![feature(type_alias_impl_trait)]
2020-03-27T08:14:40.7818466Z + LL | |
2020-03-27T08:14:40.7818624Z + LL | | use std::fmt::Debug;
---
2020-03-27T08:14:40.7819706Z 1 error: concrete type differs from previous defining opaque type use
2020-03-27T08:14:40.7820171Z 2   --> $DIR/issue-52843-closure-constrain.rs:10:16
2020-03-27T08:14:40.7820447Z 3    |
2020-03-27T08:14:40.7820536Z 
2020-03-27T08:14:40.7821438Z 10 LL |     fn _unused() -> Opaque { String::new() }
2020-03-27T08:14:40.7822058Z 12 
2020-03-27T08:14:40.7822378Z - error: aborting due to previous error
2020-03-27T08:14:40.7822628Z + error: aborting due to 2 previous errors
2020-03-27T08:14:40.7822799Z 14 
2020-03-27T08:14:40.7822799Z 14 
2020-03-27T08:14:40.7823184Z + For more information about this error, try `rustc --explain E0391`.
2020-03-27T08:14:40.7823410Z 15 
2020-03-27T08:14:40.7823500Z 
2020-03-27T08:14:40.7823583Z 
2020-03-27T08:14:40.7824012Z The actual stderr differed from the expected stderr.
2020-03-27T08:14:40.7824852Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain/issue-52843-closure-constrain.stderr
2020-03-27T08:14:40.7825497Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T08:14:40.7826089Z To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-52843-closure-constrain.rs`
2020-03-27T08:14:40.7826488Z error: 1 errors occurred comparing output.
2020-03-27T08:14:40.7826681Z status: exit code: 1
2020-03-27T08:14:40.7826681Z status: exit code: 1
2020-03-27T08:14:40.7828468Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843-closure-constrain/auxiliary"
2020-03-27T08:14:40.7829910Z ------------------------------------------
2020-03-27T08:14:40.7830053Z 
2020-03-27T08:14:40.7830364Z ------------------------------------------
2020-03-27T08:14:40.7830529Z stderr:
---
2020-03-27T08:14:40.7836285Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:7:1
2020-03-27T08:14:40.7836521Z    |
2020-03-27T08:14:40.7836657Z LL | fn main() {
2020-03-27T08:14:40.7836792Z    | ^^^^^^^^^
2020-03-27T08:14:40.7837014Z    = note: ...which requires computing whether `Opaque` is freeze...
2020-03-27T08:14:40.7837405Z    = note: ...which requires evaluating trait selection obligation `Opaque: std::marker::Freeze`...
2020-03-27T08:14:40.7837820Z    = note: ...which again requires processing `main::Opaque`, completing the cycle
2020-03-27T08:14:40.7838310Z note: cycle used when collecting item types in top-level module
2020-03-27T08:14:40.7839067Z    |
2020-03-27T08:14:40.7839227Z LL | / #![feature(type_alias_impl_trait)]
2020-03-27T08:14:40.7839401Z LL | |
2020-03-27T08:14:40.7839552Z LL | | use std::fmt::Debug;
---
2020-03-27T08:14:40.7840391Z 
2020-03-27T08:14:40.7840583Z error: concrete type differs from previous defining opaque type use
2020-03-27T08:14:40.7841189Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:10:16
2020-03-27T08:14:40.7841447Z    |
2020-03-27T08:14:40.7841748Z LL |     let null = || -> Opaque { 0 };
2020-03-27T08:14:40.7842095Z    |                ^^^^^^^^^^^^^^^^^^ expected `std::string::String`, got `i32`
2020-03-27T08:14:40.7842487Z note: previous use here
2020-03-27T08:14:40.7842951Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-52843-closure-constrain.rs:9:5
2020-03-27T08:14:40.7844108Z    |
2020-03-27T08:14:40.7844108Z    |
2020-03-27T08:14:40.7845003Z LL |     fn _unused() -> Opaque { String::new() }
2020-03-27T08:14:40.7845589Z 
2020-03-27T08:14:40.7845760Z error: aborting due to 2 previous errors
2020-03-27T08:14:40.7845900Z 
2020-03-27T08:14:40.7846832Z For more information about this error, try `rustc --explain E0391`.
---
2020-03-27T08:14:40.7852339Z test result: FAILED. 9779 passed; 10 failed; 60 ignored; 0 measured; 0 filtered out
2020-03-27T08:14:40.7852569Z 
2020-03-27T08:14:40.7852647Z 
2020-03-27T08:14:40.7852727Z 
2020-03-27T08:14:40.7856814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T08:14:40.7859173Z 
2020-03-27T08:14:40.7859251Z 
2020-03-27T08:14:40.7859675Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T08:14:40.7859974Z Build completed unsuccessfully in 1:08:12
2020-03-27T08:14:40.7859974Z Build completed unsuccessfully in 1:08:12
2020-03-27T08:14:40.7860171Z == clock drift check ==
2020-03-27T08:14:40.7860504Z   local time: Fri Mar 27 08:14:40 UTC 2020
2020-03-27T08:14:40.9749942Z   network time: Fri, 27 Mar 2020 08:14:40 GMT
2020-03-27T08:14:40.9755227Z == end clock drift check ==
2020-03-27T08:14:41.4198044Z 
2020-03-27T08:14:41.4271273Z ##[error]Bash exited with code '1'.
2020-03-27T08:14:41.4309288Z ##[section]Finishing: Run build
2020-03-27T08:14:41.4361241Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-03-27T08:14:41.4366739Z Task         : Get sources
2020-03-27T08:14:41.4367102Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T08:14:41.4367422Z Version      : 1.0.0
2020-03-27T08:14:41.4367800Z Author       : Microsoft
2020-03-27T08:14:41.4367800Z Author       : Microsoft
2020-03-27T08:14:41.4368161Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T08:14:41.4368996Z ==============================================================================
2020-03-27T08:14:41.7731016Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T08:14:41.7775771Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70450/merge to s
2020-03-27T08:14:41.7863593Z Cleaning up task key
2020-03-27T08:14:41.7864737Z Start cleaning up orphan processes.
2020-03-27T08:14:41.8043468Z Terminate orphan process: pid (3379) (python)
2020-03-27T08:14:41.8227377Z ##[section]Finishing: Finalize Job
