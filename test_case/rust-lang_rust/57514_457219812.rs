plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00c8eb66
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:33] Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --enable-debug       --enable-lld       --enable-lldb       --enable-optimize
[00:01:33]  ---> Running in 62c13371864e
[00:01:34] Removing intermediate container 62c13371864e
[00:01:34]  ---> 8f7776811a25
[00:01:34] Step 8/8 : ENV SCRIPT   python2.7 ../x.py build &&   python2.7 ../x.py test src/test/run-make-fulldeps --test-args clang
[00:01:34] Removing intermediate container 234fc4bfd405
[00:01:34]  ---> 9fcabab32197
[00:01:34] Successfully built 9fcabab32197
[00:01:34] Successfully tagged rust-ci:latest
---
[00:37:33] -- Looking for sys/resource.h - found
[00:37:33] -- Clang version: 8.0.0
[00:37:33] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG
[00:37:33] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG - Failed
[00:37:33] -- Could NOT find Z3 (missing:  Z3_LIBRARIES Z3_INCLUDE_DIR) (Required is exact version "4.7.1")
[00:37:34] -- Found PythonLibs: /usr/lib/x86_64-linux-gnu/libpython2.7.so (found version "2.7.12") 
[00:37:34] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS - Success
[00:37:34] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS
[00:37:34] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS - Success
[00:37:34] -- Performing Test CXX_SUPPORTS_NO_STRICT_ALIASING
---
[00:37:34] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES
[00:37:35] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES - Success
[00:37:35] -- LLDB version: 8.0.0
[00:37:35] -- Could NOT find LibXml2 (missing:  LIBXML2_LIBRARIES LIBXML2_INCLUDE_DIR) 
[00:37:35] CMake Error at /usr/share/cmake-3.5/Modules/FindPackageHandleStandardArgs.cmake:148 (message):
[00:37:35]   Could NOT find Curses (missing: CURSES_LIBRARY CURSES_INCLUDE_PATH)
[00:37:35] Call Stack (most recent call first):
[00:37:35]   /usr/share/cmake-3.5/Modules/FindPackageHandleStandardArgs.cmake:388 (_FPHSA_FAILURE_MESSAGE)
[00:37:35]   /usr/share/cmake-3.5/Modules/FindCurses.cmake:206 (FIND_PACKAGE_HANDLE_STANDARD_ARGS)
[00:37:35]   /checkout/src/tools/lldb/cmake/modules/LLDBConfig.cmake:379 (find_package)
[00:37:35] 
[00:37:35] 
[00:37:35] -- Configuring incomplete, errors occurred!
[00:37:35] -- Configuring incomplete, errors occurred!
[00:37:35] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
[00:37:35] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
[00:37:35] command did not execute successfully, got: exit code: 1
[00:37:35] 
[00:37:35] 
[00:37:35] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:37:35]  finished in 11.969
[00:37:35] travis_fold:end:llvm

[00:37:35] travis_time:end:llvm:start=1548334825542602338,finish=1548334837512172296,duration=11969569958
---
travis_time:end:027c14a2:start=1548334839110386455,finish=1548334839130501577,duration=20115122
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:018cedcf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:099911a6
travis_time:start:099911a6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:039e0bf6
$ dmesg | grep -i kill
