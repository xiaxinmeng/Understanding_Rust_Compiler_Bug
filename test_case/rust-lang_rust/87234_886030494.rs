plain
.................................................................................................... 3500/12164
..............................i........i.........i.................................................. 3600/12164
.................................................................................................ii. 3700/12164
.................................................................................................... 3800/12164
...............i............................................i..F.................................... 3900/12164
.................................................................................................... 4100/12164
.................................................................................................... 4200/12164
.................................................................................................... 4300/12164
.................................................................................................... 4400/12164
---
failures:

---- [ui] ui/foreign/issue-74120-lowering-of-ffi-block-bodies.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/foreign/issue-74120-lowering-of-ffi-block-bodies.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign/issue-74120-lowering-of-ffi-block-bodies" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign/issue-74120-lowering-of-ffi-block-bodies/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incorrect function inside `extern` block
   |
LL |   extern "C" {
LL |   extern "C" {
   |   ---------- `extern` blocks define existing foreign functions and functions inside of them cannot have a body
LL |       fn f() {
   |  ________^___-
   | |        cannot have a body
   | |        cannot have a body
LL | |     //~^ incorrect function inside `extern` block
LL | |         fn g() {}
LL | |     }
   | |_____- help: remove the invalid body: `;`
   |
   = help: you might have meant to write a function accessible through FFI, which can be done by writing `extern fn` outside of the `extern` block
   = note: for more information, visit https://doc.rust-lang.org/std/keyword.extern.html

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/collector.rs:99:66

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (d9e09763d 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f8ddab13dc0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbe50df220f7e30ce
   1:     0x7f8ddab88a10 - core::fmt::write::h623720a298969928
   2:     0x7f8ddab04276 - std::io::Write::write_fmt::h67033cf38469b64b
   3:     0x7f8ddab182d7 - std::panicking::default_hook::{{closure}}::h897e7e7c24740079
   4:     0x7f8ddab17cd8 - std::panicking::default_hook::h1176d744c433ebb2
   5:     0x7f8ddb406f01 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hc69b6818052886ef
   6:     0x7f8ddab18c26 - std::panicking::rust_panic_with_hook::hc0297e6c3a51d5d4
   7:     0x7f8ddab18747 - std::panicking::begin_panic_handler::{{closure}}::hb41cd971e1b8ba97
   8:     0x7f8ddab1425c - std::sys_common::backtrace::__rust_end_short_backtrace::h509674096f72bc84
   9:     0x7f8ddab186a9 - rust_begin_unwind
  10:     0x7f8ddab84fc1 - core::panicking::panic_fmt::h3adb758c567968c3
  11:     0x7f8ddab853c3 - core::result::unwrap_failed::h8333a6f3e48034ac
  12:     0x7f8ddc7f852c - rustc_query_system::query::plumbing::get_query_impl::h5767ec746356e92f
  13:     0x7f8ddc97aa3d - rustc_query_system::query::plumbing::get_query::h4db96bedffa51ecb
  14:     0x7f8ddd9a755b - rustc_middle::hir::map::Map::find::h02a574c978cbe4c4
  15:     0x7f8ddd9ac448 - rustc_middle::hir::map::Map::opt_span::hedbe3ce9d01ec1f7
  16:     0x7f8ddd9ac762 - rustc_middle::hir::map::Map::span_if_local::h9e3df0593657baf9
  17:     0x7f8ddd7ea69e - core::ops::function::FnOnce::call_once::h2022ab7fb166bddd
  18:     0x7f8ddc7cf7d8 - rustc_query_system::query::plumbing::get_query_impl::h04b9917e348d0c7d
  19:     0x7f8ddc96f200 - rustc_query_system::query::plumbing::get_query::h06918ba76bdda7fd
  20:     0x7f8ddcaf3916 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h837b24427c907e95
  21:     0x7f8ddcaf3827 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::h3e8dbb187cb1b927
  22:     0x7f8ddcb3e3a2 - rustc_query_impl::make_query::hir_owner::he5bc799fa7f8cb4b
  23:     0x7f8ddc9393d6 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::hf088336d38e1ff06
  24:     0x7f8ddcb5cd6d - rustc_query_impl::Queries::try_collect_active_jobs::had56bc2f5c5130b6
  25:     0x7f8ddcab9941 - rustc_query_system::query::job::print_query_stack::h6af967dc15557d2c
  26:     0x7f8ddb52fa5f - rustc_interface::interface::try_print_query_stack::hc62bc90cb1afdb95
  27:     0x7f8ddb4077c9 - rustc_driver::report_ice::hffe48ee0af60f3f6
  28:     0x7f8ddab18c26 - std::panicking::rust_panic_with_hook::hc0297e6c3a51d5d4
  29:     0x7f8ddab18717 - std::panicking::begin_panic_handler::{{closure}}::hb41cd971e1b8ba97
  30:     0x7f8ddab1425c - std::sys_common::backtrace::__rust_end_short_backtrace::h509674096f72bc84
  31:     0x7f8ddab186a9 - rust_begin_unwind
  32:     0x7f8ddab84fc1 - core::panicking::panic_fmt::h3adb758c567968c3
  33:     0x7f8ddab84f0d - core::panicking::panic::h91bcdec0e6463feb
  34:     0x7f8ddd97a50a - rustc_middle::hir::map::collector::NodeCollector::finalize_and_compute_crate_hash::hdf85a162e27255cb
  35:     0x7f8ddd9ad965 - rustc_middle::hir::map::index_hir::h0c1345bd62f71d67
  36:     0x7f8ddc832afb - rustc_query_system::query::plumbing::get_query_impl::hbfbd066faa48a395
  37:     0x7f8ddc970b29 - rustc_query_system::query::plumbing::get_query::h0f911e9028504477
  38:     0x7f8ddd7eb878 - core::ops::function::FnOnce::call_once::hf136cb51d570f9bc
  39:     0x7f8ddca88d34 - rustc_data_structures::stack::ensure_sufficient_stack::hd2e0a01f12f2d64d
  40:     0x7f8ddc7f81b4 - rustc_query_system::query::plumbing::get_query_impl::h5767ec746356e92f
  41:     0x7f8ddc97aa3d - rustc_query_system::query::plumbing::get_query::h4db96bedffa51ecb
  42:     0x7f8ddd9a7d96 - rustc_middle::hir::map::Map::item::hf2640a5ccbe3999b
  43:     0x7f8ddc027ea3 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h0a686a582847eec4
  44:     0x7f8ddc0776c8 - rustc_passes::hir_id_validator::check_crate::h6c1b1c7710493138
  45:     0x7f8ddb54de03 - rustc_interface::passes::analysis::h12411377a0869928
  46:     0x7f8ddc7f7758 - rustc_query_system::query::plumbing::get_query_impl::h5735c97878b03a07
  47:     0x7f8ddc9762a9 - rustc_query_system::query::plumbing::get_query::h30e37eb808fef98c
  48:     0x7f8ddb47bcbe - rustc_interface::passes::QueryContext::enter::h69675a315cbb41ae
  49:     0x7f8ddb45530d - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::ha3c241d90cd6df1b
  50:     0x7f8ddb40f90c - rustc_span::with_source_map::hc1f971a314de0033
  51:     0x7f8ddb456b7c - rustc_interface::interface::create_compiler_and_run::he8b5567653002afa
  52:     0x7f8ddb44cb73 - scoped_tls::ScopedKey<T>::set::hbd1d37fb6e0ed7b1
  53:     0x7f8ddb41064f - std::sys_common::backtrace::__rust_begin_short_backtrace::h74301b14bff4dafb
  54:     0x7f8ddb47d036 - std::panicking::try::hcbf3d7d82fa622f9
  55:     0x7f8ddb40a9ea - core::ops::function::FnOnce::call_once{{vtable.shim}}::h903f7e20fb095d36
  56:     0x7f8ddab25eaa - std::sys::unix::thread::Thread::new::thread_start::hdc0362737611c684
  57:     0x7f8dd57df6db - start_thread
  58:     0x7f8dda7ab71f - __clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (d9e09763d 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 12060 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 132.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:03
