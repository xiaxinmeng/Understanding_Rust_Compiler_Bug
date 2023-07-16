error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (c8e6a9e8b 2023-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C linker-plugin-lto -Z unstable-options

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [layout_of] computing layout of `game::player::PlayerState`
#1 [layout_of] computing layout of `game::player::Player`
#2 [layout_of] computing layout of `game::intermission::Intermission`
#3 [layout_of] computing layout of `game::State`
#4 [lint_mod] linting module `game`
#5 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'layout decided on a larger discriminant type (I32) than typeck (I16)', /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/compiler/rustc_abi/src/layout.rs:687:13
stack backtrace:
   0:     0x7feb795665aa - std::backtrace_rs::backtrace::libunwind::trace::hc4549fb01f77d87b
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7feb795665aa - std::backtrace_rs::backtrace::trace_unsynchronized::h05cb13333c68eb08
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7feb795665aa - std::sys_common::backtrace::_print_fmt::hb24673f184c0851c
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7feb795665aa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hadea210d809498bd
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7feb795c83ce - core::fmt::write::hf9ad1be74a79073a
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/core/src/fmt/mod.rs:1213:17
   5:     0x7feb79556bf5 - std::io::Write::write_fmt::had5ed065fbe61198
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/io/mod.rs:1682:15
   6:     0x7feb79566375 - std::sys_common::backtrace::_print::hef3b235b241008f8
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7feb79566375 - std::sys_common::backtrace::print::hde0ad3d49d7fc96d
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7feb7956913f - std::panicking::default_hook::{{closure}}::h42b02c3de78aff65
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/panicking.rs:267:22
   9:     0x7feb79568e7b - std::panicking::default_hook::h1bb8dbe2c450c437
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/panicking.rs:286:9
  10:     0x7feb7c8bd184 - <rustc_driver[95b46d23bf652bce]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[97748cd456f183ce]::ops::function::FnOnce<(&core[97748cd456f183ce]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7feb7956997a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hf8cd3db51ac86633
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/alloc/src/boxed.rs:2002:9
  12:     0x7feb7956997a - std::panicking::rust_panic_with_hook::hb7b7ecf2debe3baf
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/panicking.rs:692:13
  13:     0x7feb795696f9 - std::panicking::begin_panic_handler::{{closure}}::h9fd7ebbe2d13e884
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/panicking.rs:579:13
  14:     0x7feb79566a5c - std::sys_common::backtrace::__rust_end_short_backtrace::h7e26d5e8c003487a
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7feb79569402 - rust_begin_unwind
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/panicking.rs:575:5
  16:     0x7feb795c4d73 - core::panicking::panic_fmt::hfc11761ab6d92238
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/core/src/panicking.rs:64:14
  17:     0x7feb7b200a56 - <rustc_middle[cf6fe6ed69f79de8]::ty::layout::LayoutCx<rustc_middle[cf6fe6ed69f79de8]::ty::context::TyCtxt> as rustc_abi[65244fdf3a651594]::layout::LayoutCalculator>::layout_of_struct_or_enum::<rustc_target[35f696ace4059eb1]::abi::VariantIdx, rustc_target[35f696ace4059eb1]::abi::TyAndLayout<rustc_middle[cf6fe6ed69f79de8]::ty::Ty>, rustc_ty_utils[d855c86f7954c581]::layout::layout_of_uncached::{closure#6}, core[97748cd456f183ce]::iter::adapters::flatten::Flatten<core[97748cd456f183ce]::option::IntoIter<core[97748cd456f183ce]::iter::adapters::map::Map<core[97748cd456f183ce]::iter::adapters::map::Map<core[97748cd456f183ce]::iter::adapters::map::Map<core[97748cd456f183ce]::iter::adapters::enumerate::Enumerate<core[97748cd456f183ce]::slice::iter::Iter<rustc_middle[cf6fe6ed69f79de8]::ty::VariantDef>>, <rustc_index[54eace26a57da77f]::vec::IndexVec<rustc_target[35f696ace4059eb1]::abi::VariantIdx, rustc_middle[cf6fe6ed69f79de8]::ty::VariantDef>>::iter_enumerated::{closure#0}>, <rustc_middle[cf6fe6ed69f79de8]::ty::adt::AdtDef>::discriminants::{closure#0}>, rustc_ty_utils[d855c86f7954c581]::layout::layout_of_uncached::{closure#7}::{closure#0}>>>>
  18:     0x7feb7b1efc80 - rustc_ty_utils[d855c86f7954c581]::layout::layout_of
  19:     0x7feb7b3767db - rustc_query_system[ba5eddfaa9110d24]::query::plumbing::get_query::<rustc_query_impl[8abc0e1465f25a18]::queries::layout_of, rustc_query_impl[8abc0e1465f25a18]::plumbing::QueryCtxt, rustc_middle[cf6fe6ed69f79de8]::dep_graph::dep_node::DepKind>
  20:     0x7feb7b376363 - <rustc_query_impl[8abc0e1465f25a18]::Queries as rustc_middle[cf6fe6ed69f79de8]::ty::query::QueryEngine>::layout_of
  21:     0x7feb7b8f2b67 - <rustc_lint[a636c1caa3b09b71]::context::LateContext as rustc_middle[cf6fe6ed69f79de8]::ty::layout::LayoutOf>::spanned_layout_of
  22:     0x7feb7b531433 - <rustc_lint[a636c1caa3b09b71]::BuiltinCombinedModuleLateLintPass as rustc_lint[a636c1caa3b09b71]::passes::LateLintPass>::check_item
  23:     0x7feb7b52d2d5 - rustc_hir[2d870be94f3ae1aa]::intravisit::walk_mod::<rustc_lint[a636c1caa3b09b71]::late::LateContextAndPass<rustc_lint[a636c1caa3b09b71]::BuiltinCombinedModuleLateLintPass>>
  24:     0x7feb7b52cabe - rustc_lint[a636c1caa3b09b71]::late::late_lint_mod::<rustc_lint[a636c1caa3b09b71]::BuiltinCombinedModuleLateLintPass>
  25:     0x7feb7b52c7ec - rustc_lint[a636c1caa3b09b71]::lint_mod
  26:     0x7feb7be20f01 - rustc_query_system[ba5eddfaa9110d24]::query::plumbing::try_execute_query::<rustc_query_impl[8abc0e1465f25a18]::queries::lint_mod, rustc_query_impl[8abc0e1465f25a18]::plumbing::QueryCtxt>
  27:     0x7feb7c2760ad - <rustc_query_impl[8abc0e1465f25a18]::Queries as rustc_middle[cf6fe6ed69f79de8]::ty::query::QueryEngine>::lint_mod
  28:     0x7feb7be66788 - rustc_data_structures[3bf82646017727ef]::sync::par_for_each_in::<&[rustc_hir[2d870be94f3ae1aa]::hir_id::OwnerId], <rustc_middle[cf6fe6ed69f79de8]::hir::map::Map>::par_for_each_module<rustc_lint[a636c1caa3b09b71]::late::check_crate<rustc_lint[a636c1caa3b09b71]::BuiltinCombinedLateLintPass, rustc_interface[f0071e0df2442917]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  29:     0x7feb7be66482 - <rustc_session[7b43865df64c3c6e]::session::Session>::time::<(), rustc_lint[a636c1caa3b09b71]::late::check_crate<rustc_lint[a636c1caa3b09b71]::BuiltinCombinedLateLintPass, rustc_interface[f0071e0df2442917]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  30:     0x7feb7bed405f - <rustc_session[7b43865df64c3c6e]::session::Session>::time::<(), rustc_interface[f0071e0df2442917]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  31:     0x7feb7bf4ec0f - <core[97748cd456f183ce]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f0071e0df2442917]::passes::analysis::{closure#5}::{closure#1}> as core[97748cd456f183ce]::ops::function::FnOnce<()>>::call_once
  32:     0x7feb7ae5fce8 - <rustc_session[7b43865df64c3c6e]::session::Session>::time::<(), rustc_interface[f0071e0df2442917]::passes::analysis::{closure#5}>
  33:     0x7feb7ae5c746 - rustc_interface[f0071e0df2442917]::passes::analysis
  34:     0x7feb7c04f15e - rustc_query_system[ba5eddfaa9110d24]::query::plumbing::try_execute_query::<rustc_query_impl[8abc0e1465f25a18]::queries::analysis, rustc_query_impl[8abc0e1465f25a18]::plumbing::QueryCtxt>
  35:     0x7feb7c27358a - <rustc_query_impl[8abc0e1465f25a18]::Queries as rustc_middle[cf6fe6ed69f79de8]::ty::query::QueryEngine>::analysis
  36:     0x7feb7bae0210 - <rustc_interface[f0071e0df2442917]::passes::QueryContext>::enter::<rustc_driver[95b46d23bf652bce]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>>
  37:     0x7feb7baddc84 - rustc_span[590d8702e08b269b]::with_source_map::<core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>, rustc_interface[f0071e0df2442917]::interface::run_compiler<core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>, rustc_driver[95b46d23bf652bce]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7feb7bad66a4 - <scoped_tls[f4bd44dfe0f30b85]::ScopedKey<rustc_span[590d8702e08b269b]::SessionGlobals>>::set::<rustc_interface[f0071e0df2442917]::interface::run_compiler<core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>, rustc_driver[95b46d23bf652bce]::run_compiler::{closure#1}>::{closure#0}, core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>>
  39:     0x7feb7bad5da2 - std[3136dbe5650cc209]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f0071e0df2442917]::util::run_in_thread_pool_with_globals<rustc_interface[f0071e0df2442917]::interface::run_compiler<core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>, rustc_driver[95b46d23bf652bce]::run_compiler::{closure#1}>::{closure#0}, core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>>
  40:     0x7feb7bad5b4a - <<std[3136dbe5650cc209]::thread::Builder>::spawn_unchecked_<rustc_interface[f0071e0df2442917]::util::run_in_thread_pool_with_globals<rustc_interface[f0071e0df2442917]::interface::run_compiler<core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>, rustc_driver[95b46d23bf652bce]::run_compiler::{closure#1}>::{closure#0}, core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[97748cd456f183ce]::result::Result<(), rustc_errors[780dd287a8056c57]::ErrorGuaranteed>>::{closure#1} as core[97748cd456f183ce]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7feb79573833 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3384d9be08768d2e
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/alloc/src/boxed.rs:1988:9
  42:     0x7feb79573833 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfcd870ad56c96f75
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/alloc/src/boxed.rs:1988:9
  43:     0x7feb79573833 - std::sys::unix::thread::Thread::new::thread_start::hb742e799f6e8b754
                               at /rustc/c8e6a9e8b6251bbc8276cb78cabe1998deecbed7/library/std/src/sys/unix/thread.rs:108:17
  44:     0x7feb79294b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  45:     0x7feb79326a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  46:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (c8e6a9e8b 2023-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C linker-plugin-lto -Z unstable-options

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [layout_of] computing layout of `game::player::PlayerState`
#1 [lint_mod] linting module `game::player`
#2 [analysis] running analysis passes on this crate
end of query stack
