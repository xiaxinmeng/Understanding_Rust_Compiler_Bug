\nmod SomeModule {\n    pub const PRIVATE: u32 = 0x_a_bad_1dea_u32; /ilecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:23] 
[00:45:23] 
[00:45:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:23] Build completed unsuccessfully in 0:02:38
[00:45:23] Build completed unsuccessfully in 0:02:38
[00:45:23] make: *** [check] Error 1
[00:45:23] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:021b7f60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
