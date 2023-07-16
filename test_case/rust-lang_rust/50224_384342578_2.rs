\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":801,"byte_end":802,"line_start":24,"line_end":24,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{} {}\", X, Y);","highlight_start":26,"highlight_end":27}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0080]: constant evaluation error\/pub_const_err.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:38] ------------------------------------------
[00:44:38] 
[00:44:38] ------------------------------------------
[00:44:38] stderr:
[00:44:38] stderr:
[00:44:38] ------------------------------------------
[00:44:38] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/pub_const_err.rs","byte_start":527,"byte_end":532,"line_start":15,"line_end":15,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attempt to subtract with overflow\n  --> /checkout/src/test/ui/const-eval/pub_const_err.rs:15:20\n   |\nLL | pub const Z: u32 = 0 - 1;\n   |                    ^^^^^\n   |\n   = note: #[warn(const_err)] on by default\n\n"}
[00:44:38] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/pub_const_err.rs","byte_start":508,"byte_end":533,"line_start":15,"line_end":15,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":1,"highlight_end":26}],"label":"attempt to subtract with overflow","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: this constant cannot------
[00:44:38] thread '[ui] ui/const-eval/pub_const_err.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:44:38] 
[00:44:38] ---- [ui] ui/const-eval/pub_const_err_bin.rs stdout ----
[00:44:38]  
[00:44:38]  
[00:44:38] error: ui test compiled successfully!
[00:44:38] status: exit code: 0
[00:44:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/pub_const_err_bin.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/pub_const_err_bin.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/pub_const_err_bin.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:38] ------------------------------------------
[00:44:38] 
[00:44:38] ------------------------------------------
[00:44:38] stderr:
[00:44:38] stderr:
[00:44:38] ------------------------------------------
[00:44:38] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/pub_const_err_bin.rs","byte_start":503,"byte_end":508,"line_start":13,"line_end":13,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub const Z: u32 = 0 - 1;","highlight_start":20,"highlight_end":25}],"label":null,"suggestedsubtract with overflow\n  --> /checkout/src/test/ui/const-eval/pub_const_err_bin.rs:17:22\n   |\nLL | pub type Foo = [i32; 0 - 1];\n   |                      ^^^^^\n\n"}
[00:44:38] {"message":"this array length cannot be used","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/pub_const_err_bin.rs","byte_start":615,"byte_end":620,"line_start":17,"line_end":17,"column_start":22,"column_end":27,"is_primary":true,"text":[{"text":"pub type Foo = [i32; 0 - 1];","highlight_start":22,"highlight_end":27}],"label":"attempt to subtract with overflow","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: this array length cannot be used\n  --> /checkout/src/test/ui/const-eval/pub_const_err_bin.rs:17:22\n   |\nLL | pub type Foo = [i32; 0 - 1];\n   |                      ^^^^^ attempt to subtract with overflow\n\n"}
[00:44:38] ------------------------------------------
[00:44:38] 
[00:44:38] thread '[ui] ui/const-eval/pub_const_err_bin.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:44:38] 
---
[00:44:38] test result: FAILED. 1305 passed; 4 failed; 4 ignored; 0 measured; 0 filtered out
[00:44:38] 
[00:44:38] 
[00:44:38] 
[00:44:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:38] 
[00:44:38] 
[00:44:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:38] Build completed unsuccessfully in 0:02:04
[00:44:38] Build completed unsuccessfully in 0:02:04
[00:44:38] Makefile:58: recipe for target 'check' failed
[00:44:38] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01acc1d8
$ date && (curl292 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
$ date && (curl292 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
68772 ./src/llvm/lib
65412 ./src/llvm-emscripten/test/CodeGen
63348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
62864 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
60840 ./src/llvm-emscripten/lib
56748 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-2bjr8lg1wd06d
56744 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-2bjr8lg1wd06d/s-f0g8rt4skb-3gnuhm-249rn27v3te23
46384 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
46352 ./src/llvm/test/MC
45548 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
45488 ./src/test
