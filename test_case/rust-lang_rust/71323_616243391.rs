plain
2020-04-19T21:01:20.4439176Z ========================== Starting Command Output ===========================
2020-04-19T21:01:20.4456579Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e69e2f1d-47eb-4e17-b746-420e865e0a88.sh
2020-04-19T21:01:20.7244270Z 
2020-04-19T21:01:20.7314070Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T21:01:20.7344959Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-19T21:01:20.7356360Z Task         : Get sources
2020-04-19T21:01:20.7356817Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T21:01:20.7357259Z Version      : 1.0.0
2020-04-19T21:01:20.7357584Z Author       : Microsoft
---
2020-04-19T21:01:23.2845519Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T21:01:23.3116974Z ##[command]git config gc.auto 0
2020-04-19T21:01:23.3122671Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T21:01:23.3150721Z ##[command]git config --get-all http.proxy
2020-04-19T21:01:23.3232705Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71323/merge:refs/remotes/pull/71323/merge
---
2020-04-19T21:04:35.4110686Z Looks like docker image is the same as before, not uploading
2020-04-19T21:04:41.9586220Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-19T21:04:41.9790574Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-19T21:04:41.9822302Z == clock drift check ==
2020-04-19T21:04:41.9832899Z   local time: Sun Apr 19 21:04:41 UTC 2020
2020-04-19T21:04:42.2838092Z   network time: Sun, 19 Apr 2020 21:04:42 GMT
2020-04-19T21:04:42.2868196Z Starting sccache server...
2020-04-19T21:04:42.3920989Z configure: processing command line
2020-04-19T21:04:42.3921474Z configure: 
2020-04-19T21:04:42.3922749Z configure: dist.missing-tools   := True
---
2020-04-19T21:17:41.3134210Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T21:17:43.0533737Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T21:17:44.0523693Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T21:17:50.0183681Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T21:17:58.5526506Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T21:18:03.7014572Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T21:18:09.2905332Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T21:18:14.7240418Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T21:18:24.9859448Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T21:48:34.6046771Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T21:48:37.7292633Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T21:48:39.4561168Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T21:48:52.0630715Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T21:49:05.1590953Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T21:49:15.7939268Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T21:49:25.5966276Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T21:49:35.2932196Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T21:49:52.1196201Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T22:48:31.4817050Z Testing rustbook src/doc/embedded-book
2020-04-19T22:48:32.7457048Z  finished in 1.263
2020-04-19T22:48:32.7464403Z Testing rustbook src/doc/edition-guide
2020-04-19T22:48:41.3255676Z  finished in 8.578
2020-04-19T22:50:19.2377736Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-19T22:50:19.2379314Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-19T22:50:19.2380196Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-19T22:50:19.2380992Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2457`
2020-04-19T22:50:19.2381858Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-19T22:50:19.2382557Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-19T22:50:19.2383265Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47732`
2020-04-19T22:50:19.2383975Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-19T22:50:19.2401533Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/51934`
2020-04-19T22:50:19.2402468Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/56245`
2020-04-19T22:50:19.2403244Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/RELEASES.md`
2020-04-19T22:50:19.2404098Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md`
2020-04-19T22:50:19.2405027Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-19T22:50:19.2406017Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-19T22:50:19.2407766Z Received 429 (TOO_MANY_REQUESTS) for link `https://gist.github.com/nikomatsakis/631ec8b4af9a18b5d062d9d9b7d3d967`
2020-04-19T22:50:19.2408686Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-19T22:50:19.2409533Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-19T22:50:19.2410354Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs`
2020-04-19T22:50:19.2411232Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs`
2020-04-19T22:50:19.2411983Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-19T22:50:19.2412683Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues`
2020-04-19T22:50:19.2413260Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/15702`
2020-04-19T22:50:19.2413914Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/doc/unstable-book`
2020-04-19T22:50:19.2414499Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/reference`
2020-04-19T22:50:19.2415096Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/book`
2020-04-19T22:50:19.2415738Z Received 429 (TOO_MANY_REQUESTS) for link `***-by-example`
2020-04-19T22:50:19.2416322Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/32409`
2020-04-19T22:50:19.2416910Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc`
2020-04-19T22:50:19.2417614Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md`
2020-04-19T22:50:19.2418335Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/kennytm/rustup-toolchain-install-master`
2020-04-19T22:50:19.2418985Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-19T22:50:19.2419526Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme`
2020-04-19T22:50:19.2420219Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md`
2020-04-19T22:50:19.2420879Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-19T22:50:19.2422081Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/blob/master/collector/README.md`
2020-04-19T22:50:19.2423048Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/tree/master/collector/benchmarks`
2020-04-19T22:50:19.2423668Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-19T22:50:19.2424534Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-19T22:50:19.2425263Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/fmt-rfcs`
2020-04-19T22:50:19.2426078Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs`
2020-04-19T22:50:19.2426709Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rls`
2020-04-19T22:50:19.2427329Z Received 429 (TOO_MANY_REQUESTS) for link `***fix`
2020-04-19T22:50:19.2427911Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Assignment`
2020-04-19T22:50:19.2428549Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/140`
2020-04-19T22:50:19.2429185Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/221`
2020-04-19T22:50:19.2429794Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/`
2020-04-19T22:50:19.2430451Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Pinging`
2020-04-19T22:50:19.2431147Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/triagebot.toml`
2020-04-19T22:50:19.2431809Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-Cleanup-Crew`
2020-04-19T22:50:19.2432390Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/jethrogb/rust-reduce`
2020-04-19T22:50:19.2433017Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-19T22:50:19.2434024Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-19T22:50:19.2434669Z Received 429 (TOO_MANY_REQUESTS) for link `***/`
2020-04-19T22:50:19.2435277Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-LLVM`
2020-04-19T22:50:19.2435911Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-APACHE`
2020-04-19T22:50:19.2436558Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-MIT`
2020-04-19T22:50:19.2437184Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/COPYRIGHT`
2020-04-19T22:50:19.2437806Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-19T22:50:19.2438390Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide`
2020-04-19T22:50:19.2439034Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_metadata`
2020-04-19T22:50:19.2439723Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_middle/dep_graph`
2020-04-19T22:50:19.2440360Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42678`
2020-04-19T22:50:19.2441115Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md`
2020-04-19T22:50:19.2441886Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42293`
2020-04-19T22:50:19.2442471Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42633`
2020-04-19T22:50:19.2443020Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/salsa-rs/salsa`
2020-04-19T22:50:19.2443773Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/blob/master/examples/rustc-driver-example.rs`
2020-04-19T22:50:19.2444469Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustdoc`
2020-04-19T22:50:19.2445113Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/tools/rustdoc`
2020-04-19T22:50:19.2445736Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/44136`
2020-04-19T22:50:19.2446355Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_ast`
2020-04-19T22:50:19.2447030Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_expand/mbe`
2020-04-19T22:50:19.2447647Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/41710`
2020-04-19T22:50:19.2448404Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs`
2020-04-19T22:50:19.2449252Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_macros/src/type_foldable.rs`
2020-04-19T22:50:19.2450140Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-19T22:50:19.2450993Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-19T22:50:19.2451835Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-19T22:50:19.2452528Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/22019`
2020-04-19T22:50:19.2453113Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/18290`
2020-04-19T22:50:19.2453708Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48416`
2020-04-19T22:50:19.2454289Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48416`
2020-04-19T22:50:19.2454898Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/tree/master/chalk-engine`
2020-04-19T22:50:19.2455630Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_traits`
2020-04-19T22:50:19.2456332Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_middle/traits/mod.rs`
2020-04-19T22:50:19.2456999Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-ir/src/lib.rs`
2020-04-19T22:50:19.2457807Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_middle/ty/sty.rs`
2020-04-19T22:50:19.2458453Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs`
2020-04-19T22:50:19.2459140Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2089-implied-bounds.md`
2020-04-19T22:50:19.2459784Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/43786`
2020-04-19T22:50:19.2460316Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/69247`
2020-04-19T22:50:19.2460917Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/clauses.rs`
2020-04-19T22:50:19.2461599Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_traits`
2020-04-19T22:50:19.2462200Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/chalk-solve/src/wf.rs`
2020-04-19T22:50:19.2462887Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/master/tests/test/wf_lowering.rs`
2020-04-19T22:50:19.2463654Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/chalk/blob/239e4ae4e69b2785b5f99e0f2b41fc16b0b4e65e/chalk-engine/src/README.md`
2020-04-19T22:50:19.2464362Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-19T22:50:19.2464908Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47828`
2020-04-19T22:50:19.2465454Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62474`
2020-04-19T22:50:19.2465985Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62592`
2020-04-19T22:50:19.2466520Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/`
2020-04-19T22:50:19.2467166Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-19T22:50:19.2467852Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-19T22:50:19.2468552Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-19T22:50:19.2469188Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/47809`
2020-04-19T22:50:19.2469806Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-19T22:50:19.2470489Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs`
2020-04-19T22:50:19.2471220Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-19T22:50:19.2471894Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile`
2020-04-19T22:50:19.2472786Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/test/run-make-fulldeps`
2020-04-19T22:50:19.2473497Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/test/codegen/pgo-instrumentation.rs`
2020-04-19T22:50:19.2474820Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt`
2020-04-19T22:50:19.2475838Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs`
2020-04-19T22:50:19.2477435Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs`
2020-04-19T22:50:19.2478537Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs`
2020-04-19T22:50:19.2479711Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs`
2020-04-19T22:50:19.2480779Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td`
2020-04-19T22:50:19.2482011Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs`
2020-04-19T22:50:19.2483071Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs`
2020-04-19T22:50:19.2483656Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/google/sanitizers/wiki/`
2020-04-19T22:50:19.2484449Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/gdb`
2020-04-19T22:50:19.2485270Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-19T22:50:19.2486070Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-19T22:50:19.2486789Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb`
2020-04-19T22:50:19.2487547Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb/wiki`
2020-04-19T22:50:19.2488374Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/34457`
2020-04-19T22:50:19.2489139Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/33014`
2020-04-19T22:50:19.2489886Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2117`
2020-04-19T22:50:19.2490724Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-19T22:50:19.2491046Z Timeout for link `https://web.njit.edu/~dingxn/papers/BWS.pdf`
2020-04-19T22:50:19.2496209Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-19T22:50:19.2642343Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-04-19T22:50:19.5203661Z    Compiling proc-macro2 v0.4.30
2020-04-19T22:50:19.5204907Z    Compiling unicode-xid v0.1.0
---
2020-04-19T23:03:44.2582034Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T23:03:44.2582240Z 
2020-04-19T23:03:44.2588483Z 
2020-04-19T23:03:44.2588769Z running 1 test
2020-04-19T23:03:44.7050862Z test [ui] ui-toml/zero_single_char_names/zero_single_char_names.rs ... ok
2020-04-19T23:03:44.7052213Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T23:03:44.7052500Z 
2020-04-19T23:03:44.7052734Z 
2020-04-19T23:03:44.7052896Z running 1 test
---
2020-04-19T23:19:21.0378772Z    Compiling cargo v0.45.0 (/checkout/src/tools/cargo)
2020-04-19T23:19:40.6041555Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6042944Z    --> src/tools/rls/rls/src/build/cargo.rs:359:58
2020-04-19T23:19:40.6043666Z     |
2020-04-19T23:19:40.6044582Z 359 |     fn init<'a>(&self, cx: &Context<'a, '_>, unit: &Unit<'a>) {
2020-04-19T23:19:40.6049562Z 
2020-04-19T23:19:40.6056557Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6057592Z    --> src/tools/rls/rls/src/build/cargo.rs:371:41
2020-04-19T23:19:40.6058332Z     |
2020-04-19T23:19:40.6058332Z     |
2020-04-19T23:19:40.6059185Z 371 |     fn force_rebuild(&self, unit: &Unit<'_>) -> bool {
2020-04-19T23:19:40.6063343Z 
2020-04-19T23:19:40.6072968Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6074068Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:467:16
2020-04-19T23:19:40.6074809Z     |
2020-04-19T23:19:40.6074809Z     |
2020-04-19T23:19:40.6075588Z 467 | impl From<Unit<'_>> for UnitKey {
2020-04-19T23:19:40.6079544Z 
2020-04-19T23:19:40.6086218Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6087838Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:483:16
2020-04-19T23:19:40.6089985Z     |
2020-04-19T23:19:40.6089985Z     |
2020-04-19T23:19:40.6090494Z 483 | impl From<Unit<'_>> for OwnedUnit {
2020-04-19T23:19:40.6094829Z 
2020-04-19T23:19:40.6105667Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6106494Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:134:25
2020-04-19T23:19:40.6106985Z     |
2020-04-19T23:19:40.6106985Z     |
2020-04-19T23:19:40.6107568Z 134 |         Filter: Fn(Unit<'a>) -> bool,
2020-04-19T23:19:40.6112341Z 
2020-04-19T23:19:40.6120125Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6120989Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:130:20
2020-04-19T23:19:40.6121475Z     |
2020-04-19T23:19:40.6121475Z     |
2020-04-19T23:19:40.6122028Z 130 |         unit: Unit<'a>,
2020-04-19T23:19:40.6123157Z     |                    ^^ unexpected lifetime argument
2020-04-19T23:19:40.6127153Z 
2020-04-19T23:19:40.6137569Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6138293Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:468:24
2020-04-19T23:19:40.6138778Z     |
2020-04-19T23:19:40.6139371Z 468 |     fn from(unit: Unit<'_>) -> UnitKey {
2020-04-19T23:19:40.6144083Z 
2020-04-19T23:19:40.6195025Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-19T23:19:40.6195708Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:484:24
2020-04-19T23:19:40.6196167Z     |
2020-04-19T23:19:40.6196167Z     |
2020-04-19T23:19:40.6196744Z 484 |     fn from(unit: Unit<'_>) -> OwnedUnit {
2020-04-19T23:19:40.6197765Z 
2020-04-19T23:19:40.6397756Z error: aborting due to 8 previous errors
2020-04-19T23:19:40.6404347Z 
2020-04-19T23:19:40.6411303Z For more information about this error, try `rustc --explain E0107`.
---
2020-04-19T23:28:10.1359305Z test string::test::should_break_forward ... ok
2020-04-19T23:28:10.1359594Z test string::test::should_break_on_punctuation ... ok
2020-04-19T23:28:10.1359879Z test string::test::should_break_on_whitespace ... ok
2020-04-19T23:28:10.1360178Z test string::test::significant_whitespaces ... ok
2020-04-19T23:28:10.1360517Z test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
2020-04-19T23:28:10.1360916Z test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
2020-04-19T23:28:10.1361335Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
2020-04-19T23:28:10.1361868Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
2020-04-19T23:28:10.1397367Z test string::test::retain_blank_lines ... ok
2020-04-19T23:28:10.1423303Z test test::coverage_tests ... ok
2020-04-19T23:28:10.1423754Z test test::format_lines_errors_are_reported ... ok
2020-04-19T23:28:10.1432790Z test test::format_lines_errors_are_reported_with_tabs ... ok
---
2020-04-19T23:28:33.8877033Z     |
2020-04-19T23:28:33.8878693Z 344 |             fn visit_union(&mut self, v: MPlaceTy<'tcx, Tag>, fields: usize) -> InterpResult<'tcx> {
2020-04-19T23:28:33.8881451Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::num::NonZeroUsize`, found `usize`
2020-04-19T23:28:33.8883311Z     |
2020-04-19T23:28:33.8885775Z     = note: expected fn pointer `fn(&mut helpers::EvalContextExt::visit_freeze_sensitive::UnsafeCellVisitor<'ecx, 'mir, 'tcx, F>, rustc_mir::interpret::MPlaceTy<'_, _>, std::num::NonZeroUsize) -> std::result::Result<_, _>`
2020-04-19T23:28:33.8888956Z                found fn pointer `fn(&mut helpers::EvalContextExt::visit_freeze_sensitive::UnsafeCellVisitor<'ecx, 'mir, 'tcx, F>, rustc_mir::interpret::MPlaceTy<'tcx, _>, usize) -> std::result::Result<_, _>`
2020-04-19T23:28:35.7500631Z error: aborting due to previous error
2020-04-19T23:28:35.7502342Z 
2020-04-19T23:28:35.7503617Z For more information about this error, try `rustc --explain E0053`.
2020-04-19T23:28:35.7654436Z error: could not compile `miri`.
---
2020-04-19T23:28:37.1781118Z Verifying status of miri...
2020-04-19T23:28:37.1781481Z Verifying status of embedded-book...
2020-04-19T23:28:37.1781866Z Verifying status of rustc-dev-guide...
2020-04-19T23:28:37.1833816Z Cloning into 'rust-toolstate'...
2020-04-19T23:28:37.8231634Z error: Tool `rls` has regressed from test-pass to build-fail during beta week.
2020-04-19T23:28:37.8241380Z Build completed unsuccessfully in 0:00:01
2020-04-19T23:28:37.8345925Z == clock drift check ==
2020-04-19T23:28:37.8360543Z   local time: Sun Apr 19 23:28:37 UTC 2020
2020-04-19T23:28:37.8360543Z   local time: Sun Apr 19 23:28:37 UTC 2020
2020-04-19T23:28:37.8960041Z   network time: Sun, 19 Apr 2020 23:28:37 GMT
2020-04-19T23:28:38.8105664Z 
2020-04-19T23:28:38.8105664Z 
2020-04-19T23:28:38.8190517Z ##[error]Bash exited with code '1'.
2020-04-19T23:28:38.8203015Z ##[section]Finishing: Run build
2020-04-19T23:28:38.8256101Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-19T23:28:38.8261184Z Task         : Get sources
2020-04-19T23:28:38.8261494Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T23:28:38.8261778Z Version      : 1.0.0
2020-04-19T23:28:38.8262002Z Author       : Microsoft
2020-04-19T23:28:38.8262002Z Author       : Microsoft
2020-04-19T23:28:38.8262318Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T23:28:38.8262798Z ==============================================================================
2020-04-19T23:28:39.1448294Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T23:28:39.1509905Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-19T23:28:39.1607480Z Cleaning up task key
2020-04-19T23:28:39.1609034Z Start cleaning up orphan processes.
2020-04-19T23:28:39.1783387Z Terminate orphan process: pid (5534) (python)
2020-04-19T23:28:39.1997692Z ##[section]Finishing: Finalize Job
