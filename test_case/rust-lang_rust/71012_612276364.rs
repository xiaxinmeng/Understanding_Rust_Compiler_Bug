plain
2020-04-10T23:19:15.8958819Z ========================== Starting Command Output ===========================
2020-04-10T23:19:15.8961222Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fedaac29-6b56-453a-9424-18e1c731e70c.sh
2020-04-10T23:19:15.8961482Z 
2020-04-10T23:19:15.8964771Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T23:19:15.8982278Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71012/merge to s
2020-04-10T23:19:15.8985664Z Task         : Get sources
2020-04-10T23:19:15.8985974Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T23:19:15.8986277Z Version      : 1.0.0
2020-04-10T23:19:15.8986479Z Author       : Microsoft
---
2020-04-10T23:19:16.9008577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T23:19:16.9021419Z ##[command]git config gc.auto 0
2020-04-10T23:19:16.9027709Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T23:19:16.9033472Z ##[command]git config --get-all http.proxy
2020-04-10T23:19:16.9041071Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71012/merge:refs/remotes/pull/71012/merge
---
2020-04-10T23:22:28.6866058Z Looks like docker image is the same as before, not uploading
2020-04-10T23:22:36.8819002Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T23:22:36.9064647Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T23:22:36.9087177Z == clock drift check ==
2020-04-10T23:22:36.9094308Z   local time: Fri Apr 10 23:22:36 UTC 2020
2020-04-10T23:22:37.2260660Z   network time: Fri, 10 Apr 2020 23:22:37 GMT
2020-04-10T23:22:37.2285939Z Starting sccache server...
2020-04-10T23:22:37.3048516Z configure: processing command line
2020-04-10T23:22:37.3048821Z configure: 
2020-04-10T23:22:37.3049782Z configure: rust.dist-src        := False
---
2020-04-10T23:27:29.4556880Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T23:27:30.7808342Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T23:27:32.2689920Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T23:27:33.2215246Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T23:27:41.2344738Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T23:27:43.3225136Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T23:27:47.3781955Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T23:27:51.2478524Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T23:27:59.4539195Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T23:45:33.3662238Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T23:45:34.7011820Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T23:45:36.2315754Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T23:45:37.5033266Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T23:45:45.6739291Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T23:45:47.8929378Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T23:45:51.9156044Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T23:45:55.9893817Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T23:46:04.2200524Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T00:06:32.3075597Z .................................................................................................... 1700/9889
2020-04-11T00:06:36.1013907Z .................................................................................................... 1800/9889
2020-04-11T00:06:43.8456607Z .................................................................................................... 1900/9889
2020-04-11T00:06:50.9193301Z ....i............................................................................................... 2000/9889
2020-04-11T00:06:56.4641373Z ..............................................................................................iiiii. 2100/9889
2020-04-11T00:07:14.7085895Z .................................................................................................... 2300/9889
2020-04-11T00:07:16.5928819Z .................................................................................................... 2400/9889
2020-04-11T00:07:18.5635181Z .................................................................................................... 2500/9889
2020-04-11T00:07:23.6702900Z .................................................................................................... 2600/9889
---
2020-04-11T00:09:50.1144017Z .................................................................................................... 5100/9889
2020-04-11T00:09:56.7456218Z .................................................................................................... 5200/9889
2020-04-11T00:10:01.6080673Z ..............i..................................................................................... 5300/9889
2020-04-11T00:10:10.6136042Z .................................................................................................... 5400/9889
2020-04-11T00:10:15.3201816Z ...ii.ii........i...i............................................................................... 5500/9889
2020-04-11T00:10:22.0272502Z ................................................i................................................... 5700/9889
2020-04-11T00:10:30.2539181Z ....................................................................ii.............................. 5800/9889
2020-04-11T00:10:35.9699686Z .......i............................................................................................ 5900/9889
2020-04-11T00:10:41.0435273Z .................................................................................................... 6000/9889
2020-04-11T00:10:41.0435273Z .................................................................................................... 6000/9889
2020-04-11T00:10:50.0016963Z .................................................................................................... 6100/9889
2020-04-11T00:11:00.3533956Z .ii...i..ii...........i............................................................................. 6200/9889
2020-04-11T00:11:11.5906014Z .................................................................................................... 6400/9889
2020-04-11T00:11:14.5913030Z .................................................................................................... 6500/9889
2020-04-11T00:11:14.5913030Z .................................................................................................... 6500/9889
2020-04-11T00:11:24.1399912Z ...............................i..ii................................................................ 6600/9889
2020-04-11T00:11:40.6197110Z .................................................................................................... 6800/9889
2020-04-11T00:11:42.2704187Z ...............................i.................................................................... 6900/9889
2020-04-11T00:11:43.9295898Z .................................................................................................... 7000/9889
2020-04-11T00:11:45.8534043Z ......................................................................i............................. 7100/9889
---
2020-04-11T00:13:07.4338317Z .................................................................................................... 7800/9889
2020-04-11T00:13:10.5077943Z .................................................................................................... 7900/9889
2020-04-11T00:13:15.6892893Z .................................................................................................... 8000/9889
2020-04-11T00:13:20.8493564Z ....................................i............................................................... 8100/9889
2020-04-11T00:13:27.9223049Z ....................................................................................iiiiii.iiiii.i.. 8200/9889
2020-04-11T00:13:40.2162331Z ..............................i......i.............................................................. 8400/9889
2020-04-11T00:13:43.0385512Z .................................................................................................... 8500/9889
2020-04-11T00:13:51.7702632Z .................................................................................................... 8600/9889
2020-04-11T00:14:02.5872834Z .................................................................................................... 8700/9889
---
2020-04-11T00:15:40.9241184Z 
2020-04-11T00:15:40.9242090Z ---- [ui] ui/suggestions/impl-trait-with-missing-bounds.rs stdout ----
2020-04-11T00:15:40.9242396Z diff of stderr:
2020-04-11T00:15:40.9242532Z 
2020-04-11T00:15:40.9243622Z 5    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9244132Z 6 ...
2020-04-11T00:15:40.9244353Z 7 LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9245471Z +    |                --------------- required by this bound in `qux`
2020-04-11T00:15:40.9245723Z 9    |
2020-04-11T00:15:40.9246105Z 10    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9246643Z 11 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9246643Z 11 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9246899Z 
2020-04-11T00:15:40.9247635Z 20    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9248129Z 21 ...
2020-04-11T00:15:40.9248350Z 22 LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9249624Z +    |                --------------- required by this bound in `qux`
2020-04-11T00:15:40.9249876Z 24    |
2020-04-11T00:15:40.9250257Z 25    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9250861Z 26 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9250861Z 26 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9251115Z 
2020-04-11T00:15:40.9251858Z 35    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9252350Z 36 ...
2020-04-11T00:15:40.9252571Z 37 LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9253660Z +    |                --------------- required by this bound in `qux`
2020-04-11T00:15:40.9253910Z 39    |
2020-04-11T00:15:40.9254291Z 40    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9254830Z 41 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9254830Z 41 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9255079Z 
2020-04-11T00:15:40.9255819Z 50    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9256308Z 51 ...
2020-04-11T00:15:40.9256530Z 52 LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9257615Z +    |                --------------- required by this bound in `qux`
2020-04-11T00:15:40.9257866Z 54    |
2020-04-11T00:15:40.9258245Z 55    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9260377Z 56 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9260377Z 56 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9261658Z 
2020-04-11T00:15:40.9264116Z 65    |             ^^^^^^^^^^ `<impl Iterator + std::fmt::Debug as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9265406Z 66 ...
2020-04-11T00:15:40.9265646Z 67 LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9266836Z +    |                --------------- required by this bound in `qux`
2020-04-11T00:15:40.9267090Z 69    |
2020-04-11T00:15:40.9267510Z 70    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator + std::fmt::Debug as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9268093Z 71 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9268093Z 71 help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9268350Z 
2020-04-11T00:15:40.9268449Z 
2020-04-11T00:15:40.9268661Z The actual stderr differed from the expected stderr.
2020-04-11T00:15:40.9269445Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds/impl-trait-with-missing-bounds.stderr
2020-04-11T00:15:40.9270147Z To update references, rerun the tests and pass the `--bless` flag
2020-04-11T00:15:40.9270813Z To only update this specific test, also pass `--test-args suggestions/impl-trait-with-missing-bounds.rs`
2020-04-11T00:15:40.9271294Z error: 1 errors occurred comparing output.
2020-04-11T00:15:40.9271537Z status: exit code: 1
2020-04-11T00:15:40.9271537Z status: exit code: 1
2020-04-11T00:15:40.9273660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds/auxiliary"
2020-04-11T00:15:40.9282140Z ------------------------------------------
2020-04-11T00:15:40.9282473Z 
2020-04-11T00:15:40.9284737Z ------------------------------------------
2020-04-11T00:15:40.9284967Z stderr:
2020-04-11T00:15:40.9284967Z stderr:
2020-04-11T00:15:40.9285342Z ------------------------------------------
2020-04-11T00:15:40.9285982Z error[E0277]: `<impl Iterator as std::iter::Iterator>::Item` doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9286695Z   --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:6:13
2020-04-11T00:15:40.9286981Z    |
2020-04-11T00:15:40.9287161Z LL |         qux(constraint);
2020-04-11T00:15:40.9287961Z    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9288435Z ...
2020-04-11T00:15:40.9288667Z LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9289426Z    |
2020-04-11T00:15:40.9289818Z    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9290326Z help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9290593Z    |
2020-04-11T00:15:40.9290593Z    |
2020-04-11T00:15:40.9290958Z LL | fn foo<I: Iterator>(constraints: I) where <I as std::iter::Iterator>::Item: std::fmt::Debug  {
2020-04-11T00:15:40.9292911Z 
2020-04-11T00:15:40.9293590Z error[E0277]: `<impl Iterator as std::iter::Iterator>::Item` doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9294312Z   --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:14:13
2020-04-11T00:15:40.9294604Z    |
2020-04-11T00:15:40.9294604Z    |
2020-04-11T00:15:40.9294781Z LL |         qux(constraint);
2020-04-11T00:15:40.9295574Z    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9296041Z ...
2020-04-11T00:15:40.9296253Z LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9297028Z    |
2020-04-11T00:15:40.9297398Z    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9297917Z help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9298184Z    |
2020-04-11T00:15:40.9298184Z    |
2020-04-11T00:15:40.9298603Z LL | fn bar<T, I: Iterator>(t: T, constraints: I) where T: std::fmt::Debug, <I as std::iter::Iterator>::Item: std::fmt::Debug  {
2020-04-11T00:15:40.9299566Z 
2020-04-11T00:15:40.9300116Z error[E0277]: `<impl Iterator as std::iter::Iterator>::Item` doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9300831Z   --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:22:13
2020-04-11T00:15:40.9301116Z    |
2020-04-11T00:15:40.9301116Z    |
2020-04-11T00:15:40.9301292Z LL |         qux(constraint);
2020-04-11T00:15:40.9302085Z    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9302548Z ...
2020-04-11T00:15:40.9302759Z LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9303528Z    |
2020-04-11T00:15:40.9303896Z    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9304550Z help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9304819Z    |
2020-04-11T00:15:40.9304819Z    |
2020-04-11T00:15:40.9305224Z LL | fn baz<I: Iterator>(t: impl std::fmt::Debug, constraints: I) where <I as std::iter::Iterator>::Item: std::fmt::Debug  {
2020-04-11T00:15:40.9306211Z 
2020-04-11T00:15:40.9306774Z error[E0277]: `<impl Iterator as std::iter::Iterator>::Item` doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9307487Z   --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:30:13
2020-04-11T00:15:40.9307770Z    |
2020-04-11T00:15:40.9307770Z    |
2020-04-11T00:15:40.9307946Z LL |         qux(constraint);
2020-04-11T00:15:40.9308733Z    |             ^^^^^^^^^^ `<impl Iterator as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9309192Z ...
2020-04-11T00:15:40.9309408Z LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9310179Z    |
2020-04-11T00:15:40.9310551Z    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9311072Z help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9311340Z    |
2020-04-11T00:15:40.9311340Z    |
2020-04-11T00:15:40.9311770Z LL | fn bat<I, T: std::fmt::Debug, U: Iterator>(t: T, constraints: U, _: I) where <U as std::iter::Iterator>::Item: std::fmt::Debug  {
2020-04-11T00:15:40.9312754Z 
2020-04-11T00:15:40.9313359Z error[E0277]: `<impl Iterator + std::fmt::Debug as std::iter::Iterator>::Item` doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9314111Z   --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:37:13
2020-04-11T00:15:40.9314398Z    |
2020-04-11T00:15:40.9314398Z    |
2020-04-11T00:15:40.9314574Z LL |         qux(constraint);
2020-04-11T00:15:40.9315431Z    |             ^^^^^^^^^^ `<impl Iterator + std::fmt::Debug as std::iter::Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-11T00:15:40.9315931Z ...
2020-04-11T00:15:40.9316142Z LL | fn qux(_: impl std::fmt::Debug) {}
2020-04-11T00:15:40.9316912Z    |
2020-04-11T00:15:40.9317320Z    = help: the trait `std::fmt::Debug` is not implemented for `<impl Iterator + std::fmt::Debug as std::iter::Iterator>::Item`
2020-04-11T00:15:40.9317878Z help: introduce a type parameter with a trait bound instead of using `impl Trait`
2020-04-11T00:15:40.9318145Z    |
2020-04-11T00:15:40.9318145Z    |
2020-04-11T00:15:40.9318533Z LL | fn bak<I: Iterator + std::fmt::Debug>(constraints: I) where <I as std::iter::Iterator>::Item: std::fmt::Debug  {
2020-04-11T00:15:40.9319431Z 
2020-04-11T00:15:40.9319623Z error: aborting due to 5 previous errors
2020-04-11T00:15:40.9319795Z 
2020-04-11T00:15:40.9320246Z For more information about this error, try `rustc --explain E0277`.
---
2020-04-11T00:15:40.9325210Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-11T00:15:40.9325635Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-11T00:15:40.9325965Z 
2020-04-11T00:15:40.9326064Z 
2020-04-11T00:15:40.9329908Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-11T00:15:40.9332665Z 
2020-04-11T00:15:40.9332766Z 
2020-04-11T00:15:40.9333293Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-11T00:15:40.9333673Z Build completed unsuccessfully in 0:51:18
2020-04-11T00:15:40.9333673Z Build completed unsuccessfully in 0:51:18
2020-04-11T00:15:40.9348826Z == clock drift check ==
2020-04-11T00:15:40.9371451Z   local time: Sat Apr 11 00:15:40 UTC 2020
2020-04-11T00:15:41.0081719Z   network time: Sat, 11 Apr 2020 00:15:41 GMT
2020-04-11T00:15:41.5110362Z 
2020-04-11T00:15:41.5110362Z 
2020-04-11T00:15:41.5178323Z ##[error]Bash exited with code '1'.
2020-04-11T00:15:41.5191711Z ##[section]Finishing: Run build
2020-04-11T00:15:41.5243445Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71012/merge to s
2020-04-11T00:15:41.5248225Z Task         : Get sources
2020-04-11T00:15:41.5248551Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T00:15:41.5248847Z Version      : 1.0.0
2020-04-11T00:15:41.5249072Z Author       : Microsoft
2020-04-11T00:15:41.5249072Z Author       : Microsoft
2020-04-11T00:15:41.5249400Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T00:15:41.5249782Z ==============================================================================
2020-04-11T00:15:41.8246261Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T00:15:41.8293778Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71012/merge to s
2020-04-11T00:15:41.8379832Z Cleaning up task key
2020-04-11T00:15:41.8381084Z Start cleaning up orphan processes.
2020-04-11T00:15:41.8538597Z Terminate orphan process: pid (3596) (python)
2020-04-11T00:15:41.8755759Z ##[section]Finishing: Finalize Job
