compile_fail,E0308\nlet x: i32 = \"I a
[00:47:18] 
[00:47:18] thread '[ui] ui/consts/const-int-overflowing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:18] 
[00:47:18] ---- [ui] ui/consts/const-int-sign.rs stdout ----
[00:47:18] ---- [ui] ui/consts/const-int-sign.rs stdout ----
[00:47:18] 
[00:47:18] error: ui test compiled successfully!
[00:47:18] status: exit code: 0
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-sign.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/auxiliary" "-A" "unused"
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] ------------------------------------------
[00:47:18] stderr:
---
[00:47:18] ---- [ui] ui/consts/const-int-wrapping.rs stdout ----
[00:47:18] 
[00:47:18] error: ui test compiled successfully!
[00:47:18] status: exit code: 0
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-wrapping.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/auxiliary" "-A" "unused"
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] ------------------------------------------
[00:47:18] stderr:
---
151200 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148720 ./obj/build/bootstrap/debug/incremental
134288 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134284 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f487l9bm1y-yjw2xz-3b1qbyx5cod2e
