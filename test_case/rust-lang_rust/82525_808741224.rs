plain
.................................................................................................... 9400/11715
.................................................................................................... 9500/11715
.........................................................i......i................................... 9600/11715
.................................................................................................... 9700/11715
...iiiiiii...iiiiiii................................................................................ 9800/11715
.................................................................................................... 10000/11715
.................................................................................................... 10100/11715
.................................................................................................... 10200/11715
.................................................................................................... 10300/11715
---

7    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information
9 
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ warning: reference to packed field is unaligned
12    |
12    |
13 LL |         println!("{}", foo.x);
14    |                        ^^^^^
15    |
15    |
-    = note: `#[warn(safe_packed_borrows)]` on by default
+    = note: `#[warn(unaligned_references)]` on by default
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
21 warning: 2 warnings emitted
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed/repr_packed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/repr_packed.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(capture_disjoint_fields)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information


warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed.rs:25:24
   |
LL |         println!("{}", foo.x);
   |
   = note: `#[warn(unaligned_references)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/unaligned_references_external_macro.rs stdout ----
diff of stderr:

+ warning: lint `safe_packed_borrows` has been renamed to `unaligned_references`
+   --> $DIR/unaligned_references_external_macro.rs:3:10
+    |
+ LL | #![allow(safe_packed_borrows)]
+    |          ^^^^^^^^^^^^^^^^^^^ help: use the new name: `unaligned_references`
+    = note: `#[warn(renamed_and_removed_lints)]` on by default
+ 
1 error: reference to packed field is unaligned
2   --> $DIR/unaligned_references_external_macro.rs:7:1
2   --> $DIR/unaligned_references_external_macro.rs:7:1
3    |

19 LL | |     }
20 LL | | }
21    | |_^
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
22    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
24 

- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error; 1 warning emitted
26 
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unaligned_references_external_macro/unaligned_references_external_macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unaligned_references_external_macro.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unaligned_references_external_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unaligned_references_external_macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unaligned_references_external_macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `safe_packed_borrows` has been renamed to `unaligned_references`
   |
   |
LL | #![allow(safe_packed_borrows)]
   |          ^^^^^^^^^^^^^^^^^^^ help: use the new name: `unaligned_references`
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references_external_macro.rs:7:1
  --> /checkout/src/test/ui/lint/unaligned_references_external_macro.rs:7:1
   |
LL | / unaligned_references_external_crate::mac! { //~ERROR reference to packed field is unaligned
LL | |     #[repr(packed)]
LL | |     pub struct X {
LL | |         pub field: u16
LL | |     }
LL | | }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unaligned_references_external_macro.rs:7:1
   |
   |
LL | / unaligned_references_external_crate::mac! { //~ERROR reference to packed field is unaligned
LL | |     #[repr(packed)]
LL | |     pub struct X {
LL | |         pub field: u16
LL | |     }
LL | | }
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)

error: aborting due to previous error; 1 warning emitted


---
test result: FAILED. 11617 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 137.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:18
