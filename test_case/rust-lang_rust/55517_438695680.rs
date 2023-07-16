plain
travis_time:end:0a5fdd58:start=1542207737343698206,finish=1542207793850600983,duration=56506902777
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:34]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:38]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:56]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:07:06]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:30] error[E0599]: no method named `replace_late_bound_regions_with_fresh_var` found for type `&infer::InferCtxt<'_, '_, '_>` in the current scope
[00:07:30]      |
[00:07:30]      |
[00:07:30] 1448 |         infcx.replace_late_bound_regions_with_fresh_var(
[00:07:30]      |
[00:07:30]      = help: did you mean `replace_late_bound_regions_with_placeholders`?
[00:07:30] 
[00:07:30] 
[00:07:30] error[E0599]: no method named `predicate_must_hold` found for type `&infer::InferCtxt<'_, '_, '_>` in the current scope
[00:07:30]     |
[00:07:30]     |
[00:07:30] 561 |             infcx.predicate_must_hold(&obligation)
[00:07:30]     |
[00:07:30]     |
[00:07:30]     = help: did you mean `predicate_may_hold`?
[00:07:30] error[E0061]: this function takes 5 parameters but 7 parameters were supplied
[00:07:30]     --> librustc/traits/select.rs:3024:42
[00:07:30]      |
[00:07:30]      |
[00:07:30] 3024 |               let trait_obligations = self.impl_or_trait_obligations(
[00:07:30] ...
[00:07:30] ...
[00:07:30] 3654 | /     fn impl_or_trait_obligations(
[00:07:30] 3655 | |         &mut self,
[00:07:30] 3656 | |         cause: ObligationCause<'tcx>,
[00:07:30] ...    |
[00:07:30] 3723 | |         predicates
[00:07:30] 3724 | |     }
[00:07:30]      | |_____- defined here
---
[00:07:36] 
[00:07:36] To learn more, run the command again with --verbose.
[00:07:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:36] expected success, got: exit code: 101
[00:07:36] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:07:36] travis_fold:end:stage0-rustc

[00:07:36] travis_time:end:stage0-rustc:start=1542208097061533372,finish=1542208259729488577,duration=162667955205


[00:07:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:36] Build completed unsuccessfully in 0:03:47
[00:07:36] make: *** [all] Error 1
[00:07:36] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14ee587d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 14 15:10:59 UTC 2018
---
32392 ./src/libcompiler_buitravis_time:end:0baebe78:start=1542208260455513808,finish=1542208260467103317,duration=11589509
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06ae7be0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02ce8254
$ dmesg | grep -i kill
