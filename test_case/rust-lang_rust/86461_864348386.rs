plain
.................................................................................................... 5900/11994
...........................................i........................................................ 6000/11994
.................................................................................................... 6100/11994
.................................................i.................................................. 6200/11994
......................i......................F......................F............................... 6300/11994
i................................................................................................... 6500/11994
.............................i....i..................................i...........................i.. 6600/11994
.................................................................................................... 6700/11994
......................................i............................................................. 6800/11994
---
failures:

---- [ui] ui/issues/issue-24010.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24010/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24010/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:538:17: type parameter `Args/#1` (Args/1) out of range when substituting, substs=[dyn Foo<Output = i32>]
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:953:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (fd9fdfeb2 2021-06-19) running on x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `std::boxed::Box<dyn Foo<Output = i32>>: std::ops::Fn<(^1_1,)>`
#1 [typeck] type-checking `wants_foo`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-70746.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-70746.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70746" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70746/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_trait_selection/src/traits/mod.rs:702:5: Failed to find info for expected trait in vtable
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1007:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (fd9fdfeb2 2021-06-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `std::boxed::Box<dyn Callback<T, Output = ()>>: std::ops::FnMut<(^1_1,)>`
#1 [typeck] type-checking `<impl at /checkout/src/test/ui/issues/issue-70746.rs:17:1: 24:2>::test_cb`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-73229.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-73229.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-73229" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-73229/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_trait_selection/src/traits/mod.rs:702:5: Failed to find info for expected trait in vtable
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1007:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (fd9fdfeb2 2021-06-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `std::boxed::Box<dyn Callback<T, for<'r, 's> Output = ()>>: std::ops::Fn<(^1_1, ^1_2)>`
#1 [typeck] type-checking `<impl at /checkout/src/test/ui/issues/issue-73229.rs:18:1: 22:2>::event`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/unsized/issue-71659.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/issue-71659.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/issue-71659" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/issue-71659/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:538:17: type parameter `T/#1` (T/1) out of range when substituting, substs=[dyn Foo]
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:953:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (fd9fdfeb2 2021-06-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11889 passed; 4 failed; 101 ignored; 0 measured; 0 filtered out; finished in 105.14s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:35
