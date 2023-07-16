plain
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'oops', /checkout/src/test/ui/builtin-clone-unwind.rs:18:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'oops', /checkout/src/test/ui/builtin-clone-unwind.rs:18:13
  left: `1`,
 right: `3`', /checkout/src/test/ui/builtin-clone-unwind.rs:57:5

------------------------------------------
------------------------------------------


---- [ui] ui/error-codes/E0206.rs stdout ----
diff of stderr:

- error[E0206]: the trait `Copy` may not be implemented for this type
-   --> $DIR/E0206.rs:3:15
+ error[E0119]: conflicting implementations of trait `std::marker::Copy` for type `[u8; 256]`
+   --> $DIR/E0206.rs:3:1
3    |
4 LL | impl Copy for Foo { }
-    |               ^^^ type is not a structure or enumeration
+    |
+    = note: conflicting implementation in crate `core`:
+    = note: conflicting implementation in crate `core`:
+            - impl<T, N> Copy for [T; N]
+              where T: Copy;
6 
7 error[E0206]: the trait `Copy` may not be implemented for this type
8   --> $DIR/E0206.rs:10:15
23 
24 error: aborting due to 3 previous errors
25 
- Some errors have detailed explanations: E0117, E0206.
---
To only update this specific test, also pass `--test-args error-codes/E0206.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0206.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0206" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0206/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0119]: conflicting implementations of trait `std::marker::Copy` for type `[u8; 256]`
   |
   |
LL | impl Copy for Foo { }
   |
   = note: conflicting implementation in crate `core`:
   = note: conflicting implementation in crate `core`:
           - impl<T, N> Copy for [T; N]
             where T: Copy;

error[E0206]: the trait `Copy` may not be implemented for this type
   |
   |
LL | impl Copy for &'static mut Bar { }
   |               ^^^^^^^^^^^^^^^^ type is not a structure or enumeration

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | impl Copy for Foo { }
   | |             |
   | |             this is not defined in the current crate because arrays are always foreign
   | |             this is not defined in the current crate because arrays are always foreign
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0117, E0119, E0206.
For more information about an error, try `rustc --explain E0117`.
---
test result: FAILED. 11857 passed; 2 failed; 98 ignored; 0 measured; 0 filtered out; finished in 121.19s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:35
