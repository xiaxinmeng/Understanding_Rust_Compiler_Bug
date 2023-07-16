plain
travis_time:end:00a5e24d:start=1551267111386773922,finish=1551267184468501139,duration=73081727217
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:20:08]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:20:27]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:20:28]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:20:28]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:20:28] error[E0599]: no method named `cast_kinds` found for type `&'a rustc::ty::TypeckTables<'tcx>` in the current scope
[00:20:28]    --> src/librustc_passes/rvalue_promotion.rs:322:28
[00:20:28]     |
[00:20:28] 322 |             match v.tables.cast_kinds().get(from.hir_id) {
[00:20:28] 
[00:20:29] error: aborting due to previous error
[00:20:29] 
[00:20:29] For more information about this error, try `rustc --explain E0599`.
---
171884 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
156148 ./src/llvm-project/clang
155940 ./obj/build/bootstrap/debug/incremental
141172 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141168 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9vmddzezf-1n8wqd1-okq6jwyp562e
132280 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
129564 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123516 ./src/llvm-project/llvm/test/CodeGen
108528 ./src/llvm-project/lldb
---
travis_time:end:0e6cbf17:start=1551268503253337975,finish=1551268503258833897,duration=5495922
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ddfaf74
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05162d60
travis_time:start:05162d60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: N
