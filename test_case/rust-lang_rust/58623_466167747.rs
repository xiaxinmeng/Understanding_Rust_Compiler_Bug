plain
travis_time:end:0b7b47fa:start=1550783174954297002,finish=1550783175943995694,duration=989698692
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:57] 
######################################################################## 100.0%
[00:01:57] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:57]     Updating crates.io index
[00:02:09]     Updating git repository `https://github.com/Amanieu/hashbrown`
[00:02:10]   Downloaded cmake v0.1.35
[00:02:10]   Downloaded lazy_static v0.2.11
[00:02:10]   Downloaded num_cpus v1.10.0
[00:02:10]   Downloaded serde_json v1.0.38
---

[00:04:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:06] tidy error: /checkout/src/libstd/collections/hash/map.rs:440: TODO is deprecated; use FIXME
[00:04:07] Dependencies not on the whitelist:
[00:04:07] * autocfg 
[00:04:07] * fuchsia-cprng 
[00:04:07] * rand_os 
[00:04:07] * rdrand 
[00:04:07] * rdrand 
[00:04:07] invalid source: "git+https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0afdac18e6ece8c74f38e5e78e1c68ee17"
[00:04:07] some tidy checks failed
[00:04:07] 
[00:04:07] 
[00:04:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:07] 
[00:04:07] 
[00:04:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:07] Build completed unsuccessfully in 0:00:48
[00:04:07] Build completed unsuccessfully in 0:00:48
[00:04:07] Makefile:68: recipe for target 'tidy' failed
[00:04:07] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23336ad2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 21 21:10:34 UTC 2019
