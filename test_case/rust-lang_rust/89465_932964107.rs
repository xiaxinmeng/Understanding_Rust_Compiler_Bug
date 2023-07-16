plain
.................................................................................................... 7000/12240
.......ii.......................................................i................................... 7100/12240
.................................................................................................... 7200/12240
.................................................................................................... 7300/12240
.......................................F...........F..F..F.......................................... 7400/12240
..................F........................................ii................i....i..ii............. 7500/12240
.................................................................................................... 7700/12240
.................................................................................................... 7800/12240
.................................................................................................... 7900/12240
............................i..ii..............................................................ii... 8000/12240
---
failures:

---- [ui] ui/mir/issue-66851.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-66851.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-66851/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-66851/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:7 ~ issue_66851[9312]::SpecialsRes::Res::{constructor#0})
   |
   |
LL | enum SpecialsRes { Res(u64) }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=3 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `SpecialsRes::Res`
#1 [mir_built] building MIR for `SpecialsRes::Res`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/mir/issue-76740-copy-propagation.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-76740-copy-propagation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-76740-copy-propagation/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-76740-copy-propagation/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:9 ~ issue_76740_copy_propagation[aca3]::V::{constructor#0})
   |
   |
LL | pub struct V([usize; 4]);

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=4 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `V`
#1 [mir_built] building MIR for `V`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/mir/issue-78496.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-78496.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-78496/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=3" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-78496/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:8 ~ issue_78496[169a]::E::Some::{constructor#0})
   |
   |
LL |     Some(&'a E<'a>),

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=3 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `E::Some`
#1 [mir_built] building MIR for `E::Some`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/mir/mir-inlining/array-clone-with-generic-size.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir-inlining/array-clone-with-generic-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir-inlining/array-clone-with-generic-size" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "-Zvalidate-mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir-inlining/array-clone-with-generic-size/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:5 ~ array_clone_with_generic_size[4fd6]::Array::{constructor#0})
   |
   |
LL | struct Array<T, const N: usize>([T; N]);

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=3 -Z validate-mir -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `Array`
#1 [mir_built] building MIR for `Array`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/mir/ssa-analysis-regression-50041.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:5 ~ ssa_analysis_regression_50041[758d]::Box::{constructor#0})
   |
   |
LL | pub struct Box<T: ?Sized>(*mut T);

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=4 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `Box`
#1 [mir_built] building MIR for `Box`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:13 ~ simd_intrinsic_inlining_issue67557_ice[28c1]::Simd2::{constructor#0})
   |
   |
LL | struct Simd2(u8, u8);

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=4 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `Simd2`
#1 [mir_built] building MIR for `Simd2`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/simd-intrinsic/simd-intrinsic-inlining-issue67557.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/mod.rs:361:9: can't type-check body of DefId(0:17 ~ simd_intrinsic_inlining_issue67557[c4b5]::Simd2::{constructor#0})
   |
   |
LL | struct Simd2(u8, u8);

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1092:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (7c0418575 2021-10-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=4 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `Simd2`
#1 [mir_built] building MIR for `Simd2`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12118 passed; 7 failed; 115 ignored; 0 measured; 0 filtered out; finished in 126.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:06
