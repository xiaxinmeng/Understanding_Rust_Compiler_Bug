plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1556615783~hmac=9aefd04fbfdc5d8f571225295b973d4d36d8e6a4dbb73f70f97563dff17bc759&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-qPdjFUP9xk8A7dGc4yL2G1vY5mIpyxrfpJ2f10CSv31RAoGB38VsCeSt0SYejhV5YxxJi_z2-K9aRf319qGfkbEYTOMdKMezIMtR5kzoSHRhjEsXk3R2rXBu7Yi2Yebfp3U22kElBrA&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1556615795~hmac=30d70b11f47700cac6e627752960fb25251c6e46bda59fc079b36e689006c7c0&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_XTRtdv8J-xNcPRBzPpbUrta8C-h6RKw9ZpziT4t8Xh6g3WGIf_0wawOQ6mPeXpsa-GoUNzBDTj6CpWidmiP2P2hkyrWK7078fcXpVfJy25-kL4Jy1hah020_vGSlUDvygtc1ts2B3Nw&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/c0/c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1?__gda__=exp=1556615797~hmac=0a3402505c6b04e07a058c95ee0e18ba9e6984d16c53701c273df12abb267456&response-content-disposition=attachment%3Bfilename%3D%22swig-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-WuQRieRAKerKCzA1TDpu1t05sYZ0cKXrj3g8TdKmPI9uGoGmUxP6l6DHwClC0GZoDt17qAFxSnuaWqU6z8nZBAvVoaXSPtpwbE1cMo63WD4Q1PHGuY1htnxE1taGKh2nwqbf-MONmZg&response-X-Checksum-Sha1=db6e6ed21965214d5f9fba1b180517bb2587ef59&response-X-Checksum-Sha2=c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
travis_time:end:0c8daf0a:start=1556614720411828000,finish=1556615114700124000,duration=394288296000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:09]       Memory: 8 GB
[00:03:09]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:09]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:09]       SMC Version (system): 2.8f0
[00:03:09]       Serial Number (system): VMp3SFW+fQBf
[00:03:09] 
[00:03:09] hw.ncpu: 4
[00:03:09] hw.byteorder: 1234
[00:03:09] hw.memsize: 8589934592
---
Building stage2 tool cargo (i686-apple-darwin)
[01:07:52]  Downloading crates ...
[01:07:52] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:07:52] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:07:52] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.1.0+1.1.1a/download`
[01:07:52] Caused by:
[01:07:52]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:07:52] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0/bin/cargo" "build" "--target" "i686-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:07:52] expected success, got: exit code: 101
---
travis_fold:start:after_failure.2
travis_time:start:06371a80
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0d98c900
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0d98c900:start=1556619196174465000,finish=1556619196206893000,duration=32428000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:164ac20a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:021c45b9
travis_time:start:021c45b9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:077aa678
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
