plain
2020-04-20T22:38:50.3799720Z ========================== Starting Command Output ===========================
2020-04-20T22:38:50.3803365Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b8b3ed9c-7cd5-4445-a9ff-db588ecd41be.sh
2020-04-20T22:38:50.3803765Z 
2020-04-20T22:38:50.3807074Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T22:38:50.3824867Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-20T22:38:50.3828232Z Task         : Get sources
2020-04-20T22:38:50.3828532Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T22:38:50.3828805Z Version      : 1.0.0
2020-04-20T22:38:50.3828994Z Author       : Microsoft
---
2020-04-20T22:38:51.3691574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T22:38:51.3696752Z ##[command]git config gc.auto 0
2020-04-20T22:38:51.3700214Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T22:38:51.3704292Z ##[command]git config --get-all http.proxy
2020-04-20T22:38:51.3710586Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71323/merge:refs/remotes/pull/71323/merge
---
2020-04-20T22:40:52.5228671Z Looks like docker image is the same as before, not uploading
2020-04-20T22:41:00.2886786Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-20T22:41:00.3149016Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-20T22:41:00.3178970Z == clock drift check ==
2020-04-20T22:41:00.3191157Z   local time: Mon Apr 20 22:41:00 UTC 2020
2020-04-20T22:41:00.6339590Z   network time: Mon, 20 Apr 2020 22:41:00 GMT
2020-04-20T22:41:00.6371583Z Starting sccache server...
2020-04-20T22:41:00.7411086Z configure: processing command line
2020-04-20T22:41:00.7411765Z configure: 
2020-04-20T22:41:00.7412996Z configure: dist.missing-tools   := True
---
2020-04-20T22:53:34.6946617Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T22:53:36.4266492Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T22:53:37.4787007Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T22:53:43.3011863Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T22:53:52.7607738Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T22:53:57.3713773Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T22:54:02.9944085Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T22:54:08.5367382Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T22:54:19.8434463Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T23:24:31.4856507Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T23:24:34.6456938Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T23:24:36.3730268Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T23:24:50.0907591Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T23:25:02.1235942Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T23:25:13.6565159Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T23:25:23.4914948Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T23:25:33.2934001Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T23:25:50.1875644Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T00:24:43.3230595Z Testing rustbook src/doc/embedded-book
2020-04-21T00:24:44.5783464Z  finished in 1.255
2020-04-21T00:24:44.5793717Z Testing rustbook src/doc/edition-guide
2020-04-21T00:24:53.1978949Z  finished in 8.618
2020-04-21T00:26:08.7275682Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
2020-04-21T00:26:08.7277550Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-21T00:26:08.7278385Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-21T00:26:08.7279171Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-21T00:26:08.7280021Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2457`
2020-04-21T00:26:08.7280840Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-21T00:26:08.7282665Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
2020-04-21T00:26:08.7283804Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-21T00:26:08.7285658Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47732`
2020-04-21T00:26:08.7286435Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-21T00:26:08.7287203Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/51934`
2020-04-21T00:26:08.7287998Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
2020-04-21T00:26:08.7288776Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
2020-04-21T00:26:08.7289568Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/56245`
2020-04-21T00:26:08.7290655Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/RELEASES.md`
2020-04-21T00:26:08.7291570Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md`
2020-04-21T00:26:08.7292522Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-21T00:26:08.7293489Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-21T00:26:08.7295292Z Received 429 (TOO_MANY_REQUESTS) for link `https://gist.github.com/nikomatsakis/631ec8b4af9a18b5d062d9d9b7d3d967`
2020-04-21T00:26:08.7296246Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-21T00:26:08.7297115Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-21T00:26:08.7297991Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs`
2020-04-21T00:26:08.7298901Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs`
2020-04-21T00:26:08.7299687Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-21T00:26:08.7300427Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues`
2020-04-21T00:26:08.7301744Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/15702`
2020-04-21T00:26:08.7302448Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/doc/unstable-book`
2020-04-21T00:26:08.7303066Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/reference`
2020-04-21T00:26:08.7303691Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/book`
2020-04-21T00:26:08.7304369Z Received 429 (TOO_MANY_REQUESTS) for link `***-by-example`
2020-04-21T00:26:08.7304986Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/32409`
2020-04-21T00:26:08.7305977Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc`
2020-04-21T00:26:08.7307227Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md`
2020-04-21T00:26:08.7307966Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/kennytm/rustup-toolchain-install-master`
2020-04-21T00:26:08.7309043Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-21T00:26:08.7309641Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme`
2020-04-21T00:26:08.7310352Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md`
2020-04-21T00:26:08.7311079Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-21T00:26:08.7311750Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/blob/master/collector/README.md`
2020-04-21T00:26:08.7312491Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/tree/master/collector/benchmarks`
2020-04-21T00:26:08.7313122Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-21T00:26:08.7313799Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-21T00:26:08.7314448Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/fmt-rfcs`
2020-04-21T00:26:08.7315326Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs`
2020-04-21T00:26:08.7315996Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rls`
2020-04-21T00:26:08.7316660Z Received 429 (TOO_MANY_REQUESTS) for link `***fix`
2020-04-21T00:26:08.7317270Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Assignment`
2020-04-21T00:26:08.7317940Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/140`
2020-04-21T00:26:08.7318615Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/221`
2020-04-21T00:26:08.7319259Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/`
2020-04-21T00:26:08.7319926Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Pinging`
2020-04-21T00:26:08.7320682Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/triagebot.toml`
2020-04-21T00:26:08.7321371Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-Cleanup-Crew`
2020-04-21T00:26:08.7322011Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/jethrogb/rust-reduce`
2020-04-21T00:26:08.7322674Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-21T00:26:08.7323350Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-21T00:26:08.7324030Z Received 429 (TOO_MANY_REQUESTS) for link `***/`
2020-04-21T00:26:08.7324654Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-LLVM`
2020-04-21T00:26:08.7325347Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-APACHE`
2020-04-21T00:26:08.7326011Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-MIT`
2020-04-21T00:26:08.7326693Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/COPYRIGHT`
2020-04-21T00:26:08.7327345Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-21T00:26:08.7328545Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide`
2020-04-21T00:26:08.7329246Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_metadata`
2020-04-21T00:26:08.7329985Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_middle/dep_graph`
2020-04-21T00:26:08.7330646Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42678`
2020-04-21T00:26:08.7331454Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md`
2020-04-21T00:26:08.7332247Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42293`
2020-04-21T00:26:08.7332887Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42633`
2020-04-21T00:26:08.7333465Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/salsa-rs/salsa`
2020-04-21T00:26:08.7334382Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/blob/master/examples/rustc-driver-example.rs`
2020-04-21T00:26:08.7335163Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustdoc`
2020-04-21T00:26:08.7335878Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/tools/rustdoc`
2020-04-21T00:26:08.7336523Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/44136`
2020-04-21T00:26:08.7337204Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_ast`
2020-04-21T00:26:08.7337900Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_expand/mbe`
2020-04-21T00:26:08.7338575Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/41710`
2020-04-21T00:26:08.7339374Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs`
2020-04-21T00:26:08.7340184Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_macros/src/type_foldable.rs`
2020-04-21T00:26:08.7341057Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-21T00:26:08.7341941Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-21T00:26:08.7342837Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-21T00:26:08.7343551Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/22019`
2020-04-21T00:26:08.7344190Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/18290`
2020-04-21T00:26:08.7344846Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48416`
2020-04-21T00:26:08.7349534Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48416`
2020-04-21T00:26:08.7350473Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/tree/master/chalk-engine`
2020-04-21T00:26:08.7351292Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_traits`
2020-04-21T00:26:08.7352046Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_middle/traits/mod.rs`
2020-04-21T00:26:08.7352775Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-ir/src/lib.rs`
2020-04-21T00:26:08.7353556Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_middle/ty/sty.rs`
2020-04-21T00:26:08.7354286Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs`
2020-04-21T00:26:08.7355063Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2089-implied-bounds.md`
2020-04-21T00:26:08.7355803Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/43786`
2020-04-21T00:26:08.7356418Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/69247`
2020-04-21T00:26:08.7357085Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/clauses.rs`
2020-04-21T00:26:08.7357880Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_traits`
2020-04-21T00:26:08.7358571Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/wf.rs`
2020-04-21T00:26:08.7359909Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/tests/test/wf_lowering.rs`
2020-04-21T00:26:08.7361197Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/239e4ae4e69b2785b5f99e0f2b41fc16b0b4e65e/chalk-engine/src/README.md`
2020-04-21T00:26:08.7362019Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-21T00:26:08.7362652Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47828`
2020-04-21T00:26:08.7363287Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62474`
2020-04-21T00:26:08.7363898Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62592`
2020-04-21T00:26:08.7364493Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/`
2020-04-21T00:26:08.7365476Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-21T00:26:08.7366344Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-21T00:26:08.7367139Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-21T00:26:08.7367869Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/47809`
2020-04-21T00:26:08.7368580Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-21T00:26:08.7369360Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs`
2020-04-21T00:26:08.7370138Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-21T00:26:08.7370850Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile`
2020-04-21T00:26:08.7371664Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/test/run-make-fulldeps`
2020-04-21T00:26:08.7372425Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/test/codegen/pgo-instrumentation.rs`
2020-04-21T00:26:08.7373140Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt`
2020-04-21T00:26:08.7373990Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs`
2020-04-21T00:26:08.7374859Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs`
2020-04-21T00:26:08.7375750Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs`
2020-04-21T00:26:08.7376665Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs`
2020-04-21T00:26:08.7377571Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td`
2020-04-21T00:26:08.7378561Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs`
2020-04-21T00:26:08.7379792Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs`
2020-04-21T00:26:08.7380264Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/google/sanitizers/wiki/`
2020-04-21T00:26:08.7380934Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/gdb`
2020-04-21T00:26:08.7381645Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-21T00:26:08.7382291Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-21T00:26:08.7382896Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb`
2020-04-21T00:26:08.7383545Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb/wiki`
2020-04-21T00:26:08.7384221Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/34457`
2020-04-21T00:26:08.7384862Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/33014`
2020-04-21T00:26:08.7385466Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2117`
2020-04-21T00:26:08.7386465Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-21T00:26:08.7387146Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustdoc/core.rs`
2020-04-21T00:26:08.7387781Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/1wilkens/thesis-ba`
2020-04-21T00:26:08.7389496Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-21T00:26:08.7744879Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-04-21T00:26:09.0426065Z    Compiling proc-macro2 v0.4.30
2020-04-21T00:26:09.0427346Z    Compiling unicode-xid v0.1.0
---
2020-04-21T00:39:43.9504591Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-21T00:39:43.9504798Z 
2020-04-21T00:39:43.9504908Z 
2020-04-21T00:39:43.9505039Z running 1 test
2020-04-21T00:39:44.3949292Z test [ui] ui-toml/zero_single_char_names/zero_single_char_names.rs ... ok
2020-04-21T00:39:44.3949863Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-21T00:39:44.3950069Z 
2020-04-21T00:39:44.3950161Z 
2020-04-21T00:39:44.3950309Z running 1 test
---
2020-04-21T00:55:25.3457088Z    Compiling cargo v0.45.0 (/checkout/src/tools/cargo)
2020-04-21T00:55:44.9471002Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9472371Z    --> src/tools/rls/rls/src/build/cargo.rs:359:58
2020-04-21T00:55:44.9473079Z     |
2020-04-21T00:55:44.9473976Z 359 |     fn init<'a>(&self, cx: &Context<'a, '_>, unit: &Unit<'a>) {
2020-04-21T00:55:44.9478916Z 
2020-04-21T00:55:44.9483198Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9484160Z    --> src/tools/rls/rls/src/build/cargo.rs:371:41
2020-04-21T00:55:44.9484883Z     |
2020-04-21T00:55:44.9484883Z     |
2020-04-21T00:55:44.9485721Z 371 |     fn force_rebuild(&self, unit: &Unit<'_>) -> bool {
2020-04-21T00:55:44.9490495Z 
2020-04-21T00:55:44.9497398Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9498398Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:467:16
2020-04-21T00:55:44.9499043Z     |
2020-04-21T00:55:44.9499043Z     |
2020-04-21T00:55:44.9499940Z 467 | impl From<Unit<'_>> for UnitKey {
2020-04-21T00:55:44.9506251Z 
2020-04-21T00:55:44.9509014Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9510107Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:483:16
2020-04-21T00:55:44.9510890Z     |
2020-04-21T00:55:44.9510890Z     |
2020-04-21T00:55:44.9512054Z 483 | impl From<Unit<'_>> for OwnedUnit {
2020-04-21T00:55:44.9547512Z 
2020-04-21T00:55:44.9548537Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9549326Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:134:25
2020-04-21T00:55:44.9549817Z     |
2020-04-21T00:55:44.9549817Z     |
2020-04-21T00:55:44.9550420Z 134 |         Filter: Fn(Unit<'a>) -> bool,
2020-04-21T00:55:44.9551519Z 
2020-04-21T00:55:44.9552063Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9552720Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:130:20
2020-04-21T00:55:44.9553208Z     |
2020-04-21T00:55:44.9553208Z     |
2020-04-21T00:55:44.9553750Z 130 |         unit: Unit<'a>,
2020-04-21T00:55:44.9554503Z     |                    ^^ unexpected lifetime argument
2020-04-21T00:55:44.9554806Z 
2020-04-21T00:55:44.9555344Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9556003Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:468:24
2020-04-21T00:55:44.9556489Z     |
2020-04-21T00:55:44.9557070Z 468 |     fn from(unit: Unit<'_>) -> UnitKey {
2020-04-21T00:55:44.9558156Z 
2020-04-21T00:55:44.9558693Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T00:55:44.9559354Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:484:24
2020-04-21T00:55:44.9559829Z     |
2020-04-21T00:55:44.9559829Z     |
2020-04-21T00:55:44.9560431Z 484 |     fn from(unit: Unit<'_>) -> OwnedUnit {
2020-04-21T00:55:44.9561520Z 
2020-04-21T00:55:44.9734924Z error: aborting due to 8 previous errors
2020-04-21T00:55:44.9738396Z 
2020-04-21T00:55:44.9743385Z For more information about this error, try `rustc --explain E0107`.
---
2020-04-21T01:04:24.0457841Z test string::test::retain_blank_lines ... ok
2020-04-21T01:04:24.0458129Z test string::test::should_break_forward ... ok
2020-04-21T01:04:24.0458421Z test string::test::should_break_on_punctuation ... ok
2020-04-21T01:04:24.0458736Z test string::test::should_break_on_whitespace ... ok
2020-04-21T01:04:24.0459080Z test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
2020-04-21T01:04:24.0459475Z test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
2020-04-21T01:04:24.0460064Z test string::test::significant_whitespaces ... ok
2020-04-21T01:04:24.0464864Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
2020-04-21T01:04:24.0465328Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
2020-04-21T01:04:24.0548802Z test test::coverage_tests ... ok
2020-04-21T01:04:24.0549101Z test test::format_lines_errors_are_reported ... ok
2020-04-21T01:04:24.0549385Z test test::format_lines_errors_are_reported_with_tabs ... ok
2020-04-21T01:04:24.0870070Z test test::configuration_snippet::configuration_snippet_tests ... ok
---
2020-04-21T01:04:48.3373091Z    --> src/tools/miri/src/diagnostics.rs:182:12
2020-04-21T01:04:48.3373634Z     |
2020-04-21T01:04:48.3374488Z 182 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3375328Z     |            ^^^^
2020-04-21T01:04:48.3376188Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 182:6
2020-04-21T01:04:48.3377562Z     |
2020-04-21T01:04:48.3378420Z 182 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3379252Z     |      ^^^^
2020-04-21T01:04:48.3379497Z 
---
2020-04-21T01:04:48.3390949Z   --> src/tools/miri/src/helpers.rs:15:12
2020-04-21T01:04:48.3391482Z    |
2020-04-21T01:04:48.3392334Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3393169Z    |            ^^^^
2020-04-21T01:04:48.3394019Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 15:6
2020-04-21T01:04:48.3395340Z    |
2020-04-21T01:04:48.3396186Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3397297Z    |      ^^^^
2020-04-21T01:04:48.3397541Z 
2020-04-21T01:04:48.3397541Z 
2020-04-21T01:04:48.3402778Z error[E0478]: lifetime bound not satisfied
2020-04-21T01:04:48.3403527Z    --> src/tools/miri/src/helpers.rs:267:35
2020-04-21T01:04:48.3404123Z     |
2020-04-21T01:04:48.3404892Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3406450Z     |
2020-04-21T01:04:48.3407328Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the impl at 267:26
2020-04-21T01:04:48.3408174Z    --> src/tools/miri/src/helpers.rs:267:26
2020-04-21T01:04:48.3408730Z     |
2020-04-21T01:04:48.3408730Z     |
2020-04-21T01:04:48.3409475Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3410274Z     |                          ^^^^
2020-04-21T01:04:48.3411160Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 267:20
2020-04-21T01:04:48.3412500Z     |
2020-04-21T01:04:48.3412500Z     |
2020-04-21T01:04:48.3413257Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3414313Z 
2020-04-21T01:04:48.3596870Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
2020-04-21T01:04:48.3597683Z    --> src/tools/miri/src/helpers.rs:267:35
2020-04-21T01:04:48.3598149Z     |
2020-04-21T01:04:48.3598149Z     |
2020-04-21T01:04:48.3598800Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3600118Z     |
2020-04-21T01:04:48.3600118Z     |
2020-04-21T01:04:48.3600772Z note: first, the lifetime cannot outlive the lifetime `'tcx` as defined on the impl at 267:26...
2020-04-21T01:04:48.3601935Z     |
2020-04-21T01:04:48.3601935Z     |
2020-04-21T01:04:48.3602562Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3603812Z note: ...so that the types are compatible
2020-04-21T01:04:48.3604374Z    --> src/tools/miri/src/helpers.rs:267:35
2020-04-21T01:04:48.3604825Z     |
2020-04-21T01:04:48.3604825Z     |
2020-04-21T01:04:48.3605464Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3606245Z     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-21T01:04:48.3607404Z     = note: expected  `rustc_mir::interpret::ValueVisitor<'mir, 'tcx, machine::Evaluator<'tcx>>`
2020-04-21T01:04:48.3608286Z                found  `rustc_mir::interpret::ValueVisitor<'_, '_, machine::Evaluator<'_>>`
2020-04-21T01:04:48.3609101Z note: but, the lifetime must be valid for the lifetime `'mir` as defined on the impl at 267:20...
2020-04-21T01:04:48.3610253Z     |
2020-04-21T01:04:48.3610253Z     |
2020-04-21T01:04:48.3610880Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T01:04:48.3612117Z note: ...so that the types are compatible
2020-04-21T01:04:48.3612683Z    --> src/tools/miri/src/helpers.rs:272:22
2020-04-21T01:04:48.3613152Z     |
2020-04-21T01:04:48.3613698Z 272 |             type V = MPlaceTy<'tcx, Tag>;
---
2020-04-21T01:04:48.3640420Z   --> src/tools/miri/src/shims/dlsym.rs:23:12
2020-04-21T01:04:48.3640890Z    |
2020-04-21T01:04:48.3641591Z 23 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3642301Z    |            ^^^^
2020-04-21T01:04:48.3643022Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 23:6
2020-04-21T01:04:48.3644162Z    |
2020-04-21T01:04:48.3644857Z 23 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3645543Z    |      ^^^^
2020-04-21T01:04:48.3645745Z 
---
2020-04-21T01:04:48.3658847Z    --> src/tools/miri/src/shims/env.rs:100:12
2020-04-21T01:04:48.3659316Z     |
2020-04-21T01:04:48.3660016Z 100 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3660741Z     |            ^^^^
2020-04-21T01:04:48.3661450Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 100:6
2020-04-21T01:04:48.3662589Z     |
2020-04-21T01:04:48.3663289Z 100 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3663981Z     |      ^^^^
2020-04-21T01:04:48.3664200Z 
---
2020-04-21T01:04:48.3669259Z  --> src/tools/miri/src/shims/foreign_items/windows.rs:8:12
2020-04-21T01:04:48.3669744Z   |
2020-04-21T01:04:48.3670436Z 8 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3671136Z   |            ^^^^
2020-04-21T01:04:48.3671855Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 8:6
2020-04-21T01:04:48.3673030Z   |
2020-04-21T01:04:48.3673736Z 8 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3674421Z   |      ^^^^
2020-04-21T01:04:48.3674619Z 
---
2020-04-21T01:04:48.3680480Z  --> src/tools/miri/src/shims/foreign_items/posix/linux.rs:4:12
2020-04-21T01:04:48.3680971Z   |
2020-04-21T01:04:48.3681663Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3682361Z   |            ^^^^
2020-04-21T01:04:48.3683074Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 4:6
2020-04-21T01:04:48.3683798Z  --> src/tools/miri/src/shims/foreign_items/posix/linux.rs:4:6
2020-04-21T01:04:48.3685014Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3685694Z   |      ^^^^
2020-04-21T01:04:48.3685894Z 
2020-04-21T01:04:48.3700113Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T01:04:48.3704620Z  --> src/tools/miri/src/shims/foreign_items/posix/macos.rs:4:12
2020-04-21T01:04:48.3705097Z   |
2020-04-21T01:04:48.3706009Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3706740Z   |            ^^^^
2020-04-21T01:04:48.3707442Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 4:6
2020-04-21T01:04:48.3708182Z  --> src/tools/miri/src/shims/foreign_items/posix/macos.rs:4:6
2020-04-21T01:04:48.3709352Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3710052Z   |      ^^^^
2020-04-21T01:04:48.3710252Z 
2020-04-21T01:04:48.3710694Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T01:04:48.3715335Z   --> src/tools/miri/src/shims/foreign_items/posix.rs:12:12
2020-04-21T01:04:48.3715811Z    |
2020-04-21T01:04:48.3716510Z 12 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3717228Z    |            ^^^^
2020-04-21T01:04:48.3717927Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 12:6
2020-04-21T01:04:48.3719126Z    |
2020-04-21T01:04:48.3719827Z 12 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3720512Z    |      ^^^^
2020-04-21T01:04:48.3720730Z 
---
2020-04-21T01:04:48.3729446Z   --> src/tools/miri/src/shims/foreign_items.rs:15:12
2020-04-21T01:04:48.3729919Z    |
2020-04-21T01:04:48.3730621Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3731340Z    |            ^^^^
2020-04-21T01:04:48.3734268Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 15:6
2020-04-21T01:04:48.3736695Z    |
2020-04-21T01:04:48.3737413Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3738107Z    |      ^^^^
2020-04-21T01:04:48.3738325Z 
2020-04-21T01:04:48.3738325Z 
2020-04-21T01:04:48.3738772Z error[E0478]: lifetime bound not satisfied
2020-04-21T01:04:48.3742727Z   --> src/tools/miri/src/shims/fs.rs:67:18
2020-04-21T01:04:48.3743214Z    |
2020-04-21T01:04:48.3743938Z 67 | impl<'mir, 'tcx> EvalContextExtPrivate<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3748609Z    |
2020-04-21T01:04:48.3749252Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the impl at 67:12
2020-04-21T01:04:48.3750136Z   --> src/tools/miri/src/shims/fs.rs:67:12
2020-04-21T01:04:48.3750612Z    |
2020-04-21T01:04:48.3750612Z    |
2020-04-21T01:04:48.3751334Z 67 | impl<'mir, 'tcx> EvalContextExtPrivate<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3752046Z    |            ^^^^
2020-04-21T01:04:48.3752761Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 67:6
2020-04-21T01:04:48.3753885Z    |
2020-04-21T01:04:48.3753885Z    |
2020-04-21T01:04:48.3754601Z 67 | impl<'mir, 'tcx> EvalContextExtPrivate<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3755514Z 
2020-04-21T01:04:48.3755974Z error[E0478]: lifetime bound not satisfied
2020-04-21T01:04:48.3756545Z    --> src/tools/miri/src/shims/fs.rs:235:18
2020-04-21T01:04:48.3756999Z     |
---
2020-04-21T01:04:48.3760321Z    --> src/tools/miri/src/shims/fs.rs:235:12
2020-04-21T01:04:48.3760792Z     |
2020-04-21T01:04:48.3761492Z 235 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3762198Z     |            ^^^^
2020-04-21T01:04:48.3762919Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 235:6
2020-04-21T01:04:48.3764043Z     |
2020-04-21T01:04:48.3764757Z 235 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3765447Z     |      ^^^^
2020-04-21T01:04:48.3765652Z 
---
2020-04-21T01:04:48.3770469Z   --> src/tools/miri/src/shims/intrinsics.rs:10:12
2020-04-21T01:04:48.3771005Z    |
2020-04-21T01:04:48.3771758Z 10 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3772470Z    |            ^^^^
2020-04-21T01:04:48.3773185Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 10:6
2020-04-21T01:04:48.3774332Z    |
2020-04-21T01:04:48.3775046Z 10 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3775731Z    |      ^^^^
2020-04-21T01:04:48.3775930Z 
---
2020-04-21T01:04:48.3781530Z   --> src/tools/miri/src/shims/os_str.rs:63:12
2020-04-21T01:04:48.3782015Z    |
2020-04-21T01:04:48.3782731Z 63 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3783447Z    |            ^^^^
2020-04-21T01:04:48.3784468Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 63:6
2020-04-21T01:04:48.3785186Z   --> src/tools/miri/src/shims/os_str.rs:63:6
2020-04-21T01:04:48.3792476Z 63 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3793181Z    |      ^^^^
2020-04-21T01:04:48.3793382Z 
2020-04-21T01:04:48.3793828Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T01:04:48.3798544Z   --> src/tools/miri/src/shims/panic.rs:34:12
2020-04-21T01:04:48.3799001Z    |
2020-04-21T01:04:48.3799713Z 34 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3800557Z    |            ^^^^
2020-04-21T01:04:48.3801320Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 34:6
2020-04-21T01:04:48.3803039Z    |
2020-04-21T01:04:48.3803958Z 34 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3805379Z    |      ^^^^
2020-04-21T01:04:48.3805588Z 
---
2020-04-21T01:04:48.3812070Z    --> src/tools/miri/src/shims/sync.rs:203:12
2020-04-21T01:04:48.3812538Z     |
2020-04-21T01:04:48.3813274Z 203 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3814009Z     |            ^^^^
2020-04-21T01:04:48.3815106Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 203:6
2020-04-21T01:04:48.3815807Z    --> src/tools/miri/src/shims/sync.rs:203:6
2020-04-21T01:04:48.3816983Z 203 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3817677Z     |      ^^^^
2020-04-21T01:04:48.3817878Z 
2020-04-21T01:04:48.3818332Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T01:04:48.3822655Z   --> src/tools/miri/src/shims/time.rs:17:12
2020-04-21T01:04:48.3823105Z    |
2020-04-21T01:04:48.3823815Z 17 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3824512Z    |            ^^^^
2020-04-21T01:04:48.3825213Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 17:6
2020-04-21T01:04:48.3826263Z   --> src/tools/miri/src/shims/time.rs:17:6
2020-04-21T01:04:48.3827900Z 17 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3828606Z    |      ^^^^
2020-04-21T01:04:48.3828807Z 
2020-04-21T01:04:48.3829276Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T01:04:48.3834290Z    --> src/tools/miri/src/shims/tls.rs:157:12
2020-04-21T01:04:48.3834743Z     |
2020-04-21T01:04:48.3835460Z 157 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3836645Z     |            ^^^^
2020-04-21T01:04:48.3837362Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 157:6
2020-04-21T01:04:48.3838509Z     |
2020-04-21T01:04:48.3839213Z 157 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3839923Z     |      ^^^^
2020-04-21T01:04:48.3840127Z 
---
2020-04-21T01:04:48.3844901Z   --> src/tools/miri/src/shims/mod.rs:20:12
2020-04-21T01:04:48.3845349Z    |
2020-04-21T01:04:48.3846047Z 20 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3847146Z    |            ^^^^
2020-04-21T01:04:48.3847883Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 20:6
2020-04-21T01:04:48.3849017Z    |
2020-04-21T01:04:48.3849909Z 20 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3850622Z    |      ^^^^
2020-04-21T01:04:48.3850823Z 
2020-04-21T01:04:48.3850823Z 
2020-04-21T01:04:48.3851265Z error[E0478]: lifetime bound not satisfied
2020-04-21T01:04:48.3851860Z    --> src/tools/miri/src/stacked_borrows.rs:509:18
2020-04-21T01:04:48.3852344Z     |
2020-04-21T01:04:48.3853057Z 509 | impl<'mir, 'tcx> EvalContextPrivExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3854348Z     |
2020-04-21T01:04:48.3854993Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the impl at 509:12
2020-04-21T01:04:48.3855746Z    --> src/tools/miri/src/stacked_borrows.rs:509:12
2020-04-21T01:04:48.3856872Z     |
2020-04-21T01:04:48.3856872Z     |
2020-04-21T01:04:48.3857609Z 509 | impl<'mir, 'tcx> EvalContextPrivExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3858338Z     |            ^^^^
2020-04-21T01:04:48.3859045Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 509:6
2020-04-21T01:04:48.3860219Z     |
2020-04-21T01:04:48.3860219Z     |
2020-04-21T01:04:48.3860929Z 509 | impl<'mir, 'tcx> EvalContextPrivExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3861855Z 
2020-04-21T01:04:48.3862295Z error[E0478]: lifetime bound not satisfied
2020-04-21T01:04:48.3862888Z    --> src/tools/miri/src/stacked_borrows.rs:610:18
2020-04-21T01:04:48.3863370Z     |
---
2020-04-21T01:04:48.3866966Z    --> src/tools/miri/src/stacked_borrows.rs:610:12
2020-04-21T01:04:48.3867437Z     |
2020-04-21T01:04:48.3868155Z 610 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3868875Z     |            ^^^^
2020-04-21T01:04:48.3869581Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 610:6
2020-04-21T01:04:48.3870757Z     |
2020-04-21T01:04:48.3871453Z 610 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T01:04:48.3872244Z     |      ^^^^
2020-04-21T01:04:48.3872461Z 
---
2020-04-21T01:04:49.8968564Z Verifying status of miri...
2020-04-21T01:04:49.8973870Z Verifying status of embedded-book...
2020-04-21T01:04:49.8975531Z Verifying status of rustc-dev-guide...
2020-04-21T01:04:49.9040030Z Cloning into 'rust-toolstate'...
2020-04-21T01:04:50.5836890Z error: Tool `rls` has regressed from test-pass to build-fail during beta week.
2020-04-21T01:04:50.5845614Z Build completed unsuccessfully in 0:00:02
2020-04-21T01:04:50.5955169Z == clock drift check ==
2020-04-21T01:04:50.5955169Z == clock drift check ==
2020-04-21T01:04:50.5968719Z   local time: Tue Apr 21 01:04:50 UTC 2020
2020-04-21T01:04:50.7744728Z   network time: Tue, 21 Apr 2020 01:04:50 GMT
2020-04-21T01:04:51.5519995Z 
2020-04-21T01:04:51.5519995Z 
2020-04-21T01:04:51.5587350Z ##[error]Bash exited with code '1'.
2020-04-21T01:04:51.5600968Z ##[section]Finishing: Run build
2020-04-21T01:04:51.5646555Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-21T01:04:51.5651288Z Task         : Get sources
2020-04-21T01:04:51.5651612Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T01:04:51.5651911Z Version      : 1.0.0
2020-04-21T01:04:51.5652115Z Author       : Microsoft
2020-04-21T01:04:51.5652115Z Author       : Microsoft
2020-04-21T01:04:51.5652435Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T01:04:51.5652812Z ==============================================================================
2020-04-21T01:04:51.9108826Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T01:04:51.9152890Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-21T01:04:51.9239868Z Cleaning up task key
2020-04-21T01:04:51.9241147Z Start cleaning up orphan processes.
2020-04-21T01:04:51.9598833Z Terminate orphan process: pid (4308) (python)
2020-04-21T01:04:51.9666969Z ##[section]Finishing: Finalize Job
