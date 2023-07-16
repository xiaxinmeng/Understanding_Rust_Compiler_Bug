plain
travis_time:end:216f02f4:start=1552432342480426398,finish=1552432343363857100,duration=883430702
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:18] 
[01:18:18] running 120 tests
[01:18:42] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:18:47] .i......iii.i.....ii
[01:18:47] 
[01:18:47]  finished in 28.706
[01:18:47] travis_fold:end:test_debuginfo

---
[01:34:26] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:34:26]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:34:38] error[E0369]: binary operation `>` cannot be applied to type `fn() -> u32 {time::tests::system_time_math::max_time_diff_years}`
[01:34:38]    --> src/libstd/time.rs:743:16
[01:34:38]     |
[01:34:38] 743 |             if max_time_diff_years > 800 {
[01:34:38]     |
[01:34:38]     |
[01:34:38]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> u32 {time::tests::system_time_math::max_time_diff_years}`
[01:34:38] error[E0308]: mismatched types
[01:34:38]    --> src/libstd/time.rs:743:38
[01:34:38]     |
[01:34:38]     |
[01:34:38] 743 |             if max_time_diff_years > 800 {
[01:34:38]     |                                      ^^^ expected fn item, found integer
[01:34:38]     |
[01:34:38]     = note: expected type `fn() -> u32 {time::tests::system_time_math::max_time_diff_years}`
[01:34:38] 
[01:34:38] error[E0599]: no associated item named `MAX` found for type `u32` in the current scope
[01:34:38]    --> src/libstd/time.rs:722:30
[01:34:38]     |
