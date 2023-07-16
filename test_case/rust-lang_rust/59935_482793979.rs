plain
travis_time:end:04814a5d:start=1555147275867311234,finish=1555147277994247786,duration=2126936552
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1222f66a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannn/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ca58a96
$ dmesg | grep -i kill
