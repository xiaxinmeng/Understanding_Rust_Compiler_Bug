plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1557102438~hmac=052aedc5e2879661c8649ad3a8c4e6959c2161a1d30400838a7600abdf50c185&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-4qdonqVQHuRaC8OSawRkHIrYWKtRwIrGCkcIFth97DVdje1hEnE6QKyT3HZbnCZRnZNZfO00HMA2RfMMlrBZ2UTXW9-LcUQgBdMDtMrdrPL186ShiDwVim31KzystrM547jtb3WtcfA&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1557102450~hmac=ddf442daeb7d8e3022a6ca0dca35019623edb77e1f8035b07312bfa57cc3b9c4&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18IT_LVbOSyjpaDilQ-VnRMtP-fVh-m6zIkFu7x1wrNF2cYlcCNxgKRuSVuK2gQLjDcvFmR8T2LfAe726j2DK50BmaZwMfuxAe0Gy_ZWrFjmJivi0cQ98UzWBZxTyVgnxEZ-6A_zT1O2Q&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1557102453~hmac=f277a447306662210663caa5e92963e15f07b1f701232c44ea8ab014887854e4&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19Wu9S6fx3cfsW9Gb7x_9pGbUFmzYEtWVIVFdXNDNyFSP3NdKGLOmxQSPWLCZZtuqNm3TB0Bpl1GkMK5HCuHD4OHQ5lcdroMwC0LNhx2imkfSDJ93a6H9G26nUt6nCbTD6v5VlqHJDssA&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
curl: (56) Recv failure: Connection reset by peer
The command "curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz" failed. Retrying, 2 of 3.
clang+llvm-7.0.0-x86_64-apple-darwin/bin/llc: Lzma library error: Corrupted input data
tar: Error exit delayed from previous errors.
travis_time:end:10a3c74e:start=1557101396559657000,finish=1557101752202338000,duration=355642681000
The command "case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz && travis_retry brew install swig; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac" failed and exited with 1 during .
Your build has been stopped.
