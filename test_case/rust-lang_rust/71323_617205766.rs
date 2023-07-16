plain
2020-04-21T11:32:12.3636341Z ========================== Starting Command Output ===========================
2020-04-21T11:32:12.3643058Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3f2db91b-8502-45b8-a626-6bddff1213e1.sh
2020-04-21T11:32:12.3643621Z 
2020-04-21T11:32:12.3649575Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T11:32:12.3671681Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-21T11:32:12.3675735Z Task         : Get sources
2020-04-21T11:32:12.3676041Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T11:32:12.3676339Z Version      : 1.0.0
2020-04-21T11:32:12.3676574Z Author       : Microsoft
---
2020-04-21T11:32:13.3487122Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T11:32:13.3493036Z ##[command]git config gc.auto 0
2020-04-21T11:32:13.3497042Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T11:32:13.3500777Z ##[command]git config --get-all http.proxy
2020-04-21T11:32:13.3513654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71323/merge:refs/remotes/pull/71323/merge
---
2020-04-21T11:34:14.5698456Z Looks like docker image is the same as before, not uploading
2020-04-21T11:34:22.3394857Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-21T11:34:22.3625538Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-21T11:34:22.3656758Z == clock drift check ==
2020-04-21T11:34:22.3675359Z   local time: Tue Apr 21 11:34:22 UTC 2020
2020-04-21T11:34:22.5315054Z   network time: Tue, 21 Apr 2020 11:34:22 GMT
2020-04-21T11:34:22.5335080Z Starting sccache server...
2020-04-21T11:34:22.6414387Z configure: processing command line
2020-04-21T11:34:22.6414613Z configure: 
2020-04-21T11:34:22.6416616Z configure: dist.missing-tools   := True
---
2020-04-21T11:47:57.2156921Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T11:47:59.0930537Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T11:48:00.2037968Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T11:48:08.4203999Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T11:48:17.2336800Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T11:48:23.8487005Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T11:48:30.2672829Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T11:48:36.4196742Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T11:48:46.6056530Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T12:22:27.3017452Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T12:22:30.6968285Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T12:22:32.5400379Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T12:22:49.3281012Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T12:23:01.7522431Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T12:23:16.2498018Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T12:23:26.9764691Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T12:23:37.2042620Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T12:23:53.5788856Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T13:27:42.5713748Z Testing rustbook src/doc/embedded-book
2020-04-21T13:27:43.9112963Z  finished in 1.342
2020-04-21T13:27:43.9122226Z Testing rustbook src/doc/edition-guide
2020-04-21T13:27:53.1262287Z  finished in 9.213
2020-04-21T13:29:24.6681094Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-21T13:29:24.6683628Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-21T13:29:24.6684568Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
2020-04-21T13:29:24.6685556Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2457`
2020-04-21T13:29:24.6686561Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-21T13:29:24.6687382Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-21T13:29:24.6688595Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47732`
2020-04-21T13:29:24.6689716Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-21T13:29:24.6690643Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/51934`
2020-04-21T13:29:24.6691537Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/56245`
2020-04-21T13:29:24.6692683Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/RELEASES.md`
2020-04-21T13:29:24.6693598Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md`
2020-04-21T13:29:24.6694595Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-21T13:29:24.6696560Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-21T13:29:24.6697427Z Received 429 (TOO_MANY_REQUESTS) for link `https://gist.github.com/nikomatsakis/631ec8b4af9a18b5d062d9d9b7d3d967`
2020-04-21T13:29:24.6698686Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-21T13:29:24.6700367Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-21T13:29:24.6701544Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs`
2020-04-21T13:29:24.6702980Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs`
2020-04-21T13:29:24.6703965Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-21T13:29:24.6704850Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues`
2020-04-21T13:29:24.6705704Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/15702`
2020-04-21T13:29:24.6706604Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/doc/unstable-book`
2020-04-21T13:29:24.6709619Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/reference`
2020-04-21T13:29:24.6710288Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/book`
2020-04-21T13:29:24.6711123Z Received 429 (TOO_MANY_REQUESTS) for link `***-by-example`
2020-04-21T13:29:24.6711753Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/32409`
2020-04-21T13:29:24.6712618Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc`
2020-04-21T13:29:24.6713282Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md`
2020-04-21T13:29:24.6713935Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/kennytm/rustup-toolchain-install-master`
2020-04-21T13:29:24.6714568Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-21T13:29:24.6715075Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme`
2020-04-21T13:29:24.6715746Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md`
2020-04-21T13:29:24.6716382Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-21T13:29:24.6716999Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/blob/master/collector/README.md`
2020-04-21T13:29:24.6717645Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/tree/master/collector/benchmarks`
2020-04-21T13:29:24.6718805Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-21T13:29:24.6719476Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-21T13:29:24.6720105Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/fmt-rfcs`
2020-04-21T13:29:24.6720950Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs`
2020-04-21T13:29:24.6721596Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rls`
2020-04-21T13:29:24.6722369Z Received 429 (TOO_MANY_REQUESTS) for link `***fix`
2020-04-21T13:29:24.6723039Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Assignment`
2020-04-21T13:29:24.6723831Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/140`
2020-04-21T13:29:24.6724589Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/221`
2020-04-21T13:29:24.6725171Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/`
2020-04-21T13:29:24.6725752Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Pinging`
2020-04-21T13:29:24.6726432Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/triagebot.toml`
2020-04-21T13:29:24.6727043Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-Cleanup-Crew`
2020-04-21T13:29:24.6727625Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/jethrogb/rust-reduce`
2020-04-21T13:29:24.6728209Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-21T13:29:24.6729192Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-21T13:29:24.6729879Z Received 429 (TOO_MANY_REQUESTS) for link `***/`
2020-04-21T13:29:24.6730489Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-LLVM`
2020-04-21T13:29:24.6731188Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-APACHE`
2020-04-21T13:29:24.6731834Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-MIT`
2020-04-21T13:29:24.6732505Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/COPYRIGHT`
2020-04-21T13:29:24.6733338Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-21T13:29:24.6733919Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide`
2020-04-21T13:29:24.6734510Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_metadata`
2020-04-21T13:29:24.6735179Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_middle/dep_graph`
2020-04-21T13:29:24.6735761Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42678`
2020-04-21T13:29:24.6736482Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md`
2020-04-21T13:29:24.6737191Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42293`
2020-04-21T13:29:24.6737796Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42633`
2020-04-21T13:29:24.6738444Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/salsa-rs/salsa`
2020-04-21T13:29:24.6739346Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/blob/master/examples/rustc-driver-example.rs`
2020-04-21T13:29:24.6740085Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustdoc`
2020-04-21T13:29:24.6740761Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/tools/rustdoc`
2020-04-21T13:29:24.6741430Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/44136`
2020-04-21T13:29:24.6742083Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_ast`
2020-04-21T13:29:24.6743011Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_expand/mbe`
2020-04-21T13:29:24.6743583Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/41710`
2020-04-21T13:29:24.6744300Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs`
2020-04-21T13:29:24.6745258Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_macros/src/type_foldable.rs`
2020-04-21T13:29:24.6746042Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-21T13:29:24.6746832Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-21T13:29:24.6751711Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
2020-04-21T13:29:24.6752781Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47828`
2020-04-21T13:29:24.6753343Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62474`
2020-04-21T13:29:24.6753891Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62592`
2020-04-21T13:29:24.6754453Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/`
2020-04-21T13:29:24.6757028Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-21T13:29:24.6759317Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-21T13:29:24.6760242Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-21T13:29:24.6761605Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/47809`
2020-04-21T13:29:24.6762388Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-21T13:29:24.6763319Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs`
2020-04-21T13:29:24.6764180Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-21T13:29:24.6764860Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile`
2020-04-21T13:29:24.6765667Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/test/run-make-fulldeps`
2020-04-21T13:29:24.6766416Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/test/codegen/pgo-instrumentation.rs`
2020-04-21T13:29:24.6767074Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt`
2020-04-21T13:29:24.6768036Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs`
2020-04-21T13:29:24.6768886Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs`
2020-04-21T13:29:24.6769793Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs`
2020-04-21T13:29:24.6770677Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs`
2020-04-21T13:29:24.6771748Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td`
2020-04-21T13:29:24.6772683Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs`
2020-04-21T13:29:24.6773589Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs`
2020-04-21T13:29:24.6774044Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/google/sanitizers/wiki/`
2020-04-21T13:29:24.6774703Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/gdb`
2020-04-21T13:29:24.6775364Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-21T13:29:24.6776015Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-21T13:29:24.6777013Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb`
2020-04-21T13:29:24.6777638Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb/wiki`
2020-04-21T13:29:24.6778491Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/34457`
2020-04-21T13:29:24.6779129Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/33014`
2020-04-21T13:29:24.6779772Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2117`
2020-04-21T13:29:24.6780475Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-21T13:29:24.6781327Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustdoc/core.rs`
2020-04-21T13:29:24.6784421Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-21T13:29:24.7099363Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-04-21T13:29:25.0033419Z    Compiling proc-macro2 v0.4.30
2020-04-21T13:29:25.0041967Z    Compiling unicode-xid v0.1.0
---
2020-04-21T13:44:05.6004645Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-21T13:44:05.6004936Z 
2020-04-21T13:44:05.6013865Z 
2020-04-21T13:44:05.6014149Z running 1 test
2020-04-21T13:44:06.0625773Z test [ui] ui-toml/zero_single_char_names/zero_single_char_names.rs ... ok
2020-04-21T13:44:06.0627291Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-21T13:44:06.0627512Z 
2020-04-21T13:44:06.0627627Z 
2020-04-21T13:44:06.0627762Z running 1 test
---
2020-04-21T14:01:12.7613453Z    Compiling cargo v0.45.0 (/checkout/src/tools/cargo)
2020-04-21T14:01:35.4607257Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4608630Z    --> src/tools/rls/rls/src/build/cargo.rs:359:58
2020-04-21T14:01:35.4609588Z     |
2020-04-21T14:01:35.4610662Z 359 |     fn init<'a>(&self, cx: &Context<'a, '_>, unit: &Unit<'a>) {
2020-04-21T14:01:35.4615340Z 
2020-04-21T14:01:35.4621520Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4625472Z    --> src/tools/rls/rls/src/build/cargo.rs:371:41
2020-04-21T14:01:35.4625942Z     |
2020-04-21T14:01:35.4625942Z     |
2020-04-21T14:01:35.4626511Z 371 |     fn force_rebuild(&self, unit: &Unit<'_>) -> bool {
2020-04-21T14:01:35.4634000Z 
2020-04-21T14:01:35.4657163Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4657819Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:467:16
2020-04-21T14:01:35.4658284Z     |
2020-04-21T14:01:35.4658284Z     |
2020-04-21T14:01:35.4659023Z 467 | impl From<Unit<'_>> for UnitKey {
2020-04-21T14:01:35.4660103Z 
2020-04-21T14:01:35.4660633Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4661304Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:483:16
2020-04-21T14:01:35.4661773Z     |
2020-04-21T14:01:35.4661773Z     |
2020-04-21T14:01:35.4662313Z 483 | impl From<Unit<'_>> for OwnedUnit {
2020-04-21T14:01:35.4663530Z 
2020-04-21T14:01:35.4711030Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4711815Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:134:25
2020-04-21T14:01:35.4712315Z     |
2020-04-21T14:01:35.4712315Z     |
2020-04-21T14:01:35.4712911Z 134 |         Filter: Fn(Unit<'a>) -> bool,
2020-04-21T14:01:35.4714370Z 
2020-04-21T14:01:35.4714900Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4715576Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:130:20
2020-04-21T14:01:35.4716049Z     |
2020-04-21T14:01:35.4716049Z     |
2020-04-21T14:01:35.4716588Z 130 |         unit: Unit<'a>,
2020-04-21T14:01:35.4717455Z     |                    ^^ unexpected lifetime argument
2020-04-21T14:01:35.4717738Z 
2020-04-21T14:01:35.4718240Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4719018Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:468:24
2020-04-21T14:01:35.4719497Z     |
2020-04-21T14:01:35.4720080Z 468 |     fn from(unit: Unit<'_>) -> UnitKey {
2020-04-21T14:01:35.4721184Z 
2020-04-21T14:01:35.4721713Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-21T14:01:35.4722504Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:484:24
2020-04-21T14:01:35.4723165Z     |
2020-04-21T14:01:35.4723165Z     |
2020-04-21T14:01:35.4723739Z 484 |     fn from(unit: Unit<'_>) -> OwnedUnit {
2020-04-21T14:01:35.4724834Z 
2020-04-21T14:01:35.5022422Z error: aborting due to 8 previous errors
2020-04-21T14:01:35.5022731Z 
2020-04-21T14:01:35.5023616Z For more information about this error, try `rustc --explain E0107`.
---
2020-04-21T14:11:05.8092534Z test string::test::should_break_forward ... ok
2020-04-21T14:11:05.8092951Z test string::test::should_break_on_punctuation ... ok
2020-04-21T14:11:05.8093240Z test string::test::retain_blank_lines ... ok
2020-04-21T14:11:05.8093632Z test string::test::should_break_on_whitespace ... ok
2020-04-21T14:11:05.8100675Z test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
2020-04-21T14:11:05.8123753Z test string::test::significant_whitespaces ... ok
2020-04-21T14:11:05.8125039Z test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
2020-04-21T14:11:05.8125531Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
2020-04-21T14:11:05.8126046Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
2020-04-21T14:11:05.8162517Z test test::coverage_tests ... ok
2020-04-21T14:11:05.8196695Z test test::format_lines_errors_are_reported ... ok
2020-04-21T14:11:05.8207608Z test test::format_lines_errors_are_reported_with_tabs ... ok
2020-04-21T14:11:05.8525609Z test test::configuration_snippet::configuration_snippet_tests ... ok
---
2020-04-21T14:11:32.3767949Z    --> src/tools/miri/src/diagnostics.rs:182:12
2020-04-21T14:11:32.3768708Z     |
2020-04-21T14:11:32.3769750Z 182 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.3770836Z     |            ^^^^
2020-04-21T14:11:32.3771882Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 182:6
2020-04-21T14:11:32.3775251Z     |
2020-04-21T14:11:32.3779839Z 182 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.3787252Z     |      ^^^^
2020-04-21T14:11:32.3787514Z 
---
2020-04-21T14:11:32.3793413Z   --> src/tools/miri/src/helpers.rs:15:12
2020-04-21T14:11:32.3793941Z    |
2020-04-21T14:11:32.3794748Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.3795651Z    |            ^^^^
2020-04-21T14:11:32.3796592Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 15:6
2020-04-21T14:11:32.3797955Z    |
2020-04-21T14:11:32.3798756Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.3799587Z    |      ^^^^
2020-04-21T14:11:32.3799818Z 
2020-04-21T14:11:32.3799818Z 
2020-04-21T14:11:32.3808517Z error[E0478]: lifetime bound not satisfied
2020-04-21T14:11:32.3809290Z    --> src/tools/miri/src/helpers.rs:267:35
2020-04-21T14:11:32.3809836Z     |
2020-04-21T14:11:32.3810563Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3812123Z     |
2020-04-21T14:11:32.3812870Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the impl at 267:26
2020-04-21T14:11:32.3813700Z    --> src/tools/miri/src/helpers.rs:267:26
2020-04-21T14:11:32.3814234Z     |
2020-04-21T14:11:32.3814234Z     |
2020-04-21T14:11:32.3814954Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3815758Z     |                          ^^^^
2020-04-21T14:11:32.3816607Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 267:20
2020-04-21T14:11:32.3817976Z     |
2020-04-21T14:11:32.3817976Z     |
2020-04-21T14:11:32.3818695Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3819751Z 
2020-04-21T14:11:32.3978773Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
2020-04-21T14:11:32.3979662Z    --> src/tools/miri/src/helpers.rs:267:35
2020-04-21T14:11:32.3980200Z     |
2020-04-21T14:11:32.3980200Z     |
2020-04-21T14:11:32.3980924Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3982487Z     |
2020-04-21T14:11:32.3982487Z     |
2020-04-21T14:11:32.3983269Z note: first, the lifetime cannot outlive the lifetime `'tcx` as defined on the impl at 267:26...
2020-04-21T14:11:32.3984798Z     |
2020-04-21T14:11:32.3984798Z     |
2020-04-21T14:11:32.3985541Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3987315Z note: ...so that the types are compatible
2020-04-21T14:11:32.3988183Z    --> src/tools/miri/src/helpers.rs:267:35
2020-04-21T14:11:32.3988820Z     |
2020-04-21T14:11:32.3988820Z     |
2020-04-21T14:11:32.3989543Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3990455Z     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-21T14:11:32.3991506Z     = note: expected  `rustc_mir::interpret::ValueVisitor<'mir, 'tcx, machine::Evaluator<'tcx>>`
2020-04-21T14:11:32.3992558Z                found  `rustc_mir::interpret::ValueVisitor<'_, '_, machine::Evaluator<'_>>`
2020-04-21T14:11:32.3993516Z note: but, the lifetime must be valid for the lifetime `'mir` as defined on the impl at 267:20...
2020-04-21T14:11:32.3994876Z     |
2020-04-21T14:11:32.3994876Z     |
2020-04-21T14:11:32.3995591Z 267 |         impl<'ecx, 'mir, 'tcx, F> ValueVisitor<'mir, 'tcx, Evaluator<'tcx>>
2020-04-21T14:11:32.3997017Z note: ...so that the types are compatible
2020-04-21T14:11:32.3997662Z    --> src/tools/miri/src/helpers.rs:272:22
2020-04-21T14:11:32.3998183Z     |
2020-04-21T14:11:32.3998868Z 272 |             type V = MPlaceTy<'tcx, Tag>;
---
2020-04-21T14:11:32.4022954Z   --> src/tools/miri/src/shims/dlsym.rs:23:12
2020-04-21T14:11:32.4023477Z    |
2020-04-21T14:11:32.4024304Z 23 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4025324Z    |            ^^^^
2020-04-21T14:11:32.4026137Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 23:6
2020-04-21T14:11:32.4027801Z    |
2020-04-21T14:11:32.4028746Z 23 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4029668Z    |      ^^^^
2020-04-21T14:11:32.4029896Z 
---
2020-04-21T14:11:32.4063281Z    --> src/tools/miri/src/shims/env.rs:100:12
2020-04-21T14:11:32.4063836Z     |
2020-04-21T14:11:32.4064655Z 100 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4065490Z     |            ^^^^
2020-04-21T14:11:32.4066335Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 100:6
2020-04-21T14:11:32.4075297Z     |
2020-04-21T14:11:32.4076165Z 100 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4076986Z     |      ^^^^
2020-04-21T14:11:32.4077217Z 
---
2020-04-21T14:11:32.4086899Z  --> src/tools/miri/src/shims/foreign_items/windows.rs:8:12
2020-04-21T14:11:32.4087466Z   |
2020-04-21T14:11:32.4092214Z 8 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4093053Z   |            ^^^^
2020-04-21T14:11:32.4093904Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 8:6
2020-04-21T14:11:32.4095523Z   |
2020-04-21T14:11:32.4096348Z 8 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4097162Z   |      ^^^^
2020-04-21T14:11:32.4097399Z 
---
2020-04-21T14:11:32.4103303Z  --> src/tools/miri/src/shims/foreign_items/posix/linux.rs:4:12
2020-04-21T14:11:32.4103868Z   |
2020-04-21T14:11:32.4104701Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4105596Z   |            ^^^^
2020-04-21T14:11:32.4106402Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 4:6
2020-04-21T14:11:32.4107469Z  --> src/tools/miri/src/shims/foreign_items/posix/linux.rs:4:6
2020-04-21T14:11:32.4108878Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4109672Z   |      ^^^^
2020-04-21T14:11:32.4109899Z 
2020-04-21T14:11:32.4110412Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T14:11:32.4115571Z  --> src/tools/miri/src/shims/foreign_items/posix/macos.rs:4:12
2020-04-21T14:11:32.4116119Z   |
2020-04-21T14:11:32.4120829Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4121715Z   |            ^^^^
2020-04-21T14:11:32.4122537Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 4:6
2020-04-21T14:11:32.4127374Z  --> src/tools/miri/src/shims/foreign_items/posix/macos.rs:4:6
2020-04-21T14:11:32.4132412Z 4 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4137214Z   |      ^^^^
2020-04-21T14:11:32.4137460Z 
2020-04-21T14:11:32.4138011Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T14:11:32.4151344Z   --> src/tools/miri/src/shims/foreign_items/posix.rs:12:12
2020-04-21T14:11:32.4152027Z    |
2020-04-21T14:11:32.4152876Z 12 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4153716Z    |            ^^^^
2020-04-21T14:11:32.4154549Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 12:6
2020-04-21T14:11:32.4155954Z    |
2020-04-21T14:11:32.4156771Z 12 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4157589Z    |      ^^^^
2020-04-21T14:11:32.4157819Z 
---
2020-04-21T14:11:32.4165631Z   --> src/tools/miri/src/shims/foreign_items.rs:15:12
2020-04-21T14:11:32.4166188Z    |
2020-04-21T14:11:32.4166994Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4167816Z    |            ^^^^
2020-04-21T14:11:32.4168651Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 15:6
2020-04-21T14:11:32.4170016Z    |
2020-04-21T14:11:32.4170825Z 15 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4171815Z    |      ^^^^
2020-04-21T14:11:32.4172064Z 
2020-04-21T14:11:32.4172064Z 
2020-04-21T14:11:32.4172584Z error[E0478]: lifetime bound not satisfied
2020-04-21T14:11:32.4173240Z   --> src/tools/miri/src/shims/fs.rs:67:18
2020-04-21T14:11:32.4173756Z    |
2020-04-21T14:11:32.4174699Z 67 | impl<'mir, 'tcx> EvalContextExtPrivate<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4176242Z    |
2020-04-21T14:11:32.4176989Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the impl at 67:12
2020-04-21T14:11:32.4177782Z   --> src/tools/miri/src/shims/fs.rs:67:12
2020-04-21T14:11:32.4178320Z    |
2020-04-21T14:11:32.4178320Z    |
2020-04-21T14:11:32.4179152Z 67 | impl<'mir, 'tcx> EvalContextExtPrivate<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4179980Z    |            ^^^^
2020-04-21T14:11:32.4180820Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 67:6
2020-04-21T14:11:32.4182129Z    |
2020-04-21T14:11:32.4182129Z    |
2020-04-21T14:11:32.4182978Z 67 | impl<'mir, 'tcx> EvalContextExtPrivate<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4184045Z 
2020-04-21T14:11:32.4184579Z error[E0478]: lifetime bound not satisfied
2020-04-21T14:11:32.4185252Z    --> src/tools/miri/src/shims/fs.rs:235:18
2020-04-21T14:11:32.4185784Z     |
---
2020-04-21T14:11:32.4189949Z    --> src/tools/miri/src/shims/fs.rs:235:12
2020-04-21T14:11:32.4190510Z     |
2020-04-21T14:11:32.4191322Z 235 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4192145Z     |            ^^^^
2020-04-21T14:11:32.4192986Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 235:6
2020-04-21T14:11:32.4194313Z     |
2020-04-21T14:11:32.4195185Z 235 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4196012Z     |      ^^^^
2020-04-21T14:11:32.4196370Z 
---
2020-04-21T14:11:32.4202202Z   --> src/tools/miri/src/shims/intrinsics.rs:10:12
2020-04-21T14:11:32.4202747Z    |
2020-04-21T14:11:32.4203585Z 10 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4204417Z    |            ^^^^
2020-04-21T14:11:32.4205243Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 10:6
2020-04-21T14:11:32.4206605Z    |
2020-04-21T14:11:32.4207432Z 10 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4208236Z    |      ^^^^
2020-04-21T14:11:32.4208463Z 
---
2020-04-21T14:11:32.4214036Z   --> src/tools/miri/src/shims/os_str.rs:63:12
2020-04-21T14:11:32.4214560Z    |
2020-04-21T14:11:32.4215385Z 63 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4216232Z    |            ^^^^
2020-04-21T14:11:32.4217066Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 63:6
2020-04-21T14:11:32.4222943Z   --> src/tools/miri/src/shims/os_str.rs:63:6
2020-04-21T14:11:32.4228044Z 63 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4229039Z    |      ^^^^
2020-04-21T14:11:32.4229280Z 
2020-04-21T14:11:32.4229844Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T14:11:32.4235276Z   --> src/tools/miri/src/shims/panic.rs:34:12
2020-04-21T14:11:32.4235834Z    |
2020-04-21T14:11:32.4236639Z 34 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4237485Z    |            ^^^^
2020-04-21T14:11:32.4238304Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 34:6
2020-04-21T14:11:32.4239626Z    |
2020-04-21T14:11:32.4240428Z 34 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4241246Z    |      ^^^^
2020-04-21T14:11:32.4241473Z 
---
2020-04-21T14:11:32.4247046Z    --> src/tools/miri/src/shims/sync.rs:203:12
2020-04-21T14:11:32.4247577Z     |
2020-04-21T14:11:32.4248387Z 203 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4249226Z     |            ^^^^
2020-04-21T14:11:32.4250035Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 203:6
2020-04-21T14:11:32.4250826Z    --> src/tools/miri/src/shims/sync.rs:203:6
2020-04-21T14:11:32.4252178Z 203 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4253006Z     |      ^^^^
2020-04-21T14:11:32.4253241Z 
2020-04-21T14:11:32.4253761Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T14:11:32.4258931Z   --> src/tools/miri/src/shims/time.rs:17:12
2020-04-21T14:11:32.4259457Z    |
2020-04-21T14:11:32.4260350Z 17 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4261218Z    |            ^^^^
2020-04-21T14:11:32.4262035Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 17:6
2020-04-21T14:11:32.4262833Z   --> src/tools/miri/src/shims/time.rs:17:6
2020-04-21T14:11:32.4264188Z 17 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4264993Z    |      ^^^^
2020-04-21T14:11:32.4265249Z 
2020-04-21T14:11:32.4265763Z error[E0478]: lifetime bound not satisfied
---
2020-04-21T14:11:32.4271038Z    --> src/tools/miri/src/shims/tls.rs:157:12
2020-04-21T14:11:32.4271592Z     |
2020-04-21T14:11:32.4272398Z 157 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4273247Z     |            ^^^^
2020-04-21T14:11:32.4274081Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 157:6
2020-04-21T14:11:32.4275444Z     |
2020-04-21T14:11:32.4276261Z 157 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4277087Z     |      ^^^^
2020-04-21T14:11:32.4277344Z 
---
2020-04-21T14:11:32.4283122Z   --> src/tools/miri/src/shims/mod.rs:20:12
2020-04-21T14:11:32.4283660Z    |
2020-04-21T14:11:32.4284465Z 20 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4285297Z    |            ^^^^
2020-04-21T14:11:32.4286234Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 20:6
2020-04-21T14:11:32.4287571Z    |
2020-04-21T14:11:32.4288401Z 20 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4289222Z    |      ^^^^
2020-04-21T14:11:32.4289449Z 
2020-04-21T14:11:32.4289449Z 
2020-04-21T14:11:32.4289977Z error[E0478]: lifetime bound not satisfied
2020-04-21T14:11:32.4290660Z    --> src/tools/miri/src/stacked_borrows.rs:509:18
2020-04-21T14:11:32.4291199Z     |
2020-04-21T14:11:32.4292047Z 509 | impl<'mir, 'tcx> EvalContextPrivExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4293516Z     |
2020-04-21T14:11:32.4294283Z note: lifetime parameter instantiated with the lifetime `'tcx` as defined on the impl at 509:12
2020-04-21T14:11:32.4295099Z    --> src/tools/miri/src/stacked_borrows.rs:509:12
2020-04-21T14:11:32.4295656Z     |
2020-04-21T14:11:32.4295656Z     |
2020-04-21T14:11:32.4296496Z 509 | impl<'mir, 'tcx> EvalContextPrivExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4297335Z     |            ^^^^
2020-04-21T14:11:32.4298172Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 509:6
2020-04-21T14:11:32.4299525Z     |
2020-04-21T14:11:32.4299525Z     |
2020-04-21T14:11:32.4300364Z 509 | impl<'mir, 'tcx> EvalContextPrivExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4301437Z 
2020-04-21T14:11:32.4301969Z error[E0478]: lifetime bound not satisfied
2020-04-21T14:11:32.4302661Z    --> src/tools/miri/src/stacked_borrows.rs:610:18
2020-04-21T14:11:32.4303201Z     |
---
2020-04-21T14:11:32.4307312Z    --> src/tools/miri/src/stacked_borrows.rs:610:12
2020-04-21T14:11:32.4308054Z     |
2020-04-21T14:11:32.4308895Z 610 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4309708Z     |            ^^^^
2020-04-21T14:11:32.4310540Z note: but lifetime parameter must outlive the lifetime `'mir` as defined on the impl at 610:6
2020-04-21T14:11:32.4312032Z     |
2020-04-21T14:11:32.4312863Z 610 | impl<'mir, 'tcx> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
2020-04-21T14:11:32.4313671Z     |      ^^^^
2020-04-21T14:11:32.4313903Z 
---
2020-04-21T14:11:33.9806483Z Verifying status of miri...
2020-04-21T14:11:33.9806859Z Verifying status of embedded-book...
2020-04-21T14:11:33.9807234Z Verifying status of rustc-dev-guide...
2020-04-21T14:11:33.9860233Z Cloning into 'rust-toolstate'...
2020-04-21T14:11:34.9273424Z error: Tool `rls` has regressed from test-pass to build-fail during beta week.
2020-04-21T14:11:34.9284356Z Build completed unsuccessfully in 0:00:02
2020-04-21T14:11:34.9399497Z == clock drift check ==
2020-04-21T14:11:34.9399497Z == clock drift check ==
2020-04-21T14:11:34.9415362Z   local time: Tue Apr 21 14:11:34 UTC 2020
2020-04-21T14:11:35.2724780Z   network time: Tue, 21 Apr 2020 14:11:35 GMT
2020-04-21T14:11:36.0559861Z 
2020-04-21T14:11:36.0559861Z 
2020-04-21T14:11:36.0645637Z ##[error]Bash exited with code '1'.
2020-04-21T14:11:36.0669575Z ##[section]Finishing: Run build
2020-04-21T14:11:36.0723769Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-21T14:11:36.0729846Z Task         : Get sources
2020-04-21T14:11:36.0730165Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T14:11:36.0730698Z Version      : 1.0.0
2020-04-21T14:11:36.0730915Z Author       : Microsoft
2020-04-21T14:11:36.0730915Z Author       : Microsoft
2020-04-21T14:11:36.0731253Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T14:11:36.0731649Z ==============================================================================
2020-04-21T14:11:36.4166091Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T14:11:36.4212871Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-21T14:11:36.4302903Z Cleaning up task key
2020-04-21T14:11:36.4304018Z Start cleaning up orphan processes.
2020-04-21T14:11:36.4482230Z Terminate orphan process: pid (3569) (python)
2020-04-21T14:11:36.4728618Z ##[section]Finishing: Finalize Job
