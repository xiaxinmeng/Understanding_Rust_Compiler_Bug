plain
[00:50:46] -- Detecting CXX compiler ABI info
[00:50:47] -- Detecting CXX compiler ABI info - done
[00:50:47] -- Detecting CXX compile features
[00:50:47] -- Detecting CXX compile features - done
[00:50:47] CMake Error at cmake/modules/CheckCompilerVersion.cmake:40 (message):
[00:50:47]   Host GCC version should be at least 5.1 because LLVM will soon use new C++
[00:50:47]   features which your toolchain version doesn't support.  Your version is
[00:50:47]   4.8.4.  You can temporarily opt out using
[00:50:47]   LLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN, but very soon your toolchain won't be
[00:50:47] Call Stack (most recent call first):
[00:50:47]   cmake/modules/CheckCompilerVersion.cmake:45 (check_compiler_version)
[00:50:47]   cmake/config-ix.cmake:13 (include)
[00:50:47]   CMakeLists.txt:590 (include)
[00:50:47]   CMakeLists.txt:590 (include)
[00:50:47] 
[00:50:47] 
[00:50:47] -- Configuring incomplete, errors occurred!
[00:50:47] See also "/checkout/obj/build/x86_64-unknown-netbsd/llvm/build/CMakeFiles/CMakeOutput.log".
[00:50:47] command did not execute successfully, got: exit code: 1
[00:50:47] 
[00:50:47] 
[00:50:47] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:50:47]  finished in 0.907
[00:50:47] travis_fold:end:llvm

[00:50:47] travis_time:end:llvm:start=1551247970572222168,finish=1551247971479402368,duration=907180200
---
travis_time:end:0c94d996:start=1551247972906496795,finish=1551247972912434671,duration=5937876
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002c1450
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:051c1aab
travis_time:start:051c1aab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01facf73
$ dmesg | grep -i kill
