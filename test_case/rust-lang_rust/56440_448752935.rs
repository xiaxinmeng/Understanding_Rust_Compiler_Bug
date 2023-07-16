plain
travis_time:end:247d7f06:start=1545254645586090975,finish=1545254726658520552,duration=81072429577
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:10:29]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:10:32] error[E0615]: attempted to take value of method `hir` on type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[00:10:32]     --> src/librustc_typeck/check/mod.rs:1331:38
[00:10:32]      |
[00:10:32] 1331 |                 let field_span = tcx.hir.span_if_local(field.did).unwrap();
[00:10:32]      |
[00:10:32]      = help: maybe a `()` to call it is missing?
[00:10:32] 
[00:10:35] error: aborting due to previous error
---
travis_time:end:0974e7f2:start=1545255518727292935,finish=1545255518731754954,duration=4462019
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04ad2652
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02080aa0
travis_time:start:02080aa0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
