plain
travis_time:end:11034b4c:start=1558759898666420672,finish=1558759985936378545,duration=87269957873
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:47]    Compiling rustc-demangle v0.1.10
[00:04:51]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:04:51]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:51]    Compiling hashbrown v0.3.0
[00:04:54] error[E0210]: type parameter `S` must be used as the type parameter for some local type (e.g., `MyStruct<S>`)
[00:04:54]      |
[00:04:54]      |
[00:04:54] 1311 | impl<S: Borrow<CStr>> SliceConcatExt<CStr> for [S] {
[00:04:54]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `S` must be used as the type parameter for some local type
[00:04:54]      = note: only traits defined in the current crate can be implemented for a type parameter
[00:04:54] 
[00:04:54] 
[00:04:54] error[E0210]: type parameter `S` must be used as the type parameter for some local type (e.g., `MyStruct<S>`)
[00:04:54]     |
[00:04:54]     |
[00:04:54] 974 | impl<S: Borrow<OsStr>> SliceConcatExt<OsStr> for [S] {
[00:04:54]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `S` must be used as the type parameter for some local type
[00:04:54]     = note: only traits defined in the current crate can be implemented for a type parameter
[00:04:54] 
[00:04:54] error: aborting due to 2 previous errors
[00:04:54] 
