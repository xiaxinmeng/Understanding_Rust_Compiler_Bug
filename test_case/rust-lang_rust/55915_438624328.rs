plain
travis_time:end:0b765655:start=1542192471319307043,finish=1542192526385121409,duration=55065814366
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:46]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:48] error[E0433]: failed to resolve. Use of undeclared type or module `AllocationExtra`
[00:13:48]    --> librustc_mir/interpret/memory.rs:228:9
[00:13:48]     |
[00:13:48] 228 |         AllocationExtra::memory_deallocated(&mut alloc, ptr, size)?;
[00:13:48]     |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `AllocationExtra`
[00:14:04] error: aborting due to previous error
[00:14:04] 
[00:14:04] For more information about this error, try `rustc --explain E0433`.
[00:14:04] error: Could not compile `rustc_mir`.
