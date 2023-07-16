plain
travis_time:end:097dad5a:start=1556811122461833886,finish=1556811208950897350,duration=86489063464
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:34] 34  | use crate::hir::PatKind::Slice;
[00:07:34]     |
[00:07:34] 34  | use crate::hir::TyKind::Slice;
[00:07:34]     |
[00:07:34] 34  | use crate::ty::sty::TyKind::Slice;
[00:07:34] 34  | use syntax::ast::PatKind::Slice;
[00:07:34]     |
[00:07:34] and 1 other candidates
[00:07:34] 
[00:07:34] 
[00:08:02] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[00:08:02]     --> src/librustc/ty/context.rs:996:27
[00:08:02]      |
[00:08:02] 996  |             err: mk_const(ty::Const::zero_sized(types.err)),
[00:08:02]      | 
[00:08:02]     ::: src/librustc/ty/sty.rs:2156:5
[00:08:02]      |
[00:08:02]      |
[00:08:02] 2156 |     pub fn zero_sized(tcx: TyCtxt<'_, '_, 'tcx>, ty: Ty<'tcx>) -> &'tcx Self {
[00:08:02]      |     ------------------------------------------------------------------------ defined here
[00:08:02] error[E0308]: mismatched types
[00:08:02]    --> src/librustc/ty/context.rs:996:27
[00:08:02]     |
[00:08:02]     |
[00:08:02] 996 |             err: mk_const(ty::Const::zero_sized(types.err)),
[00:08:02]     |                           |
[00:08:02]     |                           |
[00:08:02]     |                           expected struct `ty::sty::Const`, found reference
[00:08:02]     |                           help: consider dereferencing the borrow: `*ty::Const::zero_sized(types.err)`
[00:08:02]     = note: expected type `ty::sty::Const<'_>`
[00:08:02]     = note: expected type `ty::sty::Const<'_>`
[00:08:02]                found type `&ty::sty::Const<'_>`
[00:08:03] error: aborting due to 3 previous errors
[00:08:03] 
[00:08:03] Some errors occurred: E0061, E0308, E0532.
[00:08:03] For more information about an error, try `rustc --explain E0061`.
---
travis_time:end:04f1f2b0:start=1556811702438622461,finish=1556811702443122472,duration=4500011
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2660dddc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b6f36a
travis_time:start:03b6f36a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i
