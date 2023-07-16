plain
travis_time:end:2f35e68a:start=1541037063465106335,finish=1541037065498413215,duration=2033306880
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:end:0053d52c:start=1541037075356955537,finish=1541037075370511011,duration=13555474
travis_fold:end:before_script.2
travis_time:start:153857fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov  1 01:51:15 UTC 2018
Thu, 01 Nov 2018 01:51:15 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_time:start:05189ae4
