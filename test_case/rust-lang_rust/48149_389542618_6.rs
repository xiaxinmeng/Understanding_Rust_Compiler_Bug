\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs","byte_start":1223,"byte_end":1230,"line_start":31,"line_end":31,"column_start":29,"column_end":36,"is_primary":true,"text":[{"text":"    type FErr2<T> = Self::E<'static, T, u32>; // Error","highlight_start":29,"highlight_end":36}],"label":"lifetime parameter not allowed","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0110]: lifetime parameters are not allowed on this type\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs:31:29\n   |\nLL |     type FErr2<T> = Self::E<'static, T, u32>; // Error\n   |                             ^^^^^^^ lifetime parameter not allowed\n\n"}
[01:16:32] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:16:32] {"message":"For more information about this error, try `rustc --explain E0110`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0110`.\n"}
[01:16:32] ------------------------------------------
[01:16:32] 
[01:16:32] thread '[ui] ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:16:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:16:32] 
[01:16:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:16:32] 
[01:16:32] 
[01:16:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:32] 
[01:16:32] 
[01:16:32] failed to run: /checkout/o028 ./obj/build/x86_64-unknown-linux-gnu/doc/std
75944 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
---
56296 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
56292 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
56088 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
54152 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-1ov3jcziodo4r
54148 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-1ov3jcziodo4r/s-f13bsop3cq-5jlowk-3qeg55kipmxns
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47984 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47400 ./src/test
46812 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
