plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMgqOoi6dxJZ
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
[RUSTC-TIMING] hashbrown test:false 2.334
[RUSTC-TIMING] panic_abort test:false 0.069
[RUSTC-TIMING] rustc_demangle test:false 1.133
    Checking std v0.0.0 (/Users/runner/work/rust/rust/library/std)
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  = note: delayed at    0: std::backtrace_rs::backtrace::libunwind::trace
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
             1: std::backtrace_rs::backtrace::trace_unsynchronized
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
             2: std::backtrace::Backtrace::create
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/backtrace.rs:333:13
             3: std::backtrace::Backtrace::force_capture
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/backtrace.rs:314:9
             4: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             5: rustc_middle::ty::print::pretty::trimmed_def_paths
             6: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
             7: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             8: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             9: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_type
            10: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_type
            11: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::comma_sep::<rustc_middle::ty::Ty, core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>>>
            12: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_fn_sig
            13: <rustc_middle::ty::sty::FnSig as rustc_middle::ty::print::Print<rustc_middle::ty::print::pretty::FmtPrinter>>::print
            14: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::in_binder::<rustc_middle::ty::sty::FnSig>
            15: <rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig> as core::fmt::Display>::fmt
            16: <rustc_lint::builtin::ClashingExternDeclarations as rustc_lint::passes::LateLintPass>::check_foreign_item
            17: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_foreign_item
            18: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            19: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            20: rustc_hir::intravisit::walk_block::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            21: rustc_hir::intravisit::walk_expr::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            22: rustc_hir::intravisit::walk_body::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            23: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_body
            24: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            25: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            26: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            27: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            28: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            29: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            30: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            31: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            32: rustc_hir::intravisit::walk_item::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass>>
            33: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            34: rustc_lint::late::late_lint_crate::<rustc_lint::BuiltinCombinedLateLintPass>
            35: <rustc_session::session::Session>::time::<(), rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
            36: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
            37: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
            38: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}>
            40: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
            41: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            42: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            43: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            43: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            44: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
            45: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
            46: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            47: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            48: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            49: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/alloc/src/boxed.rs:1938:9
            50: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/alloc/src/boxed.rs:1938:9
                       at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/sys/unix/thread.rs:108:17
            52: __pthread_start
          


thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1536:13
stack backtrace:
   0:        0x117460d02 - std::backtrace_rs::backtrace::libunwind::trace::hcc58c68f8950d453
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:        0x117460d02 - std::backtrace_rs::backtrace::trace_unsynchronized::h89ef3acc893735f0
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x117460d02 - std::sys_common::backtrace::_print_fmt::h004894c8c52eec26
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/sys_common/backtrace.rs:66:5
   3:        0x117460d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3cfa4f7a71d2437f
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/sys_common/backtrace.rs:45:22
   4:        0x1174b848a - core::fmt::write::h4eda99704eb3ee0b
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/core/src/fmt/mod.rs:1209:17
   5:        0x117452c5c - std::io::Write::write_fmt::h7681cecabaf07de5
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/io/mod.rs:1679:15
   6:        0x117463d9b - std::sys_common::backtrace::_print::hb1676fc46ce7061a
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/sys_common/backtrace.rs:48:5
   7:        0x117463d9b - std::sys_common::backtrace::print::h946b6115f331cf87
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/sys_common/backtrace.rs:35:9
   8:        0x117463d9b - std::panicking::default_hook::{{closure}}::h8aaeba06950cbc70
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/panicking.rs:267:22
   9:        0x117463aa7 - std::panicking::default_hook::he83f4ae88a324c2c
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/panicking.rs:286:9
  10:        0x10e280bda - rustc_driver[fd3fd07d2149785b]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:        0x11746450a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h2ae3e23cca3af321
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/alloc/src/boxed.rs:1952:9
  12:        0x11746450a - std::panicking::rust_panic_with_hook::hcfabd7aface28828
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/panicking.rs:673:13
  13:        0x112c4a717 - std[7680b48f5a2a7111]::panicking::begin_panic::<rustc_errors[c7433642c6a85f5b]::ExplicitBug>::{closure#0}
  14:        0x112c48a39 - std[7680b48f5a2a7111]::sys_common::backtrace::__rust_end_short_backtrace::<std[7680b48f5a2a7111]::panicking::begin_panic<rustc_errors[c7433642c6a85f5b]::ExplicitBug>::{closure#0}, !>
  15:        0x112ff7319 - std[7680b48f5a2a7111]::panicking::begin_panic::<rustc_errors[c7433642c6a85f5b]::ExplicitBug>
  16:        0x112c396e9 - std[7680b48f5a2a7111]::panic::panic_any::<rustc_errors[c7433642c6a85f5b]::ExplicitBug>
  17:        0x112c3dec5 - <rustc_errors[c7433642c6a85f5b]::HandlerInner as core[d9110ab86072c25c]::ops::drop::Drop>::drop
  18:        0x10e2b38ba - core[d9110ab86072c25c]::ptr::drop_in_place::<rustc_session[f2c565e31c84d7ab]::parse::ParseSess>
  19:        0x10e2b675b - <alloc[113290240ff174e3]::rc::Rc<rustc_session[f2c565e31c84d7ab]::session::Session> as core[d9110ab86072c25c]::ops::drop::Drop>::drop
  20:        0x10e2fece2 - core[d9110ab86072c25c]::ptr::drop_in_place::<rustc_interface[203975543703cfb1]::interface::Compiler>
  21:        0x10e2fcbe0 - rustc_span[91581a51aa2052fe]::with_source_map::<core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>, rustc_interface[203975543703cfb1]::interface::create_compiler_and_run<core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>, rustc_driver[fd3fd07d2149785b]::run_compiler::{closure#1}>::{closure#1}>
  22:        0x10e288160 - rustc_interface[203975543703cfb1]::interface::create_compiler_and_run::<core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>, rustc_driver[fd3fd07d2149785b]::run_compiler::{closure#1}>
  23:        0x10e2f12b5 - <scoped_tls[2785d8421e1cf6df]::ScopedKey<rustc_span[91581a51aa2052fe]::SessionGlobals>>::set::<rustc_interface[203975543703cfb1]::interface::run_compiler<core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>, rustc_driver[fd3fd07d2149785b]::run_compiler::{closure#1}>::{closure#0}, core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>>
  24:        0x10e2bc73a - std[7680b48f5a2a7111]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[203975543703cfb1]::util::run_in_thread_pool_with_globals<rustc_interface[203975543703cfb1]::interface::run_compiler<core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>, rustc_driver[fd3fd07d2149785b]::run_compiler::{closure#1}>::{closure#0}, core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>>::{closure#0}, core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>>
  25:        0x10e2c0a28 - <<std[7680b48f5a2a7111]::thread::Builder>::spawn_unchecked_<rustc_interface[203975543703cfb1]::util::run_in_thread_pool_with_globals<rustc_interface[203975543703cfb1]::interface::run_compiler<core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>, rustc_driver[fd3fd07d2149785b]::run_compiler::{closure#1}>::{closure#0}, core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>>::{closure#0}, core[d9110ab86072c25c]::result::Result<(), rustc_errors[c7433642c6a85f5b]::ErrorGuaranteed>>::{closure#1} as core[d9110ab86072c25c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:        0x11746d347 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4d01bfa5767ab76d
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/alloc/src/boxed.rs:1938:9
  27:        0x11746d347 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h36a598851758cd97
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/alloc/src/boxed.rs:1938:9
  28:        0x11746d347 - std::sys::unix::thread::Thread::new::thread_start::h9ae618be94cf61eb
                               at /rustc/7c6fb70c1efe59fbf97066f1e7b756fe0cef5ef9/library/std/src/sys/unix/thread.rs:108:17
  29:     0x7fff204388fc - __pthread_start
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7c6fb70c1 2022-10-01) running on x86_64-apple-darwin

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C split-debuginfo=unpacked -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -Z osx-rpath-install-name -C link-args=-Wl,-rpath,@loader_path/../lib -C split-debuginfo=unpacked -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
