plain
travis_time:end:12165780:start=1559342009982972866,finish=1559342103568658264,duration=93585685398
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:29]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:42] error[E0308]: mismatched types
[00:14:42]   --> src/librustc_mir/interpret/intrinsics.rs:84:41
[00:14:42]    |
[00:14:42] 84 |                 let ty_name = type_name(self.tcx, ty).ty.to_string();
[00:14:42]    |                                         |
[00:14:42]    |                                         expected struct `rustc::ty::TyCtxt`, found struct `rustc::ty::query::TyCtxtAt`
[00:14:42]    |                                         help: consider dereferencing the type: `*self.tcx`
[00:14:42]    |
---
travis_time:end:004a742c:start=1559343169323756185,finish=1559343169328353283,duration=4597098
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d6cab40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2922a3ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/li
