plain
travis_time:end:00e56881:start=1556531611442584547,finish=1556531707530144628,duration=96087560081
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:25:33]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:25:38]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:25:38]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:25:38]    Compiling hashbrown v0.3.0
[00:25:39] error: this form of character escape may only be used with characters in the range [\x00-\x7f]
[00:25:39]      |
[00:25:39]      |
[00:25:39] 1349 |         let s = CString::new(&b"abc\x01\x02\n\xE2\x80\xA6\xFF"[..]).unwrap();
[00:25:39] 
[00:25:39] error: aborting due to previous error
[00:25:39] 
[00:25:39] error: Could not compile `std`.
---
travis_time:end:2bbd16f7:start=1556533257014088233,finish=1556533257019279371,duration=5191138
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1349e401
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06c40e0a
travis_time:start:06c40e0a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
