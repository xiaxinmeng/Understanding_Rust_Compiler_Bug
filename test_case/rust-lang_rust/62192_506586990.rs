plain
travis_fold:end:system_info

Uninstalled oclint to prevent interference with other packages.
If you need oclint, you must explicitly install it.
-e:1:in `read': No such file or directory @ rb_sysopen - /Users/travis/.m2/settings.xml (Errno::ENOENT)
 from -e:1:in `<main>'
mv: rename settings.xml to /Users/travis/.m2/settings.xml: No such file or directory
travis_fold:start:git.checkout
travis_time:start:015b1f7a
$ git clone --depth=2 --branch=auto https://github.com/rust-lang/rust.git rust-lang/rust
---
wry
xmoto
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1561691236~hmac=e8333c17e10a70f78d5128aedf56c51bc0d47092deddb899d4c659151c6a1803&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_FvlFOP9Z_VH-0hKeBIKf2Sbs8TjFg4q5V8atqZmJMJBd2X8y3TR9xPB91TnDN2RHfbVieGArEyAJ8WxmCh5C6XrdjfEzMM5mu9NAfbyeL87u-jAdQL_em6jpt9FX1nezKj_m_i-QOOg&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig@3: pcre
==> Installing swig@3 dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1561691248~hmac=ae4b7e625e514c046b113b17afe76aa992ca3d3581d72e4c1e4125bdd6e38c32&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-t5_ERrugC-1De6yzyk1mCfALSuc9wY0oaXWAK5d3-Kvc5cJQe9JKyvTdw7lyH-2Rq-hk9hCNjR1MIGG-TEFFJ70KmmhRcjn14-ZE42FYQe8Uc8QF3LwCfNzPiwumqN0itlajcQR7ITg&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig@3
==> Downloading https://homebrew.bintray.com/bottles/swig@3-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig@3-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/73/730bd728981cc1534664ef35d08d0b285e79756c286913d868af6afa43f60f4d?__gda__=exp=1561691250~hmac=bd027cf10f40d6e3cea2c0091331af5d64a8e3e34fa90279905f7b774d364313&response-content-disposition=attachment%3Bfilename%3D%22swig%403-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19DzERH-IWtSUzWhEc1QD42CuPsCYtRVehSaTkYoYos3A14YZM1Avjt_B-EP9Y0dPwVsofIwAloyAxUrAD2xkvqj_6uLLGGh4FTZoujpcTlqcnrZaAy-eqsFcX-mKABgfPZYNim4MyV8Q&response-X-Checksum-Sha1=4dc415ab888a7792f289543bafff9d4ec27cebb3&response-X-Checksum-Sha2=730bd728981cc1534664ef35d08d0b285e79756c286913d868af6afa43f60f4d
==> Caveats
swig@3 is keg-only, which means it was not symlinked into /usr/local,
because this is an alternate version of another formula.
If you need to have swig@3 first in your PATH run:
---
[00:01:51]    Compiling serde_derive v1.0.81
[00:02:18]    Compiling toml v0.4.10
[00:02:18]    Compiling serde_json v1.0.33
[00:02:23]    Compiling bootstrap v0.0.0 (/Users/travis/build/rust-lang/rust/src/bootstrap)
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
