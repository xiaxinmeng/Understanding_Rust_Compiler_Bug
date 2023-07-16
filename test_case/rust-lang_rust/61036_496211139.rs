plain
travis_time:end:0add9bac:start=1558963111274216022,finish=1558963112062717681,duration=788501659
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:05]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:08] error[E0308]: mismatched types
[00:02:08]    --> src/bootstrap/native.rs:208:69
[00:02:08]     |
[00:02:08] 208 |         if env::var_os("RUSTBUILD_FORCE_CLANG_BASED_TESTS") == Some("1") {
[00:02:08]     |
[00:02:08]     = note: expected type `std::ffi::OsString`
[00:02:08]                found type `&'static str`
[00:02:08] 
---
[00:02:10]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:13] error[E0308]: mismatched types
[00:02:13]    --> src/bootstrap/native.rs:208:69
[00:02:13]     |
[00:02:13] 208 |         if env::var_os("RUSTBUILD_FORCE_CLANG_BASED_TESTS") == Some("1") {
[00:02:13]     |
[00:02:13]     = note: expected type `std::ffi::OsString`
[00:02:13]                found type `&'static str`
[00:02:13] 
---
[00:02:16]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:20] error[E0308]: mismatched types
[00:02:20]    --> src/bootstrap/native.rs:208:69
[00:02:20]     |
[00:02:20] 208 |         if env::var_os("RUSTBUILD_FORCE_CLANG_BASED_TESTS") == Some("1") {
[00:02:20]     |
[00:02:20]     = note: expected type `std::ffi::OsString`
[00:02:20]                found type `&'static str`
[00:02:20] 
---
[00:02:24]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:27] error[E0308]: mismatched types
[00:02:27]    --> src/bootstrap/native.rs:208:69
[00:02:27]     |
[00:02:27] 208 |         if env::var_os("RUSTBUILD_FORCE_CLANG_BASED_TESTS") == Some("1") {
[00:02:27]     |
[00:02:27]     = note: expected type `std::ffi::OsString`
[00:02:27]                found type `&'static str`
[00:02:27] 
---
[00:02:32]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:36] error[E0308]: mismatched types
[00:02:36]    --> src/bootstrap/native.rs:208:69
[00:02:36]     |
[00:02:36] 208 |         if env::var_os("RUSTBUILD_FORCE_CLANG_BASED_TESTS") == Some("1") {
[00:02:36]     |
[00:02:36]     = note: expected type `std::ffi::OsString`
[00:02:36]                found type `&'static str`
[00:02:36] 
