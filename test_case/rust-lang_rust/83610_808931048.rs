plain
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i......i................................ 9600/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.099 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.337 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- [rustdoc] rustdoc/normalize-assoc-item.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/normalize-assoc-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/normalize-assoc-item" "/checkout/src/test/rustdoc/normalize-assoc-item.rs" "-Znormalize-docs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
             3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             4: rustc_data_structures::stack::ensure_sufficient_stack
             5: rustc_query_system::query::plumbing::force_query_with_job
             6: rustc_query_system::query::plumbing::get_query_impl
             7: rustc_query_system::query::plumbing::get_query
             8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             9: rustc_middle::ty::print::Printer::default_print_def_path
            10: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
            11: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::pretty::PrettyPrinter>::generic_delimiters
            12: rustc_middle::ty::print::Printer::default_print_def_path
            13: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
            14: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::print::Print<P> for rustc_middle::ty::sty::ProjectionTy>::print
            15: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
            16: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_const_pointer
            17: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_const_value
            18: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_const
            19: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::consts::Const>::fmt
            21: core::fmt::Write::write_fmt
            22: rustdoc::clean::utils::print_evaluated_const
            23: rustdoc::clean::types::Constant::value
            24: rustdoc::html::render::print_item::print_item
            24: rustdoc::html::render::print_item::print_item
            25: rustdoc::html::layout::render
            26: rustdoc::html::render::context::Context::render_item
            27: <rustdoc::html::render::context::Context as rustdoc::formats::renderer::FormatRenderer>::item
            28: rustdoc::formats::renderer::run_format
            29: rustdoc::run_renderer
            30: rustc_interface::passes::QueryContext::enter
            31: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
            32: rustc_span::with_source_map
            34: scoped_tls::ScopedKey<T>::set
            35: rustc_span::with_session_globals
            36: std::sys_common::backtrace::__rust_begin_short_backtrace
            37: std::panic::catch_unwind
            37: std::panic::catch_unwind
            38: core::ops::function::FnOnce::call_once{{vtable.shim}}
            39: std::sys::unix::thread::Thread::new::thread_start
            40: start_thread
            41: __clone
          

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1014:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (c79444bfb 2021-03-28) running on x86_64-unknown-linux-gnu
note: compiler flags: -Z normalize-docs

query stack during panic:
end of query stack
---
test result: FAILED. 425 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 34.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:54
