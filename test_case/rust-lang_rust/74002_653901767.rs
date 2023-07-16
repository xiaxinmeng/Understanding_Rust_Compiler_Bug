
failures:

---- [codegen] codegen/codemodels.rs#MODEL-KERNEL stdout ----

error in revision `MODEL-KERNEL`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/codemodels.rs" "-Zthreads=1" "--cfg" "model_kernel" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/codemodels.MODEL-KERNEL" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target=x86_64-unknown-linux-gnu" "-C" "code-model=kernel" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/codemodels.MODEL-KERNEL/auxiliary" "--emit=llvm-ir"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.

------------------------------------------

error in revision `MODEL-MEDIUM`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/codemodels.rs" "-Zthreads=1" "--cfg" "model_medium" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/codemodels.MODEL-MEDIUM" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target=x86_64-unknown-linux-gnu" "-C" "code-model=medium" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/codemodels.MODEL-MEDIUM/auxiliary" "--emit=llvm-ir"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.

------------------------------------------


failures:
    [codegen] codegen/codemodels.rs#MODEL-KERNEL
    [codegen] codegen/codemodels.rs#MODEL-MEDIUM

test result: FAILED. 165 passed; 2 failed; 42 ignored; 0 measured; 0 filtered out
