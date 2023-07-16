plain
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 42 tests
Some tests failed in compiletest suite=codegen-units mode=codegen-units host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiF............F.....i.F..................

---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----

error: compilation failed!
error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/item-collection/generic-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/item-collection/generic-drop-glue" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/item-collection/generic-drop-glue/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: field is never read: `0`
  --> /checkout/src/test/codegen-units/item-collection/generic-drop-glue.rs:37:25
   |
37 | struct NonGenericNoDrop(i32);
   |
note: the lint level is defined here
  --> /checkout/src/test/codegen-units/item-collection/generic-drop-glue.rs:5:9
   |
   |
5  | #![deny(dead_code)]
   |         ^^^^^^^^^

error: field is never read: `0`
  --> /checkout/src/test/codegen-units/item-collection/generic-drop-glue.rs:39:27
   |
39 | struct NonGenericWithDrop(i32);

error: aborting due to 2 previous errors



------------------------------------------


---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/item-collection/transitive-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/item-collection/transitive-drop-glue" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/item-collection/transitive-drop-glue/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: field is never read: `0`
 --> /checkout/src/test/codegen-units/item-collection/transitive-drop-glue.rs:9:13
  |
9 | struct Root(Intermediate);
  |
note: the lint level is defined here
 --> /checkout/src/test/codegen-units/item-collection/transitive-drop-glue.rs:5:9
  |
---

error: field is never read: `0`
  --> /checkout/src/test/codegen-units/item-collection/transitive-drop-glue.rs:20:19
   |
20 | struct RootGen<T>(IntermediateGen<T>);

error: field is never read: `0`
  --> /checkout/src/test/codegen-units/item-collection/transitive-drop-glue.rs:21:27
   |
   |
21 | struct IntermediateGen<T>(LeafGen<T>);

error: field is never read: `0`
  --> /checkout/src/test/codegen-units/item-collection/transitive-drop-glue.rs:22:19
   |
   |
22 | struct LeafGen<T>(T);

error: aborting due to 5 previous errors



------------------------------------------


---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/item-collection/unsizing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/item-collection/unsizing" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/item-collection/unsizing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/item-collection/unsizing.rs:53:40
   |
53 |     let _bool_unsized = bool_sized as &Trait;
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
53 -     let _bool_unsized = bool_sized as &Trait;
53 +     let _bool_unsized = bool_sized as &dyn Trait;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/item-collection/unsizing.rs:59:40
   |
   |
59 |     let _char_unsized = char_sized as &Trait;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
59 -     let _char_unsized = char_sized as &Trait;
59 +     let _char_unsized = char_sized as &dyn Trait;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/item-collection/unsizing.rs:69:51
   |
   |
69 |     let _struct_unsized = struct_sized as &Struct<Trait>;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
69 -     let _struct_unsized = struct_sized as &Struct<Trait>;
69 +     let _struct_unsized = struct_sized as &Struct<dyn Trait>;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/codegen-units/item-collection/unsizing.rs:75:51
   |
   |
75 |     let _wrapper_sized = wrapper_sized as Wrapper<Trait>;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
75 -     let _wrapper_sized = wrapper_sized as Wrapper<Trait>;
75 +     let _wrapper_sized = wrapper_sized as Wrapper<dyn Trait>;

error: field is never read: `0`
  --> /checkout/src/test/codegen-units/item-collection/unsizing.rs:42:27
   |
   |
42 | struct Wrapper<T: ?Sized>(*const T);
   |
note: the lint level is defined here
  --> /checkout/src/test/codegen-units/item-collection/unsizing.rs:4:9
   |
---
test result: FAILED. 36 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen-units" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:09
