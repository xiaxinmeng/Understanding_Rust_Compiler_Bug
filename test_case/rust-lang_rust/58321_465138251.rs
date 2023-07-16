plain
travis_time:end:1c256f9a:start=1550583781458961467,finish=1550583784336939552,duration=2877978085
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:24]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:33]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:07]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:07]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:13] error[E0599]: no method named `subst` found for type `rustc::ty::FnSig<'_>` in the current scope
[00:13:13]    --> src/librustc_typeck/check/method/confirm.rs:398:37
[00:13:13]     |
[00:13:13] 398 |         let method_sig = method_sig.subst(self.tcx, all_substs);
[00:13:13]     |
[00:13:13]     = help: items from traits can only be used if the trait is in scope
[00:13:13]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:13:13]             `use rustc::ty::subst::Subst;`
---
[00:17:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:08] expected success, got: exit code: 101
[00:17:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:08] Build completed unsuccessfully in 0:12:51
[00:17:08] make: *** [all] Error 1
[00:17:08] Makefile:18: recipe for target 'all' failed
69084 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
60736 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
59032 ./.git/modules/src/tools
57404 ./src/llvm-project/llvm/test/MC
---
travis_time:end:0af6f738:start=1550584824802203491,finish=1550584824807255653,duration=5052162
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:030a0137
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10752de0
travis_time:start:10752de0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clan
