plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1554955797~hmac=4bd112398c92f6f67d592a7ed8c8860931ccaa13e184bd6af6c2fa9ea7c762c8&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1982Ru-foK9fCEO86Y8wuIqvw6pvW4JkgYmOzM0CeCgTnOPbLxZWOQgKrRH7TXmzTWijHoZtpd19_p43bX4JEydFBc4ffpkTcwPqTcnq6YaP47J6rGNq9Ny1Im3NYnjiUmM6Al5gZJaTw&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1554955810~hmac=60afc945d61006458556f2521732e2ef6c3fc0c8f4cdf845b517b5d1daf497d3&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-e2M19gT-3V1xJSq2Eu2kjvHonRTmwjgIWhClbNwdwbtu1tSAE7OBGlt1tkHFk4jfm_vXcdqDxkfV4NEFxbJLZYa22_cQbwxc3T4Z6Eheda-3YKb7uk1eUn1b1xUdsUOz7xUwk1UxDmA&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/c0/c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1?__gda__=exp=1554955812~hmac=d8b415e9d2a94a20025b36077efcd023a812df4db232305ac214772ec2835450&response-content-disposition=attachment%3Bfilename%3D%22swig-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_mOWvTkwlVhMtj1AWAFfvbUMFL3K3jTO2ezYiE8fPIUfeEYeSZ9xftCNBoJhJbU1hAMsQ2fQmYINWWbRcU9iL_ElqKIHjQD32nKsxztUhhDKQTa2XVMdhwusm6NrniqwtrztNFii7Gqg&response-X-Checksum-Sha1=db6e6ed21965214d5f9fba1b180517bb2587ef59&response-X-Checksum-Sha2=c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
travis_time:end:01869080:start=1554954745569511000,finish=1554955129605510000,duration=384035999000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:01]       Memory: 8 GB
[00:03:01]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:01]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:01]       SMC Version (system): 2.8f0
[00:03:01]       Serial Number (system): VMCeB2aHgMdk
[00:03:01] 
[00:03:01] hw.ncpu: 4
[00:03:01] hw.byteorder: 1234
[00:03:01] hw.memsize: 8589934592
---
[01:55:53] 
[01:55:53] error: Could not document `core`.
[01:55:53] 
[01:55:53] Caused by:
[01:55:53]   process didn't exit successfully: `/Users/travis/build/rust-lang/rust/build/bootstrap/debug/rustdoc --crate-name core src/libcore/lib.rs --color always --target armv7-apple-ios -o /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-std/armv7-apple-ios/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.35.0 --index-page /Users/travis/build/rust-lang/rust/src/doc/index.md -L dependency=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-std/armv7-apple-ios/release/deps -L dependency=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-std/release/deps` (exit code: 1)
[01:55:53] 
[01:55:53] 
[01:55:53] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "rustdoc" "--target" "armv7-apple-ios" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.35.0" "--index-page" "/Users/travis/build/rust-lang/rust/src/doc/index.md"
[01:55:53] 
[01:55:53] 
[01:55:53] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap doc
[01:55:53] Build completed unsuccessfully in 0:14:51
---
travis_fold:start:after_failure.2
travis_time:start:0247fe39
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:056e1b80
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:056e1b80:start=1554962090320855000,finish=1554962090351100000,duration=30245000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09e4e5dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07bb746e
travis_time:start:07bb746e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00f5bde5
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
