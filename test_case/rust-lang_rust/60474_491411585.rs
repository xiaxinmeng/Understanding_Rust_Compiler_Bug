plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1557513990~hmac=d8211a717c45f939840c53ce9c9f21816feaa5db377fd5e25fb62863080df7f9&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18JcKMkaAZHj1drqJ9PtPhn2m9Nt6NF6CyC2lMSXvlmK8jaVl-zF3KFCJPxHebxT-8KV4YiSE_YqeQj5mxhOQUgdSuIV9i4clVRxKEaB7Ujg7B0D7kLwCjfbZBtdg9IsRDPBjsSc6-5ZQ&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1557514003~hmac=a03f048509c4c7282c645035f93de26ff3791323884da7219600dc317ef2ce63&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1__9SxNyFXl8lMcfBZxta8hQWouQygOB47Vzg3i_ija6A2VwMH4plkv0BwrL3PWwgIk5tzHZ7_hc2-IQn6HbhHxqkoBOxrRXH8xpS4QW0WIgC6l9OgC7u-UIG8myFfWpmfQi-oyn2N3XA&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1557514006~hmac=ac38fc3394cb5def02bc632ec4b246c47955ed0169051def57775d56d94d5493&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18CiMNuoOEd8rNjDlmMJ5QcbXJjFiXo3ytb07SEds3tuhLUth2mvMN9V_HIe0nshvFvDqDdnGDMc-kc33hD4lFqmae_8bw7k-TLSmcuKcrNl9wlAzl4CgPUUGdUlY6KW3nVwJJjflTWnA&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:1e8458da:start=1557512942374277000,finish=1557513322994981000,duration=380620704000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:19]       Memory: 8 GB
[00:03:19]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:19]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:19]       SMC Version (system): 2.8f0
[00:03:19]       Serial Number (system): VMfegqCnyfSo
[00:03:19] 
[00:03:19] hw.ncpu: 4
[00:03:19] hw.byteorder: 1234
[00:03:19] hw.memsize: 8589934592
---
Building stage2 tool cargo (x86_64-apple-darwin)
[01:18:55]  Downloading crates ...
[01:18:55] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:18:55] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:18:55] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.1.0+1.1.1a/download`
[01:18:55] Caused by:
[01:18:55]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:18:55] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:18:55] expected success, got: exit code: 101
---
travis_fold:start:after_failure.2
travis_time:start:09c1b000
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0fd52673
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0fd52673:start=1557518067395537000,finish=1557518067411505000,duration=15968000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1739c41d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04433b28
travis_time:start:04433b28
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0854328c
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
