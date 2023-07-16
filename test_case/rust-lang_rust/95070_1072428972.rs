
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/compiler/rustc_middle/src/ty/sty.rs:1089:9
stack backtrace:
   0:     0x7f7556d8696d - std::backtrace_rs::backtrace::libunwind::trace::ha3a6b21151c60a7f
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f7556d8696d - std::backtrace_rs::backtrace::trace_unsynchronized::hef2c3337aca2e2b3
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f7556d8696d - std::sys_common::backtrace::_print_fmt::h7100e2252d44bdd1
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f7556d8696d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h388ab420cf9330db
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f7556de044c - core::fmt::write::hb92282007cec191f
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/core/src/fmt/mod.rs:1190:17
   5:     0x7f7556d77ef1 - std::io::Write::write_fmt::hd6ade7727f8519fc
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/io/mod.rs:1655:15
   6:     0x7f7556d899e5 - std::sys_common::backtrace::_print::he96f7ed8f0451fec
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f7556d899e5 - std::sys_common::backtrace::print::h827c968ad14125ac
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f7556d899e5 - std::panicking::default_hook::{{closure}}::hfcd1f9d84623f11f
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/panicking.rs:295:22
   9:     0x7f7556d89699 - std::panicking::default_hook::h7d1fe18b8808947c
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/panicking.rs:314:9
  10:     0x7f7557523721 - rustc_driver[6650adb4e8a07042]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f7556d8a130 - std::panicking::rust_panic_with_hook::haf4da044897eaa77
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/panicking.rs:702:17
  12:     0x7f7556d89f29 - std::panicking::begin_panic_handler::{{closure}}::h67c8a4c9c24c3542
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/panicking.rs:586:13
  13:     0x7f7556d86e24 - std::sys_common::backtrace::__rust_end_short_backtrace::h3f20aba2192957d4
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f7556d89c99 - rust_begin_unwind
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/panicking.rs:584:5
  15:     0x7f7556d4dc23 - core::panicking::panic_fmt::hf49e1497732e8ea4
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/core/src/panicking.rs:143:14
  16:     0x7f7556d4daed - core::panicking::panic::ha88b33e797e5f889
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/core/src/panicking.rs:48:5
  17:     0x7f755915435f - rustc_trait_selection[de84fd39e101a068]::traits::type_known_to_meet_bound_modulo_regions
  18:     0x7f75589ee1c5 - <rustc_infer[e7c0dc8a8349484a]::infer::InferCtxtBuilder>::enter::<bool, rustc_ty_utils[1c2b436d223d6e4e]::common_traits::is_item_raw::{closure#0}>
  19:     0x7f75596a3fae - rustc_ty_utils[1c2b436d223d6e4e]::common_traits::is_item_raw
  20:     0x7f7558dce88c - rustc_query_system[319694dcf5dfa03c]::query::plumbing::try_execute_query::<rustc_query_impl[2480959737b7d51e]::plumbing::QueryCtxt, rustc_query_system[319694dcf5dfa03c]::query::caches::DefaultCache<rustc_middle[5f63c5fd1ac72588]::ty::ParamEnvAnd<rustc_middle[5f63c5fd1ac72588]::ty::Ty>, bool>>
  21:     0x7f75598998fb - rustc_query_system[319694dcf5dfa03c]::query::plumbing::get_query::<rustc_query_impl[2480959737b7d51e]::queries::is_sized_raw, rustc_query_impl[2480959737b7d51e]::plumbing::QueryCtxt>
  22:     0x7f75592fe2e5 - <rustc_middle[5f63c5fd1ac72588]::ty::Ty>::is_sized
  23:     0x7f75592882fe - <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached
  24:     0x7f755929f276 - rustc_middle[5f63c5fd1ac72588]::ty::layout::layout_of
  25:     0x7f7558e2ebfa - rustc_query_system[319694dcf5dfa03c]::query::plumbing::get_query::<rustc_query_impl[2480959737b7d51e]::queries::layout_of, rustc_query_impl[2480959737b7d51e]::plumbing::QueryCtxt>
  26:     0x7f7558ecbf30 - <rustc_query_impl[2480959737b7d51e]::Queries as rustc_middle[5f63c5fd1ac72588]::ty::query::QueryEngine>::layout_of
  27:     0x7f75592aa778 - <alloc[36d2302c246c3597]::vec::Vec<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>> as alloc[36d2302c246c3597]::vec::spec_from_iter::SpecFromIter<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>, core[d9a3b528d00e4bcd]::iter::adapters::GenericShunt<core[d9a3b528d00e4bcd]::iter::adapters::map::Map<core[d9a3b528d00e4bcd]::slice::iter::Iter<rustc_middle[5f63c5fd1ac72588]::ty::FieldDef>, <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[d9a3b528d00e4bcd]::result::Result<core[d9a3b528d00e4bcd]::convert::Infallible, rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutError>>>>::from_iter
  28:     0x7f75592a9c1c - <alloc[36d2302c246c3597]::vec::Vec<alloc[36d2302c246c3597]::vec::Vec<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>>> as alloc[36d2302c246c3597]::vec::spec_from_iter::SpecFromIter<alloc[36d2302c246c3597]::vec::Vec<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>>, core[d9a3b528d00e4bcd]::iter::adapters::GenericShunt<core[d9a3b528d00e4bcd]::iter::adapters::map::Map<core[d9a3b528d00e4bcd]::slice::iter::Iter<rustc_middle[5f63c5fd1ac72588]::ty::VariantDef>, <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[d9a3b528d00e4bcd]::result::Result<core[d9a3b528d00e4bcd]::convert::Infallible, rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutError>>>>::from_iter
  29:     0x7f7559287ef0 - <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached
  30:     0x7f755929f276 - rustc_middle[5f63c5fd1ac72588]::ty::layout::layout_of
  31:     0x7f7558e2ebfa - rustc_query_system[319694dcf5dfa03c]::query::plumbing::get_query::<rustc_query_impl[2480959737b7d51e]::queries::layout_of, rustc_query_impl[2480959737b7d51e]::plumbing::QueryCtxt>
  32:     0x7f7558ecbf30 - <rustc_query_impl[2480959737b7d51e]::Queries as rustc_middle[5f63c5fd1ac72588]::ty::query::QueryEngine>::layout_of
  33:     0x7f75592aa778 - <alloc[36d2302c246c3597]::vec::Vec<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>> as alloc[36d2302c246c3597]::vec::spec_from_iter::SpecFromIter<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>, core[d9a3b528d00e4bcd]::iter::adapters::GenericShunt<core[d9a3b528d00e4bcd]::iter::adapters::map::Map<core[d9a3b528d00e4bcd]::slice::iter::Iter<rustc_middle[5f63c5fd1ac72588]::ty::FieldDef>, <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[d9a3b528d00e4bcd]::result::Result<core[d9a3b528d00e4bcd]::convert::Infallible, rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutError>>>>::from_iter
  34:     0x7f75592a9c1c - <alloc[36d2302c246c3597]::vec::Vec<alloc[36d2302c246c3597]::vec::Vec<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>>> as alloc[36d2302c246c3597]::vec::spec_from_iter::SpecFromIter<alloc[36d2302c246c3597]::vec::Vec<rustc_target[5d4685c11228a5f9]::abi::TyAndLayout<rustc_middle[5f63c5fd1ac72588]::ty::Ty>>, core[d9a3b528d00e4bcd]::iter::adapters::GenericShunt<core[d9a3b528d00e4bcd]::iter::adapters::map::Map<core[d9a3b528d00e4bcd]::slice::iter::Iter<rustc_middle[5f63c5fd1ac72588]::ty::VariantDef>, <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[d9a3b528d00e4bcd]::result::Result<core[d9a3b528d00e4bcd]::convert::Infallible, rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutError>>>>::from_iter
  35:     0x7f7559287ef0 - <rustc_middle[5f63c5fd1ac72588]::ty::layout::LayoutCx<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt>>::layout_of_uncached
  36:     0x7f755929f276 - rustc_middle[5f63c5fd1ac72588]::ty::layout::layout_of
  37:     0x7f7558e2ebfa - rustc_query_system[319694dcf5dfa03c]::query::plumbing::get_query::<rustc_query_impl[2480959737b7d51e]::queries::layout_of, rustc_query_impl[2480959737b7d51e]::plumbing::QueryCtxt>
  38:     0x7f7558ecbf30 - <rustc_query_impl[2480959737b7d51e]::Queries as rustc_middle[5f63c5fd1ac72588]::ty::query::QueryEngine>::layout_of
  39:     0x7f7558ed27e5 - rustc_codegen_ssa[82efcbacec23bfcf]::debuginfo::type_names::push_debuginfo_type_name
  40:     0x7f7558ed1f66 - rustc_codegen_ssa[82efcbacec23bfcf]::debuginfo::type_names::push_debuginfo_type_name
  41:     0x7f7558ed0770 - rustc_codegen_ssa[82efcbacec23bfcf]::debuginfo::type_names::compute_debuginfo_type_name
  42:     0x7f755877e3bc - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::type_di_node
  43:     0x7f755877c34c - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::build_pointer_or_reference_di_node
  44:     0x7f755877d805 - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::type_di_node
  45:     0x7f7558779419 - <&mut rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::build_struct_type_di_node::{closure#0}::{closure#0} as core[d9a3b528d00e4bcd]::ops::function::FnOnce<((usize, &rustc_middle[5f63c5fd1ac72588]::ty::FieldDef),)>>::call_once
  46:     0x7f755873ef3b - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::type_map::build_type_with_children::<rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::build_struct_type_di_node::{closure#0}, rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::build_struct_type_di_node::{closure#1}>
  47:     0x7f755877d545 - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::type_di_node
  48:     0x7f755877c34c - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::build_pointer_or_reference_di_node
  49:     0x7f755877d805 - rustc_codegen_llvm[3b78a86e98ab98ba]::debuginfo::metadata::type_di_node
  50:     0x7f755875ee58 - <rustc_codegen_llvm[3b78a86e98ab98ba]::context::CodegenCx as rustc_codegen_ssa[82efcbacec23bfcf]::traits::debuginfo::DebugInfoMethods>::dbg_scope_fn
  51:     0x7f755878adc2 - rustc_codegen_ssa[82efcbacec23bfcf]::mir::codegen_mir::<rustc_codegen_llvm[3b78a86e98ab98ba]::builder::Builder>
  52:     0x7f75587493d4 - rustc_codegen_llvm[3b78a86e98ab98ba]::base::compile_codegen_unit::module_codegen
  53:     0x7f7559407d36 - <rustc_query_system[319694dcf5dfa03c]::dep_graph::graph::DepGraph<rustc_middle[5f63c5fd1ac72588]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[5f63c5fd1ac72588]::ty::context::TyCtxt, rustc_span[3354b82f0c75eb5d]::symbol::Symbol, rustc_codegen_ssa[82efcbacec23bfcf]::ModuleCodegen<rustc_codegen_llvm[3b78a86e98ab98ba]::ModuleLlvm>>
  54:     0x7f75593f6969 - rustc_codegen_llvm[3b78a86e98ab98ba]::base::compile_codegen_unit
  55:     0x7f75593e4b8c - <rustc_codegen_llvm[3b78a86e98ab98ba]::LlvmCodegenBackend as rustc_codegen_ssa[82efcbacec23bfcf]::traits::backend::CodegenBackend>::codegen_crate
  56:     0x7f75593c5ec7 - <rustc_session[77248bc14774052c]::session::Session>::time::<alloc[36d2302c246c3597]::boxed::Box<dyn core[d9a3b528d00e4bcd]::any::Any>, rustc_interface[1d66963a510d2f71]::passes::start_codegen::{closure#0}>
  57:     0x7f75593b4938 - <rustc_interface[1d66963a510d2f71]::passes::QueryContext>::enter::<<rustc_interface[1d66963a510d2f71]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[d9a3b528d00e4bcd]::result::Result<alloc[36d2302c246c3597]::boxed::Box<dyn core[d9a3b528d00e4bcd]::any::Any>, rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>
  58:     0x7f75593ad0bf - <rustc_interface[1d66963a510d2f71]::queries::Queries>::ongoing_codegen
  59:     0x7f7559370ebb - <rustc_interface[1d66963a510d2f71]::interface::Compiler>::enter::<rustc_driver[6650adb4e8a07042]::run_compiler::{closure#1}::{closure#2}, core[d9a3b528d00e4bcd]::result::Result<core[d9a3b528d00e4bcd]::option::Option<rustc_interface[1d66963a510d2f71]::queries::Linker>, rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>
  60:     0x7f7559383f2f - rustc_span[3354b82f0c75eb5d]::with_source_map::<core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>, rustc_interface[1d66963a510d2f71]::interface::create_compiler_and_run<core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>, rustc_driver[6650adb4e8a07042]::run_compiler::{closure#1}>::{closure#1}>
  61:     0x7f7559371b04 - rustc_interface[1d66963a510d2f71]::interface::create_compiler_and_run::<core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>, rustc_driver[6650adb4e8a07042]::run_compiler::{closure#1}>
  62:     0x7f755936e8b2 - <scoped_tls[11b42f00ec6f76c4]::ScopedKey<rustc_span[3354b82f0c75eb5d]::SessionGlobals>>::set::<rustc_interface[1d66963a510d2f71]::interface::run_compiler<core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>, rustc_driver[6650adb4e8a07042]::run_compiler::{closure#1}>::{closure#0}, core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>
  63:     0x7f755936cd5f - std[dc1d017f401af48a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1d66963a510d2f71]::util::run_in_thread_pool_with_globals<rustc_interface[1d66963a510d2f71]::interface::run_compiler<core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>, rustc_driver[6650adb4e8a07042]::run_compiler::{closure#1}>::{closure#0}, core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>::{closure#0}, core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>
  64:     0x7f7559384eb2 - <<std[dc1d017f401af48a]::thread::Builder>::spawn_unchecked_<rustc_interface[1d66963a510d2f71]::util::run_in_thread_pool_with_globals<rustc_interface[1d66963a510d2f71]::interface::run_compiler<core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>, rustc_driver[6650adb4e8a07042]::run_compiler::{closure#1}>::{closure#0}, core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>::{closure#0}, core[d9a3b528d00e4bcd]::result::Result<(), rustc_errors[e415a2b41ae93448]::ErrorGuaranteed>>::{closure#1} as core[d9a3b528d00e4bcd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  65:     0x7f7556d942c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h410ea22b43d61d56
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/alloc/src/boxed.rs:1853:9
  66:     0x7f7556d942c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hff190b74d8f3d909
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/alloc/src/boxed.rs:1853:9
  67:     0x7f7556d942c3 - std::sys::unix::thread::Thread::new::thread_start::h8cb9e648ae9f2254
                               at /rustc/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/std/src/sys/unix/thread.rs:108:17
  68:     0x7f7556b2c947 - <unknown>
  69:     0x7f7556bbca44 - clone
  70:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (58f11791a 2022-03-17) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [is_sized_raw] computing whether `dyn std::io::Write` is `Sized`
#1 [layout_of] computing layout of `*const dyn std::io::Write`
#2 [layout_of] computing layout of `core::ptr::unique::Unique<dyn std::io::Write>`
#3 [layout_of] computing layout of `alloc::boxed::Box<dyn std::io::Write>`
end of query stack
error: could not compile `tracing-subscriber`
