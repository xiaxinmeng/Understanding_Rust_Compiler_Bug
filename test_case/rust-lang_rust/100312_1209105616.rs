plain
........................................................................................ 1848/13341
........................................................................................ 1936/13341
.......................i................................................................ 2024/13341
........................................................................................ 2112/13341
............................F..........F.F...FF......................................... 2200/13341
........................................................................................ 2376/13341
........................................................................................ 2464/13341
........................................................................................ 2552/13341
........................................................................................ 2640/13341
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-73298.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-73298.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-73298" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-73298/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<[u8; _] as std::default::Default>, [])`,
 right: `Binder(<[u8; 1] as std::default::Default>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:27:5
stack backtrace:
   0:     0x7fc71a50df9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8be032c13d1a1fb4
   1:     0x7fc71a5749f8 - core::fmt::write::hd39ffdcfa24baa9b
   2:     0x7fc71a4fe5b1 - std::io::Write::write_fmt::h4d70e6116713ca0d
   3:     0x7fc71a510f5e - std::panicking::default_hook::{{closure}}::h9254ffceb515c6fc
   4:     0x7fc71a510c1f - std::panicking::default_hook::h49ef3f212db503e7
   5:     0x7fc71aec8864 - rustc_driver[c0255ac83e4c1fad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc71a511712 - std::panicking::rust_panic_with_hook::ha9463a82d3d96cfd
   7:     0x7fc71a511537 - std::panicking::begin_panic_handler::{{closure}}::hdd83b3b3088b185d
   8:     0x7fc71a50e514 - std::sys_common::backtrace::__rust_end_short_backtrace::h34f288e12f47d353
   9:     0x7fc71a511202 - rust_begin_unwind
  10:     0x7fc71a4c0e13 - core::panicking::panic_fmt::haa22a95a317e8953
  11:     0x7fc71a570e28 - core::panicking::assert_failed_inner::hd180275305e23e71
  12:     0x7fc71ae478b4 - core[ad976174ce31ff9]::panicking::assert_failed::<rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>, rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  13:     0x7fc71d5e7401 - rustc_trait_selection[58e009e87374326d]::traits::codegen::codegen_fulfill_obligation
  14:     0x7fc71cbc6e91 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::codegen_fulfill_obligation, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  15:     0x7fc71c6d83f2 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::codegen_fulfill_obligation
  16:     0x7fc71bba9f8d - rustc_ty_utils[a98a7c6729381d88]::instance::inner_resolve_instance
  17:     0x7fc71bba93f0 - rustc_ty_utils[a98a7c6729381d88]::instance::resolve_instance
  18:     0x7fc71cb979c8 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::resolve_instance, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  19:     0x7fc71c7009ff - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::resolve_instance
  20:     0x7fc71d8d00c4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve_opt_const_arg
  21:     0x7fc71d8cfcc4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve
  22:     0x7fc71b307f1e - <rustc_monomorphize[a6ed58b846fc6559]::collector::MirNeighborCollector as rustc_middle[ecf285994cf87b1c]::mir::visit::Visitor>::visit_terminator
  23:     0x7fc71b313b4d - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_neighbours
  24:     0x7fc71b30ea8a - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  25:     0x7fc71b30efd9 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  26:     0x7fc71b31ad0d - <core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once
  27:     0x7fc71b352ec5 - std[deebe81a6960dd3c]::panicking::try::<(), core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  28:     0x7fc71b32da2e - rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in::<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  29:     0x7fc71b3520eb - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<(), rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}>
  30:     0x7fc71b30b3f7 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items
  31:     0x7fc71b31c87a - rustc_monomorphize[a6ed58b846fc6559]::partitioning::collect_and_partition_mono_items
  32:     0x7fc71caff8d2 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::try_execute_query::<rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt, rustc_query_system[13b902c4efeb0e52]::query::caches::DefaultCache<(), (&std[deebe81a6960dd3c]::collections::hash::set::HashSet<rustc_span[b828b7112f1d25f2]::def_id::DefId, core[ad976174ce31ff9]::hash::BuildHasherDefault<rustc_hash[f08e1d26a6e2cc74]::FxHasher>>, &[rustc_middle[ecf285994cf87b1c]::mir::mono::CodegenUnit])>>
  33:     0x7fc71cbd1dfa - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::collect_and_partition_mono_items, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  34:     0x7fc71c6f7219 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  35:     0x7fc71b1156ed - rustc_codegen_ssa[ed3b08cdcbcc0128]::base::codegen_crate::<rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend>
  36:     0x7fc71b1cc68d - <rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend as rustc_codegen_ssa[ed3b08cdcbcc0128]::traits::backend::CodegenBackend>::codegen_crate
  37:     0x7fc71b03d16b - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_interface[2068bcb7cb6bfb23]::passes::start_codegen::{closure#0}>
  38:     0x7fc71b00232c - <rustc_interface[2068bcb7cb6bfb23]::passes::QueryContext>::enter::<<rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ad976174ce31ff9]::result::Result<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  39:     0x7fc71afe746d - <rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen
  40:     0x7fc71aeca4ef - <rustc_interface[2068bcb7cb6bfb23]::interface::Compiler>::enter::<rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}::{closure#2}, core[ad976174ce31ff9]::result::Result<core[ad976174ce31ff9]::option::Option<rustc_interface[2068bcb7cb6bfb23]::queries::Linker>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  41:     0x7fc71aeba471 - rustc_span[b828b7112f1d25f2]::with_source_map::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7fc71aeccf1a - rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>
  43:     0x7fc71aeb43a2 - <scoped_tls[ff7f2889da7e8bff]::ScopedKey<rustc_span[b828b7112f1d25f2]::SessionGlobals>>::set::<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  44:     0x7fc71aeec0cf - std[deebe81a6960dd3c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  45:     0x7fc71af2384e - std[deebe81a6960dd3c]::panicking::try::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fc71aeedc02 - <<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1} as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fc71a51c9e5 - std::sys::unix::thread::Thread::new::thread_start::h4deef73939f2999d
  48:     0x7fc714a67609 - start_thread
  49:     0x7fc71a37a133 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (d039643f7 2022-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `core::default::Default` fulfills its obligations
#1 [resolve_instance] resolving instance `<[u8; _] as core::default::Default>::default`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-83972.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-83972.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-83972.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83972" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83972/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<FooImpl<{
            { 4 }
        }> as Foo>, [])`,
 right: `Binder(<FooImpl<4> as Foo>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:27:5
stack backtrace:
   0:     0x7fd37ae4bf9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8be032c13d1a1fb4
   1:     0x7fd37aeb29f8 - core::fmt::write::hd39ffdcfa24baa9b
   2:     0x7fd37ae3c5b1 - std::io::Write::write_fmt::h4d70e6116713ca0d
   3:     0x7fd37ae4ef5e - std::panicking::default_hook::{{closure}}::h9254ffceb515c6fc
   4:     0x7fd37ae4ec1f - std::panicking::default_hook::h49ef3f212db503e7
   5:     0x7fd37b806864 - rustc_driver[c0255ac83e4c1fad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd37ae4f712 - std::panicking::rust_panic_with_hook::ha9463a82d3d96cfd
   7:     0x7fd37ae4f537 - std::panicking::begin_panic_handler::{{closure}}::hdd83b3b3088b185d
   8:     0x7fd37ae4c514 - std::sys_common::backtrace::__rust_end_short_backtrace::h34f288e12f47d353
   9:     0x7fd37ae4f202 - rust_begin_unwind
  10:     0x7fd37adfee13 - core::panicking::panic_fmt::haa22a95a317e8953
  11:     0x7fd37aeaee28 - core::panicking::assert_failed_inner::hd180275305e23e71
  12:     0x7fd37b7858b4 - core[ad976174ce31ff9]::panicking::assert_failed::<rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>, rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>>
  13:     0x7fd37df25401 - rustc_trait_selection[58e009e87374326d]::traits::codegen::codegen_fulfill_obligation
  14:     0x7fd37d504e91 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::codegen_fulfill_obligation, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  15:     0x7fd37d0163f2 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::codegen_fulfill_obligation
  16:     0x7fd37c4e7f8d - rustc_ty_utils[a98a7c6729381d88]::instance::inner_resolve_instance
  17:     0x7fd37c4e73f0 - rustc_ty_utils[a98a7c6729381d88]::instance::resolve_instance
  18:     0x7fd37d4d59c8 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::resolve_instance, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  19:     0x7fd37d03e9ff - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::resolve_instance
  20:     0x7fd37e20e0c4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve_opt_const_arg
  21:     0x7fd37e20dcc4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve
  22:     0x7fd37bc45f1e - <rustc_monomorphize[a6ed58b846fc6559]::collector::MirNeighborCollector as rustc_middle[ecf285994cf87b1c]::mir::visit::Visitor>::visit_terminator
  23:     0x7fd37bc51b4d - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_neighbours
  24:     0x7fd37bc4ca8a - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  25:     0x7fd37bc4cfd9 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  26:     0x7fd37bc58d0d - <core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once
  27:     0x7fd37bc90ec5 - std[deebe81a6960dd3c]::panicking::try::<(), core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  28:     0x7fd37bc6ba2e - rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in::<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  29:     0x7fd37bc900eb - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<(), rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}>
  30:     0x7fd37bc493f7 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items
  31:     0x7fd37bc5a87a - rustc_monomorphize[a6ed58b846fc6559]::partitioning::collect_and_partition_mono_items
  32:     0x7fd37d43d8d2 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::try_execute_query::<rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt, rustc_query_system[13b902c4efeb0e52]::query::caches::DefaultCache<(), (&std[deebe81a6960dd3c]::collections::hash::set::HashSet<rustc_span[b828b7112f1d25f2]::def_id::DefId, core[ad976174ce31ff9]::hash::BuildHasherDefault<rustc_hash[f08e1d26a6e2cc74]::FxHasher>>, &[rustc_middle[ecf285994cf87b1c]::mir::mono::CodegenUnit])>>
  33:     0x7fd37d50fdfa - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::collect_and_partition_mono_items, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  34:     0x7fd37d035219 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  35:     0x7fd37ba536ed - rustc_codegen_ssa[ed3b08cdcbcc0128]::base::codegen_crate::<rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend>
  36:     0x7fd37bb0a68d - <rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend as rustc_codegen_ssa[ed3b08cdcbcc0128]::traits::backend::CodegenBackend>::codegen_crate
  37:     0x7fd37b97b16b - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_interface[2068bcb7cb6bfb23]::passes::start_codegen::{closure#0}>
  38:     0x7fd37b94032c - <rustc_interface[2068bcb7cb6bfb23]::passes::QueryContext>::enter::<<rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ad976174ce31ff9]::result::Result<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  39:     0x7fd37b92546d - <rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen
  40:     0x7fd37b8084ef - <rustc_interface[2068bcb7cb6bfb23]::interface::Compiler>::enter::<rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}::{closure#2}, core[ad976174ce31ff9]::result::Result<core[ad976174ce31ff9]::option::Option<rustc_interface[2068bcb7cb6bfb23]::queries::Linker>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  41:     0x7fd37b7f8471 - rustc_span[b828b7112f1d25f2]::with_source_map::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7fd37b80af1a - rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>
  43:     0x7fd37b7f23a2 - <scoped_tls[ff7f2889da7e8bff]::ScopedKey<rustc_span[b828b7112f1d25f2]::SessionGlobals>>::set::<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  44:     0x7fd37b82a0cf - std[deebe81a6960dd3c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  45:     0x7fd37b86184e - std[deebe81a6960dd3c]::panicking::try::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fd37b82bc02 - <<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1} as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fd37ae5a9e5 - std::sys::unix::thread::Thread::new::thread_start::h4deef73939f2999d
  48:     0x7fd3753a5609 - start_thread
  49:     0x7fd37acb8133 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (d039643f7 2022-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `Foo` fulfills its obligations
#1 [resolve_instance] resolving instance `<FooImpl<{
            { 4 }
        }> as Foo>::foo`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-84669.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-84669.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-84669.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-84669" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-84669/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<[u8; _] as std::convert::AsRef<[u8]>>, [])`,
 right: `Binder(<[u8; 3] as std::convert::AsRef<[u8]>>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:27:5
stack backtrace:
   0:     0x7fc719887f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8be032c13d1a1fb4
   1:     0x7fc7198ee9f8 - core::fmt::write::hd39ffdcfa24baa9b
   2:     0x7fc7198785b1 - std::io::Write::write_fmt::h4d70e6116713ca0d
   3:     0x7fc71988af5e - std::panicking::default_hook::{{closure}}::h9254ffceb515c6fc
   4:     0x7fc71988ac1f - std::panicking::default_hook::h49ef3f212db503e7
   5:     0x7fc71a242864 - rustc_driver[c0255ac83e4c1fad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc71988b712 - std::panicking::rust_panic_with_hook::ha9463a82d3d96cfd
   7:     0x7fc71988b537 - std::panicking::begin_panic_handler::{{closure}}::hdd83b3b3088b185d
   8:     0x7fc719888514 - std::sys_common::backtrace::__rust_end_short_backtrace::h34f288e12f47d353
   9:     0x7fc71988b202 - rust_begin_unwind
  10:     0x7fc71983ae13 - core::panicking::panic_fmt::haa22a95a317e8953
  11:     0x7fc7198eae28 - core::panicking::assert_failed_inner::hd180275305e23e71
  12:     0x7fc71a1c18b4 - core[ad976174ce31ff9]::panicking::assert_failed::<rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>, rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>>
  13:     0x7fc71c961401 - rustc_trait_selection[58e009e87374326d]::traits::codegen::codegen_fulfill_obligation
  14:     0x7fc71bf40e91 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::codegen_fulfill_obligation, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  15:     0x7fc71ba523f2 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::codegen_fulfill_obligation
  16:     0x7fc71af23f8d - rustc_ty_utils[a98a7c6729381d88]::instance::inner_resolve_instance
  17:     0x7fc71af233f0 - rustc_ty_utils[a98a7c6729381d88]::instance::resolve_instance
  18:     0x7fc71bf119c8 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::resolve_instance, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  19:     0x7fc71ba7a9ff - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::resolve_instance
  20:     0x7fc71cc4a0c4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve_opt_const_arg
  21:     0x7fc71cc49cc4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve
  22:     0x7fc71a681f1e - <rustc_monomorphize[a6ed58b846fc6559]::collector::MirNeighborCollector as rustc_middle[ecf285994cf87b1c]::mir::visit::Visitor>::visit_terminator
  23:     0x7fc71a68db4d - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_neighbours
  24:     0x7fc71a688a8a - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  25:     0x7fc71a688fd9 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  26:     0x7fc71a694d0d - <core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once
  27:     0x7fc71a6ccec5 - std[deebe81a6960dd3c]::panicking::try::<(), core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  28:     0x7fc71a6a7a2e - rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in::<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  29:     0x7fc71a6cc0eb - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<(), rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}>
  30:     0x7fc71a6853f7 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items
  31:     0x7fc71a69687a - rustc_monomorphize[a6ed58b846fc6559]::partitioning::collect_and_partition_mono_items
  32:     0x7fc71be798d2 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::try_execute_query::<rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt, rustc_query_system[13b902c4efeb0e52]::query::caches::DefaultCache<(), (&std[deebe81a6960dd3c]::collections::hash::set::HashSet<rustc_span[b828b7112f1d25f2]::def_id::DefId, core[ad976174ce31ff9]::hash::BuildHasherDefault<rustc_hash[f08e1d26a6e2cc74]::FxHasher>>, &[rustc_middle[ecf285994cf87b1c]::mir::mono::CodegenUnit])>>
  33:     0x7fc71bf4bdfa - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::collect_and_partition_mono_items, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  34:     0x7fc71ba71219 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  35:     0x7fc71a48f6ed - rustc_codegen_ssa[ed3b08cdcbcc0128]::base::codegen_crate::<rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend>
  36:     0x7fc71a54668d - <rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend as rustc_codegen_ssa[ed3b08cdcbcc0128]::traits::backend::CodegenBackend>::codegen_crate
  37:     0x7fc71a3b716b - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_interface[2068bcb7cb6bfb23]::passes::start_codegen::{closure#0}>
  38:     0x7fc71a37c32c - <rustc_interface[2068bcb7cb6bfb23]::passes::QueryContext>::enter::<<rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ad976174ce31ff9]::result::Result<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  39:     0x7fc71a36146d - <rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen
  40:     0x7fc71a2444ef - <rustc_interface[2068bcb7cb6bfb23]::interface::Compiler>::enter::<rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}::{closure#2}, core[ad976174ce31ff9]::result::Result<core[ad976174ce31ff9]::option::Option<rustc_interface[2068bcb7cb6bfb23]::queries::Linker>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  41:     0x7fc71a234471 - rustc_span[b828b7112f1d25f2]::with_source_map::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7fc71a246f1a - rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>
  43:     0x7fc71a22e3a2 - <scoped_tls[ff7f2889da7e8bff]::ScopedKey<rustc_span[b828b7112f1d25f2]::SessionGlobals>>::set::<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  44:     0x7fc71a2660cf - std[deebe81a6960dd3c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  45:     0x7fc71a29d84e - std[deebe81a6960dd3c]::panicking::try::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fc71a267c02 - <<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1} as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fc7198969e5 - std::sys::unix::thread::Thread::new::thread_start::h4deef73939f2999d
  48:     0x7fc713de1609 - start_thread
  49:     0x7fc7196f4133 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (d039643f7 2022-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `core::convert::AsRef` fulfills its obligations
#1 [resolve_instance] resolving instance `<[u8; _] as core::convert::AsRef<[u8]>>::as_ref`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-82268.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-82268.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-82268.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-82268" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-82268/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<(i32, (char, ())) as Collate<CollateOpImpl<{ MASK >> 1 }>>>, [])`,
 right: `Binder(<(i32, (char, ())) as Collate<CollateOpImpl<2>>>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:27:5
stack backtrace:
   0:     0x7f2d45091f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8be032c13d1a1fb4
   1:     0x7f2d450f89f8 - core::fmt::write::hd39ffdcfa24baa9b
   2:     0x7f2d450825b1 - std::io::Write::write_fmt::h4d70e6116713ca0d
   3:     0x7f2d45094f5e - std::panicking::default_hook::{{closure}}::h9254ffceb515c6fc
   4:     0x7f2d45094c1f - std::panicking::default_hook::h49ef3f212db503e7
   5:     0x7f2d45a4c864 - rustc_driver[c0255ac83e4c1fad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2d45095712 - std::panicking::rust_panic_with_hook::ha9463a82d3d96cfd
   7:     0x7f2d45095537 - std::panicking::begin_panic_handler::{{closure}}::hdd83b3b3088b185d
   8:     0x7f2d45092514 - std::sys_common::backtrace::__rust_end_short_backtrace::h34f288e12f47d353
   9:     0x7f2d45095202 - rust_begin_unwind
  10:     0x7f2d45044e13 - core::panicking::panic_fmt::haa22a95a317e8953
  11:     0x7f2d450f4e28 - core::panicking::assert_failed_inner::hd180275305e23e71
  12:     0x7f2d459cb8b4 - core[ad976174ce31ff9]::panicking::assert_failed::<rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>, rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>>
  13:     0x7f2d4816b401 - rustc_trait_selection[58e009e87374326d]::traits::codegen::codegen_fulfill_obligation
  14:     0x7f2d4774ae91 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::codegen_fulfill_obligation, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  15:     0x7f2d4725c3f2 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::codegen_fulfill_obligation
  16:     0x7f2d4672df8d - rustc_ty_utils[a98a7c6729381d88]::instance::inner_resolve_instance
  17:     0x7f2d4672d3f0 - rustc_ty_utils[a98a7c6729381d88]::instance::resolve_instance
  18:     0x7f2d4771b9c8 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::resolve_instance, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  19:     0x7f2d472849ff - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::resolve_instance
  20:     0x7f2d484540c4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve_opt_const_arg
  21:     0x7f2d48453cc4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve
  22:     0x7f2d45e8bf1e - <rustc_monomorphize[a6ed58b846fc6559]::collector::MirNeighborCollector as rustc_middle[ecf285994cf87b1c]::mir::visit::Visitor>::visit_terminator
  23:     0x7f2d45e97b4d - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_neighbours
  24:     0x7f2d45e92a8a - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  25:     0x7f2d45e92fd9 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  26:     0x7f2d45e92fd9 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  27:     0x7f2d45e9ed0d - <core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once
  28:     0x7f2d45ed6ec5 - std[deebe81a6960dd3c]::panicking::try::<(), core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f2d45eb1a2e - rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in::<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  30:     0x7f2d45ed60eb - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<(), rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}>
  31:     0x7f2d45e8f3f7 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items
  32:     0x7f2d45ea087a - rustc_monomorphize[a6ed58b846fc6559]::partitioning::collect_and_partition_mono_items
  33:     0x7f2d476838d2 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::try_execute_query::<rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt, rustc_query_system[13b902c4efeb0e52]::query::caches::DefaultCache<(), (&std[deebe81a6960dd3c]::collections::hash::set::HashSet<rustc_span[b828b7112f1d25f2]::def_id::DefId, core[ad976174ce31ff9]::hash::BuildHasherDefault<rustc_hash[f08e1d26a6e2cc74]::FxHasher>>, &[rustc_middle[ecf285994cf87b1c]::mir::mono::CodegenUnit])>>
  34:     0x7f2d47755dfa - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::collect_and_partition_mono_items, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  35:     0x7f2d4727b219 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  36:     0x7f2d45c996ed - rustc_codegen_ssa[ed3b08cdcbcc0128]::base::codegen_crate::<rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend>
  37:     0x7f2d45d5068d - <rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend as rustc_codegen_ssa[ed3b08cdcbcc0128]::traits::backend::CodegenBackend>::codegen_crate
  38:     0x7f2d45bc116b - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_interface[2068bcb7cb6bfb23]::passes::start_codegen::{closure#0}>
  39:     0x7f2d45b8632c - <rustc_interface[2068bcb7cb6bfb23]::passes::QueryContext>::enter::<<rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ad976174ce31ff9]::result::Result<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  40:     0x7f2d45b6b46d - <rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen
  41:     0x7f2d45a4e4ef - <rustc_interface[2068bcb7cb6bfb23]::interface::Compiler>::enter::<rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}::{closure#2}, core[ad976174ce31ff9]::result::Result<core[ad976174ce31ff9]::option::Option<rustc_interface[2068bcb7cb6bfb23]::queries::Linker>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  42:     0x7f2d45a3e471 - rustc_span[b828b7112f1d25f2]::with_source_map::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7f2d45a50f1a - rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>
  44:     0x7f2d45a383a2 - <scoped_tls[ff7f2889da7e8bff]::ScopedKey<rustc_span[b828b7112f1d25f2]::SessionGlobals>>::set::<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  45:     0x7f2d45a700cf - std[deebe81a6960dd3c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  46:     0x7f2d45aa784e - std[deebe81a6960dd3c]::panicking::try::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  47:     0x7f2d45a71c02 - <<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1} as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f2d450a09e5 - std::sys::unix::thread::Thread::new::thread_start::h4deef73939f2999d
  49:     0x7f2d3f5eb609 - start_thread
  50:     0x7f2d44efe133 - clone
  51:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (d039643f7 2022-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `Collate` fulfills its obligations
#1 [resolve_instance] resolving instance `<(i32, (char, ())) as Collate<CollateOpImpl<{ MASK >> 1 }>>>::collate`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-86710.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-86710.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-86710.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-86710" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-86710/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<BarImpl<{ N + M }> as Bar>, [])`,
 right: `Binder(<BarImpl<2> as Bar>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:27:5
stack backtrace:
   0:     0x7fa17c588f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8be032c13d1a1fb4
   1:     0x7fa17c5ef9f8 - core::fmt::write::hd39ffdcfa24baa9b
   2:     0x7fa17c5795b1 - std::io::Write::write_fmt::h4d70e6116713ca0d
   3:     0x7fa17c58bf5e - std::panicking::default_hook::{{closure}}::h9254ffceb515c6fc
   4:     0x7fa17c58bc1f - std::panicking::default_hook::h49ef3f212db503e7
   5:     0x7fa17cf43864 - rustc_driver[c0255ac83e4c1fad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa17c58c712 - std::panicking::rust_panic_with_hook::ha9463a82d3d96cfd
   7:     0x7fa17c58c537 - std::panicking::begin_panic_handler::{{closure}}::hdd83b3b3088b185d
   8:     0x7fa17c589514 - std::sys_common::backtrace::__rust_end_short_backtrace::h34f288e12f47d353
   9:     0x7fa17c58c202 - rust_begin_unwind
  10:     0x7fa17c53be13 - core::panicking::panic_fmt::haa22a95a317e8953
  11:     0x7fa17c5ebe28 - core::panicking::assert_failed_inner::hd180275305e23e71
  12:     0x7fa17cec28b4 - core[ad976174ce31ff9]::panicking::assert_failed::<rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>, rustc_middle[ecf285994cf87b1c]::ty::sty::Binder<rustc_middle[ecf285994cf87b1c]::ty::sty::TraitRef>>
  13:     0x7fa17f662401 - rustc_trait_selection[58e009e87374326d]::traits::codegen::codegen_fulfill_obligation
  14:     0x7fa17ec41e91 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::codegen_fulfill_obligation, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  15:     0x7fa17e7533f2 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::codegen_fulfill_obligation
  16:     0x7fa17dc24f8d - rustc_ty_utils[a98a7c6729381d88]::instance::inner_resolve_instance
  17:     0x7fa17dc243f0 - rustc_ty_utils[a98a7c6729381d88]::instance::resolve_instance
  18:     0x7fa17ec129c8 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::resolve_instance, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  19:     0x7fa17e77b9ff - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::resolve_instance
  20:     0x7fa17f94b0c4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve_opt_const_arg
  21:     0x7fa17f94acc4 - <rustc_middle[ecf285994cf87b1c]::ty::instance::Instance>::resolve
  22:     0x7fa17d382f1e - <rustc_monomorphize[a6ed58b846fc6559]::collector::MirNeighborCollector as rustc_middle[ecf285994cf87b1c]::mir::visit::Visitor>::visit_terminator
  23:     0x7fa17d38eb4d - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_neighbours
  24:     0x7fa17d389a8a - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  25:     0x7fa17d389fd9 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_items_rec
  26:     0x7fa17d395d0d - <core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once
  27:     0x7fa17d3cdec5 - std[deebe81a6960dd3c]::panicking::try::<(), core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  28:     0x7fa17d3a8a2e - rustc_data_structures[807f4c38b5ae8e00]::sync::par_for_each_in::<alloc[7987977c070a7f21]::vec::Vec<rustc_middle[ecf285994cf87b1c]::mir::mono::MonoItem>, rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  29:     0x7fa17d3cd0eb - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<(), rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items::{closure#1}>
  30:     0x7fa17d3863f7 - rustc_monomorphize[a6ed58b846fc6559]::collector::collect_crate_mono_items
  31:     0x7fa17d39787a - rustc_monomorphize[a6ed58b846fc6559]::partitioning::collect_and_partition_mono_items
  32:     0x7fa17eb7a8d2 - rustc_query_system[13b902c4efeb0e52]::query::plumbing::try_execute_query::<rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt, rustc_query_system[13b902c4efeb0e52]::query::caches::DefaultCache<(), (&std[deebe81a6960dd3c]::collections::hash::set::HashSet<rustc_span[b828b7112f1d25f2]::def_id::DefId, core[ad976174ce31ff9]::hash::BuildHasherDefault<rustc_hash[f08e1d26a6e2cc74]::FxHasher>>, &[rustc_middle[ecf285994cf87b1c]::mir::mono::CodegenUnit])>>
  33:     0x7fa17ec4cdfa - rustc_query_system[13b902c4efeb0e52]::query::plumbing::get_query::<rustc_query_impl[dcb8d6f4716e4abb]::queries::collect_and_partition_mono_items, rustc_query_impl[dcb8d6f4716e4abb]::plumbing::QueryCtxt>
  34:     0x7fa17e772219 - <rustc_query_impl[dcb8d6f4716e4abb]::Queries as rustc_middle[ecf285994cf87b1c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  35:     0x7fa17d1906ed - rustc_codegen_ssa[ed3b08cdcbcc0128]::base::codegen_crate::<rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend>
  36:     0x7fa17d24768d - <rustc_codegen_llvm[67694754f7026662]::LlvmCodegenBackend as rustc_codegen_ssa[ed3b08cdcbcc0128]::traits::backend::CodegenBackend>::codegen_crate
  37:     0x7fa17d0b816b - <rustc_session[3ba4c244d18b8cb8]::session::Session>::time::<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_interface[2068bcb7cb6bfb23]::passes::start_codegen::{closure#0}>
  38:     0x7fa17d07d32c - <rustc_interface[2068bcb7cb6bfb23]::passes::QueryContext>::enter::<<rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ad976174ce31ff9]::result::Result<alloc[7987977c070a7f21]::boxed::Box<dyn core[ad976174ce31ff9]::any::Any>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  39:     0x7fa17d06246d - <rustc_interface[2068bcb7cb6bfb23]::queries::Queries>::ongoing_codegen
  40:     0x7fa17cf454ef - <rustc_interface[2068bcb7cb6bfb23]::interface::Compiler>::enter::<rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}::{closure#2}, core[ad976174ce31ff9]::result::Result<core[ad976174ce31ff9]::option::Option<rustc_interface[2068bcb7cb6bfb23]::queries::Linker>, rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  41:     0x7fa17cf35471 - rustc_span[b828b7112f1d25f2]::with_source_map::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7fa17cf47f1a - rustc_interface[2068bcb7cb6bfb23]::interface::create_compiler_and_run::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>
  43:     0x7fa17cf2f3a2 - <scoped_tls[ff7f2889da7e8bff]::ScopedKey<rustc_span[b828b7112f1d25f2]::SessionGlobals>>::set::<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  44:     0x7fa17cf670cf - std[deebe81a6960dd3c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>
  45:     0x7fa17cf9e84e - std[deebe81a6960dd3c]::panicking::try::<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, core[ad976174ce31ff9]::panic::unwind_safe::AssertUnwindSafe<<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fa17cf68c02 - <<std[deebe81a6960dd3c]::thread::Builder>::spawn_unchecked_<rustc_interface[2068bcb7cb6bfb23]::util::run_in_thread_pool_with_globals<rustc_interface[2068bcb7cb6bfb23]::interface::run_compiler<core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>, rustc_driver[c0255ac83e4c1fad]::run_compiler::{closure#1}>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#0}, core[ad976174ce31ff9]::result::Result<(), rustc_errors[9e39027d1e9da959]::ErrorGuaranteed>>::{closure#1} as core[ad976174ce31ff9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fa17c5979e5 - std::sys::unix::thread::Thread::new::thread_start::h4deef73939f2999d
  48:     0x7fa176ae2609 - start_thread
  49:     0x7fa17c3f5133 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (d039643f7 2022-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `Bar` fulfills its obligations
#1 [resolve_instance] resolving instance `<BarImpl<{ N + M }> as Bar>::error_occurs_here`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------



