plain
 finished in 6.536 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 392 tests
i.....i..............i....i...ii.......F.FFF....iii........ii.i........i................ 88/392
iii.........i.iii....i.i...................i...ii...i.....ii..i.ii....i...............ii 264/392
........................i.i.ii.i.i............i..i....i....i..iii.......i...i.i......... 352/392
...........iiiiiiii.i...................
failures:
failures:

---- [codegen] src/test/codegen/branch-protection.rs#BKEY stdout ----

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error in revision `BKEY`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/branch-protection.rs" "-Zthreads=1" "--cfg" "bkey" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.BKEY" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.BKEY/auxiliary" "-Z" "branch-protection=pac-ret,b-key" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------



---- [codegen] src/test/codegen/branch-protection.rs#BTI stdout ----

error in revision `BTI`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/branch-protection.rs" "-Zthreads=1" "--cfg" "bti" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.BTI" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.BTI/auxiliary" "-Z" "branch-protection=bti" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------



---- [codegen] src/test/codegen/branch-protection.rs#PACRET stdout ----

error in revision `PACRET`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/branch-protection.rs" "-Zthreads=1" "--cfg" "pacret" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.PACRET" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.PACRET/auxiliary" "-Z" "branch-protection=pac-ret" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------



---- [codegen] src/test/codegen/branch-protection.rs#LEAF stdout ----

error in revision `LEAF`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/branch-protection.rs" "-Zthreads=1" "--cfg" "leaf" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.LEAF" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/branch-protection.LEAF/auxiliary" "-Z" "branch-protection=pac-ret,leaf" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
i32 8
invalid behavior operand in module flag (unexpected constant)
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------


