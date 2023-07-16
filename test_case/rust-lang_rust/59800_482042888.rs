plain
travis_time:end:1f21038d:start=1554975130198977370,finish=1554975132281435585,duration=2082458215
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:33]    Compiling toml v0.4.10
[00:02:33]    Compiling serde_json v1.0.33
[00:02:42]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:14]     Finished dev [unoptimized] target(s) in 1m 14s
[00:03:14] error: failed to write /checkout/Cargo.lock
[00:03:14] Caused by:
[00:03:14]   failed to open: /checkout/Cargo.lock
[00:03:14] 
[00:03:14] Caused by:
---
[00:03:14] make: *** [prepare] Error 1
[00:03:14] Makefile:69: recipe for target 'prepare' failed
[00:03:15] Command failed. Attempt 2/5:
[00:03:16]     Finished dev [unoptimized] target(s) in 0.35s
[00:03:16] error: failed to write /checkout/Cargo.lock
[00:03:16] Caused by:
[00:03:16]   failed to open: /checkout/Cargo.lock
[00:03:16] 
[00:03:16] Caused by:
---
[00:03:16] Makefile:69: recipe for target 'prepare' failed
[00:03:16] make: *** [prepare] Error 1
[00:03:18] Command failed. Attempt 3/5:
[00:03:19]     Finished dev [unoptimized] target(s) in 0.38s
[00:03:19] error: failed to write /checkout/Cargo.lock
[00:03:19] Caused by:
[00:03:19]   failed to open: /checkout/Cargo.lock
[00:03:19] 
[00:03:19] Caused by:
---
[00:03:19] make: *** [prepare] Error 1
[00:03:19] Makefile:69: recipe for target 'prepare' failed
[00:03:22] Command failed. Attempt 4/5:
[00:03:22]     Finished dev [unoptimized] target(s) in 0.34s
[00:03:23] error: failed to write /checkout/Cargo.lock
[00:03:23] Caused by:
[00:03:23]   failed to open: /checkout/Cargo.lock
[00:03:23] 
[00:03:23] Caused by:
---
[00:03:23] make: *** [prepare] Error 1
[00:03:23] Makefile:69: recipe for target 'prepare' failed
[00:03:27] Command failed. Attempt 5/5:
[00:03:27]     Finished dev [unoptimized] target(s) in 0.35s
[00:03:28] error: failed to write /checkout/Cargo.lock
[00:03:28] Caused by:
[00:03:28]   failed to open: /checkout/Cargo.lock
[00:03:28] 
[00:03:28] Caused by:
