plain
travis_time:end:00749598:start=1546979279108596061,finish=1546979280061836116,duration=953240055
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:2: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:8: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:10: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:18: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:20: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:22: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:25: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:31: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:36: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:39: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:40: unexplained "