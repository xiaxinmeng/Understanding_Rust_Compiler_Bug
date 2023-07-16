plain
==> Auto-updated Homebrew!
Updated 1 tap (caskroom/cask).
No changes to formulae.
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1559664047~hmac=cc7fcdcc62c334eb0aac38791c979c711ad319a892eb1a3198283e903030c631&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX181z51bgEVsyZJzHC_78wsqlaA0z1IqyRWf8uZ37xNiMNv-hmeCBH3TmB0xLxNiBTluJXHRI-LcSGjRQshkzaBleI0QpKHwulc7lULClbIJ84GcmnkoFYc4fmoZyYdsiSqlzR3pOoJk2w&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1559664061~hmac=978d5a3090e554782dfd6507d3366c9f7dfd2c5f46879a225f3e884d94d6d75d&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-yp6w121VmDUsPP3TcQ5JmfdF_k9E0FvTpGBHn6ntu0JLwJdgfEANawOxZsYYwS5DOU7dU6Qwfot0a9TwslF0qSo0NWh9ruDGweoLWf38V9E5NhlZtG2q_IrJZzYsep7xWzUZcKbz-Jw&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1559664064~hmac=cf14eeab4fda5caca38dd609ad9ebd8ceff907899d84754870a4abf91656c908&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-wgoyOuZ49MMVi6x8apYgcHzHq88456SPcg-WG6fjq2HR5EiuKTxjubrFlZNd2ZodFmuqFNBrN65B-ckdculFxr6EoQuFG0TX6G86L4lFks8gYYq8Ex_aK1scq7OX1NF3E959vrki3eA&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:07ba2460:start=1559662976685727000,finish=1559663384961537000,duration=408275810000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:38]       Memory: 8 GB
[00:03:38]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:38]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:38]       SMC Version (system): 2.8f0
[00:03:38]       Serial Number (system): VMM1GQlH2cbe
[00:03:38] 
[00:03:38] hw.ncpu: 4
[00:03:38] hw.byteorder: 1234
[00:03:38] hw.memsize: 8589934592
---
Building stage1 tool cargo (x86_64-apple-darwin)
[01:29:21]  Downloading crates ...
[01:29:21] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:29:21] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:29:21] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.1.0+1.1.1a/download`
[01:29:21] Caused by:
[01:29:21]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:29:21] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:29:21] expected success, got: exit code: 101
---
travis_fold:start:after_failure.2
travis_time:start:017d3e0c
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0874b7d7
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0874b7d7:start=1559668757641627000,finish=1559668757677846000,duration=36219000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10bd56f9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:230da818
travis_time:start:230da818
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0394a0f4
$ dmesg | grep -i kill
