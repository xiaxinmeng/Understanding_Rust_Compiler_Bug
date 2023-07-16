plain
wry
xmoto
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1560541442~hmac=92e94f5289b8f5bc49b3e33aac0b12a478e51d0a18abbefccaa3a1d6ab0b8205&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18bsMMLPH0COtUx9oz-uaHyEwnE0YxnRIKqEZEgI3L27raf84dzYKuTrvV74VANfXFWsidnQmv8K-16Wsgne5Pa-v_4HfqtJHXCjXkcZX5fY0kHNDlfifL8ibjPyloW-k8uqKNq1juZhg&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig@3: pcre
==> Installing swig@3 dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1560541454~hmac=e21319d6c5041d311aa20b7af7916dcd4ef10c41c8f272f9ae12d199364191a4&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18y0QJYza8qitJTsakGqkI5a9Xk6mfOmjvjSC7am9nW9JylBketDRStkzpPp2-M4sCIml7STOGHIUXbYy2XtTjp19aepnvsFEc9aB1VzH38pMztqchjPrzZ6021kz9r8mQ88jiOd-VExA&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig@3
==> Downloading https://homebrew.bintray.com/bottles/swig@3-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig@3-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/73/730bd728981cc1534664ef35d08d0b285e79756c286913d868af6afa43f60f4d?__gda__=exp=1560541457~hmac=5d355ee28f21872c5cf2c7df500de5381ee9e34d7c038bf5a74dbbf408051d22&response-content-disposition=attachment%3Bfilename%3D%22swig%403-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19TT3T8s_rLtmK4whEWgnyI2mLiIZ0m7agqxVWk54emZBdANVBk8UKLXQz5Hl0bjRNXtTQh05NrKlhBqiFhYlTzgdbTNZ2yHYnQwDAn0QAS3Rgpd7dcq87MXfRJgn9FqTRxpbldpmyjvA&response-X-Checksum-Sha1=4dc415ab888a7792f289543bafff9d4ec27cebb3&response-X-Checksum-Sha2=730bd728981cc1534664ef35d08d0b285e79756c286913d868af6afa43f60f4d
==> Pouring swig@3-3.0.12.high_sierra.bottle.tar.gz
==> Caveats
swig@3 is keg-only, which means it was not symlinked into /usr/local,
because this is an alternate version of another formula.
If you need to have swig@3 first in your PATH run:
  echo 'export PATH="/usr/local/opt/swig@3/bin:$PATH"' >> ~/.bash_profile
==> Summary
🍺  /usr/local/Cellar/swig@3/3.0.12: 755 files, 5.5MB
==> Caveats
==> swig@3
swig@3 is keg-only, which means it was not symlinked into /usr/local,
because this is an alternate version of another formula.
If you need to have swig@3 first in your PATH run:
  echo 'export PATH="/usr/local/opt/swig@3/bin:$PATH"' >> ~/.bash_profile
travis_fold:end:install
travis_fold:start:before_script.1
travis_time:start:126d8332
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
[00:03:17]       Memory: 8 GB
[00:03:17]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:17]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:17]       SMC Version (system): 2.8f0
[00:03:17]       Serial Number (system): VMneSsREoq0i
[00:03:17] 
[00:03:17] hw.ncpu: 4
[00:03:17] hw.byteorder: 1234
[00:03:17] hw.memsize: 8589934592
---
[00:33:53] -- Looking for __NR_process_vm_readv - not found
[00:33:53] -- Looking for compression_encode_buffer in compression
[00:33:53] -- Looking for compression_encode_buffer in compression - found
[00:33:53] -- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
[00:33:54] CMake Error at /usr/local/Cellar/cmake/3.10.2/share/cmake/Modules/FindPackageHandleStandardArgs.cmake:137 (message):
[00:33:54]   Could NOT find SWIG (missing: SWIG_EXECUTABLE SWIG_DIR)
[00:33:54] Call Stack (most recent call first):
[00:33:54]   /usr/local/Cellar/cmake/3.10.2/share/cmake/Modules/FindPackageHandleStandardArgs.cmake:378 (_FPHSA_FAILURE_MESSAGE)
[00:33:54]   /usr/local/Cellar/cmake/3.10.2/share/cmake/Modules/FindSWIG.cmake:63 (FIND_PACKAGE_HANDLE_STANDARD_ARGS)
[00:33:54]   /Users/travis/build/rust-lang/rust/src/llvm-project/lldb/scripts/CMakeLists.txt:18 (find_package)
[00:33:54] 
[00:33:54] -- Configuring incomplete, errors occurred!
[00:33:54] -- Configuring incomplete, errors occurred!
[00:33:54] See also "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/CMakeFiles/CMakeOutput.log".
[00:33:54] See also "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/CMakeFiles/CMakeError.log".
[00:33:54] command did not execute successfully, got: exit code: 1
[00:33:54] 
[00:33:54] 
[00:33:54] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:33:54]  finished in 45.144
[00:33:54] travis_fold:end:llvm

[00:33:54] travis_time:end:llvm:start=1560542766027635000,finish=1560542811170017000,duration=45142382000
---
travis_fold:start:after_failure.2
travis_time:start:04ccb0c0
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:00dd70f6
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:00dd70f6:start=1560542817596342000,finish=1560542817621856000,duration=25514000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:031efab0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:006c348f
travis_time:start:006c348f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0741b4ba
$ dmesg | grep -i kill
