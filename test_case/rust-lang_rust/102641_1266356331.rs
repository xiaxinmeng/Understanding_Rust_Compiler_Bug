plain
..................................................iii................................... 13552/13622
......................................................................
failures:

---- [ui] src/test/ui/dyn-star/box.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/auxiliary"
stdout: none
--- stderr -------------------------------
Invalid InsertValueInst operands!
  %5 = insertvalue { i64, i64* } %4, <{ i8*, [16 x i8], i8* }>* @7, 1
LLVM ERROR: Broken module found, compilation aborted!



failures:
