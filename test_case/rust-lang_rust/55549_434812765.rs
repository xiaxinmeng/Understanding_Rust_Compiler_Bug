plain
travis_time:end:2b216130:start=1541012208322839323,finish=1541012271118235768,duration=62795396445
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:14:24]    Compiling rustc_metadata_utils v0.0.0 (/checkout/src/librustc_metadata_utils)
[00:14:24]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:24]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:25]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:14:28] error[E0491]: in type `&'rt mut interpret::validity::RefTracking<'tcx, Tag>`, reference has a longer lifetime than the data it references
[00:14:28]    --> librustc_mir/interpret/validity.rs:142:5
[00:14:28]     |
[00:14:28] 142 |     ref_tracking: Option<&'rt mut RefTracking<'tcx, Tag>>,
[00:14:28]     |
[00:14:28]     |
[00:14:28] note: the pointer is valid for the lifetime 'rt as defined on the struct at 136:24
[00:14:28]    --> librustc_mir/interpret/validity.rs:136:24
[00:14:28]     |
[00:14:28] 136 | struct ValidityVisitor<'rt, 'a, 'tcx, Tag> {
[00:14:28]     |                        ^^^
[00:14:28] note: but the referenced data is only valid for the lifetime 'tcx as defined on the struct at 136:33
[00:14:28]    --> librustc_mir/interpret/validity.rs:136:33
[00:14:28]     |
[00:14:28] 136 | struct ValidityVisitor<'rt, 'a, 'tcx, Tag> {
[00:14:28] 
[00:14:28] 
[00:14:28] error[E0478]: lifetime bound not satisfied
[00:14:28]    --> librustc_mir/interpret/validity.rs:144:5
[00:14:28] 144 |     tcx: TyCtxt<'a, 'tcx, 'tcx>,
[00:14:28]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:14:28]     |
[00:14:28]     |
[00:14:28] note: lifetime parameter instantiated with the lifetime 'tcx as defined on the struct at 136:33
[00:14:28]    --> librustc_mir/interpret/validity.rs:136:33
[00:14:28]     |
[00:14:28] 136 | struct ValidityVisitor<'rt, 'a, 'tcx, Tag> {
[00:14:28]     |                                 ^^^^
[00:14:28] note: but lifetime parameter must outlive the lifetime 'a as defined on the struct at 136:29
[00:14:28]    --> librustc_mir/interpret/validity.rs:136:29
[00:14:28]     |
[00:14:28] 136 | struct ValidityVisitor<'rt, 'a, 'tcx, Tag> {
[00:14:28] 
[00:14:29] error: aborting due to 2 previous errors
[00:14:29] 
[00:14:29] Some errors occurred: E0478, E0491.
[00:14:29] Some errors occurred: E0478, E0491.
[00:14:29] For more information about an error, try `rustc --explain E0478`.
[00:14:29] error: Could not compile `rustc_mir`.
[00:14:29] warning: build failed, waiting for other jobs to finish...
[00:15:53] error: build failed
[00:15:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:53] expected success, got: exit code: 101
[00:15:53] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:15:53] travis_fold:end:stage0-rustc

[00:15:53] travis_time:end:stage0-rustc:start=1541012636314096047,finish=1541013234626202982,duration=598312106935


[00:15:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:53] Build completed unsuccessfully in 0:11:09
[00:15:53] make: *** [all] Error 1
[00:15:53] Makefile:28: recipe for target 'all' failed
1442704 ./obj
1442664 ./obj/build
1194324 ./.git
1072200 ./src
---
151412 ./src/tools/clang
149648 ./obj/build/bootstrap/debug/incremental
149116 ./src/llvm-emscripten/test
134188 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134184 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f68rcndwwa-1xzoq02-ycmuivyxm2px
111072 ./src/llvm/test/CodeGen
107668 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
