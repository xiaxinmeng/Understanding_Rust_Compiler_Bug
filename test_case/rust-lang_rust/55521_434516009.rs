plain
travis_time:end:0471fe76:start=1540943484910404978,finish=1540943539057325828,duration=54146920850
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:18:33]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:18:33]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:18:33]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:18:33]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:18:59] error: method is never used: `macro_use`
[00:18:59]   --> librustc_save_analysis/json_dumper.rs:96:5
[00:18:59]    |
[00:18:59] 96 |     pub fn macro_use(&mut self, data: MacroRef) {
[00:18:59]    |
[00:18:59]    = note: `-D dead-code` implied by `-D warnings`
[00:18:59] 
[00:18:59] 
[00:18:59] error: field is never used: `macro_calls`
[00:18:59]   --> librustc_save_analysis/dump_visitor.rs:95:5
[00:18:59]    |
[00:18:59] 95 |     macro_calls: FxHashSet<Span>,
[00:18:59] 
[00:18:59] error: aborting due to 2 previous errors
[00:18:59] 
[00:18:59] error: Could not compile `rustc_save_analysis`.
[00:18:59] error: Could not compile `rustc_save_analysis`.
[00:18:59] 
[00:18:59] To learn more, run the command again with --verbose.
[00:18:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:59] expected success, got: exit code: 101
[00:18:59] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:18:59] travis_fold:end:stage0-rustc

[00:18:59] travis_time:end:stage0-rustc:start=1540943874000957753,finish=1540944690026735694,duration=816025777941


[00:18:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:59] Build completed unsuccessfully in 0:14:46
[00:18:59] Makefile:28: recipe for target 'all' failed
[00:18:59] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06e47b68
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
