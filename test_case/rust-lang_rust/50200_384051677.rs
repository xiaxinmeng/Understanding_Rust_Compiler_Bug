plain
127.0.0.1 localhost nettuno travis vagrant
127.0.1.1 travis-job-5cff6539-afe8-44f9-8901-425bfd386108 travis-job-5cff6539-afe8-44f9-8901-425bfd386108 ip4-loopback trusty64
travis_fold:start:git.checkout
travis_time:start:0cd94354
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:1f95cb80:start=1524591862996277282,finish=1524591864186661505,duration=1190384223
travis_fold:end:before_install.3
travis_fold:start:install
travis_time:start:07093b12
$ case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -O http://releases.llvm.org/6.0.0/clang+llvm-6.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++ ;; esac
travis_fold:end:install
travis_fold:start:before_script.1
travis_time:start:3f558bac
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
