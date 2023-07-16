plain
travis_time:end:1b686d09:start=1556438709734480174,finish=1556438798229096482,duration=88494616308
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:29]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:20] error[E0515]: cannot return value referencing local variable `substs`
[00:08:20]    --> src/librustc/ty/walk.rs:108:17
[00:08:20]     |
[00:08:20] 108 |                 substs.types().rev().chain(opt_ty)
[00:08:20]     |                 |
[00:08:20]     |                 returns a value referencing data owned by the current function
[00:08:20]     |                 `substs` is borrowed here
[00:08:20] 
[00:08:20] 
[00:08:21] error[E0515]: cannot return value referencing temporary value
[00:08:21]    --> src/librustc/ty/sty.rs:347:9
[00:08:21]     |
[00:08:21] 346 |           let SplitClosureSubsts { upvar_kinds, .. } = &self.split(def_id, tcx);
[00:08:21]     |                                                         ----------------------- temporary value created here
[00:08:21] 347 | /         upvar_kinds.iter().map(|t| {
[00:08:21] 348 | |             if let UnpackedKind::Type(ty) = t.unpack() {
[00:08:21] 349 | |                 ty.clone()
[00:08:21] 350 | |             } else {
[00:08:21] 351 | |                 bug!("upvar should be type")
[00:08:21] 353 | |         })
[00:08:21]     | |__________^ returns a value referencing data owned by the current function
[00:08:21] 
[00:08:21] error[E0515]: cannot return value referencing temporary value
[00:08:21] error[E0515]: cannot return value referencing temporary value
[00:08:21]    --> src/librustc/ty/sty.rs:433:9
[00:08:21]     |
[00:08:21] 433 |           upvar_kinds.clone().iter().map(|t| {
[00:08:21]     |           ^------------------
[00:08:21]     |  _________temporary value created here
[00:08:21]     | |
[00:08:21]     | |
[00:08:21] 434 | |             if let UnpackedKind::Type(ty) = t.unpack() {
[00:08:21] 435 | |                 ty.clone()
[00:08:21] 436 | |             } else {
[00:08:21] 437 | |                 bug!("upvar should be type")
[00:08:21] 439 | |         })
[00:08:21]     | |__________^ returns a value referencing data owned by the current function
[00:08:21] 
[00:08:21] 
[00:08:21] error[E0515]: cannot return value referencing local variable `upvar_kinds`
[00:08:21]    --> src/librustc/ty/sty.rs:518:9
[00:08:21]     |
[00:08:21] 518 |           upvar_kinds.iter().map(|t| {
[00:08:21]     |           ^----------
[00:08:21]     |           |
[00:08:21]     |  _________`upvar_kinds` is borrowed here
[00:08:21]     | |
[00:08:21] 519 | |             if let UnpackedKind::Type(ty) = t.unpack() {
[00:08:21] 520 | |                 ty.clone()
[00:08:21] 521 | |             } else {
[00:08:21] 522 | |                 bug!("upvar should be type")
[00:08:21] 524 | |         })
[00:08:21]     | |__________^ returns a value referencing data owned by the current function
[00:08:21] 
[00:08:21] error: aborting due to 4 previous errors
---
travis_time:end:00426c90:start=1556439310483137146,finish=1556439310488722888,duration=5585742
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17e550d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a25817c
travis_time:start:0a25817c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i3
