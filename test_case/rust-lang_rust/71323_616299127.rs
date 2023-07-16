plain
2020-04-20T01:50:04.5682207Z ========================== Starting Command Output ===========================
2020-04-20T01:50:04.5685938Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/86bed11c-cf31-4095-9f7c-b1b620d88325.sh
2020-04-20T01:50:04.5686140Z 
2020-04-20T01:50:04.5690054Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T01:50:04.5710728Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-20T01:50:04.5713838Z Task         : Get sources
2020-04-20T01:50:04.5714233Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T01:50:04.5714665Z Version      : 1.0.0
2020-04-20T01:50:04.5714815Z Author       : Microsoft
---
2020-04-20T01:50:05.5648335Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T01:50:05.5656725Z ##[command]git config gc.auto 0
2020-04-20T01:50:05.5662648Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T01:50:05.5667923Z ##[command]git config --get-all http.proxy
2020-04-20T01:50:05.5676571Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71323/merge:refs/remotes/pull/71323/merge
---
2020-04-20T01:52:34.9223760Z Looks like docker image is the same as before, not uploading
2020-04-20T01:52:42.5798652Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-20T01:52:42.6058104Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-20T01:52:42.6081506Z == clock drift check ==
2020-04-20T01:52:42.6100123Z   local time: Mon Apr 20 01:52:42 UTC 2020
2020-04-20T01:52:42.9435101Z   network time: Mon, 20 Apr 2020 01:52:42 GMT
2020-04-20T01:52:42.9459087Z Starting sccache server...
2020-04-20T01:52:43.0567940Z configure: processing command line
2020-04-20T01:52:43.0568160Z configure: 
2020-04-20T01:52:43.0568952Z configure: rust.codegen-units-std := 1
---
2020-04-20T02:05:26.5850005Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T02:05:28.3899252Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T02:05:29.3830300Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T02:05:36.8736030Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T02:05:44.7049512Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T02:05:51.6432203Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T02:05:57.4427957Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T02:06:03.2214556Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T02:06:12.8126802Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T02:38:17.2111264Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T02:38:20.7225960Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T02:38:22.7311668Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T02:38:38.6947743Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T02:38:51.5901214Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T02:39:05.4577612Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T02:39:16.9042917Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T02:39:27.5896118Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T02:39:44.1274966Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T03:39:47.4862050Z Testing rustbook src/doc/embedded-book
2020-04-20T03:39:48.6055969Z  finished in 1.119
2020-04-20T03:39:48.6062392Z Testing rustbook src/doc/edition-guide
2020-04-20T03:39:56.7661072Z  finished in 8.159
2020-04-20T03:42:12.2950296Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-20T03:42:12.2959919Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-20T03:42:12.2961435Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47732`
2020-04-20T03:42:12.2963607Z Received 429 (TOO_MANY_REQUESTS) for link `***`
2020-04-20T03:42:12.2965380Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/51934`
2020-04-20T03:42:12.2966200Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/56245`
2020-04-20T03:42:12.2967226Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/RELEASES.md`
2020-04-20T03:42:12.2968463Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md`
2020-04-20T03:42:12.2969663Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-20T03:42:12.2970867Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-20T03:42:12.2971588Z Received 429 (TOO_MANY_REQUESTS) for link `https://gist.github.com/nikomatsakis/631ec8b4af9a18b5d062d9d9b7d3d967`
2020-04-20T03:42:12.2972722Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-20T03:42:12.2973896Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
2020-04-20T03:42:12.2975818Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs`
2020-04-20T03:42:12.2977004Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs`
2020-04-20T03:42:12.2978033Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
2020-04-20T03:42:12.2979173Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues`
2020-04-20T03:42:12.2980064Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/15702`
2020-04-20T03:42:12.2981431Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/doc/unstable-book`
2020-04-20T03:42:12.2983589Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/reference`
2020-04-20T03:42:12.2984225Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/book`
2020-04-20T03:42:12.2985050Z Received 429 (TOO_MANY_REQUESTS) for link `***-by-example`
2020-04-20T03:42:12.2985987Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/32409`
2020-04-20T03:42:12.2986908Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc`
2020-04-20T03:42:12.2987713Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md`
2020-04-20T03:42:12.2988396Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/kennytm/rustup-toolchain-install-master`
2020-04-20T03:42:12.2989230Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-20T03:42:12.2990099Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme`
2020-04-20T03:42:12.2990919Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md`
2020-04-20T03:42:12.2991779Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
2020-04-20T03:42:12.2992569Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/blob/master/collector/README.md`
2020-04-20T03:42:12.2993702Z Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/tree/master/collector/benchmarks`
2020-04-20T03:42:12.2994696Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-20T03:42:12.2995430Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
2020-04-20T03:42:12.2996235Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/fmt-rfcs`
2020-04-20T03:42:12.2998462Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs`
2020-04-20T03:42:12.2999238Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rls`
2020-04-20T03:42:12.2999947Z Received 429 (TOO_MANY_REQUESTS) for link `***fix`
2020-04-20T03:42:12.3000844Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Assignment`
2020-04-20T03:42:12.3001636Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/140`
2020-04-20T03:42:12.3002218Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/221`
2020-04-20T03:42:12.3002819Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/`
2020-04-20T03:42:12.3003769Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Pinging`
2020-04-20T03:42:12.3004966Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/triagebot.toml`
2020-04-20T03:42:12.3006090Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-Cleanup-Crew`
2020-04-20T03:42:12.3008081Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/jethrogb/rust-reduce`
2020-04-20T03:42:12.3008969Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-20T03:42:12.3009673Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
2020-04-20T03:42:12.3010377Z Received 429 (TOO_MANY_REQUESTS) for link `***/`
2020-04-20T03:42:12.3011028Z Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-LLVM`
2020-04-20T03:42:12.3011732Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-APACHE`
2020-04-20T03:42:12.3012418Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-MIT`
2020-04-20T03:42:12.3013260Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/COPYRIGHT`
2020-04-20T03:42:12.3013902Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
2020-04-20T03:42:12.3014537Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide`
2020-04-20T03:42:12.3015202Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_metadata`
2020-04-20T03:42:12.3015943Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_middle/dep_graph`
2020-04-20T03:42:12.3016617Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42678`
2020-04-20T03:42:12.3017422Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md`
2020-04-20T03:42:12.3018218Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42293`
2020-04-20T03:42:12.3018854Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42633`
2020-04-20T03:42:12.3019426Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/salsa-rs/salsa`
2020-04-20T03:42:12.3020210Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/blob/master/examples/rustc-driver-example.rs`
2020-04-20T03:42:12.3020928Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustdoc`
2020-04-20T03:42:12.3021627Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/tools/rustdoc`
2020-04-20T03:42:12.3022272Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/44136`
2020-04-20T03:42:12.3022958Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_ast`
2020-04-20T03:42:12.3023648Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_expand/mbe`
2020-04-20T03:42:12.3024291Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project`
2020-04-20T03:42:12.3024957Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/59089`
2020-04-20T03:42:12.3025582Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/55835`
2020-04-20T03:42:12.3026187Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47828`
2020-04-20T03:42:12.3037080Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62474`
2020-04-20T03:42:12.3038187Z Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62592`
2020-04-20T03:42:12.3038757Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/`
2020-04-20T03:42:12.3039615Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-20T03:42:12.3040490Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-20T03:42:12.3041443Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
2020-04-20T03:42:12.3042098Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/47809`
2020-04-20T03:42:12.3042727Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-20T03:42:12.3043419Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs`
2020-04-20T03:42:12.3044112Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
2020-04-20T03:42:12.3045443Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile`
2020-04-20T03:42:12.3046204Z Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/test/run-make-fulldeps`
2020-04-20T03:42:12.3046895Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/test/codegen/pgo-instrumentation.rs`
2020-04-20T03:42:12.3048582Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt`
2020-04-20T03:42:12.3049373Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs`
2020-04-20T03:42:12.3050168Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs`
2020-04-20T03:42:12.3050980Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs`
2020-04-20T03:42:12.3051796Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs`
2020-04-20T03:42:12.3052638Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td`
2020-04-20T03:42:12.3053533Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs`
2020-04-20T03:42:12.3054367Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs`
2020-04-20T03:42:12.3054950Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/google/sanitizers/wiki/`
2020-04-20T03:42:12.3055522Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/gdb`
2020-04-20T03:42:12.3056111Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-20T03:42:12.3056675Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-20T03:42:12.3057175Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb`
2020-04-20T03:42:12.3057722Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project`
2020-04-20T03:42:12.3058296Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb/wiki`
2020-04-20T03:42:12.3058872Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/34457`
2020-04-20T03:42:12.3059415Z Received 429 (TOO_MANY_REQUESTS) for link `***/issues/33014`
2020-04-20T03:42:12.3059928Z Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2117`
2020-04-20T03:42:12.3060537Z Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
2020-04-20T03:42:12.3061123Z Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustdoc/core.rs`
2020-04-20T03:42:12.3065815Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-20T03:42:12.3332627Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-04-20T03:42:12.5772650Z    Compiling proc-macro2 v0.4.30
2020-04-20T03:42:12.5773106Z    Compiling unicode-xid v0.1.0
---
2020-04-20T03:55:44.4838169Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-20T03:55:44.4838369Z 
2020-04-20T03:55:44.4838618Z 
2020-04-20T03:55:44.4838736Z running 1 test
2020-04-20T03:55:44.9189398Z test [ui] ui-toml/zero_single_char_names/zero_single_char_names.rs ... ok
2020-04-20T03:55:44.9190680Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-20T03:55:44.9190881Z 
2020-04-20T03:55:44.9191009Z 
2020-04-20T03:55:44.9191121Z running 1 test
---
2020-04-20T04:10:29.6136997Z    Compiling cargo v0.45.0 (/checkout/src/tools/cargo)
2020-04-20T04:12:45.8035866Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8038762Z    --> src/tools/rls/rls/src/build/cargo.rs:359:58
2020-04-20T04:12:45.8039421Z     |
2020-04-20T04:12:45.8040120Z 359 |     fn init<'a>(&self, cx: &Context<'a, '_>, unit: &Unit<'a>) {
2020-04-20T04:12:45.8043175Z 
2020-04-20T04:12:45.8044540Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8046575Z    --> src/tools/rls/rls/src/build/cargo.rs:371:41
2020-04-20T04:12:45.8047249Z     |
2020-04-20T04:12:45.8047249Z     |
2020-04-20T04:12:45.8047915Z 371 |     fn force_rebuild(&self, unit: &Unit<'_>) -> bool {
2020-04-20T04:12:45.8049529Z 
2020-04-20T04:12:45.8050342Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8059406Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:467:16
2020-04-20T04:12:45.8060565Z     |
2020-04-20T04:12:45.8060565Z     |
2020-04-20T04:12:45.8062755Z 467 | impl From<Unit<'_>> for UnitKey {
2020-04-20T04:12:45.8064509Z 
2020-04-20T04:12:45.8065162Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8065882Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:483:16
2020-04-20T04:12:45.8066414Z     |
2020-04-20T04:12:45.8066414Z     |
2020-04-20T04:12:45.8066990Z 483 | impl From<Unit<'_>> for OwnedUnit {
2020-04-20T04:12:45.8068120Z 
2020-04-20T04:12:45.8068661Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8085302Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:134:25
2020-04-20T04:12:45.8086888Z     |
2020-04-20T04:12:45.8086888Z     |
2020-04-20T04:12:45.8087399Z 134 |         Filter: Fn(Unit<'a>) -> bool,
2020-04-20T04:12:45.8088292Z 
2020-04-20T04:12:45.8088723Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8089256Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:130:20
2020-04-20T04:12:45.8089631Z     |
2020-04-20T04:12:45.8089631Z     |
2020-04-20T04:12:45.8090079Z 130 |         unit: Unit<'a>,
2020-04-20T04:12:45.8090696Z     |                    ^^ unexpected lifetime argument
2020-04-20T04:12:45.8090950Z 
2020-04-20T04:12:45.8091389Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8091922Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:468:24
2020-04-20T04:12:45.8092298Z     |
2020-04-20T04:12:45.8092775Z 468 |     fn from(unit: Unit<'_>) -> UnitKey {
2020-04-20T04:12:45.8093662Z 
2020-04-20T04:12:45.8094092Z error[E0107]: wrong number of lifetime arguments: expected 0, found 1
2020-04-20T04:12:45.8094621Z    --> src/tools/rls/rls/src/build/cargo_plan.rs:484:24
2020-04-20T04:12:45.8095005Z     |
2020-04-20T04:12:45.8095005Z     |
2020-04-20T04:12:45.8095496Z 484 |     fn from(unit: Unit<'_>) -> OwnedUnit {
2020-04-20T04:12:45.8096384Z 
2020-04-20T04:12:45.8303868Z error: aborting due to 8 previous errors
2020-04-20T04:12:45.8304086Z 
2020-04-20T04:12:45.8304477Z For more information about this error, try `rustc --explain E0107`.
---
2020-04-20T04:20:19.6656320Z test string::test::should_break_on_punctuation ... ok
2020-04-20T04:20:19.6656591Z test string::test::should_break_on_whitespace ... ok
2020-04-20T04:20:19.6656851Z test string::test::significant_whitespaces ... ok
2020-04-20T04:20:19.6657244Z test string::test::should_break_forward ... ok
2020-04-20T04:20:19.6657551Z test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
2020-04-20T04:20:19.6658091Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
2020-04-20T04:20:19.6658827Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
2020-04-20T04:20:19.6659444Z test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
2020-04-20T04:20:19.6691900Z test test::coverage_tests ... ok
2020-04-20T04:20:19.6701721Z test test::format_lines_errors_are_reported ... ok
2020-04-20T04:20:19.6709479Z test test::format_lines_errors_are_reported_with_tabs ... ok
2020-04-20T04:20:19.7030285Z test test::configuration_snippet::configuration_snippet_tests ... ok
---
2020-04-20T04:20:44.2989758Z     |
2020-04-20T04:20:44.2990672Z 344 |             fn visit_union(&mut self, v: MPlaceTy<'tcx, Tag>, fields: usize) -> InterpResult<'tcx> {
2020-04-20T04:20:44.2992259Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::num::NonZeroUsize`, found `usize`
2020-04-20T04:20:44.2993253Z     |
2020-04-20T04:20:44.2994629Z     = note: expected fn pointer `fn(&mut helpers::EvalContextExt::visit_freeze_sensitive::UnsafeCellVisitor<'ecx, 'mir, 'tcx, F>, rustc_mir::interpret::MPlaceTy<'_, _>, std::num::NonZeroUsize) -> std::result::Result<_, _>`
2020-04-20T04:20:44.2996371Z                found fn pointer `fn(&mut helpers::EvalContextExt::visit_freeze_sensitive::UnsafeCellVisitor<'ecx, 'mir, 'tcx, F>, rustc_mir::interpret::MPlaceTy<'tcx, _>, usize) -> std::result::Result<_, _>`
2020-04-20T04:20:46.3438386Z error: aborting due to previous error
2020-04-20T04:20:46.3439147Z 
2020-04-20T04:20:46.3439837Z For more information about this error, try `rustc --explain E0053`.
2020-04-20T04:20:46.3584923Z error: could not compile `miri`.
---
2020-04-20T04:20:47.6567307Z Verifying status of miri...
2020-04-20T04:20:47.6567743Z Verifying status of embedded-book...
2020-04-20T04:20:47.6568231Z Verifying status of rustc-dev-guide...
2020-04-20T04:20:47.6614768Z Cloning into 'rust-toolstate'...
2020-04-20T04:20:48.3017972Z error: Tool `rls` has regressed from test-pass to build-fail during beta week.
2020-04-20T04:20:48.3027730Z Build completed unsuccessfully in 0:00:01
2020-04-20T04:20:48.3129099Z == clock drift check ==
2020-04-20T04:20:48.3142722Z   local time: Mon Apr 20 04:20:48 UTC 2020
2020-04-20T04:20:48.3142722Z   local time: Mon Apr 20 04:20:48 UTC 2020
2020-04-20T04:20:48.6342978Z   network time: Mon, 20 Apr 2020 04:20:48 GMT
2020-04-20T04:20:49.5455521Z 
2020-04-20T04:20:49.5455521Z 
2020-04-20T04:20:49.5522878Z ##[error]Bash exited with code '1'.
2020-04-20T04:20:49.5537068Z ##[section]Finishing: Run build
2020-04-20T04:20:49.5587936Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-20T04:20:49.5592502Z Task         : Get sources
2020-04-20T04:20:49.5592760Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T04:20:49.5593324Z Version      : 1.0.0
2020-04-20T04:20:49.5593490Z Author       : Microsoft
2020-04-20T04:20:49.5593490Z Author       : Microsoft
2020-04-20T04:20:49.5593731Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T04:20:49.5594009Z ==============================================================================
2020-04-20T04:20:49.8809036Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T04:20:49.8854876Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71323/merge to s
2020-04-20T04:20:49.8949165Z Cleaning up task key
2020-04-20T04:20:49.8950272Z Start cleaning up orphan processes.
2020-04-20T04:20:49.9123958Z Terminate orphan process: pid (3600) (python)
2020-04-20T04:20:49.9354181Z ##[section]Finishing: Finalize Job
