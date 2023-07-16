plain
travis_time:end:1b5406d0:start=1561130973575231405,finish=1561130974345994096,duration=770762691
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:58] tidy error: libsyntax/feature_gate.rs:480: feature const_raw_ptr_deref is not sorted by since
[00:04:58] tidy error: libsyntax/feature_gate.rs:781: feature dyn_trait is not sorted by since
[00:04:58] * highest error code: E0731
[00:04:58] * highest error code: E0731
[00:05:00] tidy error: libsyntax/feature_gate.rs:480: feature const_raw_ptr_deref is not sorted by since
[00:05:00] tidy error: libsyntax/feature_gate.rs:781: feature dyn_trait is not sorted by since
[00:05:02] some tidy checks failed
[00:05:02] 
[00:05:02] 
[00:05:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:02] 
[00:05:02] 
[00:05:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:02] Build completed unsuccessfully in 0:01:19
