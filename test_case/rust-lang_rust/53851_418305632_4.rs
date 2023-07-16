\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-fn-error.rs","byte_start":690,"byte_end":694,"line_start":19,"line_end":19,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for i in 0..x {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0019]: constant function contains unimplemented expression type\n  --> /checkout/sr-error.rs","byte_start":839,"byte_end":843,"line_start":29,"line_end":29,"column_start":19,"column_end":23,"is_primary":false,"text":[{"text":"    let a : [i32; f(X)]; //~ ERROR E0080","highlight_start":19,"highlight_end":23}],"label":"inside call to `f`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-fn-error.rs","byte_start":833,"byte_end":844,"line_start":29,"line_end":29,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let a : [i32; f(X)]; //~ ERROR E0080","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: could not evaluate constant expression\n  --> /checkout/src/test/ui/consts/const-fn-error.rs:29:13\n   |\nLL |     for i in 0..x {\n   |              ---- inside call to `std::iter::range::<impl std::iter::Iterator for std::ops::Range<A>><usize>::next`\n...\nLL |     let a : [i32; f(X)]; //~ ERROR E0080\n   |             ^^^^^^----^\n   |                   |\n   |                   inside call to `f`\n   | \n  ::: /checkout/src/libcore/ptr.rs:195:17\n   |\nLL |         let z = read(x);\n   |                 ------- inside call to `std::ptr::read::<usize>`\n...\nLL |     let mut tmp: T = mem::uninitialized();\n   |                      -------------------- inside call to `std::mem::uninitialized::<usize>`\n   | \n  ::: /checkout/src/libcore/iter/range.rs:228:17\n   |\nLL |                 mem::swap(&mut n, &mut self.start);\n   |                 ---------------------------------- inside call to `t/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:21] 
[00:46:21] 
[00:46:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:21] Build completed unsuccessfully in 0:03:03
[00:46:21] Build completed unsuccessfully in 0:03:03
[00:46:21] make: *** [check] Error 1
[00:46:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a90dd81
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
travis_time:end:0031b808:start=1536053996420911860,finish=1536053997436380970,duration=1015469110
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:008b01b2
$ dmesg | grep -i kill
