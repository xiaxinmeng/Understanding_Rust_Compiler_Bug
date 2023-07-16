plain
travis_time:end:01dae1a4:start=1552083407583904349,finish=1552083493487169601,duration=85903265252
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:07:30]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:47] error[E0308]: `if let` arms have incompatible types
[00:07:47]     --> src/librustc/hir/lowering.rs:4010:24
[00:07:47]      |
[00:07:47] 3960 |                    let res = if let IsAsync::Async { closure_id, .. } = asyncness {
[00:07:47]      |   ___________________________-
[00:07:47] 3961 |  |                     let outer_decl = FnDecl {
[00:07:47] 3962 |  |                         inputs: decl.inputs.clone(),
[00:07:47] 3963 |  |                         output: FunctionRetTy::Default(fn_decl_span),
[00:07:47] 4010 |  |                 } else {
[00:07:47]      |  |________________________^
[00:07:47]      |  |________________________^
[00:07:47] 4011 | ||                     // Lower outside new scope to preserve `is_in_loop_condition`.
[00:07:47] 4012 | ||                     let fn_decl = self.lower_fn_decl(decl, None, false, None);
[00:07:47] ...    ||
[00:07:47] 4054 | ||
[00:07:47] 4055 | ||                 };
[00:07:47]      | ||                 ^
[00:07:47]      | ||                 ^
[00:07:47]      | ||_________________|
[00:07:47]      | |__________________`if let` arms have incompatible types
[00:07:47]      |                    expected enum `hir::ExprKind`, found ()
[00:07:47]      = note: expected type `hir::ExprKind`
[00:07:47]                 found type `()`
[00:07:47] 
[00:08:03] error: aborting due to previous error
---
travis_time:end:010c6a80:start=1552083989928731861,finish=1552083989933496880,duration=4765019
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:025c3b60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:202b71c0
travis_time:start:202b71c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt
