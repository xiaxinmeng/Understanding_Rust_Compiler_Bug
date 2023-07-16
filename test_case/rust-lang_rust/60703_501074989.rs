plain
travis_time:end:004fdb3b:start=1560298572959709429,finish=1560298667196973502,duration=94237264073
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:17:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:17:07] tidy error: /checkout/src/liballoc/collections/linked_list.rs:1354: line longer than 100 chars
[00:17:07] tidy error: /checkout/src/liballoc/collections/vec_deque.rs:597: line longer than 100 chars
[00:17:07] tidy error: /checkout/src/liballoc/vec.rs:559: line longer than 100 chars
[00:17:07] tidy error: /checkout/src/liballoc/raw_vec.rs:400: line longer than 100 chars
[00:17:09] tidy error: /checkout/src/liballoc/boxed.rs:339: malformed stability attribute: missing `feature` key
[00:17:11] some tidy checks failed
[00:17:11] 
[00:17:11] 
[00:17:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:17:11] 
[00:17:11] 
[00:17:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:17:11] Build completed unsuccessfully in 0:01:13
