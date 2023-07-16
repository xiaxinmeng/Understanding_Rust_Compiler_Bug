plain
.................................................................................................... 9200/11456
.................................................................................................... 9300/11456
.................................................................................................... 9400/11456
...............i......i............................................................................. 9500/11456
.....................................................iiiiiii..iiiiii.i.............................. 9600/11456
.................................................................................................... 9800/11456
.................................................................................................... 9900/11456
.................................................................................................... 10000/11456
.................................................................................................... 10100/11456
---

---- [ui] ui/proc-macro/ambiguous-builtin-attrs.rs stdout ----
diff of stderr:

94 LL | fn non_macro_expanded_location<#[repr(C)] T>() {
95    |                                       ^   - not a struct, enum, function, or union
96 
- error[E0517]: attribute should be applied to a struct, enum, or union
+ error[E0517]: attribute should be applied to a struct, enum, function, or union
98   --> $DIR/ambiguous-builtin-attrs.rs:24:16
99    |
100 LL |         #[repr(C)]
101    |                ^
102 ...
103 LL |         _ => {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |         ------- not a struct, enum, or union
+    |         ------- not a struct, enum, function, or union
106 error: aborting due to 9 previous errors
107 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/ambiguous-builtin-attrs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `NonExistent` in this scope
   |
   |
LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope


error[E0659]: `repr` is ambiguous (built-in attribute vs any other name)
   |
   |
LL | #[repr(C)] //~ ERROR `repr` is ambiguous
   |   ^^^^ ambiguous name
   |
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `repr` is ambiguous (built-in attribute vs any other name)
   |
   |
LL | #[cfg_attr(all(), repr(C))] //~ ERROR `repr` is ambiguous
   |                   ^^^^ ambiguous name
   |
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `repr` is ambiguous (built-in attribute vs any other name)
   |
   |
LL | fn non_macro_expanded_location<#[repr(C)] T>() {
   |                                  ^^^^ ambiguous name
   |
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `repr` is ambiguous (built-in attribute vs any other name)
   |
   |
LL |         #[repr(C)]
   |           ^^^^ ambiguous name
   |
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `allow` is ambiguous (built-in attribute vs any other name)
   |
   |
LL | #[allow(unused)] //~ ERROR `allow` is ambiguous (built-in attribute vs any other name)
   |   ^^^^^ ambiguous name
   |
   = note: `allow` could refer to a built-in attribute
note: `allow` could also refer to the built-in attribute imported here
   |
   |
LL | use deny as allow;
   |     ^^^^^^^^^^^^^
   = help: use `crate::allow` to refer to this built-in attribute unambiguously

error[E0659]: `feature` is ambiguous (built-in attribute vs any other name)
   |
   |
LL | #![feature(decl_macro)] //~ ERROR `feature` is ambiguous
   |    ^^^^^^^ ambiguous name
   |
   = note: `feature` could refer to a built-in attribute
note: `feature` could also refer to the attribute macro imported here
   |
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::feature` to refer to this attribute macro unambiguously

error[E0517]: attribute should be applied to a struct, enum, function, or union
   |
   |
LL | fn non_macro_expanded_location<#[repr(C)] T>() {
   |                                       ^   - not a struct, enum, function, or union

error[E0517]: attribute should be applied to a struct, enum, function, or union
   |
   |
LL |         #[repr(C)]
...
LL |         _ => {}
LL |         _ => {}
   |         ------- not a struct, enum, function, or union
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0425, E0517, E0659.
For more information about an error, try `rustc --explain E0425`.
---
test result: FAILED. 11362 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 137.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:12
