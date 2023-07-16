\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/const-int-sign.rs","byte_start":478,"byte_end":492,"line_start":11,"line_end":11,"column_start":12,"column_end":26,"is_primary":true,"text":[{"text":"#![feature(const_int_sign)]","highlight_start":12,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0635]: unknown feature `const_int_sign`\n  --> /checkout/src/test/run-pass/const-int-sign.rs:11:12\n   |\nLL | #![feature(const_int_sign)]\n   |            ^^^^^^^^^^^^^^\n\n"}
[01:03:14] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:03:14] {"message":"For more information about this error, try `rustc --explain E0635`.","code":null,"level":"","spanout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:14] 
[01:03:14] 
[01:03:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:14] Build completed unsuccessfully in 0:09:42
[01:03:14] Build completed unsuccessfully in 0:09:42
[01:03:14] Makefile:58: recipe for target 'check' failed
[01:03:14] make: *** [check] Error 1
4278492 .
2645360 ./obj
2632608 ./obj/build
1972796 ./obj/build/x86_64-unknown-linux-gnu
---
150156 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
150152 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
147760 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
144216 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144212 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7wajigbwr-1xzab29-c95hburtu2sh
