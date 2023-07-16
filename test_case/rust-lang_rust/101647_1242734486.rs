plain
........................................................................................ 12056/13504
........................................................................................ 12144/13504
........................................................................................ 12232/13504
........................................................................................ 12320/13504
.............................................................................F..F..F.... 12408/13504
........................................................................................ 12584/13504
........................................................i............................... 12672/13504
........................................................................................ 12760/13504
........................................................................................ 12848/13504
---
---- [ui] src/test/ui/traits/trait-upcasting/replace-vptr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/replace-vptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/replace-vptr/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/replace-vptr/auxiliary"
stdout: none
--- stderr -------------------------------
Use of instruction is not an instruction!
  %5 = load i8*, i8** %4, align 8, !invariant.load !4, !nonnull !4
LLVM ERROR: Broken module found, compilation aborted!


---- [ui] src/test/ui/traits/trait-upcasting/correct-supertrait-substitution.rs stdout ----


error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/correct-supertrait-substitution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/correct-supertrait-substitution/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/correct-supertrait-substitution/auxiliary"
stdout: none
--- stderr -------------------------------
Use of instruction is not an instruction!
  %78 = load i8*, i8** %77, align 8, !invariant.load !4, !nonnull !4
LLVM ERROR: Broken module found, compilation aborted!


---- [ui] src/test/ui/traits/trait-upcasting/diamond.rs stdout ----


error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/diamond.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/diamond/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/diamond/auxiliary"
stdout: none
--- stderr -------------------------------
Use of instruction is not an instruction!
  %379 = load i8*, i8** getelementptr inbounds (i8*, i8** getelementptr inbounds (<{ i8*, [16 x i8], i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8* }>, <{ i8*, [16 x i8], i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8* }>* @10, i32 0, i32 0), i64 16), align 8, !invariant.load !4, !nonnull !4
LLVM ERROR: Broken module found, compilation aborted!



failures:
