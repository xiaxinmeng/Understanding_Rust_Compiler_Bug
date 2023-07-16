plain
travis_time:end:30f52050:start=1550835974117271228,finish=1550836048793196476,duration=74675925248
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:53:47] travis_time:end:stage2-rustdoc:start=1550839121808604580,finish=1550839285092612844,duration=163284008264

[00:53:47] Build completed successfully in 0:49:09
[00:53:48]     Finished dev [unoptimized] target(s) in 0.34s
[00:53:49] thread 'main' panicked at 'file not found: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:997:5
[00:53:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:53:49] Build completed unsuccessfully in 0:00:01
[00:53:49] make: *** [all] Error 1
[00:53:49] Makefile:18: recipe for target 'all' failed
---
travis_time:end:13f02812:start=1550839288340974633,finish=1550839288345916730,duration=4942097
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13d42951
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
