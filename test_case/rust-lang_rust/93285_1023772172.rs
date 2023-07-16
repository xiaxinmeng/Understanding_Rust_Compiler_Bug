plain
...................................................iii.............................................. 12500/12556
........................................................
failures:

---- [ui] ui/associated-consts/assoc-const-ty-mismatch.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: Could not find associated const on trait
   |
   = note: delayed at compiler/rustc_typeck/src/collect/type_of.rs:551:27

error: internal compiler error: Const::from_anon_const: couldn't lit_to_const TypeError
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_middle/src/ty/consts.rs:104:30

error: internal compiler error: TyKind::Error constructed but no error reported
---
error: internal compiler error: ty::ConstKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:245:43

error: internal compiler error: no MIR body is available for DefId(0:12 ~ assoc_const_ty_mismatch[6aa3]::{impl#1}::T)
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:19:3
LL |   type T = usize;
   |   ^^^^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/const_eval/eval_queries.rs:293:22
error: internal compiler error: ty::ConstKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:245:43


error: internal compiler error: cannot relate constants of different types: usize != [type error]
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:559:29

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:721:18

error: internal compiler error: mir_const_qualif: MIR had errors
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_mir_transform/src/lib.rs:186:18


error: internal compiler error: PromoteTemps: MIR had errors
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22


error: internal compiler error: broken MIR in DefId(0:17 ~ assoc_const_ty_mismatch[6aa3]::foo2::{constant#0}) ("return type"): bad type [type error]
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:541:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:796:20

error: internal compiler error: broken MIR in DefId(0:17 ~ assoc_const_ty_mismatch[6aa3]::foo2::{constant#0}) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20: 24:26 (#0), scope: scope[0] } }): bad type [type error]
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:541:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:796:20

error: internal compiler error: ty::ConstKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:245:43

error: internal compiler error: Const::from_anon_const: couldn't lit_to_const TypeError
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_middle/src/ty/consts.rs:104:30


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1173:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (fbc133986 2022-01-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/feature-gates/feature-gate-associated_const_equality.rs stdout ----
diff of stderr:

7    = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
8    = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
- error: associated const equality is incomplete
-   --> $DIR/feature-gate-associated_const_equality.rs:10:28
-    |
-    |
- LL | fn foo<A: TraitWAssocConst<A=32>>() {}
-    |                            ^^^^ cannot yet relate associated const
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
17 
18 For more information about this error, try `rustc --explain E0658`.
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-associated_const_equality.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_const_equality.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_const_equality" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_const_equality/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: associated const equality is incomplete
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_const_equality.rs:10:28
   |
LL | fn foo<A: TraitWAssocConst<A=32>>() {}
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 12433 passed; 2 failed; 121 ignored; 0 measured; 0 filtered out; finished in 94.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:03
