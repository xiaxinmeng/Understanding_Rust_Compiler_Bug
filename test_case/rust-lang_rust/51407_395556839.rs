plain
travis_fold:start:worker_info
Worker information
hostname: 59534c98-0dfc-438e-9d62-758f32576086@1.production-5-worker-org-a-1-gce
version: v3.8.0 https://github.com/travis-ci/worker/tree/cccff7c89da7ba0cf47a90e5615266a45b372e30
instance: travis-job-3e53eb04-2990-4654-8374-064578f74ee8 travis-ci-connie-trusty-1512502258-986baf0 (via amqp)
startup: 26.609340627s
travis_fold:end:worker_info
Download from https://build.travis-ci.org/filter/redirect_io.rb failed. Trying https://build.travis-ci.com/filter/redirect_io.rb ...
Build system information
Build language: shell
Build group: stable
Build dist: trusty
---
curl: (7) Failed to connect to s3-us-west-1.amazonaws.com port 443: Connection timed out
The command "curl -fo /home/travis/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl" failed 3 times.
travis_time:end:284027b8:start=1528403482844337679,finish=1528403867590529085,duration=384746191406

The command "case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/6.0.0/clang+llvm-6.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac" failed and exited with 7 during .
Your build has been stopped.
