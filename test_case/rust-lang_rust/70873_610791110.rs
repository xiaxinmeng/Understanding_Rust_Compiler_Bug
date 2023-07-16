plain
2020-04-08T04:30:32.1041685Z ========================== Starting Command Output ===========================
2020-04-08T04:30:32.1045934Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/110ada18-8d96-4b2d-ab70-6adc75de3f2b.sh
2020-04-08T04:30:32.1046678Z 
2020-04-08T04:30:32.1054597Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T04:30:32.1078918Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70873/merge to s
2020-04-08T04:30:32.1083329Z Task         : Get sources
2020-04-08T04:30:32.1083661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T04:30:32.1083977Z Version      : 1.0.0
2020-04-08T04:30:32.1084192Z Author       : Microsoft
---
2020-04-08T04:30:33.1109945Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T04:30:33.1115284Z ##[command]git config gc.auto 0
2020-04-08T04:30:33.1119597Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T04:30:33.1122989Z ##[command]git config --get-all http.proxy
2020-04-08T04:30:33.1129298Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70873/merge:refs/remotes/pull/70873/merge
---
2020-04-08T04:32:26.8198496Z Looks like docker image is the same as before, not uploading
2020-04-08T04:32:34.5751642Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-08T04:32:34.6058323Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-08T04:32:34.6079320Z == clock drift check ==
2020-04-08T04:32:34.6094624Z   local time: Wed Apr  8 04:32:34 UTC 2020
2020-04-08T04:32:34.6708522Z   network time: Wed, 08 Apr 2020 04:32:34 GMT
2020-04-08T04:32:34.6734283Z Starting sccache server...
2020-04-08T04:32:34.7665089Z configure: processing command line
2020-04-08T04:32:34.7665343Z configure: 
2020-04-08T04:32:34.7666355Z configure: rust.dist-src        := False
---
2020-04-08T04:46:23.8149398Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T04:46:25.7902609Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T04:46:26.9638815Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T04:46:34.2468946Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T04:46:43.9761357Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T04:46:50.3983660Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T04:46:56.7629510Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T04:47:02.9425018Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T04:47:14.1838650Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T05:20:52.9911160Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T05:20:56.4919325Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T05:20:58.2809543Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T05:21:13.2264590Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T05:21:27.3527917Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T05:21:39.0876945Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T05:21:50.0734110Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T05:22:00.7492222Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T05:22:19.9921409Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T06:26:23.8355030Z Testing rustbook src/doc/embedded-book
2020-04-08T06:26:25.3558292Z  finished in 1.519
2020-04-08T06:26:25.3563402Z Testing rustbook src/doc/edition-guide
2020-04-08T06:26:35.3654213Z  finished in 10.008
2020-04-08T06:27:54.3658662Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-08T06:27:54.3660074Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-08T06:27:54.3661119Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-08T06:27:54.3662141Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2457`
2020-04-08T06:27:54.3663293Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-08T06:27:54.3664230Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-08T06:27:54.3665528Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47732`
2020-04-08T06:27:54.3666494Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-08T06:27:54.3667456Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/51934`
2020-04-08T06:27:54.3668898Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/56245`
2020-04-08T06:27:54.3671498Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/RELEASES.md`
2020-04-08T06:27:54.3672414Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md`
2020-04-08T06:27:54.3673386Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-08T06:27:54.3674363Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-08T06:27:54.3675031Z Received 429 (TOO_MANY_REQUESTS) for link `https://gist.github.com/nikomatsakis/631ec8b4af9a18b5d062d9d9b7d3d967`
2020-04-08T06:27:54.3676460Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-08T06:27:54.3677599Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-08T06:27:54.3678887Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs`
2020-04-08T06:27:54.3680078Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs`
2020-04-08T06:27:54.3681068Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-08T06:27:54.3681925Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues`
2020-04-08T06:27:54.3682669Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/15702`
2020-04-08T06:27:54.3683469Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/doc/unstable-book`
2020-04-08T06:27:54.3684240Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/reference`
2020-04-08T06:27:54.3684988Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/book`
2020-04-08T06:27:54.3685768Z Received 429 (TOO_MANY_REQUESTS) for link `***-by-example`
2020-04-08T06:27:54.3686524Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/32409`
2020-04-08T06:27:54.3687252Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc`
2020-04-08T06:27:54.3688155Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md`
2020-04-08T06:27:54.3689043Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/kennytm/rustup-toolchain-install-master`
2020-04-08T06:27:54.3689884Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-08T06:27:54.3690564Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme`
2020-04-08T06:27:54.3691430Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md`
2020-04-08T06:27:54.3692290Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-08T06:27:54.3693086Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/blob/master/collector/README.md`
2020-04-08T06:27:54.3693970Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/tree/master/collector/benchmarks`
2020-04-08T06:27:54.3694732Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-08T06:27:54.3695508Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-08T06:27:54.3696307Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/fmt-rfcs`
2020-04-08T06:27:54.3697305Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs`
2020-04-08T06:27:54.3698321Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rls`
2020-04-08T06:27:54.3699088Z Received 429 (TOO_MANY_REQUESTS) for link `***fix`
2020-04-08T06:27:54.3699837Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Assignment`
2020-04-08T06:27:54.3700642Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/140`
2020-04-08T06:27:54.3701427Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/221`
2020-04-08T06:27:54.3702226Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/`
2020-04-08T06:27:54.3703026Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Pinging`
2020-04-08T06:27:54.3703910Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/triagebot.toml`
2020-04-08T06:27:54.3704728Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-Cleanup-Crew`
2020-04-08T06:27:54.3705459Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/jethrogb/rust-reduce`
2020-04-08T06:27:54.3706283Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-08T06:27:54.3707091Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-08T06:27:54.3707890Z Received 429 (TOO_MANY_REQUESTS) for link `***/`
2020-04-08T06:27:54.3708631Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-LLVM`
2020-04-08T06:27:54.3709543Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-APACHE`
2020-04-08T06:27:54.3710402Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-MIT`
2020-04-08T06:27:54.3711207Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/COPYRIGHT`
2020-04-08T06:27:54.3711975Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-08T06:27:54.3712729Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide`
2020-04-08T06:27:54.3713525Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_metadata`
2020-04-08T06:27:54.3714419Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_middle/dep_graph`
2020-04-08T06:27:54.3715211Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42678`
2020-04-08T06:27:54.3765163Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md`
2020-04-08T06:27:54.3766429Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42293`
2020-04-08T06:27:54.3767226Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42633`
2020-04-08T06:27:54.3767928Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/salsa-rs/salsa`
2020-04-08T06:27:54.3768877Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/blob/master/examples/rustc-driver-example.rs`
2020-04-08T06:27:54.3769752Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustdoc`
2020-04-08T06:27:54.3770662Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/tools/rustdoc`
2020-04-08T06:27:54.3771474Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/44136`
2020-04-08T06:27:54.3772279Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_ast`
2020-04-08T06:27:54.3773113Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_expand/mbe`
2020-04-08T06:27:54.3773911Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/41710`
2020-04-08T06:27:54.3774860Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs`
2020-04-08T06:27:54.3775863Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_macros/src/type_foldable.rs`
2020-04-08T06:27:54.3776909Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-08T06:27:54.3778002Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-08T06:27:54.3779337Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-08T06:27:54.3780229Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/22019`
2020-04-08T06:27:54.3780959Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/18290`
2020-04-08T06:27:54.3781711Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48416`
2020-04-08T06:27:54.3782452Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48416`
2020-04-08T06:27:54.3783240Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/tree/master/chalk-engine`
2020-04-08T06:27:54.3784143Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_traits`
2020-04-08T06:27:54.3785029Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_middle/traits/mod.rs`
2020-04-08T06:27:54.3785879Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-ir/src/lib.rs`
2020-04-08T06:27:54.3786850Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_middle/ty/sty.rs`
2020-04-08T06:27:54.3787715Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs`
2020-04-08T06:27:54.3788652Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2089-implied-bounds.md`
2020-04-08T06:27:54.3789656Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/43786`
2020-04-08T06:27:54.3790447Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/69247`
2020-04-08T06:27:54.3791272Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/clauses.rs`
2020-04-08T06:27:54.3792202Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_traits`
2020-04-08T06:27:54.3793047Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/wf.rs`
2020-04-08T06:27:54.3793984Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/tests/test/wf_lowering.rs`
2020-04-08T06:27:54.3795033Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/239e4ae4e69b2785b5f99e0f2b41fc16b0b4e65e/chalk-engine/src/README.md`
2020-04-08T06:27:54.3796220Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-08T06:27:54.3797014Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/55835`
2020-04-08T06:27:54.3797768Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47828`
2020-04-08T06:27:54.3798485Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62474`
2020-04-08T06:27:54.3799887Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62592`
2020-04-08T06:27:54.3800615Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/`
2020-04-08T06:27:54.3801522Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-08T06:27:54.3802974Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-08T06:27:54.3803922Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-08T06:27:54.3804833Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/47809`
2020-04-08T06:27:54.3805666Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-08T06:27:54.3806629Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs`
2020-04-08T06:27:54.3807556Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-08T06:27:54.3808414Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile`
2020-04-08T06:27:54.3809357Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/test/run-make-fulldeps`
2020-04-08T06:27:54.3810528Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/test/codegen/pgo-instrumentation.rs`
2020-04-08T06:27:54.3811371Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt`
2020-04-08T06:27:54.3812422Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs`
2020-04-08T06:27:54.3813484Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs`
2020-04-08T06:27:54.3815051Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs`
2020-04-08T06:27:54.3816307Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs`
2020-04-08T06:27:54.3817425Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td`
2020-04-08T06:27:54.3818668Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs`
2020-04-08T06:27:54.3819779Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs`
2020-04-08T06:27:54.3820560Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/google/sanitizers/wiki/`
2020-04-08T06:27:54.3840917Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/gdb`
2020-04-08T06:27:54.3842049Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-08T06:27:54.3842857Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-08T06:27:54.3848137Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb`
2020-04-08T06:27:54.3848951Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb/wiki`
2020-04-08T06:27:54.3849817Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/34457`
2020-04-08T06:27:54.3850590Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/33014`
2020-04-08T06:27:54.3851313Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2117`
2020-04-08T06:27:54.3852169Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-08T06:27:54.3852996Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustdoc/core.rs`
2020-04-08T06:27:54.3854637Z 
2020-04-08T06:27:54.3855084Z     ┌── walkthrough.md:74:64 ───
2020-04-08T06:27:54.3855471Z     │
2020-04-08T06:27:54.3855471Z     │
2020-04-08T06:27:54.3856049Z  74 │ > You can find the official guidelines for when to open an RFC [here][rfcwhen].
2020-04-08T06:27:54.3857849Z     │
2020-04-08T06:27:54.3858008Z 
2020-04-08T06:27:54.3858625Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-04-08T06:27:54.3858971Z 
2020-04-08T06:27:54.3858971Z 
2020-04-08T06:27:54.3859376Z     ┌── walkthrough.md:83:1 ───
2020-04-08T06:27:54.3864716Z     │
2020-04-08T06:27:54.3865298Z  83 │ [rust-lang/rfcs](https://github.com/rust-lang/rfcs) repo on GitHub. You can
2020-04-08T06:27:54.3866718Z     │
2020-04-08T06:27:54.3866854Z 
2020-04-08T06:27:54.3867471Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-04-08T06:27:54.3867818Z 
2020-04-08T06:27:54.3867818Z 
2020-04-08T06:27:54.3868247Z     ┌── walkthrough.md:85:1 ───
2020-04-08T06:27:54.3868624Z     │
2020-04-08T06:27:54.3869153Z  85 │ [README](https://github.com/rust-lang/rfcs#what-the-process-is).
2020-04-08T06:27:54.3870736Z     │
2020-04-08T06:27:54.3870866Z 
2020-04-08T06:27:54.3871537Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2457"
2020-04-08T06:27:54.3871905Z 
2020-04-08T06:27:54.3871905Z 
2020-04-08T06:27:54.3872325Z      ┌── walkthrough.md:107:51 ───
2020-04-08T06:27:54.3875270Z      │
2020-04-08T06:27:54.3879791Z  107 │ ideas, a lot more discussion can happen (e.g. see [this RFC][nonascii] which
2020-04-08T06:27:54.3881236Z      │
2020-04-08T06:27:54.3881399Z 
2020-04-08T06:27:54.3882064Z error: The server responded with 429 Too Many Requests for "***"
2020-04-08T06:27:54.3882318Z 
---
2020-04-08T06:27:54.3887541Z  156 │ To make a change to the compiler, open a PR against the [rust-lang/rust] repo.
2020-04-08T06:27:54.3888418Z      │                                                         ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3888942Z      │
2020-04-08T06:27:54.3889094Z 
2020-04-08T06:27:54.3889769Z error: The server responded with 429 Too Many Requests for "***/pull/47732"
2020-04-08T06:27:54.3892400Z      ┌── walkthrough.md:167:58 ───
2020-04-08T06:27:54.3893206Z      │
2020-04-08T06:27:54.3893757Z  167 │ macro expansion in the compiler. Personally, I find that [improving the
2020-04-08T06:27:54.3894592Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3894592Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3895118Z      │
2020-04-08T06:27:54.3895249Z 
2020-04-08T06:27:54.3895902Z error: The server responded with 429 Too Many Requests for "***"
2020-04-08T06:27:54.3896180Z 
2020-04-08T06:27:54.3896608Z      ┌── walkthrough.md:181:27 ───
2020-04-08T06:27:54.3897007Z      │
2020-04-08T06:27:54.3897583Z  181 │ When you open a PR on the [rust-lang/rust], a bot will assign your PR to a
2020-04-08T06:27:54.3898736Z      │
2020-04-08T06:27:54.3898867Z 
2020-04-08T06:27:54.3898867Z 
2020-04-08T06:27:54.3899560Z error: The server responded with 429 Too Many Requests for "***/issues/51934"
2020-04-08T06:27:54.3900262Z      ┌── walkthrough.md:237:32 ───
2020-04-08T06:27:54.3900670Z      │
2020-04-08T06:27:54.3901134Z  237 │   from the original RFC required [another
2020-04-08T06:27:54.3901671Z      │ ╭────────────────────────────────^
2020-04-08T06:27:54.3901671Z      │ ╭────────────────────────────────^
2020-04-08T06:27:54.3902290Z  238 │ │ FCP](***/issues/51934).
2020-04-08T06:27:54.3903540Z      │
2020-04-08T06:27:54.3903669Z 
2020-04-08T06:27:54.3903669Z 
2020-04-08T06:27:54.3904347Z error: The server responded with 429 Too Many Requests for "***/pull/56245"
2020-04-08T06:27:54.3905073Z      ┌── walkthrough.md:257:13 ───
2020-04-08T06:27:54.3905461Z      │
2020-04-08T06:27:54.3905461Z      │
2020-04-08T06:27:54.3906040Z  257 │ After this, [a PR is made][stab] to remove the feature gate, enabling the feature by
2020-04-08T06:27:54.3907168Z      │
2020-04-08T06:27:54.3907299Z 
2020-04-08T06:27:54.3907299Z 
2020-04-08T06:27:54.3908015Z error: The server responded with 429 Too Many Requests for "***/blob/master/RELEASES.md"
2020-04-08T06:27:54.3909005Z      ┌── walkthrough.md:258:55 ───
2020-04-08T06:27:54.3909394Z      │
2020-04-08T06:27:54.3909394Z      │
2020-04-08T06:27:54.3909988Z  258 │ default (on the 2018 edition). A note is added to the [Release notes][relnotes]
2020-04-08T06:27:54.3911299Z      │
2020-04-08T06:27:54.3911429Z 
2020-04-08T06:27:54.3912206Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md"
2020-04-08T06:27:54.3912655Z 
---
2020-04-08T06:27:54.3927488Z  81 │ example of how such an issue should look can be [found
2020-04-08T06:27:54.3928223Z     │                                                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3928728Z     │
2020-04-08T06:27:54.3928857Z 
2020-04-08T06:27:54.3930098Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-04-08T06:27:54.3931021Z      ┌── bug-fix-procedure.md:237:65 ───
2020-04-08T06:27:54.3931471Z      │
2020-04-08T06:27:54.3932002Z  237 │ The first reference you will likely find is the lint definition [in
2020-04-08T06:27:54.3932790Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3932790Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3933344Z      │
2020-04-08T06:27:54.3933476Z 
2020-04-08T06:27:54.3934363Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-04-08T06:27:54.3935240Z      ┌── bug-fix-procedure.md:252:13 ───
2020-04-08T06:27:54.3935643Z      │
2020-04-08T06:27:54.3935643Z      │
2020-04-08T06:27:54.3936177Z  252 │ the file as [part of a `lint_array!`][lintarraysource]; remove it too,
2020-04-08T06:27:54.3937275Z      │
2020-04-08T06:27:54.3937404Z 
2020-04-08T06:27:54.3937404Z 
2020-04-08T06:27:54.3938280Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs"
2020-04-08T06:27:54.3939380Z      ┌── bug-fix-procedure.md:256:19 ───
2020-04-08T06:27:54.3939810Z      │
2020-04-08T06:27:54.3940330Z  256 │ Next, you see see [a reference to `OVERLAPPING_INHERENT_IMPLS` in
2020-04-08T06:27:54.3940974Z      │                   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3940974Z      │                   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3941439Z      │
2020-04-08T06:27:54.3941588Z 
2020-04-08T06:27:54.3942506Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs"
2020-04-08T06:27:54.3943404Z      ┌── bug-fix-procedure.md:285:16 ───
2020-04-08T06:27:54.3943806Z      │
2020-04-08T06:27:54.3943806Z      │
2020-04-08T06:27:54.3944342Z  285 │ this case, the [`add_lint` call][addlintsource] looks like this:
2020-04-08T06:27:54.3945437Z      │
2020-04-08T06:27:54.3945566Z 
2020-04-08T06:27:54.3946328Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md"
2020-04-08T06:27:54.3946759Z 
2020-04-08T06:27:54.3946759Z 
2020-04-08T06:27:54.3947209Z     ┌── implementing_new_features.md:56:4 ───
2020-04-08T06:27:54.3947646Z     │
2020-04-08T06:27:54.3948169Z  56 │ We [value the stability of Rust]. Code that works and runs on stable
2020-04-08T06:27:54.3948884Z     │    ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3949368Z     │
2020-04-08T06:27:54.3949500Z 
2020-04-08T06:27:54.3950163Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-04-08T06:27:54.3950867Z     ┌── stability.md:18:51 ───
2020-04-08T06:27:54.3951239Z     │
2020-04-08T06:27:54.3951791Z  18 │ The `issue` field specifies the associated GitHub [issue number]. This field is
2020-04-08T06:27:54.3952577Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3952577Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3953095Z     │
2020-04-08T06:27:54.3953223Z 
2020-04-08T06:27:54.3953913Z error: The server responded with 429 Too Many Requests for "***/issues/15702"
2020-04-08T06:27:54.3954601Z     ┌── stability.md:31:30 ───
2020-04-08T06:27:54.3954991Z     │
2020-04-08T06:27:54.3954991Z     │
2020-04-08T06:27:54.3955548Z  31 │ Note, however, that due to a [rustc bug], stable items inside unstable modules
2020-04-08T06:27:54.3957129Z     │
2020-04-08T06:27:54.3957258Z 
2020-04-08T06:27:54.3957258Z 
2020-04-08T06:27:54.3958006Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/doc/unstable-book"
2020-04-08T06:27:54.3958797Z     ┌── stabilization_guide.md:17:38 ───
2020-04-08T06:27:54.3959197Z     │
2020-04-08T06:27:54.3959197Z     │
2020-04-08T06:27:54.3959707Z  17 │ in the [`Unstable Book`], located at [`src/doc/unstable-book`].
2020-04-08T06:27:54.3960924Z     │
2020-04-08T06:27:54.3961053Z 
2020-04-08T06:27:54.3961695Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/reference"
2020-04-08T06:27:54.3962051Z 
---
2020-04-08T06:27:54.3966889Z  28 │ - [The Book]: This may or may not need updating, depends.
2020-04-08T06:27:54.3967683Z     │   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3968118Z     │
2020-04-08T06:27:54.3968246Z 
2020-04-08T06:27:54.3968915Z error: The server responded with 429 Too Many Requests for "***-by-example"
2020-04-08T06:27:54.3969659Z     ┌── stabilization_guide.md:35:3 ───
2020-04-08T06:27:54.3970056Z     │
2020-04-08T06:27:54.3970480Z  35 │ - [Rust by Example]: As needed.
2020-04-08T06:27:54.3971059Z     │   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3971059Z     │   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3971468Z     │
2020-04-08T06:27:54.3971596Z 
2020-04-08T06:27:54.3972354Z error: The server responded with 429 Too Many Requests for "***/issues/32409"
2020-04-08T06:27:54.3973085Z     ┌── stabilization_guide.md:97:1 ───
2020-04-08T06:27:54.3973500Z     │
2020-04-08T06:27:54.3973891Z  97 │ [rust-lang/rust#32409]:
2020-04-08T06:27:54.3974388Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3974388Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3974808Z     │
2020-04-08T06:27:54.3974951Z 
2020-04-08T06:27:54.3975610Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/cargo-bisect-rustc"
2020-04-08T06:27:54.3975985Z 
2020-04-08T06:27:54.3976437Z      ┌── compiler-debugging.md:257:5 ───
2020-04-08T06:27:54.3976842Z      │
2020-04-08T06:27:54.3977394Z  257 │ The [cargo-bisect-rustc][bisect] tool can be used as a quick and easy way to
2020-04-08T06:27:54.3978640Z      │
2020-04-08T06:27:54.3978772Z 
2020-04-08T06:27:54.3979498Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md"
2020-04-08T06:27:54.3979945Z 
2020-04-08T06:27:54.3979945Z 
2020-04-08T06:27:54.3980388Z      ┌── compiler-debugging.md:261:31 ───
2020-04-08T06:27:54.3980790Z      │
2020-04-08T06:27:54.3981341Z  261 │ on *why* it was changed.  See [this tutorial][bisect-tutorial] on how to use
2020-04-08T06:27:54.3982551Z      │
2020-04-08T06:27:54.3982681Z 
2020-04-08T06:27:54.3983388Z error: The server responded with 429 Too Many Requests for "https://github.com/kennytm/rustup-toolchain-install-master"
2020-04-08T06:27:54.3983786Z 
2020-04-08T06:27:54.3983786Z 
2020-04-08T06:27:54.3984217Z      ┌── compiler-debugging.md:269:5 ───
2020-04-08T06:27:54.3984637Z      │
2020-04-08T06:27:54.3985200Z  269 │ The [rustup-toolchain-install-master][rtim] tool by kennytm can be used to
2020-04-08T06:27:54.3986267Z      │
2020-04-08T06:27:54.3986396Z 
2020-04-08T06:27:54.3986396Z 
2020-04-08T06:27:54.3987048Z error: The server responded with 429 Too Many Requests for "***c-perf"
2020-04-08T06:27:54.3987732Z    ┌── profiling.md:8:9 ───
2020-04-08T06:27:54.3988094Z    │
2020-04-08T06:27:54.3988094Z    │
2020-04-08T06:27:54.3988893Z  8 │   - The [rustc-perf](***c-perf) project makes this easy and can be triggered to run on a PR via the `@rustc-perf` bot.
2020-04-08T06:27:54.3990329Z    │
2020-04-08T06:27:54.3990457Z 
2020-04-08T06:27:54.3991100Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme"
2020-04-08T06:27:54.3991458Z 
2020-04-08T06:27:54.3991458Z 
2020-04-08T06:27:54.3991855Z     ┌── profiling.md:11:35 ───
2020-04-08T06:27:54.3992260Z     │
2020-04-08T06:27:54.3993027Z  11 │   - The `-Zself-profile` flag and [measureme](https://github.com/rust-lang/measureme) tools offer a query-based approach to profiling.
2020-04-08T06:27:54.3994623Z     │
2020-04-08T06:27:54.3994752Z 
2020-04-08T06:27:54.3995473Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md"
2020-04-08T06:27:54.3996471Z 
2020-04-08T06:27:54.3996471Z 
2020-04-08T06:27:54.3996989Z     ┌── profiling.md:12:9 ───
2020-04-08T06:27:54.3997365Z     │
2020-04-08T06:27:54.3998050Z  12 │     See [their docs](https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md) for more information.
2020-04-08T06:27:54.3999049Z     │         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.3999653Z     │
2020-04-08T06:27:54.3999783Z 
2020-04-08T06:27:54.4000470Z error: The server responded with 429 Too Many Requests for "***c-perf"
2020-04-08T06:27:54.4001174Z     ┌── profiling/with_perf.md:59:1 ───
2020-04-08T06:27:54.4001579Z     │
2020-04-08T06:27:54.4002030Z  59 │ [the rustc-perf repository][rustc-perf-gh]:
2020-04-08T06:27:54.4002572Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4002572Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4002973Z     │
2020-04-08T06:27:54.4003121Z 
2020-04-08T06:27:54.4003889Z error: The server responded with 429 Too Many Requests for "***c-perf/blob/master/collector/README.md"
2020-04-08T06:27:54.4004669Z     ┌── profiling/with_perf.md:71:1 ───
2020-04-08T06:27:54.4005057Z     │
2020-04-08T06:27:54.4005552Z  71 │ [instructions in the rustc-perf readme][rustc-perf-readme].
2020-04-08T06:27:54.4006146Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4006146Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4006552Z     │
2020-04-08T06:27:54.4006681Z 
2020-04-08T06:27:54.4008406Z error: The server responded with 429 Too Many Requests for "***c-perf/tree/master/collector/benchmarks"
2020-04-08T06:27:54.4009273Z     ┌── profiling/with_perf.md:93:14 ───
2020-04-08T06:27:54.4009663Z     │
2020-04-08T06:27:54.4009663Z     │
2020-04-08T06:27:54.4010216Z  93 │ are found in [the `collector/benchmarks` directory][dir]. So let's go
2020-04-08T06:27:54.4011275Z     │
2020-04-08T06:27:54.4011423Z 
2020-04-08T06:27:54.4012081Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-04-08T06:27:54.4012447Z 
2020-04-08T06:27:54.4012447Z 
2020-04-08T06:27:54.4012879Z      ┌── profiling/with_perf.md:137:45 ───
2020-04-08T06:27:54.4013299Z      │
2020-04-08T06:27:54.4013823Z  137 │ helpful. For more detailed examination, the [`perf-focus` tool][pf]
2020-04-08T06:27:54.4015089Z      │
2020-04-08T06:27:54.4015219Z 
2020-04-08T06:27:54.4015855Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-04-08T06:27:54.4016220Z 
2020-04-08T06:27:54.4016220Z 
2020-04-08T06:27:54.4016673Z      ┌── profiling/with_perf.md:161:38 ───
2020-04-08T06:27:54.4017066Z      │
2020-04-08T06:27:54.4017596Z  161 │ about it. For this, I personally use [perf focus][pf]. It's a kind of
2020-04-08T06:27:54.4018831Z      │
2020-04-08T06:27:54.4018961Z 
2020-04-08T06:27:54.4019619Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-dev-tools/fmt-rfcs"
2020-04-08T06:27:54.4019983Z 
2020-04-08T06:27:54.4019983Z 
2020-04-08T06:27:54.4020392Z     ┌── conventions.md:10:36 ───
2020-04-08T06:27:54.4020770Z     │
2020-04-08T06:27:54.4021317Z  10 │ rustc is slowly moving towards the [Rust standard coding style][fmt];
2020-04-08T06:27:54.4022653Z     │
2020-04-08T06:27:54.4022804Z 
2020-04-08T06:27:54.4022804Z 
2020-04-08T06:27:54.4023687Z error: The server responded with 429 Too Many Requests for "***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs"
2020-04-08T06:27:54.4024519Z     ┌── crates-io.md:19:23 ───
2020-04-08T06:27:54.4024895Z     │
2020-04-08T06:27:54.4024895Z     │
2020-04-08T06:27:54.4025426Z  19 │ The `tidy` tool has a [whitelist] of crates that are allowed. To add a
2020-04-08T06:27:54.4026763Z     │
2020-04-08T06:27:54.4026890Z 
2020-04-08T06:27:54.4027497Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rls"
2020-04-08T06:27:54.4027862Z 
2020-04-08T06:27:54.4027862Z 
2020-04-08T06:27:54.4028273Z     ┌── diagnostics.md:81:63 ───
2020-04-08T06:27:54.4028650Z     │
2020-04-08T06:27:54.4032492Z  81 │ is passed) as JSON for consumption by tools, most notably the [Rust Language
2020-04-08T06:27:54.4033334Z     │                                                               ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4033861Z     │
2020-04-08T06:27:54.4034011Z 
2020-04-08T06:27:54.4034681Z error: The server responded with 429 Too Many Requests for "***fix"
2020-04-08T06:27:54.4035360Z     ┌── diagnostics.md:82:18 ───
2020-04-08T06:27:54.4036193Z     │
2020-04-08T06:27:54.4036193Z     │
2020-04-08T06:27:54.4036680Z  82 │ Server][rls] and [`rustfix`][rustfix].
2020-04-08T06:27:54.4037757Z     │
2020-04-08T06:27:54.4037885Z 
2020-04-08T06:27:54.4038559Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/triagebot/wiki/Assignment"
2020-04-08T06:27:54.4038951Z 
---
2020-04-08T06:27:54.4063253Z  57 │ [rustbot] a [`ping`] command with the name of the ICE-breakers
2020-04-08T06:27:54.4063870Z     │             ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4064294Z     │
2020-04-08T06:27:54.4064443Z 
2020-04-08T06:27:54.4065298Z error: The server responded with 429 Too Many Requests for "***/blob/master/triagebot.toml"
2020-04-08T06:27:54.4066107Z     ┌── ice-breaker/about.md:66:16 ───
2020-04-08T06:27:54.4066489Z     │
2020-04-08T06:27:54.4066489Z     │
2020-04-08T06:27:54.4066964Z  66 │ defined in the [`triagebot.toml`] file. For example:
2020-04-08T06:27:54.4068026Z     │
2020-04-08T06:27:54.4068154Z 
2020-04-08T06:27:54.4068154Z 
2020-04-08T06:27:54.4069056Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-Cleanup-Crew"
2020-04-08T06:27:54.4069891Z    ┌── ice-breaker/cleanup-crew.md:3:19 ───
2020-04-08T06:27:54.4070287Z    │
2020-04-08T06:27:54.4070765Z  3 │ **Github Label:** [ICEBreaker-Cleanup-Crew]
2020-04-08T06:27:54.4071368Z    │                   ^ Server responded with 429 Too Many Requests
---
2020-04-08T06:27:54.4082629Z  80 │ To learn to use [cargo-bisect-rustc], check out [this blog
2020-04-08T06:27:54.4083266Z     │                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4083707Z     │
2020-04-08T06:27:54.4083855Z 
2020-04-08T06:27:54.4084510Z error: The server responded with 429 Too Many Requests for "***/"
2020-04-08T06:27:54.4085247Z      ┌── ice-breaker/cleanup-crew.md:102:36 ───
2020-04-08T06:27:54.4085658Z      │
2020-04-08T06:27:54.4086169Z  102 │ 1. Go to an update checkout of the [rust-lang/rust] repository
2020-04-08T06:27:54.4086881Z      │                                    ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4086881Z      │                                    ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4087590Z      │
2020-04-08T06:27:54.4087720Z 
2020-04-08T06:27:54.4088457Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-LLVM"
2020-04-08T06:27:54.4089586Z    ┌── ice-breaker/llvm.md:3:19 ───
2020-04-08T06:27:54.4090015Z    │
2020-04-08T06:27:54.4090473Z  3 │ **Github Label:** [ICEBreaker-LLVM]
2020-04-08T06:27:54.4091665Z    │                   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4091665Z    │                   ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4092134Z    │
2020-04-08T06:27:54.4092263Z 
2020-04-08T06:27:54.4093022Z error: The server responded with 429 Too Many Requests for "***/blob/master/LICENSE-APACHE"
2020-04-08T06:27:54.4093729Z    ┌── licenses.md:3:78 ───
2020-04-08T06:27:54.4094116Z    │
2020-04-08T06:27:54.4094116Z    │
2020-04-08T06:27:54.4095776Z  3 │ The `rustc` compiler source and standard library are dual licensed under the [Apache License v2.0](***/blob/master/LICENSE-APACHE) and the [MIT License](***/blob/master/LICENSE-MIT) unless otherwise specified.
2020-04-08T06:27:54.4098515Z    │
2020-04-08T06:27:54.4098685Z 
2020-04-08T06:27:54.4098685Z 
2020-04-08T06:27:54.4099418Z error: The server responded with 429 Too Many Requests for "***/blob/master/LICENSE-MIT"
2020-04-08T06:27:54.4100400Z    ┌── licenses.md:3:170 ───
2020-04-08T06:27:54.4101673Z    │
2020-04-08T06:27:54.4101673Z    │
2020-04-08T06:27:54.4105356Z  3 │ The `rustc` compiler source and standard library are dual licensed under the [Apache License v2.0](***/blob/master/LICENSE-APACHE) and the [MIT License](***/blob/master/LICENSE-MIT) unless otherwise specified.
2020-04-08T06:27:54.4109488Z    │
2020-04-08T06:27:54.4109929Z 
2020-04-08T06:27:54.4109929Z 
2020-04-08T06:27:54.4111239Z error: The server responded with 429 Too Many Requests for "***/blob/master/COPYRIGHT"
2020-04-08T06:27:54.4112975Z    ┌── licenses.md:5:52 ───
2020-04-08T06:27:54.4113361Z    │
2020-04-08T06:27:54.4113361Z    │
2020-04-08T06:27:54.4114982Z  5 │ Detailed licensing information is available in the [COPYRIGHT document](***/blob/master/COPYRIGHT) of the `rust-lang/rust` repository.
2020-04-08T06:27:54.4117087Z    │
2020-04-08T06:27:54.4117216Z 
2020-04-08T06:27:54.4117216Z 
2020-04-08T06:27:54.4117914Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/issues"
2020-04-08T06:27:54.4118662Z    ┌── part-2-intro.md:8:25 ───
2020-04-08T06:27:54.4119035Z    │
2020-04-08T06:27:54.4119474Z  8 │   to file an issue on the [rustc-dev-guide
2020-04-08T06:27:54.4120001Z    │ ╭─────────────────────────^
2020-04-08T06:27:54.4120001Z    │ ╭─────────────────────────^
2020-04-08T06:27:54.4120658Z  9 │ │ repo](***c-dev-guide/issues) or contact the compiler
2020-04-08T06:27:54.4121986Z    │
2020-04-08T06:27:54.4122132Z 
2020-04-08T06:27:54.4122132Z 
2020-04-08T06:27:54.4122832Z error: The server responded with 429 Too Many Requests for "***c-dev-guide"
2020-04-08T06:27:54.4123511Z    ┌── overview.md:3:134 ───
2020-04-08T06:27:54.4123877Z    │
2020-04-08T06:27:54.4123877Z    │
2020-04-08T06:27:54.4126077Z  3 │ Coming soon!  Work is in progress on this chapter.  See ***c-dev-guide/pull/633 for the source and the [project README](***c-dev-guide) for local build instructions.
2020-04-08T06:27:54.4129426Z    │
2020-04-08T06:27:54.4129555Z 
2020-04-08T06:27:54.4129555Z 
2020-04-08T06:27:54.4130310Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_metadata"
2020-04-08T06:27:54.4131055Z      ┌── query.md:155:1 ───
2020-04-08T06:27:54.4131444Z      │
2020-04-08T06:27:54.4132002Z  155 │ [`rustc_metadata` crate][rustc_metadata], which loads the information
2020-04-08T06:27:54.4132609Z      │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4132609Z      │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4133016Z      │
2020-04-08T06:27:54.4133148Z 
2020-04-08T06:27:54.4133923Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_middle/dep_graph"
2020-04-08T06:27:54.4134731Z     ┌── queries/incremental-compilation.md:84:1 ───
2020-04-08T06:27:54.4135189Z     │
2020-04-08T06:27:54.4135189Z     │
2020-04-08T06:27:54.4135738Z  84 │ [`src/librustc_middle/dep_graph`][dep_graph]. Construction of the DAG is done
2020-04-08T06:27:54.4136765Z     │
2020-04-08T06:27:54.4136893Z 
2020-04-08T06:27:54.4136893Z 
2020-04-08T06:27:54.4137561Z error: The server responded with 429 Too Many Requests for "***/issues/42678"
2020-04-08T06:27:54.4138277Z    ┌── queries/profiling.md:8:9 ───
2020-04-08T06:27:54.4138774Z    │
2020-04-08T06:27:54.4138774Z    │
2020-04-08T06:27:54.4139430Z  8 │ address [issue 42678](***/issues/42678).
2020-04-08T06:27:54.4140712Z    │
2020-04-08T06:27:54.4140859Z 
2020-04-08T06:27:54.4141740Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md"
2020-04-08T06:27:54.4142284Z 
2020-04-08T06:27:54.4142284Z 
2020-04-08T06:27:54.4142715Z      ┌── queries/profiling.md:335:3 ───
2020-04-08T06:27:54.4143127Z      │
2020-04-08T06:27:54.4143984Z  335 │   [On-demand Rustc incremental design doc](https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md)
2020-04-08T06:27:54.4146085Z      │
2020-04-08T06:27:54.4146216Z 
2020-04-08T06:27:54.4146216Z 
2020-04-08T06:27:54.4146905Z error: The server responded with 429 Too Many Requests for "***/issues/42293"
2020-04-08T06:27:54.4147618Z      ┌── queries/profiling.md:337:3 ───
2020-04-08T06:27:54.4148005Z      │
2020-04-08T06:27:54.4148005Z      │
2020-04-08T06:27:54.4148680Z  337 │   ["Red/Green" dependency tracking in compiler](***/issues/42293)
2020-04-08T06:27:54.4150197Z      │
2020-04-08T06:27:54.4150327Z 
2020-04-08T06:27:54.4150327Z 
2020-04-08T06:27:54.4151526Z error: The server responded with 429 Too Many Requests for "***/issues/42633"
2020-04-08T06:27:54.4152385Z      ┌── queries/profiling.md:341:3 ───
2020-04-08T06:27:54.4153204Z      │
2020-04-08T06:27:54.4153204Z      │
2020-04-08T06:27:54.4153863Z  341 │ - [GitHub issue #42633](***/issues/42633)
2020-04-08T06:27:54.4155189Z      │
2020-04-08T06:27:54.4155320Z 
2020-04-08T06:27:54.4156259Z error: The server responded with 429 Too Many Requests for "https://github.com/salsa-rs/salsa"
2020-04-08T06:27:54.4156614Z 
2020-04-08T06:27:54.4156614Z 
2020-04-08T06:27:54.4157043Z    ┌── salsa.md:5:1 ───
2020-04-08T06:27:54.4157423Z    │
2020-04-08T06:27:54.4157881Z  5 │ [Salsa](https://github.com/salsa-rs/salsa).
2020-04-08T06:27:54.4159309Z    │
2020-04-08T06:27:54.4159436Z 
2020-04-08T06:27:54.4159436Z 
2020-04-08T06:27:54.4160254Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/blob/master/examples/rustc-driver-example.rs"
2020-04-08T06:27:54.4161067Z     ┌── rustc-driver.md:17:63 ───
2020-04-08T06:27:54.4161467Z     │
2020-04-08T06:27:54.4161467Z     │
2020-04-08T06:27:54.4162024Z  17 │ You can see a minimal example of how to use `rustc_interface` [here][example].
2020-04-08T06:27:54.4163371Z     │
2020-04-08T06:27:54.4163499Z 
2020-04-08T06:27:54.4163499Z 
2020-04-08T06:27:54.4164229Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustdoc"
2020-04-08T06:27:54.4164929Z    ┌── rustdoc.md:6:50 ───
2020-04-08T06:27:54.4165326Z    │
2020-04-08T06:27:54.4165326Z    │
2020-04-08T06:27:54.4165870Z  6 │ Rustdoc is implemented entirely within the crate [`librustdoc`][rd]. It runs
2020-04-08T06:27:54.4167135Z    │
2020-04-08T06:27:54.4167260Z 
2020-04-08T06:27:54.4167260Z 
2020-04-08T06:27:54.4167977Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/tools/rustdoc"
2020-04-08T06:27:54.4168836Z     ┌── rustdoc.md:26:22 ───
2020-04-08T06:27:54.4169264Z     │
2020-04-08T06:27:54.4169816Z  26 │ using the project in [`src/tools/rustdoc`][bin]. Note that literally all that
2020-04-08T06:27:54.4170513Z     │                      ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4170513Z     │                      ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4170956Z     │
2020-04-08T06:27:54.4171084Z 
2020-04-08T06:27:54.4171775Z error: The server responded with 429 Too Many Requests for "***/issues/44136"
2020-04-08T06:27:54.4172489Z      ┌── rustdoc.md:115:1 ───
2020-04-08T06:27:54.4172884Z      │
2020-04-08T06:27:54.4172884Z      │
2020-04-08T06:27:54.4173444Z  115 │ [we're trying to deprecate that][44136]. If you need finer-grain control over
2020-04-08T06:27:54.4174489Z      │
2020-04-08T06:27:54.4174619Z 
2020-04-08T06:27:54.4174619Z 
2020-04-08T06:27:54.4175333Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_ast"
2020-04-08T06:27:54.4176118Z     ┌── test-implementation.md:35:1 ───
2020-04-08T06:27:54.4176520Z     │
2020-04-08T06:27:54.4177057Z  35 │ [`librustc_ast` crate][librustc_ast]. Essentially, it's a fancy macro, that
2020-04-08T06:27:54.4177688Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4177688Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4178092Z     │
2020-04-08T06:27:54.4178220Z 
2020-04-08T06:27:54.4178971Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_expand/mbe"
2020-04-08T06:27:54.4179743Z     ┌── macro-expansion.md:13:1 ───
2020-04-08T06:27:54.4180152Z     │
2020-04-08T06:27:54.4180759Z  13 │ [`src/librustc_expand/mbe/`][code_dir]. This chapter aims to explain how macro
2020-04-08T06:27:54.4181394Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4181394Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4181821Z     │
2020-04-08T06:27:54.4181948Z 
2020-04-08T06:27:54.4182620Z error: The server responded with 429 Too Many Requests for "***/issues/41710"
2020-04-08T06:27:54.4183341Z     ┌── mir/passes.md:96:17 ───
2020-04-08T06:27:54.4183715Z     │
2020-04-08T06:27:54.4184153Z  96 │ alternatives in [rust-lang/rust#41710].
2020-04-08T06:27:54.4184754Z     │                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4184754Z     │                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4185193Z     │
2020-04-08T06:27:54.4185322Z 
2020-04-08T06:27:54.4186221Z error: The server responded with 429 Too Many Requests for "***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs"
2020-04-08T06:27:54.4187217Z      ┌── generics.md:131:1 ───
2020-04-08T06:27:54.4187618Z      │
2020-04-08T06:27:54.4187618Z      │
2020-04-08T06:27:54.4188245Z  131 │ [Here is an example of actually using `subst` in the compiler][substex].  The exact details are not
2020-04-08T06:27:54.4189333Z      │
2020-04-08T06:27:54.4189464Z 
2020-04-08T06:27:54.4189464Z 
2020-04-08T06:27:54.4190273Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_macros/src/type_foldable.rs"
2020-04-08T06:27:54.4191057Z     ┌── ty-fold.md:93:1 ───
2020-04-08T06:27:54.4191423Z     │
2020-04-08T06:27:54.4191423Z     │
2020-04-08T06:27:54.4192079Z  93 │ [here](***/blob/master/src/librustc_macros/src/type_foldable.rs).
2020-04-08T06:27:54.4193542Z     │
2020-04-08T06:27:54.4193669Z 
2020-04-08T06:27:54.4193669Z 
2020-04-08T06:27:54.4194571Z error: The server responded with 429 Too Many Requests for "***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs"
2020-04-08T06:27:54.4195422Z     ┌── ty-fold.md:95:46 ───
2020-04-08T06:27:54.4195967Z     │
2020-04-08T06:27:54.4195967Z     │
2020-04-08T06:27:54.4196460Z  95 │   **`subst`** In the case of substitutions the [actual
2020-04-08T06:27:54.4197073Z     │ ╭──────────────────────────────────────────────^
2020-04-08T06:27:54.4198112Z  96 │ │ folder](***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs#L440-L451)
2020-04-08T06:27:54.4200034Z     │
2020-04-08T06:27:54.4200163Z 
2020-04-08T06:27:54.4200163Z 
2020-04-08T06:27:54.4201066Z error: The server responded with 429 Too Many Requests for "***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs"
2020-04-08T06:27:54.4201900Z     ┌── ty-fold.md:99:1 ───
2020-04-08T06:27:54.4202266Z     │
2020-04-08T06:27:54.4202266Z     │
2020-04-08T06:27:54.4203056Z  99 │ [fold_ty](***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs#L512-L536)
2020-04-08T06:27:54.4204813Z     │
2020-04-08T06:27:54.4204943Z 
2020-04-08T06:27:54.4204943Z 
2020-04-08T06:27:54.4205830Z error: The server responded with 429 Too Many Requests for "***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs"
2020-04-08T06:27:54.4206667Z      ┌── ty-fold.md:103:1 ───
2020-04-08T06:27:54.4207042Z      │
2020-04-08T06:27:54.4207042Z      │
2020-04-08T06:27:54.4207853Z  103 │ [ty_for_param](***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs#L552-L587)
2020-04-08T06:27:54.4216868Z      │
2020-04-08T06:27:54.4217023Z 
2020-04-08T06:27:54.4217023Z 
2020-04-08T06:27:54.4217751Z error: The server responded with 429 Too Many Requests for "***/issues/22019"
2020-04-08T06:27:54.4219030Z     ┌── traits/caching.md:57:30 ───
2020-04-08T06:27:54.4219954Z     │
2020-04-08T06:27:54.4219954Z     │
2020-04-08T06:27:54.4220522Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-04-08T06:27:54.4221716Z     │
2020-04-08T06:27:54.4221846Z 
2020-04-08T06:27:54.4221846Z 
2020-04-08T06:27:54.4222559Z error: The server responded with 429 Too Many Requests for "***/issues/18290"
2020-04-08T06:27:54.4223263Z     ┌── traits/caching.md:57:43 ───
2020-04-08T06:27:54.4223641Z     │
2020-04-08T06:27:54.4223641Z     │
2020-04-08T06:27:54.4224210Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-04-08T06:27:54.4225442Z     │
2020-04-08T06:27:54.4225589Z 
2020-04-08T06:27:54.4225589Z 
2020-04-08T06:27:54.4226258Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-04-08T06:27:54.4232140Z    ┌── traits/index.md:4:3 ───
2020-04-08T06:27:54.4232706Z    │
2020-04-08T06:27:54.4232706Z    │
2020-04-08T06:27:54.4233432Z  4 │ > [process of being implemented][wg]; this chapter serves as a kind of
2020-04-08T06:27:54.4234558Z    │
2020-04-08T06:27:54.4234688Z 
2020-04-08T06:27:54.4234688Z 
2020-04-08T06:27:54.4235413Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-04-08T06:27:54.4236358Z     ┌── traits/index.md:11:3 ───
2020-04-08T06:27:54.4236731Z     │
2020-04-08T06:27:54.4236731Z     │
2020-04-08T06:27:54.4237201Z  11 │ > [Traits Working Group tracking issue][wg]!
2020-04-08T06:27:54.4238183Z     │
2020-04-08T06:27:54.4238331Z 
2020-04-08T06:27:54.4239023Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/tree/master/chalk-engine"
2020-04-08T06:27:54.4239425Z 
2020-04-08T06:27:54.4239425Z 
2020-04-08T06:27:54.4239821Z     ┌── traits/index.md:54:7 ───
2020-04-08T06:27:54.4240212Z     │
2020-04-08T06:27:54.4240693Z  54 │ * the [`chalk_engine`][chalk_engine] crate, which
2020-04-08T06:27:54.4241269Z     │       ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4241705Z     │
2020-04-08T06:27:54.4241833Z 
2020-04-08T06:27:54.4242566Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_traits"
2020-04-08T06:27:54.4243308Z     ┌── traits/index.md:60:1 ───
2020-04-08T06:27:54.4243674Z     │
2020-04-08T06:27:54.4244100Z  60 │ [`librustc_traits`][librustc_traits].
2020-04-08T06:27:54.4244673Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4244673Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4245072Z     │
2020-04-08T06:27:54.4245199Z 
2020-04-08T06:27:54.4247965Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_middle/traits/mod.rs"
2020-04-08T06:27:54.4248860Z     ┌── traits/goals-and-clauses.md:41:1 ───
2020-04-08T06:27:54.4249285Z     │
2020-04-08T06:27:54.4249285Z     │
2020-04-08T06:27:54.4249813Z  41 │ [`librustc_middle/traits/mod.rs`][traits_mod] in rustc, and in
2020-04-08T06:27:54.4253859Z     │
2020-04-08T06:27:54.4254010Z 
2020-04-08T06:27:54.4254744Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/master/chalk-ir/src/lib.rs"
2020-04-08T06:27:54.4255160Z 
2020-04-08T06:27:54.4255160Z 
2020-04-08T06:27:54.4255628Z     ┌── traits/goals-and-clauses.md:42:1 ───
2020-04-08T06:27:54.4256031Z     │
2020-04-08T06:27:54.4256480Z  42 │ [`chalk-ir/src/lib.rs`][chalk_ir] in chalk.
2020-04-08T06:27:54.4257312Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4257718Z     │
2020-04-08T06:27:54.4257847Z 
2020-04-08T06:27:54.4258653Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_middle/ty/sty.rs"
2020-04-08T06:27:54.4259445Z     ┌── traits/associated-types.md:97:22 ───
2020-04-08T06:27:54.4259845Z     │
2020-04-08T06:27:54.4259845Z     │
2020-04-08T06:27:54.4260434Z  97 │ variant, declared in [`librustc_middle/ty/sty.rs`][sty]. In chalk, we use an
2020-04-08T06:27:54.4261560Z     │
2020-04-08T06:27:54.4261708Z 
2020-04-08T06:27:54.4262448Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs"
2020-04-08T06:27:54.4262883Z 
---
2020-04-08T06:27:54.4266063Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2089-implied-bounds.md"
2020-04-08T06:27:54.4266493Z 
2020-04-08T06:27:54.4266946Z     ┌── traits/implied-bounds.md:89:34 ───
2020-04-08T06:27:54.4267468Z     │
2020-04-08T06:27:54.4268071Z  89 │ types. The full RFC can be found [here][RFC]. We'll give here a brief view
2020-04-08T06:27:54.4269276Z     │
2020-04-08T06:27:54.4269406Z 
2020-04-08T06:27:54.4269406Z 
2020-04-08T06:27:54.4270104Z error: The server responded with 429 Too Many Requests for "***/pull/43786"
2020-04-08T06:27:54.4270831Z      ┌── traits/implied-bounds.md:313:8 ───
2020-04-08T06:27:54.4271275Z      │
2020-04-08T06:27:54.4271275Z      │
2020-04-08T06:27:54.4271809Z  313 │ can be [exploited][bug]. Indeed, suppose that we define the following
2020-04-08T06:27:54.4272876Z      │
2020-04-08T06:27:54.4273009Z 
2020-04-08T06:27:54.4273009Z 
2020-04-08T06:27:54.4273677Z error: The server responded with 429 Too Many Requests for "***/pull/69247"
2020-04-08T06:27:54.4274435Z     ┌── traits/lowering-module.md:21:47 ───
2020-04-08T06:27:54.4274838Z     │
2020-04-08T06:27:54.4274838Z     │
2020-04-08T06:27:54.4275378Z  21 │ **Note: We've removed the Chalk unit tests in [rust-lang/rust#69247].
2020-04-08T06:27:54.4276896Z     │
2020-04-08T06:27:54.4277026Z 
2020-04-08T06:27:54.4277787Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/clauses.rs"
2020-04-08T06:27:54.4278238Z 
2020-04-08T06:27:54.4278238Z 
2020-04-08T06:27:54.4278676Z     ┌── traits/lowering-rules.md:31:1 ───
2020-04-08T06:27:54.4279070Z     │
2020-04-08T06:27:54.4279632Z  31 │ [`chalk/chalk-solve/src/clauses.rs`][chalk_rules]. They are also ported in
2020-04-08T06:27:54.4280239Z     │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4280643Z     │
2020-04-08T06:27:54.4280790Z 
2020-04-08T06:27:54.4281542Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_traits"
2020-04-08T06:27:54.4282339Z     ┌── traits/lowering-rules.md:32:14 ───
2020-04-08T06:27:54.4282737Z     │
2020-04-08T06:27:54.4283224Z  32 │ rustc in the [`librustc_traits`][librustc_traits] crate.
2020-04-08T06:27:54.4283835Z     │              ^ Server responded with 429 Too Many Requests
---
2020-04-08T06:27:54.4289283Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/master/tests/test/wf_lowering.rs"
2020-04-08T06:27:54.4289712Z 
2020-04-08T06:27:54.4290107Z     ┌── traits/wf.md:16:36 ───
2020-04-08T06:27:54.4290493Z     │
2020-04-08T06:27:54.4291087Z  16 │ an extended set of examples in the [`chalk/tests/test/wf_lowering.rs`][wf_test] submodule.
2020-04-08T06:27:54.4292323Z     │
2020-04-08T06:27:54.4292466Z 
2020-04-08T06:27:54.4293328Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/239e4ae4e69b2785b5f99e0f2b41fc16b0b4e65e/chalk-engine/src/README.md"
2020-04-08T06:27:54.4293830Z 
2020-04-08T06:27:54.4293830Z 
2020-04-08T06:27:54.4294246Z      ┌── traits/slg.md:293:3 ───
2020-04-08T06:27:54.4294623Z      │
2020-04-08T06:27:54.4295162Z  293 │ - [chalk_solve README][readme], which contains links to papers used and
2020-04-08T06:27:54.4296378Z      │
2020-04-08T06:27:54.4296509Z 
2020-04-08T06:27:54.4296509Z 
2020-04-08T06:27:54.4297232Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/issues"
2020-04-08T06:27:54.4297961Z    ┌── traits/chalk-overview.md:5:3 ───
2020-04-08T06:27:54.4298348Z    │
2020-04-08T06:27:54.4298348Z    │
2020-04-08T06:27:54.4298914Z  5 │ > [open an issue][rustc-issues] so we can fix it. If you are able to fix the
2020-04-08T06:27:54.4299943Z    │
2020-04-08T06:27:54.4300088Z 
2020-04-08T06:27:54.4300772Z error: The server responded with 404 Not Found for "https://rust-lang.github.io/chalk/chalk_ir/enum.ProgramClause.html"
2020-04-08T06:27:54.4301165Z 
---
2020-04-08T06:27:54.4305973Z  127 │ [`ProgramClause`]s generated by the rules module.
2020-04-08T06:27:54.4306525Z      │ ^ Server responded with 404 Not Found
2020-04-08T06:27:54.4306937Z      │
2020-04-08T06:27:54.4307067Z 
2020-04-08T06:27:54.4307749Z error: The server responded with 429 Too Many Requests for "***/pull/55835"
2020-04-08T06:27:54.4308468Z      ┌── backend/updating-llvm.md:128:1 ───
2020-04-08T06:27:54.4308866Z      │
2020-04-08T06:27:54.4308866Z      │
2020-04-08T06:27:54.4309405Z  128 │ [#55835](***/pull/55835)
2020-04-08T06:27:54.4310632Z      │
2020-04-08T06:27:54.4310762Z 
2020-04-08T06:27:54.4310762Z 
2020-04-08T06:27:54.4311419Z error: The server responded with 429 Too Many Requests for "***/pull/47828"
2020-04-08T06:27:54.4312159Z      ┌── backend/updating-llvm.md:129:1 ───
2020-04-08T06:27:54.4312555Z      │
2020-04-08T06:27:54.4312555Z      │
2020-04-08T06:27:54.4313069Z  129 │ [#47828](***/pull/47828)
2020-04-08T06:27:54.4314442Z      │
2020-04-08T06:27:54.4314573Z 
2020-04-08T06:27:54.4314573Z 
2020-04-08T06:27:54.4315268Z error: The server responded with 429 Too Many Requests for "***/pull/62474"
2020-04-08T06:27:54.4318718Z      ┌── backend/updating-llvm.md:130:1 ───
2020-04-08T06:27:54.4319166Z      │
2020-04-08T06:27:54.4319166Z      │
2020-04-08T06:27:54.4319706Z  130 │ [#62474](***/pull/62474)
2020-04-08T06:27:54.4320951Z      │
2020-04-08T06:27:54.4321081Z 
2020-04-08T06:27:54.4321081Z 
2020-04-08T06:27:54.4321742Z error: The server responded with 429 Too Many Requests for "***/pull/62592"
2020-04-08T06:27:54.4322483Z      ┌── backend/updating-llvm.md:131:1 ───
2020-04-08T06:27:54.4322885Z      │
2020-04-08T06:27:54.4322885Z      │
2020-04-08T06:27:54.4323503Z  131 │ [#62592](***/pull/62592). Note that sometimes it's
2020-04-08T06:27:54.4324767Z      │
2020-04-08T06:27:54.4324897Z 
2020-04-08T06:27:54.4325557Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/"
2020-04-08T06:27:54.4325922Z 
---
2020-04-08T06:27:54.4333544Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md"
2020-04-08T06:27:54.4333994Z 
2020-04-08T06:27:54.4334472Z      ┌── codegen/implicit-caller-location.md:246:1 ───
2020-04-08T06:27:54.4334913Z      │
2020-04-08T06:27:54.4335561Z  246 │ [non-viable alternatives] in the approved RFC for details). It took two more years until RFC 2091
2020-04-08T06:27:54.4336625Z      │
2020-04-08T06:27:54.4336774Z 
2020-04-08T06:27:54.4337512Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md"
2020-04-08T06:27:54.4337942Z 
2020-04-08T06:27:54.4337942Z 
2020-04-08T06:27:54.4338434Z      ┌── codegen/implicit-caller-location.md:247:27 ───
2020-04-08T06:27:54.4338887Z      │
2020-04-08T06:27:54.4339504Z  247 │ was approved, much of its [rationale] for this feature's design having been discovered through the
2020-04-08T06:27:54.4340718Z      │
2020-04-08T06:27:54.4340849Z 
2020-04-08T06:27:54.4340849Z 
2020-04-08T06:27:54.4341870Z error: The server responded with 429 Too Many Requests for "***/issues/47809"
2020-04-08T06:27:54.4342776Z      ┌── codegen/implicit-caller-location.md:252:59 ───
2020-04-08T06:27:54.4343208Z      │
2020-04-08T06:27:54.4343208Z      │
2020-04-08T06:27:54.4343851Z  252 │ approval of the RFC and the actual implementation work, a [revised design] was proposed and written
2020-04-08T06:27:54.4345204Z      │
2020-04-08T06:27:54.4345333Z 
2020-04-08T06:27:54.4345333Z 
2020-04-08T06:27:54.4346339Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-04-08T06:27:54.4347155Z     ┌── profile-guided-optimization.md:65:33 ───
2020-04-08T06:27:54.4347598Z     │
2020-04-08T06:27:54.4347598Z     │
2020-04-08T06:27:54.4348160Z  65 │ `rustc` instructs LLVM to do so [by setting the appropriate][pgo-gen-passmanager]
2020-04-08T06:27:54.4349378Z     │
2020-04-08T06:27:54.4349507Z 
2020-04-08T06:27:54.4349507Z 
2020-04-08T06:27:54.4350342Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs"
2020-04-08T06:27:54.4351185Z     ┌── profile-guided-optimization.md:77:25 ───
2020-04-08T06:27:54.4351601Z     │
2020-04-08T06:27:54.4351601Z     │
2020-04-08T06:27:54.4352198Z  77 │ runtime are not removed [by marking the with the right export level][pgo-gen-symbols].
2020-04-08T06:27:54.4353366Z     │
2020-04-08T06:27:54.4353517Z 
2020-04-08T06:27:54.4353517Z 
2020-04-08T06:27:54.4354271Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-04-08T06:27:54.4355096Z     ┌── profile-guided-optimization.md:88:11 ───
2020-04-08T06:27:54.4355516Z     │
2020-04-08T06:27:54.4355516Z     │
2020-04-08T06:27:54.4356642Z  88 │ basically [just telling][pgo-use-passmanager] the LLVM `PassManagerBuilder`
2020-04-08T06:27:54.4357813Z     │
2020-04-08T06:27:54.4357943Z 
2020-04-08T06:27:54.4358974Z error: The server responded with 429 Too Many Requests for "https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile"
2020-04-08T06:27:54.4359426Z 
2020-04-08T06:27:54.4359426Z 
2020-04-08T06:27:54.4359900Z      ┌── profile-guided-optimization.md:109:1 ───
2020-04-08T06:27:54.4360325Z      │
2020-04-08T06:27:54.4360902Z  109 │ [compiler-rt][compiler-rt-profile] and statically linked into any instrumented
2020-04-08T06:27:54.4361544Z      │ ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4361950Z      │
2020-04-08T06:27:54.4362079Z 
2020-04-08T06:27:54.4362860Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/run-make-fulldeps"
2020-04-08T06:27:54.4363670Z      ┌── profile-guided-optimization.md:122:4 ───
2020-04-08T06:27:54.4364116Z      │
2020-04-08T06:27:54.4364116Z      │
2020-04-08T06:27:54.4364691Z  122 │ in [run-make tests][rmake-tests] (the relevant tests have `pgo` in their name).
2020-04-08T06:27:54.4365758Z      │
2020-04-08T06:27:54.4365888Z 
2020-04-08T06:27:54.4365888Z 
2020-04-08T06:27:54.4366669Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/test/codegen/pgo-instrumentation.rs"
2020-04-08T06:27:54.4367530Z      ┌── profile-guided-optimization.md:123:17 ───
2020-04-08T06:27:54.4367972Z      │
2020-04-08T06:27:54.4368536Z  123 │ There is also a [codegen test][codegen-test] that checks that some expected
2020-04-08T06:27:54.4369199Z      │                 ^ Server responded with 429 Too Many Requests
---
2020-04-08T06:27:54.4372235Z  24 │ *  The sanitizer runtime libraries are part of the [compiler-rt] project, and
2020-04-08T06:27:54.4372997Z     │                                                    ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4373518Z     │
2020-04-08T06:27:54.4373645Z 
2020-04-08T06:27:54.4374498Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs"
2020-04-08T06:27:54.4375522Z     ┌── sanitizers.md:25:4 ───
2020-04-08T06:27:54.4375896Z     │
2020-04-08T06:27:54.4375896Z     │
2020-04-08T06:27:54.4376495Z  25 │    [will be built on supported targets][sanitizer-build] when enabled in `config.toml`:
2020-04-08T06:27:54.4378067Z     │
2020-04-08T06:27:54.4378210Z 
2020-04-08T06:27:54.4378210Z 
2020-04-08T06:27:54.4379144Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs"
2020-04-08T06:27:54.4379963Z     ┌── sanitizers.md:32:21 ───
2020-04-08T06:27:54.4380363Z     │
2020-04-08T06:27:54.4380873Z  32 │    The runtimes are [placed into target libdir][sanitizer-copy].
2020-04-08T06:27:54.4381518Z     │                     ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4381518Z     │                     ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4381982Z     │
2020-04-08T06:27:54.4382109Z 
2020-04-08T06:27:54.4383027Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs"
2020-04-08T06:27:54.4383879Z     ┌── sanitizers.md:35:4 ───
2020-04-08T06:27:54.4384252Z     │
2020-04-08T06:27:54.4384252Z     │
2020-04-08T06:27:54.4384785Z  35 │    [marked][sanitizer-attribute] with appropriate LLVM attribute:
2020-04-08T06:27:54.4385787Z     │
2020-04-08T06:27:54.4386129Z 
2020-04-08T06:27:54.4386129Z 
2020-04-08T06:27:54.4387116Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs"
2020-04-08T06:27:54.4387974Z     ┌── sanitizers.md:42:65 ───
2020-04-08T06:27:54.4388352Z     │
2020-04-08T06:27:54.4388872Z  42 │    functions it might be necessary to inhibit inlining, both at [MIR
2020-04-08T06:27:54.4389677Z     │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4389677Z     │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4390229Z     │
2020-04-08T06:27:54.4390358Z 
2020-04-08T06:27:54.4391247Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td"
2020-04-08T06:27:54.4391794Z 
2020-04-08T06:27:54.4392195Z     ┌── sanitizers.md:43:27 ───
2020-04-08T06:27:54.4392566Z     │
2020-04-08T06:27:54.4393071Z  43 │    level][inline-mir] and [LLVM level][inline-llvm].
2020-04-08T06:27:54.4394167Z     │
2020-04-08T06:27:54.4394294Z 
2020-04-08T06:27:54.4394294Z 
2020-04-08T06:27:54.4395214Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs"
2020-04-08T06:27:54.4396429Z     ┌── sanitizers.md:45:54 ───
2020-04-08T06:27:54.4396815Z     │
2020-04-08T06:27:54.4396815Z     │
2020-04-08T06:27:54.4397565Z  45 │ *  The LLVM IR generated by rustc is instrumented by [dedicated LLVM
2020-04-08T06:27:54.4398854Z     │
2020-04-08T06:27:54.4398984Z 
2020-04-08T06:27:54.4398984Z 
2020-04-08T06:27:54.4399961Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs"
2020-04-08T06:27:54.4400813Z     ┌── sanitizers.md:50:4 ───
2020-04-08T06:27:54.4401209Z     │
2020-04-08T06:27:54.4401767Z  50 │    [linked in][sanitizer-link]. The libraries are searched for in target libdir
2020-04-08T06:27:54.4402388Z     │    ^ Server responded with 429 Too Many Requests
---
2020-04-08T06:27:54.4409765Z  45 │ We have our own fork of GDB - [https://github.com/rust-dev-tools/gdb]
2020-04-08T06:27:54.4410477Z     │                               ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4410935Z     │
2020-04-08T06:27:54.4411061Z 
2020-04-08T06:27:54.4411788Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
2020-04-08T06:27:54.4412565Z     ┌── debugging-support-in-rustc.md:80:1 ───
2020-04-08T06:27:54.4413004Z     │
2020-04-08T06:27:54.4413004Z     │
2020-04-08T06:27:54.4413660Z  80 │ [This comment by Tom](***c-dev-guide/pull/316#discussion_r284027340)
2020-04-08T06:27:54.4415576Z     │
2020-04-08T06:27:54.4415714Z 
2020-04-08T06:27:54.4415714Z 
2020-04-08T06:27:54.4416444Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
2020-04-08T06:27:54.4417208Z     ┌── debugging-support-in-rustc.md:86:1 ───
2020-04-08T06:27:54.4417622Z     │
2020-04-08T06:27:54.4417622Z     │
2020-04-08T06:27:54.4418311Z  86 │ [This question by Aman](***c-dev-guide/pull/316#discussion_r285401353)
2020-04-08T06:27:54.4419850Z     │
2020-04-08T06:27:54.4419978Z 
2020-04-08T06:27:54.4420589Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb"
2020-04-08T06:27:54.4420936Z 
---
2020-04-08T06:27:54.4424327Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb/wiki"
2020-04-08T06:27:54.4424703Z 
2020-04-08T06:27:54.4425169Z      ┌── debugging-support-in-rustc.md:121:43 ───
2020-04-08T06:27:54.4429420Z      │
2020-04-08T06:27:54.4430045Z  121 │ * None of the LLDB work is upstream. This [rust-lang/lldb wiki page] explains a few details.
2020-04-08T06:27:54.4431550Z      │
2020-04-08T06:27:54.4431682Z 
2020-04-08T06:27:54.4431682Z 
2020-04-08T06:27:54.4432395Z error: The server responded with 429 Too Many Requests for "***/issues/34457"
2020-04-08T06:27:54.4433151Z      ┌── debugging-support-in-rustc.md:174:17 ───
2020-04-08T06:27:54.4433600Z      │
2020-04-08T06:27:54.4433600Z      │
2020-04-08T06:27:54.4434170Z  174 │ Tracking issue: [***/issues/34457]
2020-04-08T06:27:54.4435238Z      │
2020-04-08T06:27:54.4435370Z 
2020-04-08T06:27:54.4435370Z 
2020-04-08T06:27:54.4436243Z error: The server responded with 429 Too Many Requests for "***/issues/33014"
2020-04-08T06:27:54.4437044Z      ┌── debugging-support-in-rustc.md:229:18 ───
2020-04-08T06:27:54.4437469Z      │
2020-04-08T06:27:54.4437469Z      │
2020-04-08T06:27:54.4438051Z  229 │ Issue on Github: [***/issues/33014]
2020-04-08T06:27:54.4439116Z      │
2020-04-08T06:27:54.4439265Z 
2020-04-08T06:27:54.4439906Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2117"
2020-04-08T06:27:54.4440274Z 
2020-04-08T06:27:54.4440274Z 
2020-04-08T06:27:54.4440734Z      ┌── debugging-support-in-rustc.md:265:6 ───
2020-04-08T06:27:54.4441179Z      │
2020-04-08T06:27:54.4441799Z  265 │ RFC: [https://github.com/rust-lang/rfcs/pull/2117]
2020-04-08T06:27:54.4442454Z      │      ^ Server responded with 429 Too Many Requests
2020-04-08T06:27:54.4442897Z      │
2020-04-08T06:27:54.4443028Z 
2020-04-08T06:27:54.4443724Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
2020-04-08T06:27:54.4444507Z      ┌── debugging-support-in-rustc.md:279:1 ───
2020-04-08T06:27:54.4444927Z      │
2020-04-08T06:27:54.4444927Z      │
2020-04-08T06:27:54.4445604Z  279 │ [Question on Github](***c-dev-guide/pull/316#discussion_r283062536).
2020-04-08T06:27:54.4447117Z      │
2020-04-08T06:27:54.4447267Z 
2020-04-08T06:27:54.4447267Z 
2020-04-08T06:27:54.4447998Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustdoc/core.rs"
2020-04-08T06:27:54.4448792Z     ┌── appendix/code-index.md:15:131 ───
2020-04-08T06:27:54.4449187Z     │
2020-04-08T06:27:54.4449187Z     │
2020-04-08T06:27:54.4450230Z  15 │ `DocContext` | struct | A state container used by rustdoc when crawling through a crate to gather its documentation | [Rustdoc] | [src/librustdoc/core.rs](***/blob/master/src/librustdoc/core.rs)
2020-04-08T06:27:54.4452619Z     │
2020-04-08T06:27:54.4452769Z 
2020-04-08T06:27:54.4452880Z 
2020-04-08T06:27:54.4452990Z 
---
2020-04-08T06:31:59.5563703Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2020-04-08T06:32:06.3340528Z error[E0631]: type mismatch in closure arguments
2020-04-08T06:32:06.3341273Z   --> src/tools/clippy/clippy_lints/src/let_underscore.rs:79:55
2020-04-08T06:32:06.3341804Z    |
2020-04-08T06:32:06.3343161Z 78 |                 let check_ty = |ty| SYNC_GUARD_PATHS.iter().any(|path| match_type(cx, ty, path));
2020-04-08T06:32:06.3344394Z    |                                ----------------------------------------------------------------- found signature of `fn(&rustc_middle::ty::TyS<'_>) -> _`
2020-04-08T06:32:06.3345435Z 79 |                 if cx.tables.expr_ty(init).walk().any(check_ty) {
2020-04-08T06:32:06.3346526Z    |                                                       ^^^^^^^^ expected signature of `fn(rustc_middle::ty::subst::GenericArg<'_>) -> _`
2020-04-08T06:32:07.1156662Z error[E0308]: mismatched types
2020-04-08T06:32:07.1158116Z     --> src/tools/clippy/clippy_lints/src/methods/mod.rs:1475:64
2020-04-08T06:32:07.1159066Z      |
2020-04-08T06:32:07.1159066Z      |
2020-04-08T06:32:07.1159908Z 1475 |             if ret_ty.walk().any(|inner_type| same_tys(cx, ty, inner_type)) {
2020-04-08T06:32:07.1161300Z      |                                                                ^^^^^^^^^^ expected `&rustc_middle::ty::TyS<'_>`, found struct `rustc_middle::ty::subst::GenericArg`
2020-04-08T06:32:07.3504331Z error[E0308]: mismatched types
2020-04-08T06:32:07.3505068Z     --> src/tools/clippy/clippy_lints/src/methods/mod.rs:1490:53
2020-04-08T06:32:07.3505624Z      |
2020-04-08T06:32:07.3506205Z 1490 | ...                   if same_tys(cx, ty, inner_type) {
2020-04-08T06:32:07.3506205Z 1490 | ...                   if same_tys(cx, ty, inner_type) {
2020-04-08T06:32:07.3507767Z      |                                           ^^^^^^^^^^ expected `&rustc_middle::ty::TyS<'_>`, found struct `rustc_middle::ty::subst::GenericArg`
2020-04-08T06:32:07.3508437Z 
2020-04-08T06:32:08.8561598Z error[E0277]: can't compare `&rustc_middle::ty::TyS<'_>` with `rustc_middle::ty::subst::GenericArg<'_>`
2020-04-08T06:32:08.8563108Z     |
2020-04-08T06:32:08.8563690Z 104 |             if self.item_type == impl_ty;
2020-04-08T06:32:08.8563690Z 104 |             if self.item_type == impl_ty;
2020-04-08T06:32:08.8565100Z     |                               ^^ no implementation for `&rustc_middle::ty::TyS<'_> == rustc_middle::ty::subst::GenericArg<'_>`
2020-04-08T06:32:08.8565971Z     |
2020-04-08T06:32:08.8566899Z     = help: the trait `std::cmp::PartialEq<rustc_middle::ty::subst::GenericArg<'_>>` is not implemented for `&rustc_middle::ty::TyS<'_>`
2020-04-08T06:32:09.0705452Z error: aborting due to 4 previous errors
2020-04-08T06:32:09.0705774Z 
2020-04-08T06:32:09.0711540Z Some errors have detailed explanations: E0277, E0308, E0631.
2020-04-08T06:32:09.0718304Z For more information about an error, try `rustc --explain E0277`.
---
2020-04-08T06:51:33.4557324Z    Compiling cargo v0.45.0 (/checkout/src/tools/cargo)
2020-04-08T06:51:55.5360886Z warning: unnecessary braces around block return value
2020-04-08T06:51:55.5362475Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-08T06:51:55.5363379Z    |
2020-04-08T06:51:55.5364504Z 36 |         static ref PROGRESS_ID_COUNTER: AtomicUsize = { AtomicUsize::new(0) };
2020-04-08T06:51:55.5366068Z    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
2020-04-08T06:51:55.5369372Z    = note: `#[warn(unused_braces)]` on by default
2020-04-08T06:51:55.5369694Z 
2020-04-08T06:59:44.6194844Z     Finished release [optimized] target(s) in 27m 30s
2020-04-08T06:59:44.9960160Z    Compiling slab v0.4.2
---
2020-04-08T07:00:30.8424494Z    Compiling tokio v0.1.22
2020-04-08T07:00:33.9216054Z warning: unnecessary braces around block return value
2020-04-08T07:00:33.9217412Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-08T07:00:33.9218265Z    |
2020-04-08T07:00:33.9219274Z 36 |         static ref PROGRESS_ID_COUNTER: AtomicUsize = { AtomicUsize::new(0) };
2020-04-08T07:00:33.9220641Z    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
2020-04-08T07:00:33.9224234Z    = note: `#[warn(unused_braces)]` on by default
2020-04-08T07:00:33.9224540Z 
2020-04-08T07:00:38.9686848Z warning: unnecessary braces around block return value
2020-04-08T07:00:38.9688527Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-08T07:00:38.9688527Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-08T07:00:38.9689411Z    |
2020-04-08T07:00:38.9690539Z 36 |         static ref PROGRESS_ID_COUNTER: AtomicUsize = { AtomicUsize::new(0) };
2020-04-08T07:00:38.9694565Z    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
2020-04-08T07:00:38.9695981Z    = note: `#[warn(unused_braces)]` on by default
2020-04-08T07:00:38.9696291Z 
2020-04-08T07:03:25.6816679Z     Finished release [optimized] target(s) in 3m 41s
2020-04-08T07:03:25.7405980Z      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/rls-cd24307f2016a8f3
---
2020-04-08T07:09:37.4496211Z test string::test::should_break_forward ... ok
2020-04-08T07:09:37.4496723Z test string::test::should_break_on_punctuation ... ok
2020-04-08T07:09:37.4497152Z test string::test::should_break_on_whitespace ... ok
2020-04-08T07:09:37.4497532Z test string::test::significant_whitespaces ... ok
2020-04-08T07:09:37.4504669Z test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
2020-04-08T07:09:37.4511426Z test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
2020-04-08T07:09:37.4511997Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
2020-04-08T07:09:37.4512604Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
2020-04-08T07:09:37.4581120Z test test::coverage_tests ... ok
2020-04-08T07:09:37.4604100Z test test::format_lines_errors_are_reported ... ok
2020-04-08T07:09:37.4646041Z test test::format_lines_errors_are_reported_with_tabs ... ok
2020-04-08T07:09:37.4971812Z test test::configuration_snippet::configuration_snippet_tests ... ok
---
2020-04-08T07:10:05.1204287Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `machine::MiriMemoryKind` cannot be formatted with the default formatter
2020-04-08T07:10:05.1205200Z     | 
2020-04-08T07:10:05.1205908Z    ::: /checkout/src/librustc_mir/interpret/machine.rs:82:10
2020-04-08T07:10:05.1206556Z     |
2020-04-08T07:10:05.1207450Z 82  |     type MemoryKind: ::std::fmt::Debug + ::std::fmt::Display + MayLeak + Eq + 'static;
2020-04-08T07:10:05.1208716Z     |          ---------- associated type defined here
2020-04-08T07:10:05.1211123Z     = help: the trait `std::fmt::Display` is not implemented for `machine::MiriMemoryKind`
2020-04-08T07:10:05.1212407Z     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2020-04-08T07:10:05.1212889Z 
2020-04-08T07:10:05.1978589Z error: aborting due to previous error
---
2020-04-08T07:10:06.8090564Z 
2020-04-08T07:10:06.8091386Z If you do intend to update 'rustc-dev-guide', please check the error messages above and
2020-04-08T07:10:06.8091946Z commit another update.
2020-04-08T07:10:06.8092235Z 
2020-04-08T07:10:06.8092967Z If you do NOT intend to update 'rustc-dev-guide', please ensure you did not accidentally
2020-04-08T07:10:06.8093910Z change the submodule at 'src/doc/rustc-dev-guide'. You may ask your reviewer for the
2020-04-08T07:10:06.8094455Z proper steps.
2020-04-08T07:10:06.8095885Z Build completed unsuccessfully in 0:00:01
2020-04-08T07:10:06.8142371Z == clock drift check ==
2020-04-08T07:10:06.8164095Z   local time: Wed Apr  8 07:10:06 UTC 2020
2020-04-08T07:10:06.8164095Z   local time: Wed Apr  8 07:10:06 UTC 2020
2020-04-08T07:10:08.0137155Z   network time: Wed, 08 Apr 2020 07:10:07 GMT
2020-04-08T07:10:08.0138392Z 
2020-04-08T07:10:08.0138392Z 
2020-04-08T07:10:08.0184087Z ##[error]Bash exited with code '1'.
2020-04-08T07:10:08.0237000Z ##[section]Finishing: Run build
2020-04-08T07:10:08.0300824Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70873/merge to s
2020-04-08T07:10:08.0306449Z Task         : Get sources
2020-04-08T07:10:08.0306845Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T07:10:08.0307595Z Version      : 1.0.0
2020-04-08T07:10:08.0307899Z Author       : Microsoft
2020-04-08T07:10:08.0307899Z Author       : Microsoft
2020-04-08T07:10:08.0308395Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T07:10:08.0308875Z ==============================================================================
2020-04-08T07:10:08.4324613Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T07:10:08.4366830Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70873/merge to s
2020-04-08T07:10:08.4478538Z Cleaning up task key
2020-04-08T07:10:08.4480014Z Start cleaning up orphan processes.
2020-04-08T07:10:08.4718996Z Terminate orphan process: pid (3769) (python)
2020-04-08T07:10:08.5021865Z ##[section]Finishing: Finalize Job
