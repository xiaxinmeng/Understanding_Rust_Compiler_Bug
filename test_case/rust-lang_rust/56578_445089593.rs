plain
travis_time:end:10fd7874:start=1544145227330470591,finish=1544145230867568664,duration=3537098073
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:06:52]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:55] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:06:55]    --> src/librustc/traits/util.rs:540:24
[00:06:55]     |
[00:06:55] 540 |                 if let hir::ItemKind::Impl(_, _, defaultness, ..) = item.node {
[00:06:55] 
[00:06:55] error[E0433]: failed to resolve. Use of undeclared type or module `hir`
[00:06:55]    --> src/librustc/traits/util.rs:554:58
[00:06:55]     |
[00:06:55]     |
[00:06:55] 554 |     pub fn impl_item_is_final(self, node_item: &NodeItem<hir::Defaultness>) -> bool {
[00:06:55] 
[00:06:56] error[E0412]: cannot find type `Substs` in this scope
[00:06:56]    --> src/librustc/traits/util.rs:370:63
[00:06:56]     |
[00:06:56]     |
[00:06:56] 370 |                                                 impl_substs: &Substs<'tcx>)
[00:06:56]     |                                                               ^^^^^^ did you mean `Subst`?
[00:06:56]     |
[00:06:56] 11  | use ty::subst::Substs;
[00:06:56]     |
[00:06:56] 
[00:06:56] 
[00:06:56] error[E0412]: cannot find type `Rc` in this scope
[00:06:56]    --> src/librustc/traits/mod.rs:264:18
[00:06:56]     |
[00:06:56] 264 |     parent_code: Rc<ObligationCauseCode<'tcx>>
[00:06:56]     |                  ^^ not found in this scope
[00:06:56]     |
[00:06:56] 31  | use std::rc::Rc;
[00:06:56]     |
[00:06:56] 
[00:06:56] 
[00:06:56] error[E0405]: cannot find trait `Debug` in this scope
[00:06:56]     --> src/librustc/traits/mod.rs:1176:26
[00:06:56]      |
[00:06:56] 1176 |     type LiftedExClause: Debug + 'tcx;
[00:06:56] help: possible candidates are found in other modules, you can import them into scope
[00:06:56]      |
[00:06:56] 31   | use core::fmt::Debug;
[00:06:56]      |
---
[00:07:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:19] expected success, got: exit code: 101
[00:07:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:19] Build completed unsuccessfully in 0:03:53
[00:07:19] make: *** [all] Error 1
[00:07:19] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0070b951
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 01:21:19 UTC 2018
