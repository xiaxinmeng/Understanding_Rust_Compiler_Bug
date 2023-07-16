plain
failures:

---- [ui] src/test/ui/object-lifetime/object-lifetime-default.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default/auxiliary"
stdout: none
--- stderr -------------------------------
error: BaseDefault
   |
   |
LL | struct A<T>(T); //~ ERROR BaseDefault

thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `LifetimeParam`,
 right: `TyParam`', compiler/rustc_resolve/src/late/lifetimes.rs:1155:5
stack backtrace:
   0:     0x7fa7de71c12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h11d7c6353afb93cb
   1:     0x7fa7de784ca8 - core::fmt::write::h1663b98a0fbcb0ea
   2:     0x7fa7de70c901 - std::io::Write::write_fmt::h366715c6ae4bd989
   3:     0x7fa7de71f11e - std::panicking::default_hook::{{closure}}::hab6ef6af9445752a
   4:     0x7fa7de71ede7 - std::panicking::default_hook::h2e75a0e1d0d8b26f
   5:     0x7fa7df0aa6f4 - rustc_driver[519882be0d921c5d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa7de71f8d1 - std::panicking::rust_panic_with_hook::h1d3285a9b5d7129e
   7:     0x7fa7de71f6f7 - std::panicking::begin_panic_handler::{{closure}}::hb3d94baa55576238
   8:     0x7fa7de71c6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3b3fda597f11907a
   9:     0x7fa7de71f3c2 - rust_begin_unwind
  10:     0x7fa7de6cfe43 - core::panicking::panic_fmt::h1c04ef22d02030e0
  11:     0x7fa7de781578 - core::panicking::assert_failed_inner::h7865700104c06626
  12:     0x7fa7def33beb - core[29d90b0ddf2f6d61]::panicking::assert_failed::<rustc_hir[5378adb4fcec988e]::def::DefKind, rustc_hir[5378adb4fcec988e]::def::DefKind>
  13:     0x7fa7dfef1634 - rustc_resolve[76706de4773c99f]::late::lifetimes::object_lifetime_default
  14:     0x7fa7e0bef5ee - rustc_query_system[ef9a5177632894b5]::query::plumbing::try_execute_query::<rustc_query_impl[b06e9040643c7421]::plumbing::QueryCtxt, rustc_query_system[ef9a5177632894b5]::query::caches::DefaultCache<rustc_span[b02b2e6ce352d014]::def_id::DefId, core[29d90b0ddf2f6d61]::option::Option<rustc_middle[624ff6496f5ef055]::middle::resolve_lifetime::ObjectLifetimeDefault>>>
  15:     0x7fa7e0ce805e - rustc_query_system[ef9a5177632894b5]::query::plumbing::get_query::<rustc_query_impl[b06e9040643c7421]::queries::object_lifetime_default, rustc_query_impl[b06e9040643c7421]::plumbing::QueryCtxt>
  16:     0x7fa7e0b38ac9 - <rustc_query_impl[b06e9040643c7421]::Queries as rustc_middle[624ff6496f5ef055]::ty::query::QueryEngine>::object_lifetime_default
  17:     0x7fa7dff8ff38 - <&mut <rustc_passes[ec5bf76ea9767c99]::check_attr::CheckAttrVisitor>::check_object_lifetime_default::{closure#0} as core[29d90b0ddf2f6d61]::ops::function::FnMut<(&rustc_hir[5378adb4fcec988e]::hir::GenericParam,)>>::call_mut
  18:     0x7fa7dffbdefe - <alloc[3129b6f781c5556b]::vec::Vec<alloc[3129b6f781c5556b]::string::String> as alloc[3129b6f781c5556b]::vec::spec_from_iter::SpecFromIter<alloc[3129b6f781c5556b]::string::String, core[29d90b0ddf2f6d61]::iter::adapters::filter_map::FilterMap<core[29d90b0ddf2f6d61]::slice::iter::Iter<rustc_hir[5378adb4fcec988e]::hir::GenericParam>, <rustc_passes[ec5bf76ea9767c99]::check_attr::CheckAttrVisitor>::check_object_lifetime_default::{closure#0}>>>::from_iter
  19:     0x7fa7dff9358e - <rustc_passes[ec5bf76ea9767c99]::check_attr::CheckAttrVisitor>::check_attributes
  20:     0x7fa7dff96a0f - <rustc_passes[ec5bf76ea9767c99]::check_attr::CheckAttrVisitor as rustc_hir[5378adb4fcec988e]::intravisit::Visitor>::visit_item
  21:     0x7fa7dff70bc0 - <rustc_middle[624ff6496f5ef055]::hir::map::Map>::visit_item_likes_in_module::<rustc_passes[ec5bf76ea9767c99]::check_attr::CheckAttrVisitor>
  22:     0x7fa7dff96e4d - rustc_passes[ec5bf76ea9767c99]::check_attr::check_mod_attrs
  23:     0x7fa7e0be3f79 - rustc_query_system[ef9a5177632894b5]::query::plumbing::try_execute_query::<rustc_query_impl[b06e9040643c7421]::plumbing::QueryCtxt, rustc_query_system[ef9a5177632894b5]::query::caches::DefaultCache<rustc_span[b02b2e6ce352d014]::def_id::LocalDefId, ()>>
  24:     0x7fa7e0cb9236 - rustc_query_system[ef9a5177632894b5]::query::plumbing::get_query::<rustc_query_impl[b06e9040643c7421]::queries::check_mod_attrs, rustc_query_impl[b06e9040643c7421]::plumbing::QueryCtxt>
  25:     0x7fa7e0b0f501 - <rustc_query_impl[b06e9040643c7421]::Queries as rustc_middle[624ff6496f5ef055]::ty::query::QueryEngine>::check_mod_attrs
  26:     0x7fa7df246074 - <core[29d90b0ddf2f6d61]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[ac90fdc15110a252]::sync::par_for_each_in<&[rustc_span[b02b2e6ce352d014]::def_id::LocalDefId], <rustc_middle[624ff6496f5ef055]::hir::map::Map>::par_for_each_module<rustc_interface[f3abaeafc22365db]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[29d90b0ddf2f6d61]::ops::function::FnOnce<()>>::call_once
  27:     0x7fa7df1c7149 - std[93d296517b615082]::panic::catch_unwind::<core[29d90b0ddf2f6d61]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[ac90fdc15110a252]::sync::par_for_each_in<&[rustc_span[b02b2e6ce352d014]::def_id::LocalDefId], <rustc_middle[624ff6496f5ef055]::hir::map::Map>::par_for_each_module<rustc_interface[f3abaeafc22365db]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  28:     0x7fa7df1b3edd - rustc_data_structures[ac90fdc15110a252]::sync::par_for_each_in::<&[rustc_span[b02b2e6ce352d014]::def_id::LocalDefId], <rustc_middle[624ff6496f5ef055]::hir::map::Map>::par_for_each_module<rustc_interface[f3abaeafc22365db]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  29:     0x7fa7df24730c - <core[29d90b0ddf2f6d61]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f3abaeafc22365db]::passes::analysis::{closure#0}::{closure#1}> as core[29d90b0ddf2f6d61]::ops::function::FnOnce<()>>::call_once
  30:     0x7fa7df1c7276 - std[93d296517b615082]::panic::catch_unwind::<core[29d90b0ddf2f6d61]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f3abaeafc22365db]::passes::analysis::{closure#0}::{closure#1}>, ()>
  31:     0x7fa7df2394c8 - <rustc_session[6c73152c60147261]::session::Session>::time::<(), rustc_interface[f3abaeafc22365db]::passes::analysis::{closure#0}>
  32:     0x7fa7df201116 - rustc_interface[f3abaeafc22365db]::passes::analysis
  33:     0x7fa7e0c24334 - rustc_query_system[ef9a5177632894b5]::query::plumbing::try_execute_query::<rustc_query_impl[b06e9040643c7421]::plumbing::QueryCtxt, rustc_query_system[ef9a5177632894b5]::query::caches::DefaultCache<(), core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>>
  34:     0x7fa7e0d06b44 - rustc_query_system[ef9a5177632894b5]::query::plumbing::get_query::<rustc_query_impl[b06e9040643c7421]::queries::analysis, rustc_query_impl[b06e9040643c7421]::plumbing::QueryCtxt>
  35:     0x7fa7e0af82ed - <rustc_query_impl[b06e9040643c7421]::Queries as rustc_middle[624ff6496f5ef055]::ty::query::QueryEngine>::analysis
  36:     0x7fa7df1162fd - <rustc_interface[f3abaeafc22365db]::passes::QueryContext>::enter::<rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>
  37:     0x7fa7df0b2881 - <rustc_interface[f3abaeafc22365db]::interface::Compiler>::enter::<rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}::{closure#2}, core[29d90b0ddf2f6d61]::result::Result<core[29d90b0ddf2f6d61]::option::Option<rustc_interface[f3abaeafc22365db]::queries::Linker>, rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>
  38:     0x7fa7df095005 - rustc_span[b02b2e6ce352d014]::with_source_map::<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_interface[f3abaeafc22365db]::interface::create_compiler_and_run<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}>::{closure#1}>
  39:     0x7fa7df0cdb21 - rustc_interface[f3abaeafc22365db]::interface::create_compiler_and_run::<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}>
  40:     0x7fa7df096c32 - <scoped_tls[a121d0197b834095]::ScopedKey<rustc_span[b02b2e6ce352d014]::SessionGlobals>>::set::<rustc_interface[f3abaeafc22365db]::interface::run_compiler<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>
  41:     0x7fa7df10d0d9 - std[93d296517b615082]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f3abaeafc22365db]::util::run_in_thread_pool_with_globals<rustc_interface[f3abaeafc22365db]::interface::run_compiler<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>
  42:     0x7fa7df09945e - std[93d296517b615082]::panicking::try::<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, core[29d90b0ddf2f6d61]::panic::unwind_safe::AssertUnwindSafe<<std[93d296517b615082]::thread::Builder>::spawn_unchecked_<rustc_interface[f3abaeafc22365db]::util::run_in_thread_pool_with_globals<rustc_interface[f3abaeafc22365db]::interface::run_compiler<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7fa7df111700 - <<std[93d296517b615082]::thread::Builder>::spawn_unchecked_<rustc_interface[f3abaeafc22365db]::util::run_in_thread_pool_with_globals<rustc_interface[f3abaeafc22365db]::interface::run_compiler<core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>, rustc_driver[519882be0d921c5d]::run_compiler::{closure#1}>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>::{closure#0}, core[29d90b0ddf2f6d61]::result::Result<(), rustc_errors[47f1fb2a06e805f0]::ErrorGuaranteed>>::{closure#1} as core[29d90b0ddf2f6d61]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7fa7de72c645 - std::sys::unix::thread::Thread::new::thread_start::hbe9c52487bdd8c1d
  45:     0x7fa7de4c8b43 - <unknown>
  46:     0x7fa7de55aa00 - <unknown>
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (dc147157f 2022-08-30) running on x86_64-unknown-linux-gnu

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [object_lifetime_default] looking up lifetime defaults for generic parameter `B::'a`
#1 [check_mod_attrs] checking attributes in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


