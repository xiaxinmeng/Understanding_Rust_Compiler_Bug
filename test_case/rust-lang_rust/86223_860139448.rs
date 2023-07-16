plain
diff of stderr:

2   --> $DIR/unnamable-types.rs:6:7
3    |
4 LL | const A = 5;
-    |       ^ help: provide a type for the item: `A: i32`
+    |       ^ help: provide a type for the constant: `A: i32`
6 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
8   --> $DIR/unnamable-types.rs:10:11
9    |
10 LL | static B: _ = "abc";
13    |           not allowed in type signatures
14    |           help: replace with the correct type: `&str`
15 
15 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
17   --> $DIR/unnamable-types.rs:17:10
18    |
19 LL | const C: _ = || 42;
41   --> $DIR/unnamable-types.rs:29:7
42    |
43 LL | const E = foo;
43 LL | const E = foo;
-    |       ^ help: provide a type for the item: `E: fn() -> i32`
+    |       ^ help: provide a type for the constant: `E: fn() -> i32`
46 error: missing type for `const` item
47   --> $DIR/unnamable-types.rs:32:7

48    |
48    |
49 LL | const F = S { t: foo };
-    |       ^ help: provide a type for the item: `F: S<fn() -> i32>`
+    |       ^ help: provide a type for the constant: `F: S<fn() -> i32>`
52 error: missing type for `const` item
53   --> $DIR/unnamable-types.rs:37:7


---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/unnamable-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unnamable-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unnamable-types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:6:7
   |
LL | const A = 5;
   |       ^ help: provide a type for the constant: `A: i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL | static B: _ = "abc";
   |           |
   |           not allowed in type signatures
   |           help: replace with the correct type: `&str`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL | const C: _ = || 42;
   |          ^ not allowed in type signatures
   |
note: however, the inferred type `[closure@/checkout/src/test/ui/suggestions/unnamable-types.rs:17:14: 17:19]` cannot be named
   |
   |
LL | const C: _ = || 42;

error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:23:7
   |
   |
LL | const D = S { t: { let i = 0; move || -> i32 { i } } };
   |
   |
note: however, the inferred type `S<[closure@/checkout/src/test/ui/suggestions/unnamable-types.rs:23:31: 23:51]>` cannot be named
   |
   |
LL | const D = S { t: { let i = 0; move || -> i32 { i } } };

error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:29:7
   |
   |
LL | const E = foo;
   |       ^ help: provide a type for the constant: `E: fn() -> i32`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:32:7
   |
   |
LL | const F = S { t: foo };
   |       ^ help: provide a type for the constant: `F: S<fn() -> i32>`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:37:7
   |
   |
LL | const G = || -> i32 { yield 0; return 1; };
   |
   |
note: however, the inferred type `[generator@/checkout/src/test/ui/suggestions/unnamable-types.rs:37:11: 37:43]` cannot be named
   |
   |
LL | const G = || -> i32 { yield 0; return 1; };

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0121`.
---
test result: FAILED. 11885 passed; 1 failed; 101 ignored; 0 measured; 0 filtered out; finished in 115.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:02
