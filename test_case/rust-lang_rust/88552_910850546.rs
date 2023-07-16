plain
 finished in 0.168 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 146 tests
....................................................................F....F..F................F...... 100/146
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] incremental/incremental_proc_macro.rs stdout ----


error in revision `cfail1`: auxiliary build of "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/incremental_proc_macro.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused variable: `input`
  --> /checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs:15:15
   |
LL | pub fn derive(input: TokenStream) -> TokenStream {
   |               ^^^^^ help: if this is intentional, prefix it with an underscore: `_input`
   = note: `#[warn(unused_variables)]` on by default


error: internal compiler error: compiler/rustc_middle/src/ich/impls_ty.rs:94:17: StableHasher: unexpected region '_#0r
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (57e4bf318 2021-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C incremental -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `dyn for<'r, 's> std::ops::Fn(&'r std::panic::PanicInfo<'s>) + std::marker::Send + std::marker::Sync: std::ops::Fn<(&std::panic::PanicInfo,)>`
#1 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [incremental] incremental/issue-49482.rs stdout ----

error in revision `rpass1`: auxiliary build of "/checkout/src/test/incremental/auxiliary/issue-49482-macro-def.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/issue-49482-macro-def.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49482/issue-49482.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49482/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49482/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ich/impls_ty.rs:94:17: StableHasher: unexpected region '_#0r
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (57e4bf318 2021-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C incremental -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `dyn for<'r, 's> std::ops::Fn(&'r std::panic::PanicInfo<'s>) + std::marker::Send + std::marker::Sync: std::ops::Fn<(&std::panic::PanicInfo,)>`
#1 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [incremental] incremental/issue-54059.rs stdout ----

error in revision `rpass1`: auxiliary build of "/checkout/src/test/incremental/auxiliary/issue-54059.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/issue-54059.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-54059/issue-54059.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-54059/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-54059/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused variable: `input`
  --> /checkout/src/test/incremental/auxiliary/issue-54059.rs:33:26
   |
LL |               pub fn $func(input: proc_macro_tokenstream!()) -> proc_macro_tokenstream!() {
   |                            ^^^^^ help: if this is intentional, prefix it with an underscore: `_input`
...
LL | / proc_macro_expr_impl! {
LL | |     pub fn base2_impl(input: &str) -> String {
LL | |         panic!()
LL | |     }
LL | | }
   |
   = note: `#[warn(unused_variables)]` on by default
   = note: this warning originates in the macro `proc_macro_expr_impl` (in Nightly builds, run with -Z macro-backtrace for more info)


error: internal compiler error: compiler/rustc_middle/src/ich/impls_ty.rs:94:17: StableHasher: unexpected region '_#0r
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (57e4bf318 2021-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C incremental -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `dyn for<'r, 's> std::ops::Fn(&'r std::panic::PanicInfo<'s>) + std::marker::Send + std::marker::Sync: std::ops::Fn<(&std::panic::PanicInfo,)>`
#1 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [incremental] incremental/issue-85197-invalid-span/invalid_span_main.rs stdout ----

error in revision `rpass1`: auxiliary build of "/checkout/src/test/incremental/issue-85197-invalid-span/auxiliary/respan.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-85197-invalid-span/auxiliary/respan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-85197-invalid-span/invalid_span_main/invalid_span_main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-85197-invalid-span/invalid_span_main/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-85197-invalid-span/invalid_span_main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ich/impls_ty.rs:94:17: StableHasher: unexpected region '_#0r
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (57e4bf318 2021-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C incremental -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `dyn for<'r, 's> std::ops::Fn(&'r std::panic::PanicInfo<'s>) + std::marker::Send + std::marker::Sync: std::ops::Fn<(&std::panic::PanicInfo,)>`
#1 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 142 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:51
