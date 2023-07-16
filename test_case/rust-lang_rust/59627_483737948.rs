plain
travis_time:end:095a2568:start=1555431985119545105,finish=1555432110596930974,duration=125477385869
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:01]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:08]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:12]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:13]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:38] error[E0599]: no variant named `NullPointer` found for type `mir::interpret::allocation::CheckInAllocMsg` in the current scope
[00:08:38]   --> src/librustc/mir/interpret/allocation.rs:38:30
[00:08:38]    |
[00:08:38] 27 | pub enum CheckInAllocMsg {
[00:08:38]    | ------------------------ variant `NullPointer` not found here
[00:08:38] ...
[00:08:38] 38 |             CheckInAllocMsg::NullPointer => "null pointer",
[00:08:38]    |             |
[00:08:38]    |             |
[00:08:38]    |             variant not found in `mir::interpret::allocation::CheckInAllocMsg`
[00:08:46] error: aborting due to previous error
[00:08:46] 
[00:08:46] For more information about this error, try `rustc --explain E0599`.
[00:08:46] error: Could not compile `rustc`.
---
travis_time:end:149bda44:start=1555432647142692130,finish=1555432647147067060,duration=4374930
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:026f0b19
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bdfa818
travis_time:start:1bdfa818
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: 
