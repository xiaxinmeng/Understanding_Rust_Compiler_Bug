plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5360f0ac-5005-4a0d-91ec-aca60d341270.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71875/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71875/merge:refs/remotes/pull/71875/merge
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
 finished in 61.650
Testing rustbook src/doc/nomicon
Error: Rustdoc returned an error: 
running 1 test
test /tmp/mdbook-P6AGfS/vec-final.md - The_Final_Code (line 3) ... FAILED
failures:


---- /tmp/mdbook-P6AGfS/vec-final.md - The_Final_Code (line 3) stdout ----
error[E0599]: no function or associated item named `empty` found for struct `std::ptr::Unique<_>` in the current scope
  --> /tmp/mdbook-P6AGfS/vec-final.md:33:31
   |
31 |         RawVec { ptr: Unique::empty(), cap: cap }
   |                               ^^^^^ function or associated item not found in `std::ptr::Unique<_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    /tmp/mdbook-P6AGfS/vec-final.md - The_Final_Code (line 3)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out



---
Testing rustbook src/doc/embedded-book
 finished in 1.181
Testing rustbook src/doc/edition-guide
 finished in 8.082
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/51336`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/51587`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2457`
Received 429 (TOO_MANY_REQUESTS) for link `***`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
Received 429 (TOO_MANY_REQUESTS) for link `***`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47732`
Received 429 (TOO_MANY_REQUESTS) for link `***`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/51336`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/51587`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/51934`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/48075`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/56245`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/RELEASES.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://gist.github.com/nikomatsakis/631ec8b4af9a18b5d062d9d9b7d3d967`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/15702`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/doc/unstable-book`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/reference`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/book`
Received 429 (TOO_MANY_REQUESTS) for link `***-by-example`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/32409`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/kennytm/rustup-toolchain-install-master`
Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md`
Received 429 (TOO_MANY_REQUESTS) for link `***c-perf`
Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/blob/master/collector/README.md`
Received 429 (TOO_MANY_REQUESTS) for link `***c-perf/tree/master/collector/benchmarks`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/perf-focus`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/fmt-rfcs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rls`
Received 429 (TOO_MANY_REQUESTS) for link `***fix`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Assignment`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/140`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/team/pull/221`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/triagebot/wiki/Pinging`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/triagebot.toml`
Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-Cleanup-Crew`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/jethrogb/rust-reduce`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/cargo-bisect-rustc/`
Received 429 (TOO_MANY_REQUESTS) for link `***/`
Received 429 (TOO_MANY_REQUESTS) for link `***/labels/ICEBreaker-LLVM`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-APACHE`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/LICENSE-MIT`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/COPYRIGHT`
Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/issues`
Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_metadata`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_middle/dep_graph`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42678`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42293`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/42633`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/salsa-rs/salsa`
Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/blob/master/examples/rustc-driver-example.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustdoc`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/tools/rustdoc`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/44136`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_ast`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/librustc_expand/mbe`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/41710`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustc_macros/src/type_foldable.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/47828`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62474`
Received 429 (TOO_MANY_REQUESTS) for link `***/pull/62592`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/47809`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/1.34.1/src/rustllvm/PassWrapper.cpp`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile`
Received 429 (TOO_MANY_REQUESTS) for link `***/tree/master/src/test/run-make-fulldeps`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/test/codegen/pgo-instrumentation.rs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/llvm/llvm-project/tree/master/compiler-rt`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/google/sanitizers/wiki/`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-dev-tools/gdb`
Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/lldb/wiki`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/34457`
Received 429 (TOO_MANY_REQUESTS) for link `***/issues/33014`
Received 429 (TOO_MANY_REQUESTS) for link `https://github.com/rust-lang/rfcs/pull/2117`
Received 429 (TOO_MANY_REQUESTS) for link `***c-dev-guide/pull/316`
Received 429 (TOO_MANY_REQUESTS) for link `***/blob/master/src/librustdoc/core.rs`
error: The server responded with 429 Too Many Requests for "***/pull/51336"
    ┌── walkthrough.md:55:27 ───
    │
    │
 55 │   went [through][impl2] a [number][impl3] of [iterations][impl4].
    │


error: The server responded with 429 Too Many Requests for "***/pull/51587"
    ┌── walkthrough.md:55:46 ───
    │
    │
 55 │   went [through][impl2] a [number][impl3] of [iterations][impl4].
    │


