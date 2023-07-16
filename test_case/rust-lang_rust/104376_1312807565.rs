plain
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
error: internal compiler error: /checkout/compiler/rustc_middle/src/ty/layout.rs:701:30: TyAndLayout::field(TyAndLayout { ty: &T, layout: Layout { size: Size(16 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 1..=18446744073709551615 }, Initialized { value: Pointer, valid_range: 1..=18446744073709551615 }), fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: Pointer, valid_range: 1..=18446744073709551615 }), variants: Single { index: 0 } } }): not applicable
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
   0:     0x7fa4eaed46f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   0:     0x7fa4eaed46f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7fa4eaf427a8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7fa4eaec51c1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7fa4eaed44b5 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7fa4eaed7867 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7fa4eaed75c5 - std::panicking::default_hook::h5ab5b9712723f5dd
   6:     0x7fa4eb85c014 - rustc_driver[3b78b5da40b87e7b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa4eaed8173 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7fa4ee29af93 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[c6390cd32a21b219]::ExplicitBug>::{closure#0}
   9:     0x7fa4ee294196 - std[389b380b19480123]::sys_common::backtrace::__rust_end_short_backtrace::<std[389b380b19480123]::panicking::begin_panic<rustc_errors[c6390cd32a21b219]::ExplicitBug>::{closure#0}, !>
  10:     0x7fa4eb802896 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[c6390cd32a21b219]::ExplicitBug>
  11:     0x7fa4ee337696 - std[389b380b19480123]::panic::panic_any::<rustc_errors[c6390cd32a21b219]::ExplicitBug>
  12:     0x7fa4ee332057 - <rustc_errors[c6390cd32a21b219]::HandlerInner>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  13:     0x7fa4ee32e240 - <rustc_errors[c6390cd32a21b219]::Handler>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  14:     0x7fa4ee3f3800 - rustc_middle[890156e5e48c7d12]::ty::context::tls::with_context_opt::<rustc_middle[890156e5e48c7d12]::ty::context::tls::with_opt<rustc_middle[890156e5e48c7d12]::util::bug::opt_span_bug_fmt<rustc_span[b4ca3e966db910d5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fa4ee3f4a59 - rustc_middle[890156e5e48c7d12]::util::bug::opt_span_bug_fmt::<rustc_span[b4ca3e966db910d5]::span_encoding::Span>
  16:     0x7fa4eb807c85 - rustc_middle[890156e5e48c7d12]::util::bug::bug_fmt
  17:     0x7fa4ebcbedcc - <rustc_middle[890156e5e48c7d12]::ty::Ty as rustc_target[7278ac0112bb3806]::abi::TyAbiInterface<_>>::ty_and_layout_field::field_ty_or_layout::<rustc_middle[890156e5e48c7d12]::ty::layout::LayoutCx<rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt>>
  18:     0x7fa4ebccd16a - <rustc_middle[890156e5e48c7d12]::ty::Ty as rustc_target[7278ac0112bb3806]::abi::TyAbiInterface<rustc_middle[890156e5e48c7d12]::ty::layout::LayoutCx<rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt>>>::ty_and_layout_field
  19:     0x7fa4ebc9baa1 - rustc_ty_utils[ed851c0b658f582]::layout_sanity_check::sanity_check_layout::skip_newtypes
  20:     0x7fa4ebc99faf - rustc_ty_utils[ed851c0b658f582]::layout_sanity_check::sanity_check_layout
  21:     0x7fa4ebc94c85 - rustc_ty_utils[ed851c0b658f582]::layout::layout_of
  22:     0x7fa4ed5b26f3 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::layout_of, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  23:     0x7fa4ed180bd8 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::layout_of
  24:     0x7fa4ec2fc290 - <rustc_mir_transform[dac78d353aec7647]::const_prop::CanConstProp>::check
  25:     0x7fa4ec3f550a - <rustc_mir_transform[dac78d353aec7647]::const_prop_lint::ConstProp as rustc_mir_transform[dac78d353aec7647]::pass_manager::MirLint>::run_lint
  26:     0x7fa4ec21dfb3 - rustc_mir_transform[dac78d353aec7647]::pass_manager::run_passes_inner
  27:     0x7fa4ec32e24e - rustc_mir_transform[dac78d353aec7647]::run_analysis_to_runtime_passes
  28:     0x7fa4ec32da52 - rustc_mir_transform[dac78d353aec7647]::mir_drops_elaborated_and_const_checked
  29:     0x7fa4ed48037d - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_middle[890156e5e48c7d12]::ty::WithOptConstParam<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId>, &rustc_data_structures[d4b8fe12b073a5bc]::steal::Steal<rustc_middle[890156e5e48c7d12]::mir::Body>>>
  30:     0x7fa4ed5ac741 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  31:     0x7fa4ed14b3b3 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  32:     0x7fa4ec32ed66 - rustc_mir_transform[dac78d353aec7647]::optimized_mir
  33:     0x7fa4ed4b8eb0 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::DefId, &rustc_middle[890156e5e48c7d12]::mir::Body>>
  34:     0x7fa4ed564ee8 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::optimized_mir, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  35:     0x7fa4ed14d3ee - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::optimized_mir
  36:     0x7fa4ee3a8c84 - <rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt>::instance_mir
  37:     0x7fa4ec1b2d85 - rustc_monomorphize[244b002183c67a85]::collector::collect_neighbours
  38:     0x7fa4ec1afda9 - rustc_monomorphize[244b002183c67a85]::collector::collect_items_rec
  39:     0x7fa4ec1b023e - rustc_monomorphize[244b002183c67a85]::collector::collect_items_rec
  40:     0x7fa4ec1b023e - rustc_monomorphize[244b002183c67a85]::collector::collect_items_rec
  41:     0x7fa4ec1b023e - rustc_monomorphize[244b002183c67a85]::collector::collect_items_rec
  42:     0x7fa4ec1bbd44 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d4b8fe12b073a5bc]::sync::par_for_each_in<alloc[a40b22d2678a71d4]::vec::Vec<rustc_middle[890156e5e48c7d12]::mir::mono::MonoItem>, rustc_monomorphize[244b002183c67a85]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  43:     0x7fa4ec1b6645 - std[389b380b19480123]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d4b8fe12b073a5bc]::sync::par_for_each_in<alloc[a40b22d2678a71d4]::vec::Vec<rustc_middle[890156e5e48c7d12]::mir::mono::MonoItem>, rustc_monomorphize[244b002183c67a85]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  44:     0x7fa4ec1de8db - <rustc_session[9207d45749bb4341]::session::Session>::time::<(), rustc_monomorphize[244b002183c67a85]::collector::collect_crate_mono_items::{closure#1}>
  45:     0x7fa4ec1acc28 - rustc_monomorphize[244b002183c67a85]::collector::collect_crate_mono_items
  46:     0x7fa4ec1c144a - rustc_monomorphize[244b002183c67a85]::partitioning::collect_and_partition_mono_items
  47:     0x7fa4ed4de4ab - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<(), (&std[389b380b19480123]::collections::hash::set::HashSet<rustc_span[b4ca3e966db910d5]::def_id::DefId, core[69c2305d6fa5d54f]::hash::BuildHasherDefault<rustc_hash[9b58754660d0c792]::FxHasher>>, &[rustc_middle[890156e5e48c7d12]::mir::mono::CodegenUnit])>>
  48:     0x7fa4ed5a8262 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::collect_and_partition_mono_items, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  49:     0x7fa4ed19ee0f - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::collect_and_partition_mono_items
  50:     0x7fa4ebae6a64 - rustc_codegen_ssa[a1c28592740daff5]::base::codegen_crate::<rustc_codegen_llvm[79bd9af424f07822]::LlvmCodegenBackend>
  51:     0x7fa4eba78d6d - <rustc_codegen_llvm[79bd9af424f07822]::LlvmCodegenBackend as rustc_codegen_ssa[a1c28592740daff5]::traits::backend::CodegenBackend>::codegen_crate
  52:     0x7fa4eb973cdb - <rustc_session[9207d45749bb4341]::session::Session>::time::<alloc[a40b22d2678a71d4]::boxed::Box<dyn core[69c2305d6fa5d54f]::any::Any>, rustc_interface[81bbe172f7d4a9aa]::passes::start_codegen::{closure#0}>
  53:     0x7fa4eb99f52b - rustc_interface[81bbe172f7d4a9aa]::passes::start_codegen
  54:     0x7fa4eb99eb7e - <rustc_interface[81bbe172f7d4a9aa]::passes::QueryContext>::enter::<<rustc_interface[81bbe172f7d4a9aa]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<alloc[a40b22d2678a71d4]::boxed::Box<dyn core[69c2305d6fa5d54f]::any::Any>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  55:     0x7fa4eb985c22 - <rustc_interface[81bbe172f7d4a9aa]::queries::Queries>::ongoing_codegen
  56:     0x7fa4eb8c894b - <rustc_interface[81bbe172f7d4a9aa]::interface::Compiler>::enter::<rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[81bbe172f7d4a9aa]::queries::Linker>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  57:     0x7fa4eb85d6d2 - rustc_span[b4ca3e966db910d5]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  58:     0x7fa4eb8bc0ec - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[b4ca3e966db910d5]::SessionGlobals>>::set::<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  59:     0x7fa4eb8b7c5a - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  60:     0x7fa4eb8bd116 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  61:     0x7fa4eb86dc39 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7fa4eaee512e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
  63:     0x7fa4eac7ab43 - <unknown>
  64:     0x7fa4ead0ca00 - <unknown>
  65:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (4b3144a4c 2022-11-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [layout_of] computing layout of `&T`
#0 [layout_of] computing layout of `&T`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `ptr::thin_box::<impl at library/core/tests/ptr.rs:633:5: 633:23>::new`
#2 [optimized_mir] optimizing MIR for `ptr::thin_box::<impl at library/core/tests/ptr.rs:633:5: 633:23>::new`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:20:39
