plain
[00:46:31] ....................................................................................................
[00:46:34] ....................................................................................................
[00:46:36] ....................................................................................................
[00:46:38] ...................................................................................................i
[00:46:41] ..........F........................................................i................................
[00:46:47] ....................................................................................................
[00:46:49] ..............................................i.....................................................
[00:46:52] ....................................................................................................
[00:46:55] ....................................................................................................
---
[00:47:37] ....................................................................................................
[00:47:41] ................i...................................................................................
[00:47:43] ....................................................................................................
[00:47:46] ....................................................................................................
[00:47:50] .................F...............................i..................................................
[00:47:56] ....................................................................................................
[00:47:59] ....................................................................................................
[00:48:00] i............................................................
[00:48:00] failures:
[00:48:00] failures:
[00:48:00] 
[00:48:00] ---- [ui] ui/gated-attr-literals.rs stdout ----
[00:48:00] 
[00:48:00] error: ui test compiled successfully!
[00:48:00] status: exit code: 0
[00:48:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/gated-attr-literals.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/gated-attr-literals/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/gated-attr-literals/auxiliary" "-A" "unused"
[00:48:00] ------------------------------------------
[00:48:00] 
[00:48:00] ------------------------------------------
[00:48:00] stderr:
---
[00:48:00] 1 error: suffixed literals are not allowed in attributes
[00:48:00] -   --> $DIR/suffixed-literal-meta.rs:13:10
[00:48:00] +   --> $DIR/suffixed-literal-meta.rs:11:10
[00:48:00] 3    |
[00:48:00] 4 LL | #[path = 1usize] //~ ERROR: suffixed literals are not allowed in attributes
[00:48:00] 
[00:48:00] 
[00:48:00] 7    = help: instead of using a suffixed literal (1u8, 1.0f32, etc.), use an unsuffixed version (1, 1.0, etc.).
[00:48:00] 9 error: suffixed literals are not allowed in attributes
[00:48:00] -   --> $DIR/suffixed-literal-meta.rs:14:10
[00:48:00] +   --> $DIR/suffixed-literal-meta.rs:12:10
[00:48:00] 11    |
[00:48:00] 11    |
[00:48:00] 12 LL | #[path = 1u8] //~ ERROR: suffixed literals are not allowed in attributes
[00:48:00] 
[00:48:00] 
[00:48:00] 15    = help: instead of using a suffixed literal (1u8, 1.0f32, etc.), use an unsuffixed version (1, 1.0, etc.).
[00:48:00] 17 error: suffixed literals are not allowed in attributes
[00:48:00] -   --> $DIR/suffixed-literal-meta.rs:15:10
[00:48:00] +   --> $DIR/suffixed-literal-meta.rs:13:10
[00:48:00] 19    |
[00:48:00] 19    |
[00:48:00] 20 LL | #[path = 1u16] //~ ERROR: suffixed literals are not al-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:00] 
[00:48:00] 
[00:48:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:00] Build completed unsuccessfully in 0:03:06
[00:48:00] Build completed unsuccessfully in 0:03:06
[00:48:00] Makefile:58: recipe for target 'check' failed
[00:48:00] make: *** [check] Error 1
