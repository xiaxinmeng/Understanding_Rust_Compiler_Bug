plain
failures:

---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs#aarch64 stdout ----

error in revision `aarch64`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
error: requires `callable` lang_item
  --> /checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs:25:5
   |
25 |     f(arg)
---
error: aborting due to 3 previous errors
------------------------------------------


---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
error: requires `callable` lang_item
  --> /checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs:25:5
   |
25 |     f(arg)
