plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'downcast to 1 turned ScalarPair layout into non-ScalarPair layout: TyAndLayout {
    ty: option::Option<u64>,
    layout: Layout {
        fields: Arbitrary {
            offsets: [
                    raw: 0,
                },
            ],
            memory_index: [
---
                    false,
                ),
                valid_range: 0..=1,
            },
            tag_encoding: Direct,
            tag_field: 0,
            variants: [
                    fields: Arbitrary {
                        offsets: [],
                        memory_index: [],
                    },
                    },
                    variants: Single {
                        index: 0,
                    },
                    abi: Aggregate {
                        sized: true,
                    },
                    largest_niche: None,
                    align: AbiAndPrefAlign {
                        abi: Align {
                            pow2: 0,
                        },
                        pref: Align {
                            pow2: 3,
                    },
                    size: Size {
                        raw: 8,
                    },
---
                    },
                    abi: Aggregate {
                        sized: true,
                    },
                    largest_niche: None,
                    align: AbiAndPrefAlign {
                        abi: Align {
                            pow2: 3,
                        },
                        pref: Align {
                            pow2: 3,
                    },
                    size: Size {
                        raw: 16,
                    },
---
                ),
                valid_range: 0..=1,
            },
        ),
        align: AbiAndPrefAlign {
            abi: Align {
                pow2: 3,
            },
            pref: Align {
                pow2: 3,
        },
        size: Size {
            raw: 16,
        },
---
        },
        abi: Aggregate {
            sized: true,
        },
        largest_niche: None,
        align: AbiAndPrefAlign {
            abi: Align {
                pow2: 3,
            },
            pref: Align {
                pow2: 3,
        },
        size: Size {
            raw: 16,
        },
        },
    },
}', compiler/rustc_const_eval/src/interpret/operand.rs:445:21
stack backtrace:
   0:     0x7f1306864bf2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f13068cbf08 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f1306855161 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f1306867f36 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f1306867b2d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f130737fdd1 - rustc_driver[610de2f075f3812e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f13068688d0 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f13068686e7 - std::panicking::begin_panic_handler::{{closure}}::hdc297c549f81c3b7
   8:     0x7f1306865194 - std::sys_common::backtrace::__rust_end_short_backtrace::h7b90b067d1e7c19a
   9:     0x7f13068683d9 - rust_begin_unwind
  10:     0x7f130681c0b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f13087ecc21 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::operand_downcast
  12:     0x7f13087ece36 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::operand_projection
  13:     0x7f1308889a0e - <core[10878fb91fc84a80]::iter::adapters::copied::Copied<core[10878fb91fc84a80]::slice::iter::Iter<rustc_middle[87143a0b6987fc74]::mir::ProjectionElem<rustc_middle[87143a0b6987fc74]::mir::Local, rustc_middle[87143a0b6987fc74]::ty::Ty>>> as core[10878fb91fc84a80]::iter::traits::iterator::Iterator>::try_fold::<rustc_const_eval[da0e58d07d0e471]::interpret::operand::OpTy, <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::eval_place_to_op::{closure#0}, core[10878fb91fc84a80]::result::Result<rustc_const_eval[da0e58d07d0e471]::interpret::operand::OpTy, rustc_middle[87143a0b6987fc74]::mir::interpret::error::InterpErrorInfo>>
  14:     0x7f13087ed69f - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::eval_place_to_op
  15:     0x7f13087edbb8 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::eval_operand
  16:     0x7f13087fcb6e - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::statement
  17:     0x7f13087f9295 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_const_eval[da0e58d07d0e471]::const_eval::machine::CompileTimeInterpreter>>::run
  18:     0x7f13088fd43f - rustc_const_eval[da0e58d07d0e471]::const_eval::eval_queries::eval_to_allocation_raw_provider
  19:     0x7f1308af4bb9 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::eval_to_allocation_raw, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  20:     0x7f1308e52cd5 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::eval_to_allocation_raw
  21:     0x7f13088fb858 - rustc_const_eval[da0e58d07d0e471]::const_eval::eval_queries::eval_to_const_value_raw_provider
  22:     0x7f1308afe5c6 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::eval_to_const_value_raw, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  23:     0x7f1308e52d15 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::eval_to_const_value_raw
  24:     0x7f13088fb302 - rustc_const_eval[da0e58d07d0e471]::const_eval::eval_queries::eval_to_const_value_raw_provider
  25:     0x7f1308afe5c6 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::eval_to_const_value_raw, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  26:     0x7f1308e52d15 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::eval_to_const_value_raw
  27:     0x7f1309a96ede - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::const_eval_global_id
  28:     0x7f1309abe15a - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::const_eval_resolve
  29:     0x7f1309724bcc - <rustc_trait_selection[ee9dd18565913941]::traits::query::normalize::QueryNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::FallibleTypeFolder>::try_fold_const
  30:     0x7f1309724dbb - <rustc_trait_selection[ee9dd18565913941]::traits::query::normalize::QueryNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  31:     0x7f130864dc62 - <rustc_infer[e9909202d8948eff]::infer::at::At as rustc_trait_selection[ee9dd18565913941]::traits::query::normalize::AtExt>::normalize::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  32:     0x7f130865b8dc - <rustc_infer[e9909202d8948eff]::infer::InferCtxtBuilder>::enter::<core[10878fb91fc84a80]::result::Result<rustc_middle[87143a0b6987fc74]::mir::ConstantKind, rustc_middle[87143a0b6987fc74]::traits::query::NoSolution>, rustc_traits[cff765eab9023830]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>::{closure#0}>
  33:     0x7f130873f86e - <rustc_traits[cff765eab9023830]::normalize_erasing_regions::provide::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt, rustc_middle[87143a0b6987fc74]::ty::ParamEnvAnd<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>)>>::call_once
  34:     0x7f1308b266b1 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  35:     0x7f1308e53bf6 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  36:     0x7f1309c233ff - <rustc_middle[87143a0b6987fc74]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[87143a0b6987fc74]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  37:     0x7f1307a46497 - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  38:     0x7f1307a47f11 - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  39:     0x7f1307849f97 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop::ConstPropMachine>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  40:     0x7f1307874f03 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropMachine>>::eval_operand
  41:     0x7f130785a70e - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  42:     0x7f130799b543 - <rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropagator as rustc_middle[87143a0b6987fc74]::mir::visit::Visitor>::visit_statement
  43:     0x7f130799afdf - <rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropagator as rustc_middle[87143a0b6987fc74]::mir::visit::Visitor>::visit_body
  44:     0x7f130799792d - <rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstProp as rustc_mir_transform[4aaf9bfc3db13ff7]::pass_manager::MirLint>::run_lint
  45:     0x7f1307a1ce55 - rustc_mir_transform[4aaf9bfc3db13ff7]::pass_manager::run_passes
  46:     0x7f1307a05e69 - rustc_mir_transform[4aaf9bfc3db13ff7]::run_post_borrowck_cleanup_passes
  47:     0x7f1307a05a5d - rustc_mir_transform[4aaf9bfc3db13ff7]::mir_drops_elaborated_and_const_checked
  48:     0x7f13089ee39c - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_middle[87143a0b6987fc74]::ty::WithOptConstParam<rustc_span[e033c2886c1ea87]::def_id::LocalDefId>, &rustc_data_structures[e873bb7798a1662c]::steal::Steal<rustc_middle[87143a0b6987fc74]::mir::Body>>>
  49:     0x7f1308b243eb - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  50:     0x7f1308e52526 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  51:     0x7f13074900a2 - <rustc_session[490bd8b11d3080dc]::session::Session>::time::<(), rustc_interface[8c85a1b7802599b3]::passes::analysis::{closure#3}>
  52:     0x7f130748afb2 - rustc_interface[8c85a1b7802599b3]::passes::analysis
  53:     0x7f1308a385bc - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>>
  54:     0x7f1308b2b1c9 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::analysis, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  55:     0x7f130737181a - <rustc_interface[8c85a1b7802599b3]::passes::QueryContext>::enter::<rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  56:     0x7f1307315036 - <rustc_interface[8c85a1b7802599b3]::interface::Compiler>::enter::<rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[8c85a1b7802599b3]::queries::Linker>, rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  57:     0x7f13072f6b46 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[8c85a1b7802599b3]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#1}>
  58:     0x7f13073280c7 - rustc_interface[8c85a1b7802599b3]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>
  59:     0x7f130732b822 - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  60:     0x7f1307373799 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8c85a1b7802599b3]::util::run_in_thread_pool_with_globals<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  61:     0x7f130732dbce - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[8c85a1b7802599b3]::util::run_in_thread_pool_with_globals<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  62:     0x7f130736e8f0 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[8c85a1b7802599b3]::util::run_in_thread_pool_with_globals<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7f1306875223 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  64:     0x7f1300dc6609 - start_thread
  65:     0x7f13066d9163 - clone
  66:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (fe70a35db 2022-04-20) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `time::<impl at library/core/src/time.rs:77:1: 917:2>::MAX`
#1 [eval_to_const_value_raw] simplifying constant for the type system `time::<impl at library/core/src/time.rs:77:1: 917:2>::MAX`
#2 [eval_to_const_value_raw] simplifying constant for the type system `time::<impl at library/core/src/time.rs:77:1: 917:2>::MAX`
#3 [try_normalize_mir_const_after_erasing_regions] normalizing `time::<impl at library/core/src/time.rs:77:1: 917:2>::MAX`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `time::<impl at library/core/src/time.rs:77:1: 917:2>::saturating_add`
#5 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:04:51
