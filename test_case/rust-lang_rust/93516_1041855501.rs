plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...iiiiii...................
failures:

---- [codegen] codegen/branch-protection.rs#DISABLED stdout ----

error in revision `DISABLED`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/branch-protection.rs" "-Zthreads=1" "--cfg" "disabled" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.DISABLED" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "branch-protection=" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.DISABLED/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incorrect value `` for debugging option `branch-protection` - a `,` separated combination of `bti`, `b-key`, `pac-ret`, or `leaf` was expected

------------------------------------------


