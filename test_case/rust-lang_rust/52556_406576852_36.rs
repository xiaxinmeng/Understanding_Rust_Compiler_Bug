\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/privacy-struct-ctor.rs","byte_start":1411,"byte_end":1426,"line_start":55,"line_end":55,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    xcrate::m::n::Z;","highlight_start":5,"highlight_end":20}],"label"own-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:06] 
[00:44:06] 
[00:44:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:06] Build completed unsuccessfully in 0:01:40
[00:44:06] Build completed unsuccessfully in 0:01:40
[00:44:06] make: *** [check] Error 1
[00:44:06] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04d370f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
143676 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
133184 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133180 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130536 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130532 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f32tyfis2o-aq09rl-2iqfvo5raelnm
128724 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122252 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121712 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
107600 ./src/llvm/test/CodeGen
---
30728 ./.git/modules/src/tools/lld/objects/pack
lure.4
travis_fold:start:after_failure.5
travis_time:start:158fe464
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:062ba022
$ dmesg | grep -i kill
