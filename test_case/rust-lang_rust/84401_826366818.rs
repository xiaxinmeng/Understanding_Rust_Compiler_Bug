plain
.................................................................................................... 9400/11801
.................................................................................................... 9500/11801
.................................................................................................... 9600/11801
...........................i......i................................................................. 9700/11801
.........................................................................iiiiiii...iiiiiii.......... 9800/11801
.................................................................................................... 10000/11801
.................................................................................................... 10100/11801
.................................................................................................... 10200/11801
.................................................................................................... 10300/11801
---
failures:

---- [ui] ui/proc-macro/issue-59191-replace-root-with-fn.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-59191-replace-root-with-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn/auxiliary" "--extern" "issue_59191=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-59191-replace-root-with-fn/auxiliary/libissue_59191.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected crate top-level item to be a module after macro expansion, found a function
   |
   |
LL | #![issue_59191::no_main]
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /checkout/compiler/rustc_middle/src/hir/map/mod.rs:180:30

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (3f0bd7f9e 2021-04-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_middle/src/hir/map/mod.rs:180:30
stack backtrace:
   0:     0x7f0766cac2b0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f0766d2049c - core::fmt::write::hf00778d011964c9e
   2:     0x7f0766ca0086 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f0766cb0647 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f0766cb0054 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f07675803fd - rustc_driver::report_ice::h8bf5dda5073c20a4
   6:     0x7f0766cb0f82 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f0766cb0ab7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f0766cac74c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f0766cb0a19 - rust_begin_unwind
  10:     0x7f0766d1caa1 - core::panicking::panic_fmt::hacef841ef98e28f3
  11:     0x7f0766d1ca62 - core::panicking::panic_bounds_check::hce8ba9b35db91873
  12:     0x7f076988e435 - core::ops::function::FnOnce::call_once::h0ac4dda825aff491
  13:     0x7f07685c16a4 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hd19c4befb2cbe7a4
  14:     0x7f0768371229 - rustc_data_structures::stack::ensure_sufficient_stack::hf88de971a98abcf8
  15:     0x7f07681c5f5d - rustc_query_system::query::plumbing::force_query_with_job::h73d4532f756c39c3
  16:     0x7f0768116586 - rustc_query_system::query::plumbing::get_query_impl::h2fc2391bfa535b29
  17:     0x7f076827a0ef - rustc_query_system::query::plumbing::get_query::h68fe38bc73f976cd
  18:     0x7f076842be66 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h0d0ee101c7b2e0df
  19:     0x7f076862794c - rustc_query_impl::make_query::fn_sig::h6a423b096050486a
  20:     0x7f07683bf305 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h555936254f4d90ca
  21:     0x7f07686c843a - <hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<(K,V)>>::extend::he9519ed5e29d07b8
  22:     0x7f076823860c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h95a99c2dfa0889a1
  23:     0x7f07684f98b8 - rustc_query_impl::Queries::try_collect_active_jobs::h978feddf1e82c551
  24:     0x7f0768344331 - rustc_query_system::query::job::print_query_stack::h6c9e06a78f062d0f
  25:     0x7f07676bd4e4 - rustc_interface::interface::try_print_query_stack::hc03f552e3a07727c
  26:     0x7f0767580c5a - rustc_driver::report_ice::h8bf5dda5073c20a4
  27:     0x7f0766cb0f82 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
  28:     0x7f0766cb0ab7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
  29:     0x7f0766cac74c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
  30:     0x7f0766cb0a19 - rust_begin_unwind
  31:     0x7f0766d1caa1 - core::panicking::panic_fmt::hacef841ef98e28f3
  32:     0x7f0766d1ca62 - core::panicking::panic_bounds_check::hce8ba9b35db91873
  33:     0x7f0767cdc28a - rustc_typeck::collect::fn_sig::h65f2d86fd9907712
  34:     0x7f076829b3dc - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::fn_sig>::compute::h890089d9c5e30cd7
  35:     0x7f07685961c4 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h301f82ed0c22fc22
  36:     0x7f076835fda1 - rustc_data_structures::stack::ensure_sufficient_stack::h931ba15e4efe169c
  37:     0x7f07681d398e - rustc_query_system::query::plumbing::force_query_with_job::hd09a69411263ace1
  38:     0x7f07681692c9 - rustc_query_system::query::plumbing::get_query_impl::he11952c401032649
  39:     0x7f0768275971 - rustc_query_system::query::plumbing::get_query::h55db37c229930ef6
  40:     0x7f07684fbbd8 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig::h03262356fc3d421d
  41:     0x7f0767ce9435 - rustc_typeck::check_for_entry_fn::hc09308723ac591df
  42:     0x7f0767cec338 - rustc_typeck::check_crate::he2e8553544497184
  43:     0x7f07676dc242 - rustc_interface::passes::analysis::hf9ce45f8847807a2
  44:     0x7f07685c2e4c - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hd7379b177fecb9b0
  45:     0x7f07685ce909 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h3844f6091680bcd4
  46:     0x7f0768360106 - rustc_data_structures::stack::ensure_sufficient_stack::h93c3868d557b8597
  47:     0x7f07681b4a07 - rustc_query_system::query::plumbing::force_query_with_job::h0d9627a72c79b840
  48:     0x7f0768170727 - rustc_query_system::query::plumbing::get_query_impl::heb8b9059ed7c7a28
  49:     0x7f0768271c23 - rustc_query_system::query::plumbing::get_query::h3e364939e49ffa43
  50:     0x7f07675bad8c - rustc_interface::passes::QueryContext::enter::hdf8c5d76d81da0d0
  51:     0x7f07675989bd - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h35c0578a937cd313
  52:     0x7f07675819a0 - rustc_span::with_source_map::h1f55dbec7a0af24a
  53:     0x7f076759776c - scoped_tls::ScopedKey<T>::set::hf839ddefa16035a8
  54:     0x7f07675826f3 - rustc_span::with_session_globals::h1c12a3478ee17e2a
  55:     0x7f07675bc900 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1fa5c3594f6cfa7f
  56:     0x7f07675bdc9a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h72f7b001ab47a8ba
  57:     0x7f0766cc0fda - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  58:     0x7f07619766db - start_thread
  59:     0x7f076694271f - __clone
  60:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (3f0bd7f9e 2021-04-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 11703 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 126.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:52
