plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1556385506~hmac=34bb261f6e8af9736f11ad3373ca0c096b4be0824bd7a7a0d370aba028458f2f&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-VOCRFBBAaASMyFhsyp88X39fKYn9Jt735ACl1I064FcUWXJja0VJDfBhO649HoS0Bmy7FfuL2a2KfaVnSGrtGAKGwn9Pn0kCYi2GnhJO6xCX9vJj7XqAmtVr3Ucx9dYaDEwVvcHueWw&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1556385519~hmac=be5816cd47eed0fcab0da74e73b2dbe2e81b531ab0557b07db13cfff3136d6e6&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_3Jg8IVpdLHAfnQn2AU1RUnq9PnzB_GA5m0RtdZbHtrfF8PE7ttOX3NUXwctPZK-dT1USzqTWNd_CjXmnmHRTqsNSDayb_1XoqRt75XNUkSXImwaldDcSgdnJw9AJE71uLPex45oek5A&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/c0/c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1?__gda__=exp=1556385522~hmac=1a25566467ed7bb37ebf1b229f0e4c3a485648408836abab3e5fd6d53c365095&response-content-disposition=attachment%3Bfilename%3D%22swig-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_FoLLjTfcjt2RZnJFsHDdkvppaYIlmQCjn6T3PXr2eh9emB0_mYsKJThkONtSbnW7mK7yLwnWuG43H-ee2KrfJ3XjC3XbACbw_6cxkDU27uIo7q-_4SByWRCoWhbMjN2sau5BSVURVSw&response-X-Checksum-Sha1=db6e6ed21965214d5f9fba1b180517bb2587ef59&response-X-Checksum-Sha2=c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
curl: (56) Recv failure: Connection reset by peer
The command "curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz" failed. Retrying, 2 of 3.
clang+llvm-7.0.0-x86_64-apple-darwin/lib/libLLVMBPFCodeGen.a: Lzma library error: Corrupted input data
tar: Error exit delayed from previous errors.
travis_time:end:11dbed90:start=1556384458505224000,finish=1556384810965664000,duration=352460440000
The command "case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz && travis_retry brew install swig; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac" failed and exited with 1 during .
Your build has been stopped.
