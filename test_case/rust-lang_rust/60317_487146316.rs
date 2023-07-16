plain
travis_time:end:09da5e99:start=1556298194956616060,finish=1556298197186123155,duration=2229507095
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:53:34]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:53:44] error: passing `TyCtxt<'_, '_, '_>` by reference
[00:53:44]    --> src/librustdoc/clean/auto_trait.rs:853:29
[00:53:44]     |
[00:53:44] 853 |     fn is_fn_ty(&self, tcx: &TyCtxt<'_, '_, '_>, ty: &Type) -> bool {
[00:53:44]     |                             ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_, '_>`
[00:53:44] note: lint level defined here
[00:53:44]    --> src/librustdoc/lib.rs:2:9
[00:53:44]     |
[00:53:44] 2   | #![deny(internal)]
[00:53:44] 2   | #![deny(internal)]
[00:53:44]     |         ^^^^^^^^
[00:53:44]     = note: #[deny(ty_pass_by_reference)] implied by #[deny(internal)]
[00:53:44] error: passing `TyCtxt<'_, '_, '_>` by reference
[00:53:44]     --> src/librustdoc/clean/mod.rs:4400:31
[00:53:44]      |
[00:53:44]      |
[00:53:44] 4400 | pub fn path_to_def_local(tcx: &TyCtxt<'_, '_, '_>, path: &[&str]) -> Option<DefId> {
[00:53:44]      |                               ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_, '_>`
[00:53:44] error: passing `TyCtxt<'_, '_, '_>` by reference
[00:53:44]     --> src/librustdoc/clean/mod.rs:4425:25
[00:53:44]      |
[00:53:44]      |
[00:53:44] 4425 | pub fn path_to_def(tcx: &TyCtxt<'_, '_, '_>, path: &[&str]) -> Option<DefId> {
[00:53:44]      |                         ^^^^^^^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_, '_, '_>`
[00:53:45] error: aborting due to 3 previous errors
[00:53:45] 
[00:53:45] error: Could not compile `rustdoc`.
[00:53:45] 
---
travis_time:end:0c88f092:start=1556301434970716526,finish=1556301434976965554,duration=6249028
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0106019e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0120959c
travis_time:start:0120959c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09d21b63
$ dmesg | grep -i kill
