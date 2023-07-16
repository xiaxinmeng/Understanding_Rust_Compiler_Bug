plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1042 tests
---
failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0094 (line 1978) stdout ----
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Unsafe`,
 right: `Normal`: Intrinsic safety mismatch between list of intrinsics within the compiler and core library intrinsics for intrinsic `size_of`', compiler/rustc_typeck/src/check/intrinsic.rs:118:5
stack backtrace:
   0:     0x7f473194a79e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf59a7b4e2d434620
   1:     0x7f47319b3948 - core::fmt::write::ha2091e91361753a7
   2:     0x7f473193b0a1 - std::io::Write::write_fmt::h5295384f2cf988e3
   3:     0x7f473194d7ae - std::panicking::default_hook::{{closure}}::h1a4a7c656812fb23
   4:     0x7f473194d477 - std::panicking::default_hook::h375a8262cbf87d78
   5:     0x7f4732312174 - rustc_driver[8a450591bc686d17]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f473194df61 - std::panicking::rust_panic_with_hook::h4f679a357a9ccd79
   7:     0x7f473194dd87 - std::panicking::begin_panic_handler::{{closure}}::h4e87b160fb35c7c3
   8:     0x7f473194acd4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb01360a6cd9f56db
   9:     0x7f473194da52 - rust_begin_unwind
  10:     0x7f47318fcf13 - core::panicking::panic_fmt::h8700365ed7eef2a2
  11:     0x7f47319b030e - core::panicking::assert_failed_inner::h37448dc7036e8bb4
  12:     0x7f473217594b - core[2e31ec2f599a4931]::panicking::assert_failed::<rustc_hir[a09fe2c49dca5e34]::hir::Unsafety, rustc_hir[a09fe2c49dca5e34]::hir::Unsafety>
  13:     0x7f4732e602e2 - rustc_typeck[9d52801361d57d]::check::intrinsic::intrinsic_operation_unsafety
  14:     0x7f4732d18554 - rustc_typeck[9d52801361d57d]::collect::fn_sig
  15:     0x7f4733f0130e - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<rustc_span[d12d2727cd9c9f44]::def_id::DefId, rustc_middle[ef618a27e98a180]::ty::sty::Binder<rustc_middle[ef618a27e98a180]::ty::sty::FnSig>>>
  16:     0x7f473400cbf3 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::fn_sig, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  17:     0x7f4733e0a9ac - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::fn_sig
  18:     0x7f4732d0f3e6 - rustc_typeck[9d52801361d57d]::collect::convert_item
  19:     0x7f4732d0af1a - <rustc_typeck[9d52801361d57d]::collect::CollectItemTypesVisitor as rustc_hir[a09fe2c49dca5e34]::intravisit::Visitor>::visit_item
  20:     0x7f4732d32180 - <rustc_middle[ef618a27e98a180]::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck[9d52801361d57d]::collect::CollectItemTypesVisitor>
  21:     0x7f4732d0a4ad - rustc_typeck[9d52801361d57d]::collect::collect_mod_item_types
  22:     0x7f4733eedfe0 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<rustc_span[d12d2727cd9c9f44]::def_id::LocalDefId, ()>>
  23:     0x7f4733fe11ef - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::collect_mod_item_types, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  24:     0x7f4733e11110 - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::collect_mod_item_types
  25:     0x7f4732d317da - <rustc_middle[ef618a27e98a180]::hir::map::Map>::for_each_module::<rustc_typeck[9d52801361d57d]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  26:     0x7f4732d7da2b - <rustc_session[492dfb63ec0bb8ae]::session::Session>::track_errors::<rustc_typeck[9d52801361d57d]::check_crate::{closure#0}, ()>
  27:     0x7f4732fd6119 - rustc_typeck[9d52801361d57d]::check_crate
  28:     0x7f4732450631 - rustc_interface[53f1f933a2d2ec9]::passes::analysis
  29:     0x7f4733f2d16f - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<(), core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>>
  30:     0x7f473400d2a0 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::analysis, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  31:     0x7f4733dead9a - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::analysis
  32:     0x7f47323764f6 - <rustc_interface[53f1f933a2d2ec9]::passes::QueryContext>::enter::<rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  33:     0x7f4732329751 - rustc_interface[53f1f933a2d2ec9]::interface::create_compiler_and_run::<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>
  34:     0x7f47322fd742 - <scoped_tls[537ffb9e41f61595]::ScopedKey<rustc_span[d12d2727cd9c9f44]::SessionGlobals>>::set::<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  35:     0x7f473237a6cf - std[2e4d6ca4d7177998]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  36:     0x7f473232838e - std[2e4d6ca4d7177998]::panic::catch_unwind::<core[2e31ec2f599a4931]::panic::unwind_safe::AssertUnwindSafe<<std[2e4d6ca4d7177998]::thread::Builder>::spawn_unchecked_<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  37:     0x7f473237db52 - <<std[2e4d6ca4d7177998]::thread::Builder>::spawn_unchecked_<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#1} as core[2e31ec2f599a4931]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f473195adc5 - std::sys::unix::thread::Thread::new::thread_start::hbd1073ee1be91ca5
  39:     0x7f47316f6b43 - <unknown>
  40:     0x7f4731788a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4bad79f8f 2022-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -Z normalize-docs -Z unstable-options
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `main::_doctest_main__checkout_obj_build_x86_64_unknown_linux_gnu_test_error_index_md_1978_0::size_of`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
Some expected error codes were not found: ["E0094"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0094 (line 1991) stdout ----
  left: `Unsafe`,
  left: `Unsafe`,
 right: `Normal`: Intrinsic safety mismatch between list of intrinsics within the compiler and core library intrinsics for intrinsic `size_of`', compiler/rustc_typeck/src/check/intrinsic.rs:118:5
stack backtrace:
   0:     0x7fae539ce79e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf59a7b4e2d434620
   1:     0x7fae53a37948 - core::fmt::write::ha2091e91361753a7
   2:     0x7fae539bf0a1 - std::io::Write::write_fmt::h5295384f2cf988e3
   3:     0x7fae539d17ae - std::panicking::default_hook::{{closure}}::h1a4a7c656812fb23
   4:     0x7fae539d1477 - std::panicking::default_hook::h375a8262cbf87d78
   5:     0x7fae54396174 - rustc_driver[8a450591bc686d17]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fae539d1f61 - std::panicking::rust_panic_with_hook::h4f679a357a9ccd79
   7:     0x7fae539d1d87 - std::panicking::begin_panic_handler::{{closure}}::h4e87b160fb35c7c3
   8:     0x7fae539cecd4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb01360a6cd9f56db
   9:     0x7fae539d1a52 - rust_begin_unwind
  10:     0x7fae53980f13 - core::panicking::panic_fmt::h8700365ed7eef2a2
  11:     0x7fae53a3430e - core::panicking::assert_failed_inner::h37448dc7036e8bb4
  12:     0x7fae541f994b - core[2e31ec2f599a4931]::panicking::assert_failed::<rustc_hir[a09fe2c49dca5e34]::hir::Unsafety, rustc_hir[a09fe2c49dca5e34]::hir::Unsafety>
  13:     0x7fae54ee42e2 - rustc_typeck[9d52801361d57d]::check::intrinsic::intrinsic_operation_unsafety
  14:     0x7fae54d9c554 - rustc_typeck[9d52801361d57d]::collect::fn_sig
  15:     0x7fae55f8530e - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<rustc_span[d12d2727cd9c9f44]::def_id::DefId, rustc_middle[ef618a27e98a180]::ty::sty::Binder<rustc_middle[ef618a27e98a180]::ty::sty::FnSig>>>
  16:     0x7fae56090bf3 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::fn_sig, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  17:     0x7fae55e8e9ac - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::fn_sig
  18:     0x7fae54d933e6 - rustc_typeck[9d52801361d57d]::collect::convert_item
  19:     0x7fae54d8ef1a - <rustc_typeck[9d52801361d57d]::collect::CollectItemTypesVisitor as rustc_hir[a09fe2c49dca5e34]::intravisit::Visitor>::visit_item
  20:     0x7fae54db6180 - <rustc_middle[ef618a27e98a180]::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck[9d52801361d57d]::collect::CollectItemTypesVisitor>
  21:     0x7fae54d8e4ad - rustc_typeck[9d52801361d57d]::collect::collect_mod_item_types
  22:     0x7fae55f71fe0 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<rustc_span[d12d2727cd9c9f44]::def_id::LocalDefId, ()>>
  23:     0x7fae560651ef - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::collect_mod_item_types, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  24:     0x7fae55e95110 - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::collect_mod_item_types
  25:     0x7fae54db57da - <rustc_middle[ef618a27e98a180]::hir::map::Map>::for_each_module::<rustc_typeck[9d52801361d57d]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  26:     0x7fae54e01a2b - <rustc_session[492dfb63ec0bb8ae]::session::Session>::track_errors::<rustc_typeck[9d52801361d57d]::check_crate::{closure#0}, ()>
  27:     0x7fae5505a119 - rustc_typeck[9d52801361d57d]::check_crate
  28:     0x7fae544d4631 - rustc_interface[53f1f933a2d2ec9]::passes::analysis
  29:     0x7fae55fb116f - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<(), core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>>
  30:     0x7fae560912a0 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::analysis, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  31:     0x7fae55e6ed9a - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::analysis
  32:     0x7fae543fa4f6 - <rustc_interface[53f1f933a2d2ec9]::passes::QueryContext>::enter::<rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  33:     0x7fae543ad751 - rustc_interface[53f1f933a2d2ec9]::interface::create_compiler_and_run::<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>
  34:     0x7fae54381742 - <scoped_tls[537ffb9e41f61595]::ScopedKey<rustc_span[d12d2727cd9c9f44]::SessionGlobals>>::set::<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  35:     0x7fae543fe6cf - std[2e4d6ca4d7177998]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  36:     0x7fae543ac38e - std[2e4d6ca4d7177998]::panic::catch_unwind::<core[2e31ec2f599a4931]::panic::unwind_safe::AssertUnwindSafe<<std[2e4d6ca4d7177998]::thread::Builder>::spawn_unchecked_<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  37:     0x7fae54401b52 - <<std[2e4d6ca4d7177998]::thread::Builder>::spawn_unchecked_<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#1} as core[2e31ec2f599a4931]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fae539dedc5 - std::sys::unix::thread::Thread::new::thread_start::hbd1073ee1be91ca5
  39:     0x7fae5377ab43 - <unknown>
  40:     0x7fae5380ca00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4bad79f8f 2022-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -Z normalize-docs -Z unstable-options
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `main::_doctest_main__checkout_obj_build_x86_64_unknown_linux_gnu_test_error_index_md_1991_0::size_of`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0211::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3843) stdout ----
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Unsafe`,
  left: `Unsafe`,
 right: `Normal`: Intrinsic safety mismatch between list of intrinsics within the compiler and core library intrinsics for intrinsic `size_of`', compiler/rustc_typeck/src/check/intrinsic.rs:118:5
