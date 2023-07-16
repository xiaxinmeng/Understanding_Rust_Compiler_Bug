plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 68 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....F.F............................................................

---- [ui] ui-fulldeps/internal-lints/pass_ty_by_ref_self.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref_self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref_self" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref_self/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `rustc::ty_pass_by_reference`
  --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref_self.rs:8:9
   |
LL | #![deny(rustc::ty_pass_by_reference)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: 1 warning emitted
---
---- [ui] ui-fulldeps/internal-lints/pass_ty_by_ref.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/pass_ty_by_ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `rustc::ty_pass_by_reference`
  --> /checkout/src/test/ui-fulldeps/internal-lints/pass_ty_by_ref.rs:4:9
   |
LL | #![deny(rustc::ty_pass_by_reference)]
   |
   = note: `#[warn(unknown_lints)]` on by default


warning: passing `Ty<'_>` by reference
   |
   |
LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |             ^^^^^^^ help: try passing by value: `Ty<'_>`
   = note: `#[warn(pass_by_value)]` on by default


warning: passing `Ty<'_>` by reference
   |
   |
LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
   |                            ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |                 ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
   |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |                 ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
   |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |             ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
   |                            ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |                 ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
   |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |                 ^^^^^^^ help: try passing by value: `Ty<'_>`

warning: passing `Ty<'_>` by reference
   |
   |
LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
   |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
warning: 13 warnings emitted


------------------------------------------
---
test result: FAILED. 66 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:59
