plain
........................................................................................ 10560/13074
....................i................................................................... 10648/13074
.............................iiiiii.i..iiiiii.i......................................... 10736/13074
........................................................................................ 10824/13074
...................................F.....F..............F............................... 10912/13074
........................................................................................ 11088/13074
........................................................................................ 11176/13074
........................................................................................ 11264/13074
........................................................................................ 11352/13074
---
---- [ui] src/test/ui/simd/issue-17170.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-17170.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-17170/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-17170/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(32 bytes)`,
 right: `Size(24 bytes)`: size mismatch between ABI and layout in Layout {
    fields: Array {
        stride: Size(8 bytes),
        count: 3,
    variants: Single {
        index: 0,
    },
    abi: Vector {
    abi: Vector {
        element: Initialized {
            value: F64,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
            valid_range: 0..=18446744073709551615,
        },
        count: 3,
    },
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align(32 bytes),
        pref: Align(32 bytes),
    size: Size(32 bytes),
}', compiler/rustc_middle/src/ty/layout.rs:257:21
stack backtrace:
stack backtrace:
   0:     0x7fbb46c689ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3e179c5dee025621
   1:     0x7fbb46ccef28 - core::fmt::write::hea82302efebf9141
   2:     0x7fbb46c58841 - std::io::Write::write_fmt::h64b1ad44f66f31ca
   3:     0x7fbb46c6b9de - std::panicking::default_hook::{{closure}}::h742860da356df9e1
   4:     0x7fbb46c6b60c - std::panicking::default_hook::h3a601028c8f58c22
   5:     0x7fbb477ea1d1 - rustc_driver[a98a92c00c701caf]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbb46c6c23e - std::panicking::rust_panic_with_hook::h318ebd36736be86a
   7:     0x7fbb46c6c037 - std::panicking::begin_panic_handler::{{closure}}::h7898bd1de465703a
   8:     0x7fbb46c68ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4bcdc1450fcc36d1
   9:     0x7fbb46c6bd19 - rust_begin_unwind
  10:     0x7fbb46c20073 - core::panicking::panic_fmt::hcb25134d916a2ec6
  11:     0x7fbb46ccb56e - core::panicking::assert_failed_inner::h66ffbf09e7dbfcbf
  12:     0x7fbb476eadeb - core[e7599f4a3d786bf9]::panicking::assert_failed::<rustc_target[8011f39e06f9ca66]::abi::Size, rustc_target[8011f39e06f9ca66]::abi::Size>
  13:     0x7fbb49f9cfbd - rustc_middle[7173a0b96d44f42d]::ty::layout::sanity_check_layout::check_layout_abi
  14:     0x7fbb49fb37e1 - rustc_middle[7173a0b96d44f42d]::ty::context::tls::with_context::<rustc_middle[7173a0b96d44f42d]::ty::context::tls::with_related_context<rustc_middle[7173a0b96d44f42d]::ty::layout::layout_of::{closure#0}, core[e7599f4a3d786bf9]::result::Result<rustc_target[8011f39e06f9ca66]::abi::TyAndLayout<rustc_middle[7173a0b96d44f42d]::ty::Ty>, rustc_middle[7173a0b96d44f42d]::ty::layout::LayoutError>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<rustc_target[8011f39e06f9ca66]::abi::TyAndLayout<rustc_middle[7173a0b96d44f42d]::ty::Ty>, rustc_middle[7173a0b96d44f42d]::ty::layout::LayoutError>>::{closure#0}
  15:     0x7fbb49fb7703 - rustc_middle[7173a0b96d44f42d]::ty::layout::layout_of
  16:     0x7fbb4929e0e4 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::layout_of, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  17:     0x7fbb490f46d3 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::layout_of
  18:     0x7fbb4830417e - rustc_typeck[261112ec7b3076f5]::check::check::check_static_inhabited
  19:     0x7fbb48305b6f - rustc_typeck[261112ec7b3076f5]::check::check::check_item_type
  20:     0x7fbb48310877 - rustc_typeck[261112ec7b3076f5]::check::check::check_mod_item_types
  21:     0x7fbb4918b57f - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<rustc_span[eb4f81bc7a12d0ae]::def_id::LocalDefId, ()>>
  22:     0x7fbb49266e94 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::check_mod_item_types, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  23:     0x7fbb490daca4 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::check_mod_item_types
  24:     0x7fbb48285ef6 - <rustc_middle[7173a0b96d44f42d]::hir::map::Map>::for_each_module::<rustc_typeck[261112ec7b3076f5]::check_crate::{closure#6}::{closure#0}>
  25:     0x7fbb48185872 - <rustc_session[72bf6ac283d0085c]::session::Session>::time::<(), rustc_typeck[261112ec7b3076f5]::check_crate::{closure#6}>
  26:     0x7fbb4814a140 - rustc_typeck[261112ec7b3076f5]::check_crate
  27:     0x7fbb478bc941 - rustc_interface[6fec859ab01ba9fc]::passes::analysis
  28:     0x7fbb491c3fce - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<(), core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>>
  29:     0x7fbb4929cfd2 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::analysis, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  30:     0x7fbb490c1c0e - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::analysis
  31:     0x7fbb477ceb34 - <rustc_interface[6fec859ab01ba9fc]::passes::QueryContext>::enter::<rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  32:     0x7fbb4777ab0e - <rustc_interface[6fec859ab01ba9fc]::interface::Compiler>::enter::<rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}::{closure#2}, core[e7599f4a3d786bf9]::result::Result<core[e7599f4a3d786bf9]::option::Option<rustc_interface[6fec859ab01ba9fc]::queries::Linker>, rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  33:     0x7fbb4775bceb - rustc_span[eb4f81bc7a12d0ae]::with_source_map::<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_interface[6fec859ab01ba9fc]::interface::create_compiler_and_run<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7fbb4777bca9 - <scoped_tls[62b5de94f67e9e6e]::ScopedKey<rustc_span[eb4f81bc7a12d0ae]::SessionGlobals>>::set::<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  35:     0x7fbb477d1fc9 - std[798f2a17af60ab34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  36:     0x7fbb4777d1a1 - std[798f2a17af60ab34]::panicking::try::<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, core[e7599f4a3d786bf9]::panic::unwind_safe::AssertUnwindSafe<<std[798f2a17af60ab34]::thread::Builder>::spawn_unchecked_<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7fbb477d2cf2 - <<std[798f2a17af60ab34]::thread::Builder>::spawn_unchecked_<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#1} as core[e7599f4a3d786bf9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fbb46c77593 - std::sys::unix::thread::Thread::new::thread_start::h379f8ab2a7d23392
  39:     0x7fbb411ca609 - start_thread
  40:     0x7fbb46add163 - clone
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (0404d6839 2022-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-39720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-39720/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-39720/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(8 bytes)`,
 right: `Size(6 bytes)`: size mismatch between ABI and layout in Layout {
    fields: Array {
        stride: Size(2 bytes),
        count: 3,
    variants: Single {
        index: 0,
    },
    abi: Vector {
---
            valid_range: 0..=65535,
        },
        count: 3,
    },
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align(8 bytes),
        pref: Align(8 bytes),
    size: Size(8 bytes),
}', compiler/rustc_middle/src/ty/layout.rs:257:21
stack backtrace:
stack backtrace:
   0:     0x7fa3c44859ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3e179c5dee025621
   1:     0x7fa3c44ebf28 - core::fmt::write::hea82302efebf9141
   2:     0x7fa3c4475841 - std::io::Write::write_fmt::h64b1ad44f66f31ca
   3:     0x7fa3c44889de - std::panicking::default_hook::{{closure}}::h742860da356df9e1
   4:     0x7fa3c448860c - std::panicking::default_hook::h3a601028c8f58c22
   5:     0x7fa3c50071d1 - rustc_driver[a98a92c00c701caf]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa3c448923e - std::panicking::rust_panic_with_hook::h318ebd36736be86a
   7:     0x7fa3c4489037 - std::panicking::begin_panic_handler::{{closure}}::h7898bd1de465703a
   8:     0x7fa3c4485ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4bcdc1450fcc36d1
   9:     0x7fa3c4488d19 - rust_begin_unwind
  10:     0x7fa3c443d073 - core::panicking::panic_fmt::hcb25134d916a2ec6
  11:     0x7fa3c44e856e - core::panicking::assert_failed_inner::h66ffbf09e7dbfcbf
  12:     0x7fa3c4f07deb - core[e7599f4a3d786bf9]::panicking::assert_failed::<rustc_target[8011f39e06f9ca66]::abi::Size, rustc_target[8011f39e06f9ca66]::abi::Size>
  13:     0x7fa3c77b9fbd - rustc_middle[7173a0b96d44f42d]::ty::layout::sanity_check_layout::check_layout_abi
  14:     0x7fa3c77d07e1 - rustc_middle[7173a0b96d44f42d]::ty::context::tls::with_context::<rustc_middle[7173a0b96d44f42d]::ty::context::tls::with_related_context<rustc_middle[7173a0b96d44f42d]::ty::layout::layout_of::{closure#0}, core[e7599f4a3d786bf9]::result::Result<rustc_target[8011f39e06f9ca66]::abi::TyAndLayout<rustc_middle[7173a0b96d44f42d]::ty::Ty>, rustc_middle[7173a0b96d44f42d]::ty::layout::LayoutError>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<rustc_target[8011f39e06f9ca66]::abi::TyAndLayout<rustc_middle[7173a0b96d44f42d]::ty::Ty>, rustc_middle[7173a0b96d44f42d]::ty::layout::LayoutError>>::{closure#0}
  15:     0x7fa3c77d4703 - rustc_middle[7173a0b96d44f42d]::ty::layout::layout_of
  16:     0x7fa3c6abb0e4 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::layout_of, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  17:     0x7fa3c69116d3 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::layout_of
  18:     0x7fa3c55a11fb - <rustc_mir_transform[1ebcd31043024ed2]::const_prop_lint::ConstProp as rustc_mir_transform[1ebcd31043024ed2]::pass_manager::MirLint>::run_lint
  19:     0x7fa3c54f18b2 - rustc_mir_transform[1ebcd31043024ed2]::pass_manager::run_passes
  20:     0x7fa3c55e2c89 - rustc_mir_transform[1ebcd31043024ed2]::run_post_borrowck_cleanup_passes
  21:     0x7fa3c55e2549 - rustc_mir_transform[1ebcd31043024ed2]::mir_drops_elaborated_and_const_checked
  22:     0x7fa3c6995108 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<rustc_middle[7173a0b96d44f42d]::ty::WithOptConstParam<rustc_span[eb4f81bc7a12d0ae]::def_id::LocalDefId>, &rustc_data_structures[87d9e8410fe1bd84]::steal::Steal<rustc_middle[7173a0b96d44f42d]::mir::Body>>>
  23:     0x7fa3c6ab4a33 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  24:     0x7fa3c68e5d97 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  25:     0x7fa3c55e3216 - rustc_mir_transform[1ebcd31043024ed2]::optimized_mir
  26:     0x7fa3c69c9558 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<rustc_span[eb4f81bc7a12d0ae]::def_id::DefId, &rustc_middle[7173a0b96d44f42d]::mir::Body>>
  27:     0x7fa3c6a6c8e2 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::optimized_mir, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  28:     0x7fa3c68e7839 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::optimized_mir
  29:     0x7fa3c7861b4c - <rustc_middle[7173a0b96d44f42d]::ty::context::TyCtxt>::instance_mir
  30:     0x7fa3c542a7e7 - rustc_monomorphize[9fbda94d12f5d850]::collector::collect_neighbours
  31:     0x7fa3c5422f64 - rustc_monomorphize[9fbda94d12f5d850]::collector::collect_items_rec
  32:     0x7fa3c543f1c1 - <rustc_session[72bf6ac283d0085c]::session::Session>::time::<(), rustc_monomorphize[9fbda94d12f5d850]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7fa3c541f9bf - rustc_monomorphize[9fbda94d12f5d850]::collector::collect_crate_mono_items
  34:     0x7fa3c543de49 - rustc_monomorphize[9fbda94d12f5d850]::partitioning::collect_and_partition_mono_items
  35:     0x7fa3c69ebaca - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<(), (&std[798f2a17af60ab34]::collections::hash::set::HashSet<rustc_span[eb4f81bc7a12d0ae]::def_id::DefId, core[e7599f4a3d786bf9]::hash::BuildHasherDefault<rustc_hash[2f727f30a4cebe25]::FxHasher>>, &[rustc_middle[7173a0b96d44f42d]::mir::mono::CodegenUnit])>>
  36:     0x7fa3c6aafd8a - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::collect_and_partition_mono_items, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  37:     0x7fa3c692abd9 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7fa3c5245dc9 - rustc_codegen_ssa[a087ee977d1ca801]::base::codegen_crate::<rustc_codegen_llvm[a4c03861b36eb012]::LlvmCodegenBackend>
  39:     0x7fa3c5293f7b - <rustc_codegen_llvm[a4c03861b36eb012]::LlvmCodegenBackend as rustc_codegen_ssa[a087ee977d1ca801]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7fa3c50f02d1 - <rustc_session[72bf6ac283d0085c]::session::Session>::time::<alloc[9971c807a4bec97f]::boxed::Box<dyn core[e7599f4a3d786bf9]::any::Any>, rustc_interface[6fec859ab01ba9fc]::passes::start_codegen::{closure#0}>
  41:     0x7fa3c50d8873 - <rustc_interface[6fec859ab01ba9fc]::passes::QueryContext>::enter::<<rustc_interface[6fec859ab01ba9fc]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e7599f4a3d786bf9]::result::Result<alloc[9971c807a4bec97f]::boxed::Box<dyn core[e7599f4a3d786bf9]::any::Any>, rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  42:     0x7fa3c50c3f4e - <rustc_interface[6fec859ab01ba9fc]::queries::Queries>::ongoing_codegen
  43:     0x7fa3c4f97b70 - <rustc_interface[6fec859ab01ba9fc]::interface::Compiler>::enter::<rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}::{closure#2}, core[e7599f4a3d786bf9]::result::Result<core[e7599f4a3d786bf9]::option::Option<rustc_interface[6fec859ab01ba9fc]::queries::Linker>, rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  44:     0x7fa3c4f78ceb - rustc_span[eb4f81bc7a12d0ae]::with_source_map::<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_interface[6fec859ab01ba9fc]::interface::create_compiler_and_run<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7fa3c4f98ca9 - <scoped_tls[62b5de94f67e9e6e]::ScopedKey<rustc_span[eb4f81bc7a12d0ae]::SessionGlobals>>::set::<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  46:     0x7fa3c4feefc9 - std[798f2a17af60ab34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  47:     0x7fa3c4f9a1a1 - std[798f2a17af60ab34]::panicking::try::<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, core[e7599f4a3d786bf9]::panic::unwind_safe::AssertUnwindSafe<<std[798f2a17af60ab34]::thread::Builder>::spawn_unchecked_<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  48:     0x7fa3c4fefcf2 - <<std[798f2a17af60ab34]::thread::Builder>::spawn_unchecked_<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#1} as core[e7599f4a3d786bf9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7fa3c4494593 - std::sys::unix::thread::Thread::new::thread_start::h379f8ab2a7d23392
  50:     0x7fa3be9e7609 - start_thread
  51:     0x7fa3c42fa163 - clone
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (0404d6839 2022-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/type-generic-monomorphisation-power-of-two.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-power-of-two/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-power-of-two/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(16 bytes)`,
 right: `Size(12 bytes)`: size mismatch between ABI and layout in Layout {
    fields: Arbitrary {
        offsets: [
            Size(0 bytes),
        memory_index: [
            0,
        ],
    },
---
            valid_range: 0..=4294967295,
        },
        count: 3,
    },
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align(16 bytes),
        pref: Align(16 bytes),
    size: Size(16 bytes),
}', compiler/rustc_middle/src/ty/layout.rs:257:21
stack backtrace:
stack backtrace:
   0:     0x7fcb10a6b9ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3e179c5dee025621
   1:     0x7fcb10ad1f28 - core::fmt::write::hea82302efebf9141
   2:     0x7fcb10a5b841 - std::io::Write::write_fmt::h64b1ad44f66f31ca
   3:     0x7fcb10a6e9de - std::panicking::default_hook::{{closure}}::h742860da356df9e1
   4:     0x7fcb10a6e60c - std::panicking::default_hook::h3a601028c8f58c22
   5:     0x7fcb115ed1d1 - rustc_driver[a98a92c00c701caf]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fcb10a6f23e - std::panicking::rust_panic_with_hook::h318ebd36736be86a
   7:     0x7fcb10a6f037 - std::panicking::begin_panic_handler::{{closure}}::h7898bd1de465703a
   8:     0x7fcb10a6bec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4bcdc1450fcc36d1
   9:     0x7fcb10a6ed19 - rust_begin_unwind
  10:     0x7fcb10a23073 - core::panicking::panic_fmt::hcb25134d916a2ec6
  11:     0x7fcb10ace56e - core::panicking::assert_failed_inner::h66ffbf09e7dbfcbf
  12:     0x7fcb114eddeb - core[e7599f4a3d786bf9]::panicking::assert_failed::<rustc_target[8011f39e06f9ca66]::abi::Size, rustc_target[8011f39e06f9ca66]::abi::Size>
  13:     0x7fcb13d9ffbd - rustc_middle[7173a0b96d44f42d]::ty::layout::sanity_check_layout::check_layout_abi
  14:     0x7fcb13db67e1 - rustc_middle[7173a0b96d44f42d]::ty::context::tls::with_context::<rustc_middle[7173a0b96d44f42d]::ty::context::tls::with_related_context<rustc_middle[7173a0b96d44f42d]::ty::layout::layout_of::{closure#0}, core[e7599f4a3d786bf9]::result::Result<rustc_target[8011f39e06f9ca66]::abi::TyAndLayout<rustc_middle[7173a0b96d44f42d]::ty::Ty>, rustc_middle[7173a0b96d44f42d]::ty::layout::LayoutError>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<rustc_target[8011f39e06f9ca66]::abi::TyAndLayout<rustc_middle[7173a0b96d44f42d]::ty::Ty>, rustc_middle[7173a0b96d44f42d]::ty::layout::LayoutError>>::{closure#0}
  15:     0x7fcb13dba703 - rustc_middle[7173a0b96d44f42d]::ty::layout::layout_of
  16:     0x7fcb130a10e4 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::layout_of, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  17:     0x7fcb12ef76d3 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::layout_of
  18:     0x7fcb11b871fb - <rustc_mir_transform[1ebcd31043024ed2]::const_prop_lint::ConstProp as rustc_mir_transform[1ebcd31043024ed2]::pass_manager::MirLint>::run_lint
  19:     0x7fcb11ad78b2 - rustc_mir_transform[1ebcd31043024ed2]::pass_manager::run_passes
  20:     0x7fcb11bc8c89 - rustc_mir_transform[1ebcd31043024ed2]::run_post_borrowck_cleanup_passes
  21:     0x7fcb11bc8549 - rustc_mir_transform[1ebcd31043024ed2]::mir_drops_elaborated_and_const_checked
  22:     0x7fcb12f7b108 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<rustc_middle[7173a0b96d44f42d]::ty::WithOptConstParam<rustc_span[eb4f81bc7a12d0ae]::def_id::LocalDefId>, &rustc_data_structures[87d9e8410fe1bd84]::steal::Steal<rustc_middle[7173a0b96d44f42d]::mir::Body>>>
  23:     0x7fcb1309aa33 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  24:     0x7fcb12ecbd97 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  25:     0x7fcb11bc9216 - rustc_mir_transform[1ebcd31043024ed2]::optimized_mir
  26:     0x7fcb12faf558 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<rustc_span[eb4f81bc7a12d0ae]::def_id::DefId, &rustc_middle[7173a0b96d44f42d]::mir::Body>>
  27:     0x7fcb130528e2 - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::optimized_mir, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  28:     0x7fcb12ecd839 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::optimized_mir
  29:     0x7fcb13e47b4c - <rustc_middle[7173a0b96d44f42d]::ty::context::TyCtxt>::instance_mir
  30:     0x7fcb11a107e7 - rustc_monomorphize[9fbda94d12f5d850]::collector::collect_neighbours
  31:     0x7fcb11a08f64 - rustc_monomorphize[9fbda94d12f5d850]::collector::collect_items_rec
  32:     0x7fcb11a251c1 - <rustc_session[72bf6ac283d0085c]::session::Session>::time::<(), rustc_monomorphize[9fbda94d12f5d850]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7fcb11a059bf - rustc_monomorphize[9fbda94d12f5d850]::collector::collect_crate_mono_items
  34:     0x7fcb11a23e49 - rustc_monomorphize[9fbda94d12f5d850]::partitioning::collect_and_partition_mono_items
  35:     0x7fcb12fd1aca - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::try_execute_query::<rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt, rustc_query_system[6e66952dbd08d4eb]::query::caches::DefaultCache<(), (&std[798f2a17af60ab34]::collections::hash::set::HashSet<rustc_span[eb4f81bc7a12d0ae]::def_id::DefId, core[e7599f4a3d786bf9]::hash::BuildHasherDefault<rustc_hash[2f727f30a4cebe25]::FxHasher>>, &[rustc_middle[7173a0b96d44f42d]::mir::mono::CodegenUnit])>>
  36:     0x7fcb13095d8a - rustc_query_system[6e66952dbd08d4eb]::query::plumbing::get_query::<rustc_query_impl[c08b18e0bf033907]::queries::collect_and_partition_mono_items, rustc_query_impl[c08b18e0bf033907]::plumbing::QueryCtxt>
  37:     0x7fcb12f10bd9 - <rustc_query_impl[c08b18e0bf033907]::Queries as rustc_middle[7173a0b96d44f42d]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7fcb1182bdc9 - rustc_codegen_ssa[a087ee977d1ca801]::base::codegen_crate::<rustc_codegen_llvm[a4c03861b36eb012]::LlvmCodegenBackend>
  39:     0x7fcb11879f7b - <rustc_codegen_llvm[a4c03861b36eb012]::LlvmCodegenBackend as rustc_codegen_ssa[a087ee977d1ca801]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7fcb116d62d1 - <rustc_session[72bf6ac283d0085c]::session::Session>::time::<alloc[9971c807a4bec97f]::boxed::Box<dyn core[e7599f4a3d786bf9]::any::Any>, rustc_interface[6fec859ab01ba9fc]::passes::start_codegen::{closure#0}>
  41:     0x7fcb116be873 - <rustc_interface[6fec859ab01ba9fc]::passes::QueryContext>::enter::<<rustc_interface[6fec859ab01ba9fc]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e7599f4a3d786bf9]::result::Result<alloc[9971c807a4bec97f]::boxed::Box<dyn core[e7599f4a3d786bf9]::any::Any>, rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  42:     0x7fcb116a9f4e - <rustc_interface[6fec859ab01ba9fc]::queries::Queries>::ongoing_codegen
  43:     0x7fcb1157db70 - <rustc_interface[6fec859ab01ba9fc]::interface::Compiler>::enter::<rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}::{closure#2}, core[e7599f4a3d786bf9]::result::Result<core[e7599f4a3d786bf9]::option::Option<rustc_interface[6fec859ab01ba9fc]::queries::Linker>, rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  44:     0x7fcb1155eceb - rustc_span[eb4f81bc7a12d0ae]::with_source_map::<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_interface[6fec859ab01ba9fc]::interface::create_compiler_and_run<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7fcb1157eca9 - <scoped_tls[62b5de94f67e9e6e]::ScopedKey<rustc_span[eb4f81bc7a12d0ae]::SessionGlobals>>::set::<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  46:     0x7fcb115d4fc9 - std[798f2a17af60ab34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>
  47:     0x7fcb115801a1 - std[798f2a17af60ab34]::panicking::try::<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, core[e7599f4a3d786bf9]::panic::unwind_safe::AssertUnwindSafe<<std[798f2a17af60ab34]::thread::Builder>::spawn_unchecked_<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  48:     0x7fcb115d5cf2 - <<std[798f2a17af60ab34]::thread::Builder>::spawn_unchecked_<rustc_interface[6fec859ab01ba9fc]::util::run_in_thread_pool_with_globals<rustc_interface[6fec859ab01ba9fc]::interface::run_compiler<core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>, rustc_driver[a98a92c00c701caf]::run_compiler::{closure#1}>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#0}, core[e7599f4a3d786bf9]::result::Result<(), rustc_errors[dfdb34d0b768ced0]::ErrorGuaranteed>>::{closure#1} as core[e7599f4a3d786bf9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7fcb10a7a593 - std::sys::unix::thread::Thread::new::thread_start::h379f8ab2a7d23392
  50:     0x7fcb0afcd609 - start_thread
  51:     0x7fcb108e0163 - clone
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (0404d6839 2022-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `Simd<3_usize>`
#0 [layout_of] computing layout of `Simd<3_usize>`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#2 [optimized_mir] optimizing MIR for `main`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------