error: The server responded with 429 Too Many Requests for "***/issues/48075"
    ┌── walkthrough.md:57:3 ───
    │
    │
 57 │   [propose to stabilize it][merge]. If there is consensus, this is done.
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"


    ┌── walkthrough.md:74:64 ───
    │
 74 │ > You can find the official guidelines for when to open an RFC [here][rfcwhen].
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"


    ┌── walkthrough.md:83:1 ───
    │
 83 │ [rust-lang/rfcs](https://github.com/rust-lang/rfcs) repo on GitHub. You can
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"


    ┌── walkthrough.md:85:1 ───
    │
 85 │ [README](https://github.com/rust-lang/rfcs#what-the-process-is).
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2457"


     ┌── walkthrough.md:107:51 ───
     │
 107 │ ideas, a lot more discussion can happen (e.g. see [this RFC][nonascii] which
     │

error: The server responded with 429 Too Many Requests for "***"


     ┌── walkthrough.md:146:26 ───
     │
 146 │ issue_ is created in the [rust-lang/rust] repo to track progress on the feature
     │                          ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/issues/48075"
     ┌── walkthrough.md:148:39 ───
     │
     │
 148 │ Here is the tracking issue on for our [`?` macro feature][tracking].
     │

error: The server responded with 429 Too Many Requests for "***"


     ┌── walkthrough.md:156:57 ───
     │
 156 │ To make a change to the compiler, open a PR against the [rust-lang/rust] repo.
     │                                                         ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/pull/47732"
     ┌── walkthrough.md:167:58 ───
     │
 167 │ macro expansion in the compiler. Personally, I find that [improving the
     │                                                          ^ Server responded with 429 Too Many Requests
     │                                                          ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***"

     ┌── walkthrough.md:181:27 ───
     │
 181 │ When you open a PR on the [rust-lang/rust], a bot will assign your PR to a
     │


error: The server responded with 429 Too Many Requests for "***/pull/51336"
     ┌── walkthrough.md:229:38 ───
     │
     │
 229 │ original implementation: [1][impl2], [2][impl3], [3][impl4].
     │


error: The server responded with 429 Too Many Requests for "***/pull/51587"
     ┌── walkthrough.md:229:50 ───
     │
     │
 229 │ original implementation: [1][impl2], [2][impl3], [3][impl4].
     │


error: The server responded with 429 Too Many Requests for "***/issues/51934"
     ┌── walkthrough.md:237:32 ───
     │
 237 │   from the original RFC required [another
     │ ╭────────────────────────────────^
     │ ╭────────────────────────────────^
 238 │ │ FCP](***/issues/51934).
     │


error: The server responded with 429 Too Many Requests for "***/issues/48075"
     ┌── walkthrough.md:243:1 ───
     │
     │
 243 │ [moved to stabilize it][stabilizefcp].
     │


error: The server responded with 429 Too Many Requests for "***/issues/48075"
     ┌── walkthrough.md:253:45 ───
     │
     │
 253 │ The stabilization report for our feature is [here][stabrep].
     │


error: The server responded with 429 Too Many Requests for "***/pull/56245"
     ┌── walkthrough.md:257:13 ───
     │
     │
 257 │ After this, [a PR is made][stab] to remove the feature gate, enabling the feature by
     │


error: The server responded with 429 Too Many Requests for "***/blob/master/RELEASES.md"
     ┌── walkthrough.md:258:55 ───
     │
     │
 258 │ default (on the 2018 edition). A note is added to the [Release notes][relnotes]
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md"

---
 81 │ example of how such an issue should look can be [found
    │                                                 ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
     ┌── bug-fix-procedure.md:237:65 ───
     │
 237 │ The first reference you will likely find is the lint definition [in
     │                                                                 ^ Server responded with 429 Too Many Requests
     │                                                                 ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
     ┌── bug-fix-procedure.md:252:13 ───
     │
     │
 252 │ the file as [part of a `lint_array!`][lintarraysource]; remove it too,
     │


error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs"
     ┌── bug-fix-procedure.md:256:19 ───
     │
 256 │ Next, you see see [a reference to `OVERLAPPING_INHERENT_IMPLS` in
     │                   ^ Server responded with 429 Too Many Requests
     │                   ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs"
     ┌── bug-fix-procedure.md:285:16 ───
     │
     │
 285 │ this case, the [`add_lint` call][addlintsource] looks like this:
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md"


    ┌── implementing_new_features.md:56:4 ───
    │
 56 │ We [value the stability of Rust]. Code that works and runs on stable
    │    ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/issues"
    ┌── stability.md:18:51 ───
    │
 18 │ The `issue` field specifies the associated GitHub [issue number]. This field is
    │                                                   ^ Server responded with 429 Too Many Requests
    │                                                   ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/issues/15702"
    ┌── stability.md:31:30 ───
    │
    │
 31 │ Note, however, that due to a [rustc bug], stable items inside unstable modules
    │


error: The server responded with 429 Too Many Requests for "***/tree/master/src/doc/unstable-book"
    ┌── stabilization_guide.md:17:38 ───
    │
    │
 17 │ in the [`Unstable Book`], located at [`src/doc/unstable-book`].
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/reference"

---
 28 │ - [The Book]: This may or may not need updating, depends.
    │   ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***-by-example"
    ┌── stabilization_guide.md:35:3 ───
    │
 35 │ - [Rust by Example]: As needed.
    │   ^ Server responded with 429 Too Many Requests
    │   ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/issues/32409"
    ┌── stabilization_guide.md:97:1 ───
    │
 97 │ [rust-lang/rust#32409]:
    │ ^ Server responded with 429 Too Many Requests
    │ ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/cargo-bisect-rustc"

     ┌── compiler-debugging.md:257:5 ───
     │
 257 │ The [cargo-bisect-rustc][bisect] tool can be used as a quick and easy way to
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md"


     ┌── compiler-debugging.md:261:31 ───
     │
 261 │ on *why* it was changed.  See [this tutorial][bisect-tutorial] on how to use
     │

error: The server responded with 429 Too Many Requests for "https://github.com/kennytm/rustup-toolchain-install-master"


     ┌── compiler-debugging.md:269:5 ───
     │
 269 │ The [rustup-toolchain-install-master][rtim] tool by kennytm can be used to
     │


error: The server responded with 429 Too Many Requests for "***c-perf"
   ┌── profiling.md:8:9 ───
   │
   │
 8 │   - The [rustc-perf](***c-perf) project makes this easy and can be triggered to run on a PR via the `@rustc-perf` bot.
   │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme"


    ┌── profiling.md:11:35 ───
    │
 11 │   - The `-Zself-profile` flag and [measureme](https://github.com/rust-lang/measureme) tools offer a query-based approach to profiling.
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md"


    ┌── profiling.md:12:9 ───
    │
 12 │     See [their docs](https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md) for more information.
    │         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***c-perf"
    ┌── profiling/with_perf.md:59:1 ───
    │
 59 │ [the rustc-perf repository][rustc-perf-gh]:
    │ ^ Server responded with 429 Too Many Requests
    │ ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***c-perf/blob/master/collector/README.md"
    ┌── profiling/with_perf.md:71:1 ───
    │
 71 │ [instructions in the rustc-perf readme][rustc-perf-readme].
    │ ^ Server responded with 429 Too Many Requests
    │ ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***c-perf/tree/master/collector/benchmarks"
    ┌── profiling/with_perf.md:93:14 ───
    │
    │
 93 │ are found in [the `collector/benchmarks` directory][dir]. So let's go
    │

error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"


     ┌── profiling/with_perf.md:137:45 ───
     │
 137 │ helpful. For more detailed examination, the [`perf-focus` tool][pf]
     │

error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"


     ┌── profiling/with_perf.md:161:38 ───
     │
 161 │ about it. For this, I personally use [perf focus][pf]. It's a kind of
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-dev-tools/fmt-rfcs"


    ┌── conventions.md:10:36 ───
    │
 10 │ rustc is slowly moving towards the [Rust standard coding style][fmt];
    │


error: The server responded with 429 Too Many Requests for "***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs"
    ┌── crates-io.md:19:23 ───
    │
    │
 19 │ The `tidy` tool has a [whitelist] of crates that are allowed. To add a
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rls"


    ┌── diagnostics.md:81:63 ───
    │
 81 │ is passed) as JSON for consumption by tools, most notably the [Rust Language
    │                                                               ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***fix"
    ┌── diagnostics.md:82:18 ───
    │
    │
 82 │ Server][rls] and [`rustfix`][rustfix].
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/triagebot/wiki/Assignment"

---
 57 │ [rustbot] a [`ping`] command with the name of the ICE-breakers
    │             ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/blob/master/triagebot.toml"
    ┌── ice-breaker/about.md:66:16 ───
    │
    │
 66 │ defined in the [`triagebot.toml`] file. For example:
    │


error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-Cleanup-Crew"
   ┌── ice-breaker/cleanup-crew.md:3:19 ───
   │
 3 │ **Github Label:** [ICEBreaker-Cleanup-Crew]
   │                   ^ Server responded with 429 Too Many Requests
---
 80 │ To learn to use [cargo-bisect-rustc], check out [this blog
    │                 ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/"
     ┌── ice-breaker/cleanup-crew.md:102:36 ───
     │
 102 │ 1. Go to an update checkout of the [rust-lang/rust] repository
     │                                    ^ Server responded with 429 Too Many Requests
     │                                    ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-LLVM"
   ┌── ice-breaker/llvm.md:3:19 ───
   │
 3 │ **Github Label:** [ICEBreaker-LLVM]
   │                   ^ Server responded with 429 Too Many Requests
   │                   ^ Server responded with 429 Too Many Requests
   │

error: The server responded with 429 Too Many Requests for "***/blob/master/LICENSE-APACHE"
   ┌── licenses.md:3:78 ───
   │
   │
 3 │ The `rustc` compiler source and standard library are dual licensed under the [Apache License v2.0](***/blob/master/LICENSE-APACHE) and the [MIT License](***/blob/master/LICENSE-MIT) unless otherwise specified.
   │


error: The server responded with 429 Too Many Requests for "***/blob/master/LICENSE-MIT"
   ┌── licenses.md:3:170 ───
   │
   │
 3 │ The `rustc` compiler source and standard library are dual licensed under the [Apache License v2.0](***/blob/master/LICENSE-APACHE) and the [MIT License](***/blob/master/LICENSE-MIT) unless otherwise specified.
   │


error: The server responded with 429 Too Many Requests for "***/blob/master/COPYRIGHT"
   ┌── licenses.md:5:52 ───
   │
   │
 5 │ Detailed licensing information is available in the [COPYRIGHT document](***/blob/master/COPYRIGHT) of the `rust-lang/rust` repository.
   │


error: The server responded with 429 Too Many Requests for "***c-dev-guide/issues"
   ┌── part-2-intro.md:8:25 ───
   │
 8 │   to file an issue on the [rustc-dev-guide
   │ ╭─────────────────────────^
   │ ╭─────────────────────────^
 9 │ │ repo](***c-dev-guide/issues) or contact the compiler
   │


error: The server responded with 429 Too Many Requests for "***c-dev-guide"
   ┌── overview.md:3:134 ───
   │
   │
 3 │ Coming soon!  Work is in progress on this chapter.  See ***c-dev-guide/pull/633 for the source and the [project README](***c-dev-guide) for local build instructions.
   │


error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_metadata"
     ┌── query.md:155:1 ───
     │
 155 │ [`rustc_metadata` crate][rustc_metadata], which loads the information
     │ ^ Server responded with 429 Too Many Requests
     │ ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_middle/dep_graph"
    ┌── queries/incremental-compilation.md:84:1 ───
    │
    │
 84 │ [`src/librustc_middle/dep_graph`][dep_graph]. Construction of the DAG is done
    │


error: The server responded with 429 Too Many Requests for "***/issues/42678"
   ┌── queries/profiling.md:8:9 ───
   │
   │
 8 │ address [issue 42678](***/issues/42678).
   │

error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md"


     ┌── queries/profiling.md:335:3 ───
     │
 335 │   [On-demand Rustc incremental design doc](https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md)
     │


error: The server responded with 429 Too Many Requests for "***/issues/42293"
     ┌── queries/profiling.md:337:3 ───
     │
     │
 337 │   ["Red/Green" dependency tracking in compiler](***/issues/42293)
     │


error: The server responded with 429 Too Many Requests for "***/issues/42633"
     ┌── queries/profiling.md:341:3 ───
     │
     │
 341 │ - [GitHub issue #42633](***/issues/42633)
     │

error: The server responded with 429 Too Many Requests for "https://github.com/salsa-rs/salsa"


   ┌── salsa.md:5:1 ───
   │
 5 │ [Salsa](https://github.com/salsa-rs/salsa).
   │


error: The server responded with 429 Too Many Requests for "***c-dev-guide/blob/master/examples/rustc-driver-example.rs"
    ┌── rustc-driver.md:17:63 ───
    │
    │
 17 │ You can see a minimal example of how to use `rustc_interface` [here][example].
    │


error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustdoc"
   ┌── rustdoc.md:6:50 ───
   │
   │
 6 │ Rustdoc is implemented entirely within the crate [`librustdoc`][rd]. It runs
   │


error: The server responded with 429 Too Many Requests for "***/tree/master/src/tools/rustdoc"
    ┌── rustdoc.md:26:22 ───
    │
 26 │ using the project in [`src/tools/rustdoc`][bin]. Note that literally all that
    │                      ^ Server responded with 429 Too Many Requests
    │                      ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/issues/44136"
     ┌── rustdoc.md:115:1 ───
     │
     │
 115 │ [we're trying to deprecate that][44136]. If you need finer-grain control over
     │


error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_ast"
    ┌── test-implementation.md:35:1 ───
    │
 35 │ [`librustc_ast` crate][librustc_ast]. Essentially, it's a fancy macro, that
    │ ^ Server responded with 429 Too Many Requests
    │ ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_expand/mbe"
    ┌── macro-expansion.md:13:1 ───
    │
 13 │ [`src/librustc_expand/mbe/`][code_dir]. This chapter aims to explain how macro
    │ ^ Server responded with 429 Too Many Requests
    │ ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/issues/41710"
    ┌── mir/passes.md:96:17 ───
    │
 96 │ alternatives in [rust-lang/rust#41710].
    │                 ^ Server responded with 429 Too Many Requests
    │                 ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs"
     ┌── generics.md:131:1 ───
     │
     │
 131 │ [Here is an example of actually using `subst` in the compiler][substex].  The exact details are not
     │


error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_macros/src/type_foldable.rs"
    ┌── ty-fold.md:93:1 ───
    │
    │
 93 │ [here](***/blob/master/src/librustc_macros/src/type_foldable.rs).
    │


error: The server responded with 429 Too Many Requests for "***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs"
    ┌── ty-fold.md:95:46 ───
    │
    │
 95 │   **`subst`** In the case of substitutions the [actual
    │ ╭──────────────────────────────────────────────^
 96 │ │ folder](***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs#L440-L451)
    │


error: The server responded with 429 Too Many Requests for "***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs"
    ┌── ty-fold.md:99:1 ───
    │
    │
 99 │ [fold_ty](***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs#L512-L536)
    │


error: The server responded with 429 Too Many Requests for "***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs"
     ┌── ty-fold.md:103:1 ───
     │
     │
 103 │ [ty_for_param](***/blob/75ff3110ac6d8a0259023b83fd20d7ab295f8dd6/src/librustc_middle/ty/subst.rs#L552-L587)
     │

error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_traits/lowering/"


   ┌── traits/lowering-module.md:5:16 ───
   │
 5 │ created in the [`rustc_traits::lowering`][lowering] module.
   │


error: The server responded with 429 Too Many Requests for "***/pull/47828"
     ┌── backend/updating-llvm.md:129:1 ───
     │
     │
 129 │ [#47828](***/pull/47828)
     │


error: The server responded with 429 Too Many Requests for "***/pull/62474"
     ┌── backend/updating-llvm.md:130:1 ───
     │
     │
 130 │ [#62474](***/pull/62474)
     │


error: The server responded with 429 Too Many Requests for "***/pull/62592"
     ┌── backend/updating-llvm.md:131:1 ───
     │
     │
 131 │ [#62592](***/pull/62592). Note that sometimes it's
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/"

---
error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md"

     ┌── codegen/implicit-caller-location.md:246:1 ───
     │
 246 │ [non-viable alternatives] in the approved RFC for details). It took two more years until RFC 2091
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md"


     ┌── codegen/implicit-caller-location.md:247:27 ───
     │
 247 │ was approved, much of its [rationale] for this feature's design having been discovered through the
     │


error: The server responded with 429 Too Many Requests for "***/issues/47809"
     ┌── codegen/implicit-caller-location.md:252:59 ───
     │
     │
 252 │ approval of the RFC and the actual implementation work, a [revised design] was proposed and written
     │


error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
    ┌── profile-guided-optimization.md:65:33 ───
    │
    │
 65 │ `rustc` instructs LLVM to do so [by setting the appropriate][pgo-gen-passmanager]
    │


error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs"
    ┌── profile-guided-optimization.md:77:25 ───
    │
    │
 77 │ runtime are not removed [by marking the with the right export level][pgo-gen-symbols].
    │


error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
    ┌── profile-guided-optimization.md:88:11 ───
    │
    │
 88 │ basically [just telling][pgo-use-passmanager] the LLVM `PassManagerBuilder`
    │

error: The server responded with 429 Too Many Requests for "https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile"


     ┌── profile-guided-optimization.md:109:1 ───
     │
 109 │ [compiler-rt][compiler-rt-profile] and statically linked into any instrumented
     │ ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/run-make-fulldeps"
     ┌── profile-guided-optimization.md:122:4 ───
     │
     │
 122 │ in [run-make tests][rmake-tests] (the relevant tests have `pgo` in their name).
     │


error: The server responded with 429 Too Many Requests for "***/blob/master/src/test/codegen/pgo-instrumentation.rs"
     ┌── profile-guided-optimization.md:123:17 ───
     │
 123 │ There is also a [codegen test][codegen-test] that checks that some expected
     │                 ^ Server responded with 429 Too Many Requests
---
 24 │ *  The sanitizer runtime libraries are part of the [compiler-rt] project, and
    │                                                    ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs"
    ┌── sanitizers.md:25:4 ───
    │
    │
 25 │    [will be built on supported targets][sanitizer-build] when enabled in `config.toml`:
    │


error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs"
    ┌── sanitizers.md:32:21 ───
    │
 32 │    The runtimes are [placed into target libdir][sanitizer-copy].
    │                     ^ Server responded with 429 Too Many Requests
---
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)


error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs"
    ┌── sanitizers.md:35:4 ───
    │
    │
 35 │    [marked][sanitizer-attribute] with appropriate LLVM attribute:
    │


error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs"
    ┌── sanitizers.md:42:65 ───
    │
 42 │    functions it might be necessary to inhibit inlining, both at [MIR
    │                                                                 ^ Server responded with 429 Too Many Requests
    │                                                                 ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td"

    ┌── sanitizers.md:43:27 ───
    │
 43 │    level][inline-mir] and [LLVM level][inline-llvm].
    │


error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs"
    ┌── sanitizers.md:45:54 ───
    │
    │
 45 │ *  The LLVM IR generated by rustc is instrumented by [dedicated LLVM
    │


error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs"
    ┌── sanitizers.md:50:4 ───
    │
 50 │    [linked in][sanitizer-link]. The libraries are searched for in target libdir
    │    ^ Server responded with 429 Too Many Requests
---
 45 │ We have our own fork of GDB - [https://github.com/rust-dev-tools/gdb]
    │                               ^ Server responded with 429 Too Many Requests
    │

error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
    ┌── debugging-support-in-rustc.md:80:1 ───
    │
    │
 80 │ [This comment by Tom](***c-dev-guide/pull/316#discussion_r284027340)
    │


error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
    ┌── debugging-support-in-rustc.md:86:1 ───
    │
    │
 86 │ [This question by Aman](***c-dev-guide/pull/316#discussion_r285401353)
    │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb"

---
error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb/wiki"

     ┌── debugging-support-in-rustc.md:121:43 ───
     │
 121 │ * None of the LLDB work is upstream. This [rust-lang/lldb wiki page] explains a few details.
     │


error: The server responded with 429 Too Many Requests for "***/issues/34457"
     ┌── debugging-support-in-rustc.md:174:17 ───
     │
     │
 174 │ Tracking issue: [***/issues/34457]
     │


error: The server responded with 429 Too Many Requests for "***/issues/33014"
     ┌── debugging-support-in-rustc.md:229:18 ───
     │
     │
 229 │ Issue on Github: [***/issues/33014]
     │

error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2117"


     ┌── debugging-support-in-rustc.md:265:6 ───
     │
 265 │ RFC: [https://github.com/rust-lang/rfcs/pull/2117]
     │      ^ Server responded with 429 Too Many Requests
     │

error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
     ┌── debugging-support-in-rustc.md:279:1 ───
     │
     │
 279 │ [Question on Github](***c-dev-guide/pull/316#discussion_r283062536).
     │


error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustdoc/core.rs"
    ┌── appendix/code-index.md:15:131 ───
    │
    │
 15 │ `DocContext` | struct | A state container used by rustdoc when crawling through a crate to gather its documentation | [Rustdoc] | [src/librustdoc/core.rs](***/blob/master/src/librustdoc/core.rs)
    │

Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
   Compiling proc-macro2 v0.4.30
---

------------------------------------------
stderr:
------------------------------------------
{"message":"This generic shadows the built-in type `u32`","code":{"code":"clippy::builtin_type_shadow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":78,"byte_end":81,"line_start":4,"line_end":4,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":8,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::builtin-type-shadow` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: This generic shadows the built-in type `u32`\n  --> tests/ui/builtin-type-shadow.rs:4:8\n   |\nLL | fn foo<u32>(a: u32) -> u32 {\n   |        ^^^\n   |\n   = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`\n\n"}
{"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.\n\nErroneous code example:\n\n