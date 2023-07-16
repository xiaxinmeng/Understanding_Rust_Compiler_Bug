plain
.................................................................................................... 9500/11856
.................................................................................................... 9600/11856
................................................................i......i............................ 9700/11856
.................................................................................................... 9800/11856
..........iiiiiii..iiiiii.i......................................................................... 9900/11856
.................................................................................................... 10100/11856
.................................................................................................... 10200/11856
.................................................................................................... 10300/11856
.................................................................................................... 10400/11856
---

---- [ui] ui/associated-type-bounds/inside-adt.rs stdout ----
diff of stderr:

1 error: associated type bounds are not allowed within structs, enums, or unions
-   --> $DIR/inside-adt.rs:4:29
-    |
- LL | struct S1 { f: dyn Iterator<Item: Copy> }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- 
- 
- error: associated type bounds are not allowed within structs, enums, or unions
-   --> $DIR/inside-adt.rs:6:33
-    |
- LL | struct S2 { f: Box<dyn Iterator<Item: Copy>> }
- 
- 
- error: associated type bounds are not allowed within structs, enums, or unions
-   --> $DIR/inside-adt.rs:8:29
-    |
- LL | struct S3 { f: dyn Iterator<Item: 'static> }
- 
- 
- error: associated type bounds are not allowed within structs, enums, or unions
21    |
21    |
22 LL | enum E1 { V(dyn Iterator<Item: Copy>) }

34 LL | enum E3 { V(dyn Iterator<Item: 'static>) }
36 
36 
- error: associated type bounds are not allowed within structs, enums, or unions
-   --> $DIR/inside-adt.rs:20:28
-    |
- LL | union U1 { f: dyn Iterator<Item: Copy> }
- 
- 
- error: associated type bounds are not allowed within structs, enums, or unions
-   --> $DIR/inside-adt.rs:23:32
-    |
- LL | union U2 { f: Box<dyn Iterator<Item: Copy>> }
- 
- 
- error: associated type bounds are not allowed within structs, enums, or unions
-   --> $DIR/inside-adt.rs:25:28
-    |
- LL | union U3 { f: dyn Iterator<Item: 'static> }
- 
- 
55 error[E0277]: the size for values of type `(dyn Iterator<Item = impl Copy> + 'static)` cannot be known at compilation time
57    |


124 LL | union U3 { f: Box<dyn Iterator<Item: 'static>> }
125    |               ^^^^                           ^
- error: aborting due to 13 previous errors
+ error: aborting due to 7 previous errors
128 
129 For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args associated-type-bounds/inside-adt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/inside-adt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/inside-adt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/inside-adt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: associated type bounds are not allowed within structs, enums, or unions
   |
   |
LL | enum E1 { V(dyn Iterator<Item: Copy>) }


error: associated type bounds are not allowed within structs, enums, or unions
   |
   |
LL | enum E2 { V(Box<dyn Iterator<Item: Copy>>) }


error: associated type bounds are not allowed within structs, enums, or unions
   |
   |
LL | enum E3 { V(dyn Iterator<Item: 'static>) }


error[E0277]: the size for values of type `(dyn Iterator<Item = impl Copy> + 'static)` cannot be known at compilation time
   |
   |
LL | enum E1 { V(dyn Iterator<Item: Copy>) }
   |             ^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `(dyn Iterator<Item = impl Copy> + 'static)`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL | enum E1 { V(&dyn Iterator<Item: Copy>) }
   |             ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL | enum E1 { V(Box<dyn Iterator<Item: Copy>>) }
   |             ^^^^                        ^

error[E0277]: the size for values of type `(dyn Iterator<Item = impl Sized> + 'static)` cannot be known at compilation time
   |
   |
LL | enum E3 { V(dyn Iterator<Item: 'static>) }
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `(dyn Iterator<Item = impl Sized> + 'static)`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL | enum E3 { V(&dyn Iterator<Item: 'static>) }
   |             ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL | enum E3 { V(Box<dyn Iterator<Item: 'static>>) }
   |             ^^^^                           ^

error[E0277]: the size for values of type `(dyn Iterator<Item = impl Copy> + 'static)` cannot be known at compilation time
   |
   |
LL | union U1 { f: dyn Iterator<Item: Copy> }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `(dyn Iterator<Item = impl Copy> + 'static)`
   = note: no field of a union may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL | union U1 { f: &dyn Iterator<Item: Copy> }
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL | union U1 { f: Box<dyn Iterator<Item: Copy>> }
   |               ^^^^                        ^

error[E0277]: the size for values of type `(dyn Iterator<Item = impl Sized> + 'static)` cannot be known at compilation time
   |
   |
LL | union U3 { f: dyn Iterator<Item: 'static> }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `(dyn Iterator<Item = impl Sized> + 'static)`
   = note: no field of a union may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL | union U3 { f: &dyn Iterator<Item: 'static> }
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL | union U3 { f: Box<dyn Iterator<Item: 'static>> }
   |               ^^^^                           ^
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11759 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 117.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:44
