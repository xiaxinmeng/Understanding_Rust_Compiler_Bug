plain
travis_time:end:17d7d9e2:start=1558758922080727490,finish=1558759008700052615,duration=86619325125
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:30]    Compiling hashbrown v0.3.0
[00:04:31] error[E0412]: cannot find type `OsStr` in this scope
[00:04:31]     --> src/libstd/ffi/c_str.rs:1311:38
[00:04:31]      |
[00:04:31] 1311 | impl<S: Borrow<CStr>> SliceConcatExt<OsStr> for [S] {
[00:04:31] help: possible candidate is found in another module, you can import it into scope
[00:04:31]      |
[00:04:31] 1    | use crate::ffi::os_str::OsStr;
[00:04:31]      |
[00:04:31]      |
[00:04:31] 
[00:04:32] error[E0119]: conflicting implementations of trait `alloc_crate::slice::SliceConcatExt<[type error]>` for type `[_]`:
[00:04:32]      |
[00:04:32]      |
[00:04:32] 974  | impl<S: Borrow<OsStr>> SliceConcatExt<OsStr> for [S] {
[00:04:32]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `[_]`
[00:04:32]     ::: src/libstd/ffi/c_str.rs:1311:1
[00:04:32]      |
[00:04:32]      |
[00:04:32] 1311 | impl<S: Borrow<CStr>> SliceConcatExt<OsStr> for [S] {
[00:04:32]      | --------------------------------------------------- first implementation here
[00:04:32] 
[00:04:32] error[E0210]: type parameter `S` must be used as the type parameter for some local type (e.g., `MyStruct<S>`)
[00:04:32]      |
[00:04:32]      |
[00:04:32] 1311 | impl<S: Borrow<CStr>> SliceConcatExt<OsStr> for [S] {
[00:04:32]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `S` must be used as the type parameter for some local type
[00:04:32]      = note: only traits defined in the current crate can be implemented for a type parameter
[00:04:32] 
[00:04:32] 
[00:04:32] error[E0210]: type parameter `S` must be used as the type parameter for some local type (e.g., `MyStruct<S>`)
[00:04:32]     |
[00:04:32]     |
[00:04:32] 974 | impl<S: Borrow<OsStr>> SliceConcatExt<OsStr> for [S] {
[00:04:32]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `S` must be used as the type parameter for some local type
[00:04:32]     = note: only traits defined in the current crate can be implemented for a type parameter
[00:04:32] 
[00:04:32] error: aborting due to 4 previous errors
[00:04:32] 
---
travis_time:end:15b0d622:start=1558759290664066720,finish=1558759290668572745,duration=4506025
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ed8f38
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:108caf9e
travis_time:start:108caf9e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file
