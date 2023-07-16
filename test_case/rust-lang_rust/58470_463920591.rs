plain
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Pouring swig-3.0.12.high_sierra.bottle.tar.gz
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
curl: (56) Recv failure: Operation timed out
The command "curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz" failed. Retrying, 2 of 3.
clang+llvm-7.0.0-x86_64-apple-darwin/bin/opt: Lzma library error: Corrupted input data
tar: Error exit delayed from previous errors.
travis_time:end:1f9a4999:start=1550210497616176000,finish=1550211067419837000,duration=569803661000
The command "case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz && travis_retry brew install swig; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac" failed and exited with 1 during .
Your build has been stopped.
