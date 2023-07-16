plain
failures:

---- [ui] ui/parser/import-from-path.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-from-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-path/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found `::`
   |
   |
LL | use foo::{bar}::baz
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |               ^^ expected `;`
   |
   = note: glob-like brace syntax must be last on the path

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_interface/src/queries.rs:125:56

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8dad0168b 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/import-from-rename.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-from-rename.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-rename" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-rename/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found keyword `as`
   |
   |
LL | use foo::{bar} as baz;
   |                ^^ expected `;`
   |
   = note: glob-like brace syntax must be last on the path

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_interface/src/queries.rs:125:56

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8dad0168b 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/import-glob-path.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-glob-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-path/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found `::`
   |
   |
LL | use foo::*::bar
   |           ^^ expected `;`
   = note: the wildcard token must be last on the path


thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_interface/src/queries.rs:125:56

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8dad0168b 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/import-glob-rename.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-glob-rename.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-rename" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-rename/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found keyword `as`
   |
   |
LL | use foo::* as baz;
   |            ^^ expected `;`
   = note: the wildcard token must be last on the path


thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_interface/src/queries.rs:125:56

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8dad0168b 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/span/issue-24356.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24356.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_typeck/src/check/compare_method.rs:383:33

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8dad0168b 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1847:35


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1190:13
   0:     0x7f5d4a6dceb0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2316588a2353c161
   1:     0x7f5d4a74b0df - core::fmt::write::h2ee44590e506e0ff
   1:     0x7f5d4a74b0df - core::fmt::write::h2ee44590e506e0ff
   2:     0x7f5d4a6c99f5 - std::io::Write::write_fmt::h343b15be0908db7e
   3:     0x7f5d4a6e15e7 - std::panicking::default_hook::{{closure}}::h1b6658456a1000eb
   4:     0x7f5d4a6e1010 - std::panicking::default_hook::hd38baea213260207
   5:     0x7f5d4b18a461 - rustc_driver[144c1019e81ac299]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5d4a6e1d83 - std::panicking::rust_panic_with_hook::hf8cbe15f5b093f29
   7:     0x7f5d4a6e1bd7 - std::panicking::begin_panic_handler::{{closure}}::hd5717416a3d23ba9
   8:     0x7f5d4a6dd3a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h9c06c558915ba456
   9:     0x7f5d4a6e18c9 - rust_begin_unwind
  10:     0x7f5d4a6948a3 - core::panicking::panic_fmt::h225defce7386fd13
  11:     0x7f5d4db16f28 - core[f31692673b9214cd]::panicking::panic_display::<&str>
  12:     0x7f5d4db13bff - <rustc_errors[a9bd18aa40ff294e]::HandlerInner>::flush_delayed
  13:     0x7f5d4db10576 - <rustc_errors[a9bd18aa40ff294e]::HandlerInner as core[f31692673b9214cd]::ops::drop::Drop>::drop
  14:     0x7f5d4b170c92 - core[f31692673b9214cd]::ptr::drop_in_place::<rustc_session[1ad93584016f2635]::parse::ParseSess>
  15:     0x7f5d4b17661a - <alloc[3e88aa93ea267f66]::rc::Rc<rustc_session[1ad93584016f2635]::session::Session> as core[f31692673b9214cd]::ops::drop::Drop>::drop
  16:     0x7f5d4b1657cc - core[f31692673b9214cd]::ptr::drop_in_place::<rustc_interface[711f824d4953b2b3]::interface::Compiler>
  17:     0x7f5d4b16a7ae - rustc_span[e69846e9234a17ed]::with_source_map::<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, rustc_interface[711f824d4953b2b3]::interface::create_compiler_and_run<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, rustc_driver[144c1019e81ac299]::run_compiler::{closure#1}>::{closure#1}>
  18:     0x7f5d4b1ec91d - rustc_interface[711f824d4953b2b3]::interface::create_compiler_and_run::<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, rustc_driver[144c1019e81ac299]::run_compiler::{closure#1}>
  19:     0x7f5d4b19afd4 - std[d22bc16dd5737445]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[711f824d4953b2b3]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[711f824d4953b2b3]::interface::run_compiler<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, rustc_driver[144c1019e81ac299]::run_compiler::{closure#1}>::{closure#0}, core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>>::{closure#0}, core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>>
  20:     0x7f5d4b1f10a1 - std[d22bc16dd5737445]::panicking::try::<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, core[f31692673b9214cd]::panic::unwind_safe::AssertUnwindSafe<<std[d22bc16dd5737445]::thread::Builder>::spawn_unchecked_<rustc_interface[711f824d4953b2b3]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[711f824d4953b2b3]::interface::run_compiler<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, rustc_driver[144c1019e81ac299]::run_compiler::{closure#1}>::{closure#0}, core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>>::{closure#0}, core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>>::{closure#1}::{closure#0}>>
  21:     0x7f5d4b1923ea - <<std[d22bc16dd5737445]::thread::Builder>::spawn_unchecked_<rustc_interface[711f824d4953b2b3]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[711f824d4953b2b3]::interface::run_compiler<core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>, rustc_driver[144c1019e81ac299]::run_compiler::{closure#1}>::{closure#0}, core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>>::{closure#0}, core[f31692673b9214cd]::result::Result<(), rustc_errors[a9bd18aa40ff294e]::ErrorReported>>::{closure#1} as core[f31692673b9214cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f5d4a6efc73 - std::sys::unix::thread::Thread::new::thread_start::h8675f7c31d0f25bb
  23:     0x7f5d44a5f609 - start_thread
  24:     0x7f5d4a556293 - clone
  25:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8dad0168b 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 12427 passed; 5 failed; 121 ignored; 0 measured; 0 filtered out; finished in 98.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:31
