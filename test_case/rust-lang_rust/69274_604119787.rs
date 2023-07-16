plain
2020-03-25T21:18:06.9146558Z ========================== Starting Command Output ===========================
2020-03-25T21:18:06.9149614Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/88f219a9-0249-48a7-93ad-522d2a7eb12f.sh
2020-03-25T21:18:06.9149911Z 
2020-03-25T21:18:06.9155011Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-25T21:18:06.9174335Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-25T21:18:06.9177887Z Task         : Get sources
2020-03-25T21:18:06.9178224Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T21:18:06.9178542Z Version      : 1.0.0
2020-03-25T21:18:06.9178754Z Author       : Microsoft
---
2020-03-25T21:18:08.1406377Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-25T21:18:08.1414551Z ##[command]git config gc.auto 0
2020-03-25T21:18:08.1419506Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-25T21:18:08.1424466Z ##[command]git config --get-all http.proxy
2020-03-25T21:18:08.1436921Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-03-25T21:26:44.8531374Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-25T21:26:45.0502426Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-25T21:26:54.8721320Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-25T21:27:03.2144959Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T21:27:07.1809603Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T21:27:09.8854827Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T21:27:43.2431944Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T21:27:51.4877365Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T21:28:40.5402485Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-25T21:48:51.4357393Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-25T21:49:03.9334288Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-25T21:49:12.1990595Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-25T21:49:27.0525819Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-25T21:49:30.7123507Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-25T21:49:32.4439483Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-25T21:50:16.9614658Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-25T21:50:27.2183658Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-25T21:51:32.3064011Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-25T22:15:10.3901921Z .................................................................................................... 1700/9840
2020-03-25T22:15:14.3640914Z .................................................................................................... 1800/9840
2020-03-25T22:15:24.3224207Z .........................................................................................i.......... 1900/9840
2020-03-25T22:15:31.0205751Z .................................................................................................... 2000/9840
2020-03-25T22:15:37.3790915Z ...............................................................................iiiii................ 2100/9840
2020-03-25T22:15:58.1803120Z .................................................................................................... 2300/9840
2020-03-25T22:16:00.3556616Z .................................................................................................... 2400/9840
2020-03-25T22:16:02.7345922Z .................................................................................................... 2500/9840
2020-03-25T22:16:09.2549448Z .................................................................................................... 2600/9840
---
2020-03-25T22:19:04.0783255Z ......................................................i...............i............................. 5000/9840
2020-03-25T22:19:11.9123290Z .................................................................................................... 5100/9840
2020-03-25T22:19:19.4372630Z ...................................................................................................i 5200/9840
2020-03-25T22:19:24.4768552Z .................................................................................................... 5300/9840
2020-03-25T22:19:34.9591228Z ..................................................................................ii.ii........i...i 5400/9840
2020-03-25T22:19:42.2614814Z ......................i............................................................................. 5600/9840
2020-03-25T22:19:49.6320844Z ...........................i........................................................................ 5700/9840
2020-03-25T22:19:57.4166027Z ............................................ii....................................i................. 5800/9840
2020-03-25T22:20:04.6847477Z .................................................................................................... 5900/9840
2020-03-25T22:20:04.6847477Z .................................................................................................... 5900/9840
2020-03-25T22:20:09.8840672Z .................................................................................................... 6000/9840
2020-03-25T22:20:19.2795669Z ............................................................................ii...i..ii...........i.. 6100/9840
2020-03-25T22:20:39.8788423Z .................................................................................................... 6300/9840
2020-03-25T22:20:44.8191980Z .................................................................................................... 6400/9840
2020-03-25T22:20:48.3867236Z .................................................................................................... 6500/9840
2020-03-25T22:20:48.3867236Z .................................................................................................... 6500/9840
2020-03-25T22:21:00.5243874Z ......i..ii......................................................................................... 6600/9840
2020-03-25T22:21:20.2838742Z .................................................................................................... 6800/9840
2020-03-25T22:21:22.3932649Z .....i.............................................................................................. 6900/9840
2020-03-25T22:21:24.4902108Z .................................................................................................... 7000/9840
2020-03-25T22:21:26.7755614Z ........................................i........................................................... 7100/9840
---
2020-03-25T22:23:06.8757123Z .................................................................................................... 7800/9840
2020-03-25T22:23:11.5471903Z .................................................................................................... 7900/9840
2020-03-25T22:23:18.5255831Z .................................................................................................i.. 8000/9840
2020-03-25T22:23:25.9036473Z .................................................................................................... 8100/9840
2020-03-25T22:23:33.6826516Z ...................................................iiiiiiiiii.i..................................... 8200/9840
2020-03-25T22:23:47.4601411Z ..i................................................................................................. 8400/9840
2020-03-25T22:23:52.5299725Z .................................................................................................... 8500/9840
2020-03-25T22:24:05.4832797Z .................................................................................................... 8600/9840
2020-03-25T22:24:14.8328995Z .................................................................................................... 8700/9840
---
2020-03-25T22:26:07.3346100Z 
2020-03-25T22:26:07.3347053Z ---- [ui] ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs stdout ----
2020-03-25T22:26:07.3347397Z diff of stderr:
2020-03-25T22:26:07.3347532Z 
2020-03-25T22:26:07.3348591Z 67    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3349532Z 68    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3350262Z - error: aborting due to 8 previous errors
2020-03-25T22:26:07.3352567Z + error[E0183]: manual implementations of `Fn` are experimental
2020-03-25T22:26:07.3353190Z +   --> $DIR/feature-gate-unboxed-closures-manual-impls.rs:9:1
2020-03-25T22:26:07.3353464Z +    |
---
2020-03-25T22:26:07.3364877Z 73 For more information about an error, try `rustc --explain E0229`.
2020-03-25T22:26:07.3365125Z 
2020-03-25T22:26:07.3365234Z 
2020-03-25T22:26:07.3365450Z The actual stderr differed from the expected stderr.
2020-03-25T22:26:07.3366308Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls/feature-gate-unboxed-closures-manual-impls.stderr
2020-03-25T22:26:07.3367096Z To update references, rerun the tests and pass the `--bless` flag
2020-03-25T22:26:07.3367800Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-unboxed-closures-manual-impls.rs`
2020-03-25T22:26:07.3368326Z error: 1 errors occurred comparing output.
2020-03-25T22:26:07.3368571Z status: exit code: 1
2020-03-25T22:26:07.3368571Z status: exit code: 1
2020-03-25T22:26:07.3370846Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls/auxiliary"
2020-03-25T22:26:07.3372686Z ------------------------------------------
2020-03-25T22:26:07.3372868Z 
2020-03-25T22:26:07.3373241Z ------------------------------------------
2020-03-25T22:26:07.3373470Z stderr:
2020-03-25T22:26:07.3373470Z stderr:
2020-03-25T22:26:07.3373850Z ------------------------------------------
2020-03-25T22:26:07.3374325Z error[E0658]: rust-call ABI is subject to change
2020-03-25T22:26:07.3374981Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:11:12
2020-03-25T22:26:07.3376458Z    |
2020-03-25T22:26:07.3377008Z LL |     extern "rust-call" fn call(self, args: ()) -> () {}
2020-03-25T22:26:07.3377529Z    |
2020-03-25T22:26:07.3377529Z    |
2020-03-25T22:26:07.3378139Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3378571Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3379231Z error[E0658]: rust-call ABI is subject to change
2020-03-25T22:26:07.3379882Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:17:12
2020-03-25T22:26:07.3380201Z    |
2020-03-25T22:26:07.3380201Z    |
2020-03-25T22:26:07.3380678Z LL |     extern "rust-call" fn call_once(self, args: ()) -> () {}
2020-03-25T22:26:07.3381186Z    |
2020-03-25T22:26:07.3381186Z    |
2020-03-25T22:26:07.3381748Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3382165Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3382952Z error[E0658]: rust-call ABI is subject to change
2020-03-25T22:26:07.3383607Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:23:12
2020-03-25T22:26:07.3383928Z    |
2020-03-25T22:26:07.3383928Z    |
2020-03-25T22:26:07.3384485Z LL |     extern "rust-call" fn call_mut(&self, args: ()) -> () {}
2020-03-25T22:26:07.3384999Z    |
2020-03-25T22:26:07.3384999Z    |
2020-03-25T22:26:07.3385572Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3385993Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3386644Z error[E0658]: rust-call ABI is subject to change
2020-03-25T22:26:07.3387275Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:29:12
2020-03-25T22:26:07.3387612Z    |
2020-03-25T22:26:07.3387612Z    |
2020-03-25T22:26:07.3388087Z LL |     extern "rust-call" fn call_once(&self, args: ()) -> () {}
2020-03-25T22:26:07.3388600Z    |
2020-03-25T22:26:07.3388600Z    |
2020-03-25T22:26:07.3389162Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3389580Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3389827Z 
2020-03-25T22:26:07.3390354Z error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
2020-03-25T22:26:07.3393436Z    |
2020-03-25T22:26:07.3393612Z LL | impl Fn<()> for Foo {
2020-03-25T22:26:07.3394182Z    |      ^^^^^^ help: use parenthetical notation instead: `Fn() -> ()`
2020-03-25T22:26:07.3394487Z    |
2020-03-25T22:26:07.3394487Z    |
2020-03-25T22:26:07.3395077Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3395498Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3395993Z error[E0229]: associated type bindings are not allowed here
2020-03-25T22:26:07.3396649Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:15:6
2020-03-25T22:26:07.3396982Z    |
2020-03-25T22:26:07.3397165Z LL | impl FnOnce() for Foo1 {
2020-03-25T22:26:07.3397165Z LL | impl FnOnce() for Foo1 {
2020-03-25T22:26:07.3397434Z    |      ^^^^^^^^ associated type not allowed here
2020-03-25T22:26:07.3397645Z 
2020-03-25T22:26:07.3398166Z error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
2020-03-25T22:26:07.3399214Z    |
2020-03-25T22:26:07.3399396Z LL | impl FnMut<()> for Bar {
2020-03-25T22:26:07.3399953Z    |      ^^^^^^^^^ help: use parenthetical notation instead: `FnMut() -> ()`
2020-03-25T22:26:07.3400248Z    |
2020-03-25T22:26:07.3400248Z    |
2020-03-25T22:26:07.3400827Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3401229Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3401499Z 
2020-03-25T22:26:07.3402021Z error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
2020-03-25T22:26:07.3403064Z    |
2020-03-25T22:26:07.3403249Z LL | impl FnOnce<()> for Baz {
2020-03-25T22:26:07.3403810Z    |      ^^^^^^^^^^ help: use parenthetical notation instead: `FnOnce() -> ()`
2020-03-25T22:26:07.3404113Z    |
2020-03-25T22:26:07.3404113Z    |
2020-03-25T22:26:07.3404692Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3405099Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3405601Z error[E0183]: manual implementations of `Fn` are experimental
2020-03-25T22:26:07.3406257Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:9:1
2020-03-25T22:26:07.3406691Z    |
2020-03-25T22:26:07.3406881Z LL | impl Fn<()> for Foo {
---
2020-03-25T22:26:07.3417809Z 
2020-03-25T22:26:07.3418276Z ---- [ui] ui/feature-gates/feature-gate-unboxed-closures.rs stdout ----
2020-03-25T22:26:07.3418556Z diff of stderr:
2020-03-25T22:26:07.3418686Z 
2020-03-25T22:26:07.3419273Z 16    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3419697Z 17    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3420388Z - error: aborting due to 2 previous errors
2020-03-25T22:26:07.3420728Z + error[E0183]: manual implementations of `FnOnce` are experimental
2020-03-25T22:26:07.3421272Z +   --> $DIR/feature-gate-unboxed-closures.rs:5:1
2020-03-25T22:26:07.3421526Z +    |
2020-03-25T22:26:07.3421526Z +    |
2020-03-25T22:26:07.3421733Z + LL | impl FnOnce<(u32, u32)> for Test {
2020-03-25T22:26:07.3422094Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ manual implementations of `FnOnce` are experimental
2020-03-25T22:26:07.3422685Z +    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3422961Z + 
2020-03-25T22:26:07.3423177Z + error: aborting due to 3 previous errors
2020-03-25T22:26:07.3423382Z 20 
2020-03-25T22:26:07.3423382Z 20 
2020-03-25T22:26:07.3423843Z 21 For more information about this error, try `rustc --explain E0658`.
2020-03-25T22:26:07.3424112Z 22 
2020-03-25T22:26:07.3424221Z 
2020-03-25T22:26:07.3424322Z 
2020-03-25T22:26:07.3424536Z The actual stderr differed from the expected stderr.
2020-03-25T22:26:07.3425326Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures/feature-gate-unboxed-closures.stderr
2020-03-25T22:26:07.3426120Z To update references, rerun the tests and pass the `--bless` flag
2020-03-25T22:26:07.3426834Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-unboxed-closures.rs`
2020-03-25T22:26:07.3427350Z error: 1 errors occurred comparing output.
2020-03-25T22:26:07.3427599Z status: exit code: 1
2020-03-25T22:26:07.3427599Z status: exit code: 1
2020-03-25T22:26:07.3429791Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unboxed-closures/auxiliary"
2020-03-25T22:26:07.3482783Z ------------------------------------------
2020-03-25T22:26:07.3482993Z 
2020-03-25T22:26:07.3483389Z ------------------------------------------
2020-03-25T22:26:07.3483621Z stderr:
2020-03-25T22:26:07.3483621Z stderr:
2020-03-25T22:26:07.3484007Z ------------------------------------------
2020-03-25T22:26:07.3484492Z error[E0658]: rust-call ABI is subject to change
2020-03-25T22:26:07.3485109Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures.rs:9:12
2020-03-25T22:26:07.3485403Z    |
2020-03-25T22:26:07.3486170Z LL |     extern "rust-call" fn call_once(self, (a, b): (u32, u32)) -> u32 {
2020-03-25T22:26:07.3487318Z    |
2020-03-25T22:26:07.3487318Z    |
2020-03-25T22:26:07.3488178Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3488612Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3488865Z 
2020-03-25T22:26:07.3489813Z error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
2020-03-25T22:26:07.3491226Z    |
2020-03-25T22:26:07.3491226Z    |
2020-03-25T22:26:07.3491429Z LL | impl FnOnce<(u32, u32)> for Test {
2020-03-25T22:26:07.3492078Z    |      ^^^^^^^^^^^^^^^^^^ help: use parenthetical notation instead: `FnOnce(u32, u32) -> ()`
2020-03-25T22:26:07.3492415Z    |
2020-03-25T22:26:07.3492996Z    = note: see issue #29625 <***/issues/29625> for more information
2020-03-25T22:26:07.3493418Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3493912Z error[E0183]: manual implementations of `FnOnce` are experimental
2020-03-25T22:26:07.3494545Z   --> /checkout/src/test/ui/feature-gates/feature-gate-unboxed-closures.rs:5:1
2020-03-25T22:26:07.3494840Z    |
2020-03-25T22:26:07.3494840Z    |
2020-03-25T22:26:07.3495042Z LL | impl FnOnce<(u32, u32)> for Test {
2020-03-25T22:26:07.3495412Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ manual implementations of `FnOnce` are experimental
2020-03-25T22:26:07.3495979Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2020-03-25T22:26:07.3496425Z 
2020-03-25T22:26:07.3496641Z error: aborting due to 3 previous errors
2020-03-25T22:26:07.3496820Z 
---
2020-03-25T22:26:07.3505559Z 
2020-03-25T22:26:07.3505662Z 
2020-03-25T22:26:07.3505892Z The actual stderr differed from the expected stderr.
2020-03-25T22:26:07.3506567Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214/issue-3214.stderr
2020-03-25T22:26:07.3507214Z To update references, rerun the tests and pass the `--bless` flag
2020-03-25T22:26:07.3507839Z To only update this specific test, also pass `--test-args issues/issue-3214.rs`
2020-03-25T22:26:07.3508460Z error: 1 errors occurred comparing output.
2020-03-25T22:26:07.3508724Z status: exit code: 1
2020-03-25T22:26:07.3508724Z status: exit code: 1
2020-03-25T22:26:07.3511153Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3214.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3214/auxiliary"
2020-03-25T22:26:07.3513001Z ------------------------------------------
2020-03-25T22:26:07.3513186Z 
2020-03-25T22:26:07.3513577Z ------------------------------------------
2020-03-25T22:26:07.3513787Z stderr:
---
2020-03-25T22:26:07.3516248Z    |    --- - type parameter from outer function
2020-03-25T22:26:07.3516476Z    |    |
2020-03-25T22:26:07.3516749Z    |    try adding a local generic parameter in this method instead
2020-03-25T22:26:07.3517037Z LL |     struct Foo {
2020-03-25T22:26:07.3517558Z LL |         x: T, //~ ERROR can't use generic parameters from outer function
2020-03-25T22:26:07.3518090Z    |            ^ use of generic parameter from outer function
2020-03-25T22:26:07.3518572Z error[E0107]: wrong number of type arguments: expected 0, found 1
2020-03-25T22:26:07.3519144Z   --> /checkout/src/test/ui/issues/issue-3214.rs:6:26
2020-03-25T22:26:07.3519397Z    |
2020-03-25T22:26:07.3519590Z LL |     impl<T> Drop for Foo<T> {
---
2020-03-25T22:26:07.3528209Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-25T22:26:07.3528636Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-25T22:26:07.3528895Z 
2020-03-25T22:26:07.3528995Z 
2020-03-25T22:26:07.3533138Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-25T22:26:07.3535969Z 
2020-03-25T22:26:07.3536074Z 
2020-03-25T22:26:07.3536609Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-25T22:26:07.3537013Z Build completed unsuccessfully in 1:02:50
2020-03-25T22:26:07.3537013Z Build completed unsuccessfully in 1:02:50
2020-03-25T22:26:07.3537272Z == clock drift check ==
2020-03-25T22:26:07.3537529Z   local time: Wed Mar 25 22:26:07 UTC 2020
2020-03-25T22:26:07.9019636Z   network time: Wed, 25 Mar 2020 22:26:07 GMT
2020-03-25T22:26:07.9021900Z == end clock drift check ==
2020-03-25T22:26:08.3086852Z 
2020-03-25T22:26:08.3131313Z ##[error]Bash exited with code '1'.
2020-03-25T22:26:08.3146034Z ##[section]Finishing: Run build
2020-03-25T22:26:08.3198987Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-25T22:26:08.3204295Z Task         : Get sources
2020-03-25T22:26:08.3204655Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T22:26:08.3205005Z Version      : 1.0.0
2020-03-25T22:26:08.3205240Z Author       : Microsoft
2020-03-25T22:26:08.3205240Z Author       : Microsoft
2020-03-25T22:26:08.3205609Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-25T22:26:08.3206052Z ==============================================================================
2020-03-25T22:26:08.6544885Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-25T22:26:08.6588507Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-25T22:26:08.6681111Z Cleaning up task key
2020-03-25T22:26:08.6682413Z Start cleaning up orphan processes.
2020-03-25T22:26:08.6865795Z Terminate orphan process: pid (3771) (python)
2020-03-25T22:26:08.7021899Z ##[section]Finishing: Finalize Job
