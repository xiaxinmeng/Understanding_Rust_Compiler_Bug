plain
travis_time:end:0c646b52:start=1553770199581150818,finish=1553770200679817968,duration=1098667150
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
18940 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen
17940 ./src/llvm-project/lldb/www/python_reference
travis_time:end:034c1d09:start=1553771999465456737,finish=1553772000416824880,duration=951368143
travis_fold:end:after_failure.1
trabuild/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a3c5be
$ dmesg | grep -i kill
