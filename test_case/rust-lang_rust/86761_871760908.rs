plain
---- [ui] ui/issues/issue-31109.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31109.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31109" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31109/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/pattern/issue-68396-let-float-bug.rs stdout ----
diff of stderr:

- error[E0080]: could not evaluate float literal (see issue #31407)
+ warning: floating-point types cannot be used in patterns
3    |
3    |
4 LL |     let 1234567890123456789012345678901234567890e-340: f64 = 0.0;
5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+    = note: `#[warn(illegal_floating_point_literal_pattern)]` on by default
+    = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
6 
6 
- error[E0080]: could not evaluate float literal (see issue #31407)
+ error[E0005]: refutable pattern in local binding: `_` not covered
+    |
+    |
+ LL |     let 1234567890123456789012345678901234567890e-340: f64 = 0.0;
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `_` not covered
+    |
+    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
+    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
+    = note: the matched value is of type `f64`
+ help: you might want to use `if let` to ignore the variant that isn't matched
+    |
+ LL |     if let 1234567890123456789012345678901234567890e-340: f64 = 0.0 { /* */ }
+ 
+ warning: floating-point types cannot be used in patterns
8   --> $DIR/issue-68396-let-float-bug.rs:5:14
9    |
9    |
10 LL |     fn param(1234567890123456789012345678901234567890e-340: f64) {}
11    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
+    = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
12 
- error: aborting due to 2 previous errors
+ error[E0005]: refutable pattern in function argument: `_` not covered
+    |
+    |
+ LL |     fn param(1234567890123456789012345678901234567890e-340: f64) {}
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `_` not covered
+    = note: the matched value is of type `f64`
14 
- For more information about this error, try `rustc --explain E0080`.
+ warning: floating-point types cannot be used in patterns
+ warning: floating-point types cannot be used in patterns
+   --> $DIR/issue-68396-let-float-bug.rs:2:9
+    |
+ LL |     let 1234567890123456789012345678901234567890e-340: f64 = 0.0;
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
+ 
+ 
+ warning: floating-point types cannot be used in patterns
+   --> $DIR/issue-68396-let-float-bug.rs:5:14
+    |
+ LL |     fn param(1234567890123456789012345678901234567890e-340: f64) {}
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
+ 
---
To only update this specific test, also pass `--test-args pattern/issue-68396-let-float-bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/issue-68396-let-float-bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-68396-let-float-bug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-68396-let-float-bug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: floating-point types cannot be used in patterns
  --> /checkout/src/test/ui/pattern/issue-68396-let-float-bug.rs:2:9
   |
LL |     let 1234567890123456789012345678901234567890e-340: f64 = 0.0;
   |
   |
   = note: `#[warn(illegal_floating_point_literal_pattern)]` on by default
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>


error[E0005]: refutable pattern in local binding: `_` not covered
   |
   |
LL |     let 1234567890123456789012345678901234567890e-340: f64 = 0.0;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `_` not covered
   |
   = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
   = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
   = note: the matched value is of type `f64`
help: you might want to use `if let` to ignore the variant that isn't matched
   |
LL |     if let 1234567890123456789012345678901234567890e-340: f64 = 0.0 { /* */ }

warning: floating-point types cannot be used in patterns
  --> /checkout/src/test/ui/pattern/issue-68396-let-float-bug.rs:5:14
   |
   |
LL |     fn param(1234567890123456789012345678901234567890e-340: f64) {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>


error[E0005]: refutable pattern in function argument: `_` not covered
   |
   |
LL |     fn param(1234567890123456789012345678901234567890e-340: f64) {}
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `_` not covered
   = note: the matched value is of type `f64`

warning: floating-point types cannot be used in patterns
  --> /checkout/src/test/ui/pattern/issue-68396-let-float-bug.rs:2:9
  --> /checkout/src/test/ui/pattern/issue-68396-let-float-bug.rs:2:9
   |
LL |     let 1234567890123456789012345678901234567890e-340: f64 = 0.0;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>


warning: floating-point types cannot be used in patterns
  --> /checkout/src/test/ui/pattern/issue-68396-let-float-bug.rs:5:14
   |
LL |     fn param(1234567890123456789012345678901234567890e-340: f64) {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>

---
test result: FAILED. 11928 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 103.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:29
