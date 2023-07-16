plain
---- [codegen] src/test/codegen/issue-99551.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/issue-99551.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-99551" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-99551/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
Invalid InsertValueInst operands!
  %5 = insertvalue { {}*, [3 x i64]* } %4, i8* %3, 1
LLVM ERROR: Broken module found, compilation aborted!



failures:
