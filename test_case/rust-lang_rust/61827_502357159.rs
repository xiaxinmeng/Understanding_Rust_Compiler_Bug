plain
travis_time:end:0a679640:start=1560596573215584404,finish=1560596661210865901,duration=87995281497
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_fold:start:before_install.3
travis_time:start:0b72f064
$ if [ "$TRAVIS_OS_NAME" = linux ]; then echo '{"ipv6":true,"fixed-cidr-v6":"fd9a:8454:6789:13f7::/64"}' | sudo tee /etc/docker/daemon.json; sudo service docker restart; fi
{"ipv6":true,"fixed-cidr-v6":"fd9a:8454:6789:13f7::/64"}
is_retry brew update && travis_retry brew install xz && travis_retry brew install swig@3; brew link --force swig@3 fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac '
travis_time:end:09d300a4:start=1560596668524290032,finish=1560596668527636134,duration=3346102
The command "case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$SCRIPT" == "./x.py dist" ]]; then travis_retry brew update && travis_retry brew install xz && travis_retry brew install swig@3; brew link --force swig@3 fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac" failed and exited with 1 during .
Your build has been stopped.
