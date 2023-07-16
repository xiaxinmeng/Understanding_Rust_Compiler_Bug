plain
travis_time:end:153e6cc0:start=1555522437871441125,finish=1555522569973745835,duration=132102304710
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:49:56]    Compiling tempfile v3.0.5
[00:49:56] error[E0603]: extern crate `libc` is private
[00:49:56]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.4.0/src/thread_parker/unix.rs:226:28
[00:49:56]     |
[00:49:56] 226 |     if timeout.as_secs() > libc::time_t::max_value() as u64 {
[00:49:56] 
[00:49:56] error: aborting due to previous error
[00:49:56] 
[00:49:56] For more information about this error, try `rustc --explain E0603`.
