plain
travis_time:end:0ab03768:start=1558047789072367083,finish=1558047876315381796,duration=87243014713
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:16]    Compiling toml v0.4.10
[00:02:16]    Compiling serde_json v1.0.33
[00:02:19]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:53]     Finished dev [unoptimized] target(s) in 1m 25s
[00:02:53] error: failed to write /checkout/Cargo.lock
[00:02:53] Caused by:
[00:02:53]   failed to open: /checkout/Cargo.lock
[00:02:53] 
[00:02:53] Caused by:
---
[00:02:53] Makefile:69: recipe for target 'prepare' failed
[00:02:53] make: *** [prepare] Error 1
[00:02:54] Command failed. Attempt 2/5:
[00:02:55]     Finished dev [unoptimized] target(s) in 0.39s
[00:02:55] error: failed to write /checkout/Cargo.lock
[00:02:55] Caused by:
[00:02:55]   failed to open: /checkout/Cargo.lock
[00:02:55] 
[00:02:55] Caused by:
---
[00:02:55] make: *** [prepare] Error 1
[00:02:55] Makefile:69: recipe for target 'prepare' failed
[00:02:57] Command failed. Attempt 3/5:
[00:02:57]     Finished dev [unoptimized] target(s) in 0.38s
[00:02:58] error: failed to write /checkout/Cargo.lock
[00:02:58] Caused by:
[00:02:58]   failed to open: /checkout/Cargo.lock
[00:02:58] 
[00:02:58] Caused by:
---
[00:02:58] make: *** [prepare] Error 1
[00:02:58] Makefile:69: recipe for target 'prepare' failed
[00:03:01] Command failed. Attempt 4/5:
[00:03:01]     Finished dev [unoptimized] target(s) in 0.39s
[00:03:02] error: failed to write /checkout/Cargo.lock
[00:03:02] Caused by:
[00:03:02]   failed to open: /checkout/Cargo.lock
[00:03:02] 
[00:03:02] Caused by:
---
[00:03:02] Makefile:69: recipe for target 'prepare' failed
[00:03:02] make: *** [prepare] Error 1
[00:03:06] Command failed. Attempt 5/5:
[00:03:06]     Finished dev [unoptimized] target(s) in 0.39s
[00:03:07] error: failed to write /checkout/Cargo.lock
[00:03:07] Caused by:
[00:03:07]   failed to open: /checkout/Cargo.lock
[00:03:07] 
[00:03:07] Caused by:
