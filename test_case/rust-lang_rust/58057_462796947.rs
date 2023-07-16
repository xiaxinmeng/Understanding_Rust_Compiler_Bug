plain
travis_time:end:20685ab8:start=1549982594154840062,finish=1549982596561081686,duration=2406241624
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:26:50]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:27:24]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:27:25]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:27:25]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:27:26] error[E0407]: method `cross_lang_lto` is not a member of trait `Linker`
[00:27:26]     --> src/librustc_codegen_ssa/back/linker.rs:1210:5
[00:27:26]      |
[00:27:26] 1210 | /     fn cross_lang_lto(&mut self) {
[00:27:26]      | |_____^ not a member of trait `Linker`
[00:27:26] 
[00:27:26] 
[00:27:26] error[E0046]: not all trait items implemented, missing: `linker_plugin_lto`
[00:27:26]     --> src/librustc_codegen_ssa/back/linker.rs:1095:1
[00:27:26]      |
[00:27:26] 130  |     fn linker_plugin_lto(&mut self);
[00:27:26]      |     -------------------------------- `linker_plugin_lto` from trait
[00:27:26] ...
[00:27:26] 1095 | impl<'a> Linker for PtxLinker<'a> {
[00:27:26]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `linker_plugin_lto` in implementation
[00:27:26] error: aborting due to 2 previous errors
[00:27:26] 
[00:27:26] Some errors occurred: E0046, E0407.
[00:27:26] For more information about an error, try `rustc --explain E0046`.
[00:27:26] For more information about an error, try `rustc --explain E0046`.
[00:27:26] error: Could not compile `rustc_codegen_ssa`.
[00:27:26] warning: build failed, waiting for other jobs to finish...
[00:28:02] error: build failed
[00:28:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:28:02] expected success, got: exit code: 101
[00:28:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:28:02] Build completed unsuccessfully in 0:17:45
[00:28:02] make: *** [all] Error 1
[00:28:02] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:209f1b34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 15:11:30 UTC 2019
