plain
.i...................................................................................... 11088/13915
.........................................................................i.............. 11176/13915
...................................................................................iiiii 11264/13915
i.i...iiiiiiiiiiii...................................................................... 11352/13915
.....................................................................................F.. 11440/13915
....F...................F............................................................... 11528/13915
........................................................................................ 11704/13915
........................................................................................ 11792/13915
........................................................................................ 11880/13915
........................................................................................ 11968/13915
---
---- [ui] src/test/ui/simd/issue-17170.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-17170.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-17170/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-17170/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(32 bytes)`,
 right: `Size(24 bytes)`: size mismatch between ABI and layout in TyAndLayout {
    ty: T,
    layout: Layout {
        size: Size(32 bytes),
        align: AbiAndPrefAlign {
            abi: Align(32 bytes),
            pref: Align(32 bytes),
        abi: Vector {
            element: Initialized {
                value: F64,
                valid_range: 0..=18446744073709551615,
                valid_range: 0..=18446744073709551615,
            },
            count: 3,
        },
        fields: Array {
            stride: Size(8 bytes),
            count: 3,
        largest_niche: None,
        variants: Single {
            index: 0,
        },
        },
    },
}', compiler/rustc_ty_utils/src/layout_sanity_check.rs:242:17
stack backtrace:
   0:     0x7fd218d84ed5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9a6391f0722571d
   1:     0x7fd218df4ac8 - core::fmt::write::h8a8552afd9155037
   2:     0x7fd218d76be1 - std::io::Write::write_fmt::h5f8a47337628422b
   3:     0x7fd218d84ce1 - std::sys_common::backtrace::print::h3e356927c1034bd2
   4:     0x7fd218d88024 - std::panicking::default_hook::{{closure}}::h01d846064c3ea768
   5:     0x7fd218d87cea - std::panicking::default_hook::he9a2d425b77df7c2
   6:     0x7fd2197cbe64 - rustc_driver[afec5ad26ec07cdc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd218d88794 - std::panicking::rust_panic_with_hook::hc0d58aea4f7dc0e7
   8:     0x7fd218d884f9 - std::panicking::begin_panic_handler::{{closure}}::hdf5fa2cc07caae7b
   9:     0x7fd218d853f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h40e04619b5b52913
  10:     0x7fd218d881a2 - rust_begin_unwind
  11:     0x7fd218d38ff3 - core::panicking::panic_fmt::he689818fecf15ba4
  12:     0x7fd218df1506 - core::panicking::assert_failed_inner::hd29e1918d7530aa7
  13:     0x7fd2195a988b - core[acc128b6d3448550]::panicking::assert_failed::<rustc_abi[3b1da883587c704d]::Size, rustc_abi[3b1da883587c704d]::Size>
  14:     0x7fd219ca41b0 - rustc_ty_utils[c1ffd4c2863c4617]::layout_sanity_check::sanity_check_layout
  15:     0x7fd219c2bc38 - rustc_ty_utils[c1ffd4c2863c4617]::layout::layout_of
  16:     0x7fd21b6ce51f - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::layout_of, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  17:     0x7fd21b2bf05f - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::layout_of
  18:     0x7fd219fd51ea - rustc_hir_analysis[765f1053246efdba]::check::check::check_static_inhabited
  19:     0x7fd219fddfaf - rustc_hir_analysis[765f1053246efdba]::check::check::check_item_type
  20:     0x7fd219fe949a - rustc_hir_analysis[765f1053246efdba]::check::check::check_mod_item_types
  21:     0x7fd21b5dd2e4 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::VecCache<rustc_span[9fc4562c5627a501]::def_id::LocalDefId, ()>>
  22:     0x7fd21b69afd3 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::check_mod_item_types, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  23:     0x7fd21b29a6f0 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::check_mod_item_types
  24:     0x7fd21a02c0ca - <rustc_middle[661ae43798c879ae]::hir::map::Map>::for_each_module::<rustc_hir_analysis[765f1053246efdba]::check_crate::{closure#6}::{closure#0}>
  25:     0x7fd219feb112 - <rustc_session[dcd906aff22a5eb0]::session::Session>::time::<(), rustc_hir_analysis[765f1053246efdba]::check_crate::{closure#6}>
  26:     0x7fd21a17e921 - rustc_hir_analysis[765f1053246efdba]::check_crate
  27:     0x7fd219918cd1 - rustc_interface[42619a81f47d969f]::passes::analysis
  28:     0x7fd21b5b5bd8 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<(), core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>>
  29:     0x7fd21b6cd36f - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::analysis, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  30:     0x7fd21b27600a - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::analysis
  31:     0x7fd21982711c - <rustc_interface[42619a81f47d969f]::passes::QueryContext>::enter::<rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  32:     0x7fd21983145f - <rustc_interface[42619a81f47d969f]::interface::Compiler>::enter::<rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}::{closure#2}, core[acc128b6d3448550]::result::Result<core[acc128b6d3448550]::option::Option<rustc_interface[42619a81f47d969f]::queries::Linker>, rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  33:     0x7fd2197cd5c6 - rustc_span[9fc4562c5627a501]::with_source_map::<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  34:     0x7fd2198321ea - <scoped_tls[7c6cc4b66463b95e]::ScopedKey<rustc_span[9fc4562c5627a501]::SessionGlobals>>::set::<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  35:     0x7fd2197eba6f - std[fe331575ae875469]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  36:     0x7fd21984ca06 - std[fe331575ae875469]::panicking::try::<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, core[acc128b6d3448550]::panic::unwind_safe::AssertUnwindSafe<<std[fe331575ae875469]::thread::Builder>::spawn_unchecked_<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7fd2197dc375 - <<std[fe331575ae875469]::thread::Builder>::spawn_unchecked_<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#1} as core[acc128b6d3448550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fd218d94dae - std::sys::unix::thread::Thread::new::thread_start::heb8d1d55b4c4f697
  39:     0x7fd218b2ab43 - <unknown>
  40:     0x7fd218bbca00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (495c2d052 2022-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `T`
#0 [layout_of] computing layout of `T`
#1 [check_mod_item_types] checking item types in top-level module
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/simd/issue-39720.rs stdout ----
---- [ui] src/test/ui/simd/issue-39720.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-39720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-39720/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-39720/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(8 bytes)`,
 right: `Size(6 bytes)`: size mismatch between ABI and layout in TyAndLayout {
    ty: Short3,
    layout: Layout {
        size: Size(8 bytes),
        align: AbiAndPrefAlign {
            abi: Align(8 bytes),
            pref: Align(8 bytes),
        abi: Vector {
            element: Initialized {
                value: Int(
                    I16,
                    I16,
                    true,
                ),
                valid_range: 0..=65535,
            },
            count: 3,
        },
        fields: Array {
            stride: Size(2 bytes),
            count: 3,
        largest_niche: None,
        variants: Single {
            index: 0,
        },
        },
    },
}', compiler/rustc_ty_utils/src/layout_sanity_check.rs:242:17
stack backtrace:
   0:     0x7f1a02ef4ed5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9a6391f0722571d
   1:     0x7f1a02f64ac8 - core::fmt::write::h8a8552afd9155037
   2:     0x7f1a02ee6be1 - std::io::Write::write_fmt::h5f8a47337628422b
   3:     0x7f1a02ef4ce1 - std::sys_common::backtrace::print::h3e356927c1034bd2
   4:     0x7f1a02ef8024 - std::panicking::default_hook::{{closure}}::h01d846064c3ea768
   5:     0x7f1a02ef7cea - std::panicking::default_hook::he9a2d425b77df7c2
   6:     0x7f1a0393be64 - rustc_driver[afec5ad26ec07cdc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1a02ef8794 - std::panicking::rust_panic_with_hook::hc0d58aea4f7dc0e7
   8:     0x7f1a02ef84f9 - std::panicking::begin_panic_handler::{{closure}}::hdf5fa2cc07caae7b
   9:     0x7f1a02ef53f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h40e04619b5b52913
  10:     0x7f1a02ef81a2 - rust_begin_unwind
  11:     0x7f1a02ea8ff3 - core::panicking::panic_fmt::he689818fecf15ba4
  12:     0x7f1a02f61506 - core::panicking::assert_failed_inner::hd29e1918d7530aa7
  13:     0x7f1a0371988b - core[acc128b6d3448550]::panicking::assert_failed::<rustc_abi[3b1da883587c704d]::Size, rustc_abi[3b1da883587c704d]::Size>
  14:     0x7f1a03e141b0 - rustc_ty_utils[c1ffd4c2863c4617]::layout_sanity_check::sanity_check_layout
  15:     0x7f1a03d9bc38 - rustc_ty_utils[c1ffd4c2863c4617]::layout::layout_of
  16:     0x7f1a0583e51f - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::layout_of, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  17:     0x7f1a0542f05f - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::layout_of
  18:     0x7f1a04473959 - <rustc_mir_transform[91e7eeb7e8f7d892]::const_prop::CanConstProp>::check
  19:     0x7f1a0458b00e - <rustc_mir_transform[91e7eeb7e8f7d892]::const_prop_lint::ConstProp as rustc_mir_transform[91e7eeb7e8f7d892]::pass_manager::MirLint>::run_lint
  20:     0x7f1a0446b5f9 - rustc_mir_transform[91e7eeb7e8f7d892]::pass_manager::run_passes_inner
  21:     0x7f1a0450f05a - rustc_mir_transform[91e7eeb7e8f7d892]::run_analysis_to_runtime_passes
  22:     0x7f1a0450eb29 - rustc_mir_transform[91e7eeb7e8f7d892]::mir_drops_elaborated_and_const_checked
  23:     0x7f1a056f1618 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<rustc_middle[661ae43798c879ae]::ty::WithOptConstParam<rustc_span[9fc4562c5627a501]::def_id::LocalDefId>, &rustc_data_structures[7ca766ba47c4a380]::steal::Steal<rustc_middle[661ae43798c879ae]::mir::Body>>>
  24:     0x7f1a058386bf - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  25:     0x7f1a053f1314 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  26:     0x7f1a0450fe24 - rustc_mir_transform[91e7eeb7e8f7d892]::optimized_mir
  27:     0x7f1a05716b3c - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<rustc_span[9fc4562c5627a501]::def_id::DefId, &rustc_middle[661ae43798c879ae]::mir::Body>>
  28:     0x7f1a057f0482 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::optimized_mir, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  29:     0x7f1a053f3855 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::optimized_mir
  30:     0x7f1a06713ae0 - <rustc_middle[661ae43798c879ae]::ty::context::TyCtxt>::instance_mir
  31:     0x7f1a043153f2 - rustc_monomorphize[83f95f3b526aa398]::collector::collect_neighbours
  32:     0x7f1a04311881 - rustc_monomorphize[83f95f3b526aa398]::collector::collect_items_rec
  33:     0x7f1a0431f504 - std[fe331575ae875469]::panicking::try::<(), core[acc128b6d3448550]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7ca766ba47c4a380]::sync::par_for_each_in<alloc[8304736b6e0e8e20]::vec::Vec<rustc_middle[661ae43798c879ae]::mir::mono::MonoItem>, rustc_monomorphize[83f95f3b526aa398]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  34:     0x7f1a043460a5 - <rustc_session[dcd906aff22a5eb0]::session::Session>::time::<(), rustc_monomorphize[83f95f3b526aa398]::collector::collect_crate_mono_items::{closure#1}>
  35:     0x7f1a0430e596 - rustc_monomorphize[83f95f3b526aa398]::collector::collect_crate_mono_items
  36:     0x7f1a0432efda - rustc_monomorphize[83f95f3b526aa398]::partitioning::collect_and_partition_mono_items
  37:     0x7f1a0573036a - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<(), (&std[fe331575ae875469]::collections::hash::set::HashSet<rustc_span[9fc4562c5627a501]::def_id::DefId, core[acc128b6d3448550]::hash::BuildHasherDefault<rustc_hash[52bdf98ab26bca14]::FxHasher>>, &[rustc_middle[661ae43798c879ae]::mir::mono::CodegenUnit])>>
  38:     0x7f1a05834124 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::collect_and_partition_mono_items, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  39:     0x7f1a05451e48 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::collect_and_partition_mono_items
  40:     0x7f1a03bd6b88 - rustc_codegen_ssa[5fb601c50b68a84b]::base::codegen_crate::<rustc_codegen_llvm[554f2d2cbdcf22c2]::LlvmCodegenBackend>
  41:     0x7f1a03cd21a7 - <rustc_codegen_llvm[554f2d2cbdcf22c2]::LlvmCodegenBackend as rustc_codegen_ssa[5fb601c50b68a84b]::traits::backend::CodegenBackend>::codegen_crate
  42:     0x7f1a03a6ab9f - <rustc_session[dcd906aff22a5eb0]::session::Session>::time::<alloc[8304736b6e0e8e20]::boxed::Box<dyn core[acc128b6d3448550]::any::Any>, rustc_interface[42619a81f47d969f]::passes::start_codegen::{closure#0}>
  43:     0x7f1a03a8900b - rustc_interface[42619a81f47d969f]::passes::start_codegen
  44:     0x7f1a03a885d7 - <rustc_interface[42619a81f47d969f]::passes::QueryContext>::enter::<<rustc_interface[42619a81f47d969f]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<alloc[8304736b6e0e8e20]::boxed::Box<dyn core[acc128b6d3448550]::any::Any>, rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  45:     0x7f1a03b0034b - <rustc_interface[42619a81f47d969f]::queries::Queries>::ongoing_codegen
  46:     0x7f1a039a14ac - <rustc_interface[42619a81f47d969f]::interface::Compiler>::enter::<rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}::{closure#2}, core[acc128b6d3448550]::result::Result<core[acc128b6d3448550]::option::Option<rustc_interface[42619a81f47d969f]::queries::Linker>, rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  47:     0x7f1a0393d5c6 - rustc_span[9fc4562c5627a501]::with_source_map::<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  48:     0x7f1a039a21ea - <scoped_tls[7c6cc4b66463b95e]::ScopedKey<rustc_span[9fc4562c5627a501]::SessionGlobals>>::set::<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  49:     0x7f1a0395ba6f - std[fe331575ae875469]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  50:     0x7f1a039bca06 - std[fe331575ae875469]::panicking::try::<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, core[acc128b6d3448550]::panic::unwind_safe::AssertUnwindSafe<<std[fe331575ae875469]::thread::Builder>::spawn_unchecked_<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7f1a0394c375 - <<std[fe331575ae875469]::thread::Builder>::spawn_unchecked_<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#1} as core[acc128b6d3448550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f1a02f04dae - std::sys::unix::thread::Thread::new::thread_start::heb8d1d55b4c4f697
  53:     0x7f1a02c9ab43 - <unknown>
  54:     0x7f1a02d2ca00 - <unknown>
  55:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (495c2d052 2022-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `Short3`
#0 [layout_of] computing layout of `Short3`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#2 [optimized_mir] optimizing MIR for `main`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/simd/type-generic-monomorphisation-power-of-two.rs stdout ----
---- [ui] src/test/ui/simd/type-generic-monomorphisation-power-of-two.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/type-generic-monomorphisation-power-of-two.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-power-of-two/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-power-of-two/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(16 bytes)`,
 right: `Size(12 bytes)`: size mismatch between ABI and layout in TyAndLayout {
    ty: Simd<3>,
    layout: Layout {
        size: Size(16 bytes),
        align: AbiAndPrefAlign {
            abi: Align(16 bytes),
            pref: Align(16 bytes),
        abi: Vector {
            element: Initialized {
                value: F32,
                valid_range: 0..=4294967295,
---
        },
    },
}', compiler/rustc_ty_utils/src/layout_sanity_check.rs:242:17
stack backtrace:
   0:     0x7fa089cffed5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9a6391f0722571d
   1:     0x7fa089d6fac8 - core::fmt::write::h8a8552afd9155037
   2:     0x7fa089cf1be1 - std::io::Write::write_fmt::h5f8a47337628422b
   3:     0x7fa089cffce1 - std::sys_common::backtrace::print::h3e356927c1034bd2
   4:     0x7fa089d03024 - std::panicking::default_hook::{{closure}}::h01d846064c3ea768
   5:     0x7fa089d02cea - std::panicking::default_hook::he9a2d425b77df7c2
   6:     0x7fa08a746e64 - rustc_driver[afec5ad26ec07cdc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa089d03794 - std::panicking::rust_panic_with_hook::hc0d58aea4f7dc0e7
   8:     0x7fa089d034f9 - std::panicking::begin_panic_handler::{{closure}}::hdf5fa2cc07caae7b
   9:     0x7fa089d003f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h40e04619b5b52913
  10:     0x7fa089d031a2 - rust_begin_unwind
  11:     0x7fa089cb3ff3 - core::panicking::panic_fmt::he689818fecf15ba4
  12:     0x7fa089d6c506 - core::panicking::assert_failed_inner::hd29e1918d7530aa7
  13:     0x7fa08a52488b - core[acc128b6d3448550]::panicking::assert_failed::<rustc_abi[3b1da883587c704d]::Size, rustc_abi[3b1da883587c704d]::Size>
  14:     0x7fa08ac1f1b0 - rustc_ty_utils[c1ffd4c2863c4617]::layout_sanity_check::sanity_check_layout
  15:     0x7fa08aba6c38 - rustc_ty_utils[c1ffd4c2863c4617]::layout::layout_of
  16:     0x7fa08c64951f - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::layout_of, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  17:     0x7fa08c23a05f - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::layout_of
  18:     0x7fa08b27e959 - <rustc_mir_transform[91e7eeb7e8f7d892]::const_prop::CanConstProp>::check
  19:     0x7fa08b39600e - <rustc_mir_transform[91e7eeb7e8f7d892]::const_prop_lint::ConstProp as rustc_mir_transform[91e7eeb7e8f7d892]::pass_manager::MirLint>::run_lint
  20:     0x7fa08b2765f9 - rustc_mir_transform[91e7eeb7e8f7d892]::pass_manager::run_passes_inner
  21:     0x7fa08b31a05a - rustc_mir_transform[91e7eeb7e8f7d892]::run_analysis_to_runtime_passes
  22:     0x7fa08b319b29 - rustc_mir_transform[91e7eeb7e8f7d892]::mir_drops_elaborated_and_const_checked
  23:     0x7fa08c4fc618 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<rustc_middle[661ae43798c879ae]::ty::WithOptConstParam<rustc_span[9fc4562c5627a501]::def_id::LocalDefId>, &rustc_data_structures[7ca766ba47c4a380]::steal::Steal<rustc_middle[661ae43798c879ae]::mir::Body>>>
  24:     0x7fa08c6436bf - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  25:     0x7fa08c1fc314 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  26:     0x7fa08b31ae24 - rustc_mir_transform[91e7eeb7e8f7d892]::optimized_mir
  27:     0x7fa08c521b3c - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<rustc_span[9fc4562c5627a501]::def_id::DefId, &rustc_middle[661ae43798c879ae]::mir::Body>>
  28:     0x7fa08c5fb482 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::optimized_mir, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  29:     0x7fa08c1fe855 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::optimized_mir
  30:     0x7fa08d51eae0 - <rustc_middle[661ae43798c879ae]::ty::context::TyCtxt>::instance_mir
  31:     0x7fa08b1203f2 - rustc_monomorphize[83f95f3b526aa398]::collector::collect_neighbours
  32:     0x7fa08b11c881 - rustc_monomorphize[83f95f3b526aa398]::collector::collect_items_rec
  33:     0x7fa08b12a504 - std[fe331575ae875469]::panicking::try::<(), core[acc128b6d3448550]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7ca766ba47c4a380]::sync::par_for_each_in<alloc[8304736b6e0e8e20]::vec::Vec<rustc_middle[661ae43798c879ae]::mir::mono::MonoItem>, rustc_monomorphize[83f95f3b526aa398]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  34:     0x7fa08b1510a5 - <rustc_session[dcd906aff22a5eb0]::session::Session>::time::<(), rustc_monomorphize[83f95f3b526aa398]::collector::collect_crate_mono_items::{closure#1}>
  35:     0x7fa08b119596 - rustc_monomorphize[83f95f3b526aa398]::collector::collect_crate_mono_items
  36:     0x7fa08b139fda - rustc_monomorphize[83f95f3b526aa398]::partitioning::collect_and_partition_mono_items
  37:     0x7fa08c53b36a - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::try_execute_query::<rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt, rustc_query_system[ed3be45091aa1ac2]::query::caches::DefaultCache<(), (&std[fe331575ae875469]::collections::hash::set::HashSet<rustc_span[9fc4562c5627a501]::def_id::DefId, core[acc128b6d3448550]::hash::BuildHasherDefault<rustc_hash[52bdf98ab26bca14]::FxHasher>>, &[rustc_middle[661ae43798c879ae]::mir::mono::CodegenUnit])>>
  38:     0x7fa08c63f124 - rustc_query_system[ed3be45091aa1ac2]::query::plumbing::get_query::<rustc_query_impl[b01ca022a0e8c2b]::queries::collect_and_partition_mono_items, rustc_query_impl[b01ca022a0e8c2b]::plumbing::QueryCtxt>
  39:     0x7fa08c25ce48 - <rustc_query_impl[b01ca022a0e8c2b]::Queries as rustc_middle[661ae43798c879ae]::ty::query::QueryEngine>::collect_and_partition_mono_items
  40:     0x7fa08a9e1b88 - rustc_codegen_ssa[5fb601c50b68a84b]::base::codegen_crate::<rustc_codegen_llvm[554f2d2cbdcf22c2]::LlvmCodegenBackend>
  41:     0x7fa08aadd1a7 - <rustc_codegen_llvm[554f2d2cbdcf22c2]::LlvmCodegenBackend as rustc_codegen_ssa[5fb601c50b68a84b]::traits::backend::CodegenBackend>::codegen_crate
  42:     0x7fa08a875b9f - <rustc_session[dcd906aff22a5eb0]::session::Session>::time::<alloc[8304736b6e0e8e20]::boxed::Box<dyn core[acc128b6d3448550]::any::Any>, rustc_interface[42619a81f47d969f]::passes::start_codegen::{closure#0}>
  43:     0x7fa08a89400b - rustc_interface[42619a81f47d969f]::passes::start_codegen
  44:     0x7fa08a8935d7 - <rustc_interface[42619a81f47d969f]::passes::QueryContext>::enter::<<rustc_interface[42619a81f47d969f]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<alloc[8304736b6e0e8e20]::boxed::Box<dyn core[acc128b6d3448550]::any::Any>, rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  45:     0x7fa08a90b34b - <rustc_interface[42619a81f47d969f]::queries::Queries>::ongoing_codegen
  46:     0x7fa08a7ac4ac - <rustc_interface[42619a81f47d969f]::interface::Compiler>::enter::<rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}::{closure#2}, core[acc128b6d3448550]::result::Result<core[acc128b6d3448550]::option::Option<rustc_interface[42619a81f47d969f]::queries::Linker>, rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  47:     0x7fa08a7485c6 - rustc_span[9fc4562c5627a501]::with_source_map::<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  48:     0x7fa08a7ad1ea - <scoped_tls[7c6cc4b66463b95e]::ScopedKey<rustc_span[9fc4562c5627a501]::SessionGlobals>>::set::<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  49:     0x7fa08a766a6f - std[fe331575ae875469]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>
  50:     0x7fa08a7c7a06 - std[fe331575ae875469]::panicking::try::<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, core[acc128b6d3448550]::panic::unwind_safe::AssertUnwindSafe<<std[fe331575ae875469]::thread::Builder>::spawn_unchecked_<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7fa08a757375 - <<std[fe331575ae875469]::thread::Builder>::spawn_unchecked_<rustc_interface[42619a81f47d969f]::util::run_in_thread_pool_with_globals<rustc_interface[42619a81f47d969f]::interface::run_compiler<core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>, rustc_driver[afec5ad26ec07cdc]::run_compiler::{closure#1}>::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[acc128b6d3448550]::result::Result<(), rustc_errors[44c76a689f3302d2]::ErrorGuaranteed>>::{closure#1} as core[acc128b6d3448550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7fa089d0fdae - std::sys::unix::thread::Thread::new::thread_start::heb8d1d55b4c4f697
  53:     0x7fa089aa5b43 - <unknown>
  54:     0x7fa089b37a00 - <unknown>
  55:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (495c2d052 2022-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `Simd<3>`
#0 [layout_of] computing layout of `Simd<3>`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#2 [optimized_mir] optimizing MIR for `main`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------



