plain
travis_time:end:095a77c0:start=1556196840215455735,finish=1556196928156953855,duration=87941498120
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:27]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:34]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:38]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:40]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:51] error[E0207]: the type parameter `E` is not constrained by the impl trait, self type, or predicates
[00:07:51]      |
[00:07:51]      |
[00:07:51] 1614 | impl<'a, 'tcx, C, E> TyLayoutMethods<'tcx, C> for Ty<'tcx>
[00:07:51]      |                   ^ unconstrained type parameter
[00:07:51] error: aborting due to previous error
[00:07:51] 
[00:07:51] For more information about this error, try `rustc --explain E0207`.
[00:07:51] error: Could not compile `rustc`.
---
travis_time:end:1902fb06:start=1556197423866236068,finish=1556197423871700808,duration=5464740
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16f3020d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3dae5ba6
travis_time:start:3dae5ba6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0eabe6f8
$ dmesg | grep -i kill
