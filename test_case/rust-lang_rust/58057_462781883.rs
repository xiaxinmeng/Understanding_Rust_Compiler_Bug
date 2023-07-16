plain
travis_time:end:01d33016:start=1549980741696992857,finish=1549980743804666333,duration=2107673476
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:19:26]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:19:26]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:20:19]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:20:19]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:20:19] error[E0407]: method `cross_lang_lto` is not a member of trait `Linker`
[00:20:19]     --> src/librustc_codegen_ssa/back/linker.rs:1210:5
[00:20:19]      |
[00:20:19] 1210 | /     fn cross_lang_lto(&mut self) {
[00:20:19]      | |_____^ not a member of trait `Linker`
[00:20:19] 
[00:20:19] 
[00:20:20] error[E0046]: not all trait items implemented, missing: `linker_plugin_lto`
[00:20:20]     --> src/librustc_codegen_ssa/back/linker.rs:1095:1
[00:20:20]      |
[00:20:20] 130  |     fn linker_plugin_lto(&mut self);
[00:20:20]      |     -------------------------------- `linker_plugin_lto` from trait
[00:20:20] ...
[00:20:20] 1095 | impl<'a> Linker for PtxLinker<'a> {
[00:20:20]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `linker_plugin_lto` in implementation
[00:20:20] error: aborting due to 2 previous errors
[00:20:20] 
[00:20:20] Some errors occurred: E0046, E0407.
[00:20:20] For more information about an error, try `rustc --explain E0046`.
[00:20:20] For more information about an error, try `rustc --explain E0046`.
[00:20:20] error: Could not compile `rustc_codegen_ssa`.
[00:20:20] warning: build failed, waiting for other jobs to finish...
[00:20:42] error: build failed
[00:20:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:20:42] expected success, got: exit code: 101
[00:20:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:42] Build completed unsuccessfully in 0:16:54
[00:20:42] Makefile:18: recipe for target 'all' failed
[00:20:42] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0648cb58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 14:33:16 UTC 2019
---
travis_time:end:26a7903e:start=1549981996940294554,finish=1549981997526385583,duration=586091029
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0050f738
$ ls -lat $HOME/Library/Logs/Diagnosti/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2502eff9
$ dmesg | grep -i kill
