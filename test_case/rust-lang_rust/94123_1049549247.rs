plain
---- [ui] ui/abi/variadic-ffi.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/variadic-ffi.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/abi/variadic-ffi/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/abi/variadic-ffi/auxiliary"
stdout: none
--- stderr -------------------------------
PHI node entries do not match predecessors!
  %23 = phi double [ %15, %22 ], [ %21, %22 ]
label %22
label %10
Instruction does not dominate all uses!
  %15 = load double, double* %14, align 8
  %23 = phi double [ %15, %22 ], [ %21, %22 ]
Instruction does not dominate all uses!
  %21 = load double, double* %20, align 8
  %23 = phi double [ %15, %22 ], [ %21, %22 ]
PHI node entries do not match predecessors!
  %23 = phi i64 [ %15, %22 ], [ %21, %22 ]
label %22
label %10
Instruction does not dominate all uses!
  %15 = load i64, i64* %14, align 8
  %23 = phi i64 [ %15, %22 ], [ %21, %22 ]
Instruction does not dominate all uses!
  %21 = load i64, i64* %20, align 8
  %23 = phi i64 [ %15, %22 ], [ %21, %22 ]
LLVM ERROR: Broken module found, compilation aborted!



failures:
