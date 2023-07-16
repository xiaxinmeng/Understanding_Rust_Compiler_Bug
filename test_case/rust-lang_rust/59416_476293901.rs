plain
travis_time:end:01c9f7d3:start=1553533652303591452,finish=1553533653242623397,duration=939031945
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:20]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:25] error[E0061]: this function takes 3 parameters but 2 parameters were supplied
[00:04:25]    --> src/libcore/slice/mod.rs:507:17
[00:04:25]     |
[00:04:25] 507 |                 ptr::swap_nonoverlapping(pa, pb);
[00:04:25]     | 
[00:04:25]    ::: src/libcore/ptr.rs:347:1
[00:04:25]     |
[00:04:25]     |
[00:04:25] 347 | pub unsafe fn swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize) {
[00:04:25]     | ------------------------------------------------------------------------ defined here
[00:04:28] error: aborting due to previous error
[00:04:28] 
[00:04:28] For more information about this error, try `rustc --explain E0061`.
[00:04:28] error: Could not compile `core`.
---
travis_time:end:02131620:start=1553533933186893285,finish=1553533933191294976,duration=4401691
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e64e4e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23263266
travis_time:start:23263266
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0646c026
$ dmesg | grep -i kill