stack backtrace:
   0:     0x7f1d9d77579e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf59a7b4e2d434620
   1:     0x7f1d9d7de948 - core::fmt::write::ha2091e91361753a7
   2:     0x7f1d9d7660a1 - std::io::Write::write_fmt::h5295384f2cf988e3
   3:     0x7f1d9d7787ae - std::panicking::default_hook::{{closure}}::h1a4a7c656812fb23
   4:     0x7f1d9d778477 - std::panicking::default_hook::h375a8262cbf87d78
   5:     0x7f1d9e13d174 - rustc_driver[8a450591bc686d17]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1d9d778f61 - std::panicking::rust_panic_with_hook::h4f679a357a9ccd79
   7:     0x7f1d9d778d87 - std::panicking::begin_panic_handler::{{closure}}::h4e87b160fb35c7c3
   8:     0x7f1d9d775cd4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb01360a6cd9f56db
   9:     0x7f1d9d778a52 - rust_begin_unwind
  10:     0x7f1d9d727f13 - core::panicking::panic_fmt::h8700365ed7eef2a2
  11:     0x7f1d9d7db30e - core::panicking::assert_failed_inner::h37448dc7036e8bb4
  12:     0x7f1d9dfa094b - core[2e31ec2f599a4931]::panicking::assert_failed::<rustc_hir[a09fe2c49dca5e34]::hir::Unsafety, rustc_hir[a09fe2c49dca5e34]::hir::Unsafety>
  13:     0x7f1d9ec8b2e2 - rustc_typeck[9d52801361d57d]::check::intrinsic::intrinsic_operation_unsafety
  14:     0x7f1d9eb43554 - rustc_typeck[9d52801361d57d]::collect::fn_sig
  15:     0x7f1d9fd2c30e - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<rustc_span[d12d2727cd9c9f44]::def_id::DefId, rustc_middle[ef618a27e98a180]::ty::sty::Binder<rustc_middle[ef618a27e98a180]::ty::sty::FnSig>>>
  16:     0x7f1d9fe37bf3 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::fn_sig, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  17:     0x7f1d9fc359ac - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::fn_sig
  18:     0x7f1d9eb3a3e6 - rustc_typeck[9d52801361d57d]::collect::convert_item
  19:     0x7f1d9eb35f1a - <rustc_typeck[9d52801361d57d]::collect::CollectItemTypesVisitor as rustc_hir[a09fe2c49dca5e34]::intravisit::Visitor>::visit_item
  20:     0x7f1d9eb5d180 - <rustc_middle[ef618a27e98a180]::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck[9d52801361d57d]::collect::CollectItemTypesVisitor>
  21:     0x7f1d9eb354ad - rustc_typeck[9d52801361d57d]::collect::collect_mod_item_types
  22:     0x7f1d9fd18fe0 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<rustc_span[d12d2727cd9c9f44]::def_id::LocalDefId, ()>>
  23:     0x7f1d9fe0c1ef - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::collect_mod_item_types, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  24:     0x7f1d9fc3c110 - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::collect_mod_item_types
  25:     0x7f1d9eb5c7da - <rustc_middle[ef618a27e98a180]::hir::map::Map>::for_each_module::<rustc_typeck[9d52801361d57d]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  26:     0x7f1d9eba8a2b - <rustc_session[492dfb63ec0bb8ae]::session::Session>::track_errors::<rustc_typeck[9d52801361d57d]::check_crate::{closure#0}, ()>
  27:     0x7f1d9ee01119 - rustc_typeck[9d52801361d57d]::check_crate
  28:     0x7f1d9e27b631 - rustc_interface[53f1f933a2d2ec9]::passes::analysis
  29:     0x7f1d9fd5816f - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::try_execute_query::<rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt, rustc_query_system[5bbfffd82d4c24d0]::query::caches::DefaultCache<(), core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>>
  30:     0x7f1d9fe382a0 - rustc_query_system[5bbfffd82d4c24d0]::query::plumbing::get_query::<rustc_query_impl[9ac8801b4aecdd7]::queries::analysis, rustc_query_impl[9ac8801b4aecdd7]::plumbing::QueryCtxt>
  31:     0x7f1d9fc15d9a - <rustc_query_impl[9ac8801b4aecdd7]::Queries as rustc_middle[ef618a27e98a180]::ty::query::QueryEngine>::analysis
  32:     0x7f1d9e1a14f6 - <rustc_interface[53f1f933a2d2ec9]::passes::QueryContext>::enter::<rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  33:     0x7f1d9e154751 - rustc_interface[53f1f933a2d2ec9]::interface::create_compiler_and_run::<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>
  34:     0x7f1d9e128742 - <scoped_tls[537ffb9e41f61595]::ScopedKey<rustc_span[d12d2727cd9c9f44]::SessionGlobals>>::set::<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  35:     0x7f1d9e1a56cf - std[2e4d6ca4d7177998]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  36:     0x7f1d9e15338e - std[2e4d6ca4d7177998]::panic::catch_unwind::<core[2e31ec2f599a4931]::panic::unwind_safe::AssertUnwindSafe<<std[2e4d6ca4d7177998]::thread::Builder>::spawn_unchecked_<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>
  37:     0x7f1d9e1a8b52 - <<std[2e4d6ca4d7177998]::thread::Builder>::spawn_unchecked_<rustc_interface[53f1f933a2d2ec9]::util::run_in_thread_pool_with_globals<rustc_interface[53f1f933a2d2ec9]::interface::run_compiler<core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>, rustc_driver[8a450591bc686d17]::run_compiler::{closure#1}>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#0}, core[2e31ec2f599a4931]::result::Result<(), rustc_errors[cf93369e4ced8f3e]::ErrorGuaranteed>>::{closure#1} as core[2e31ec2f599a4931]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f1d9d785dc5 - std::sys::unix::thread::Thread::new::thread_start::hbd1073ee1be91ca5
  39:     0x7f1d9d521b43 - <unknown>
  40:     0x7f1d9d5b3a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4bad79f8f 2022-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -Z normalize-docs -Z unstable-options
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `main::_doctest_main__checkout_obj_build_x86_64_unknown_linux_gnu_test_error_index_md_3843_0::size_of`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
Couldn't compile the test.

failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0094 (line 1978)
