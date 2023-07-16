plain
failures:

---- [ui] src/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments/auxiliary"
stdout: none
--- stderr -------------------------------
error: function can not have more than 65535 arguments
   |
   |
LL |         fn _f($($t: ()),*) {} //~ ERROR function can not have more than 65535 arguments
...
...
LL | many_args!{[_]########## ######}
   |
   = note: this error originates in the macro `many_args` (in Nightly builds, run with -Z macro-backtrace for more info)


thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: TryFromIntError(())', compiler/rustc_typeck/src/check/wfcheck.rs:1500:41
stack backtrace:
   0:     0x7f4a68901a0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7592cb92b786d66f
   1:     0x7f4a6896a1a8 - core::fmt::write::h9e34e910456abc52
   2:     0x7f4a688f2701 - std::io::Write::write_fmt::h97aa2de75db2a565
   3:     0x7f4a689049fe - std::panicking::default_hook::{{closure}}::hb67c8c41bf9a0c43
   4:     0x7f4a689046c7 - std::panicking::default_hook::hcb8c9edd61a05b33
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   5:     0x7f4a69293b54 - rustc_driver[4d21b457761b35ef]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4a689051b1 - std::panicking::rust_panic_with_hook::hc2243692f1553f6d
   7:     0x7f4a68904fd7 - std::panicking::begin_panic_handler::{{closure}}::h0682efc6ccd7f67b
   8:     0x7f4a68901f84 - std::sys_common::backtrace::__rust_end_short_backtrace::h5654225b8fcd818b
   9:     0x7f4a68904ca2 - rust_begin_unwind
  10:     0x7f4a688b4e43 - core::panicking::panic_fmt::hd757845550088711
  11:     0x7f4a688b4fe3 - core::result::unwrap_failed::h72a7a40086ea3996
  12:     0x7f4a69e71441 - <&mut rustc_typeck[555e1c475cc20670]::check::wfcheck::check_fn_or_method::{closure#0} as core[2ccd7d94af1ecfbc]::ops::function::FnOnce<((usize, rustc_middle[9dc72e5127094bc0]::ty::Ty),)>>::call_once
  13:     0x7f4a69d39ff6 - <smallvec[994a9c2da00c2246]::SmallVec<[rustc_middle[9dc72e5127094bc0]::ty::Ty; 8usize]> as core[2ccd7d94af1ecfbc]::iter::traits::collect::Extend<rustc_middle[9dc72e5127094bc0]::ty::Ty>>::extend::<core[2ccd7d94af1ecfbc]::iter::adapters::map::Map<core[2ccd7d94af1ecfbc]::iter::adapters::enumerate::Enumerate<core[2ccd7d94af1ecfbc]::iter::adapters::copied::Copied<core[2ccd7d94af1ecfbc]::slice::iter::Iter<rustc_middle[9dc72e5127094bc0]::ty::Ty>>>, rustc_typeck[555e1c475cc20670]::check::wfcheck::check_fn_or_method::{closure#0}>>
  14:     0x7f4a69dcbe46 - <rustc_middle[9dc72e5127094bc0]::ty::Ty as rustc_type_ir[26f29c59c9c1f77d]::InternIteratorElement<rustc_middle[9dc72e5127094bc0]::ty::Ty, &rustc_middle[9dc72e5127094bc0]::ty::list::List<rustc_middle[9dc72e5127094bc0]::ty::Ty>>>::intern_with::<core[2ccd7d94af1ecfbc]::iter::adapters::map::Map<core[2ccd7d94af1ecfbc]::iter::adapters::enumerate::Enumerate<core[2ccd7d94af1ecfbc]::iter::adapters::copied::Copied<core[2ccd7d94af1ecfbc]::slice::iter::Iter<rustc_middle[9dc72e5127094bc0]::ty::Ty>>>, rustc_typeck[555e1c475cc20670]::check::wfcheck::check_fn_or_method::{closure#0}>, <rustc_middle[9dc72e5127094bc0]::ty::context::TyCtxt>::mk_type_list<core[2ccd7d94af1ecfbc]::iter::adapters::map::Map<core[2ccd7d94af1ecfbc]::iter::adapters::enumerate::Enumerate<core[2ccd7d94af1ecfbc]::iter::adapters::copied::Copied<core[2ccd7d94af1ecfbc]::slice::iter::Iter<rustc_middle[9dc72e5127094bc0]::ty::Ty>>>, rustc_typeck[555e1c475cc20670]::check::wfcheck::check_fn_or_method::{closure#0}>>::{closure#0}>
  15:     0x7f4a69e03032 - <rustc_middle[9dc72e5127094bc0]::ty::context::TyCtxt>::mk_type_list::<core[2ccd7d94af1ecfbc]::iter::adapters::map::Map<core[2ccd7d94af1ecfbc]::iter::adapters::enumerate::Enumerate<core[2ccd7d94af1ecfbc]::iter::adapters::copied::Copied<core[2ccd7d94af1ecfbc]::slice::iter::Iter<rustc_middle[9dc72e5127094bc0]::ty::Ty>>>, rustc_typeck[555e1c475cc20670]::check::wfcheck::check_fn_or_method::{closure#0}>>
  16:     0x7f4a69e93461 - rustc_typeck[555e1c475cc20670]::check::wfcheck::check_fn_or_method
  17:     0x7f4a69d6f265 - <rustc_infer[45d9bff005b98e4f]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[555e1c475cc20670]::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck[555e1c475cc20670]::check::wfcheck::check_item_fn::{closure#0}>::{closure#0}>
  18:     0x7f4a69e81f42 - rustc_typeck[555e1c475cc20670]::check::wfcheck::check_item_fn
  19:     0x7f4a69e783f9 - rustc_typeck[555e1c475cc20670]::check::wfcheck::check_well_formed
  20:     0x7f4a6ae45591 - rustc_query_system[dd89469add690dfe]::query::plumbing::try_execute_query::<rustc_query_impl[366ad33e38471c8f]::plumbing::QueryCtxt, rustc_query_system[dd89469add690dfe]::query::caches::DefaultCache<rustc_span[7c148210c840c6a2]::def_id::LocalDefId, ()>>
  21:     0x7f4a6af1c0c4 - rustc_query_system[dd89469add690dfe]::query::plumbing::get_query::<rustc_query_impl[366ad33e38471c8f]::queries::check_well_formed, rustc_query_impl[366ad33e38471c8f]::plumbing::QueryCtxt>
  22:     0x7f4a6aa9ba74 - <rustc_query_impl[366ad33e38471c8f]::Queries as rustc_middle[9dc72e5127094bc0]::ty::query::QueryEngine>::check_well_formed
  23:     0x7f4a69e40715 - <core[2ccd7d94af1ecfbc]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[ced57eb37c691592]::sync::par_for_each_in<&[rustc_hir[8b6800a805bfa620]::hir::ItemId], <rustc_middle[9dc72e5127094bc0]::hir::ModuleItems>::par_items<rustc_typeck[555e1c475cc20670]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[2ccd7d94af1ecfbc]::ops::function::FnOnce<()>>::call_once
  24:     0x7f4a69ceb9b9 - std[bfd0f7b812973759]::panicking::try::<(), core[2ccd7d94af1ecfbc]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[ced57eb37c691592]::sync::par_for_each_in<&[rustc_hir[8b6800a805bfa620]::hir::ItemId], <rustc_middle[9dc72e5127094bc0]::hir::ModuleItems>::par_items<rustc_typeck[555e1c475cc20670]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  25:     0x7f4a69ee1e6d - rustc_data_structures[ced57eb37c691592]::sync::par_for_each_in::<&[rustc_hir[8b6800a805bfa620]::hir::ItemId], <rustc_middle[9dc72e5127094bc0]::hir::ModuleItems>::par_items<rustc_typeck[555e1c475cc20670]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  26:     0x7f4a69e848e9 - rustc_typeck[555e1c475cc20670]::check::wfcheck::check_mod_type_wf
  27:     0x7f4a6ae45591 - rustc_query_system[dd89469add690dfe]::query::plumbing::try_execute_query::<rustc_query_impl[366ad33e38471c8f]::plumbing::QueryCtxt, rustc_query_system[dd89469add690dfe]::query::caches::DefaultCache<rustc_span[7c148210c840c6a2]::def_id::LocalDefId, ()>>
  28:     0x7f4a6af1bf94 - rustc_query_system[dd89469add690dfe]::query::plumbing::get_query::<rustc_query_impl[366ad33e38471c8f]::queries::check_mod_type_wf, rustc_query_impl[366ad33e38471c8f]::plumbing::QueryCtxt>
  29:     0x7f4a6aa7ce24 - <rustc_query_impl[366ad33e38471c8f]::Queries as rustc_middle[9dc72e5127094bc0]::ty::query::QueryEngine>::check_mod_type_wf
  30:     0x7f4a69dfc83a - <rustc_middle[9dc72e5127094bc0]::hir::map::Map>::for_each_module::<rustc_typeck[555e1c475cc20670]::check_crate::{closure#5}::{closure#0}::{closure#0}>
  31:     0x7f4a69d0d52b - <rustc_session[29c24c042bd6c315]::session::Session>::track_errors::<rustc_typeck[555e1c475cc20670]::check_crate::{closure#5}, ()>
  32:     0x7f4a69f0a8f0 - rustc_typeck[555e1c475cc20670]::check_crate
  33:     0x7f4a693d1db1 - rustc_interface[f5666d880ed2374b]::passes::analysis
  34:     0x7f4a6ae7da45 - rustc_query_system[dd89469add690dfe]::query::plumbing::try_execute_query::<rustc_query_impl[366ad33e38471c8f]::plumbing::QueryCtxt, rustc_query_system[dd89469add690dfe]::query::caches::DefaultCache<(), core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>>
  35:     0x7f4a6af59ec2 - rustc_query_system[dd89469add690dfe]::query::plumbing::get_query::<rustc_query_impl[366ad33e38471c8f]::queries::analysis, rustc_query_impl[366ad33e38471c8f]::plumbing::QueryCtxt>
  36:     0x7f4a6aa61c6e - <rustc_query_impl[366ad33e38471c8f]::Queries as rustc_middle[9dc72e5127094bc0]::ty::query::QueryEngine>::analysis
  37:     0x7f4a692f334b - <rustc_interface[f5666d880ed2374b]::passes::QueryContext>::enter::<rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>
  38:     0x7f4a6929aff4 - <rustc_interface[f5666d880ed2374b]::interface::Compiler>::enter::<rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}::{closure#2}, core[2ccd7d94af1ecfbc]::result::Result<core[2ccd7d94af1ecfbc]::option::Option<rustc_interface[f5666d880ed2374b]::queries::Linker>, rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>
  39:     0x7f4a692800d1 - rustc_span[7c148210c840c6a2]::with_source_map::<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_interface[f5666d880ed2374b]::interface::create_compiler_and_run<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f4a692b5761 - rustc_interface[f5666d880ed2374b]::interface::create_compiler_and_run::<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}>
  41:     0x7f4a69277bf2 - <scoped_tls[a943fe7183fb970f]::ScopedKey<rustc_span[7c148210c840c6a2]::SessionGlobals>>::set::<rustc_interface[f5666d880ed2374b]::interface::run_compiler<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>
  42:     0x7f4a69283e49 - std[bfd0f7b812973759]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f5666d880ed2374b]::util::run_in_thread_pool_with_globals<rustc_interface[f5666d880ed2374b]::interface::run_compiler<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>
  43:     0x7f4a692fe5de - std[bfd0f7b812973759]::panicking::try::<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, core[2ccd7d94af1ecfbc]::panic::unwind_safe::AssertUnwindSafe<<std[bfd0f7b812973759]::thread::Builder>::spawn_unchecked_<rustc_interface[f5666d880ed2374b]::util::run_in_thread_pool_with_globals<rustc_interface[f5666d880ed2374b]::interface::run_compiler<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7f4a692f9d90 - <<std[bfd0f7b812973759]::thread::Builder>::spawn_unchecked_<rustc_interface[f5666d880ed2374b]::util::run_in_thread_pool_with_globals<rustc_interface[f5666d880ed2374b]::interface::run_compiler<core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>, rustc_driver[4d21b457761b35ef]::run_compiler::{closure#1}>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>::{closure#0}, core[2ccd7d94af1ecfbc]::result::Result<(), rustc_errors[4d615d652b42c95]::ErrorGuaranteed>>::{closure#1} as core[2ccd7d94af1ecfbc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f4a68911d45 - std::sys::unix::thread::Thread::new::thread_start::h29a3663456a41d37
  46:     0x7f4a686aeb43 - <unknown>
  47:     0x7f4a68740a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (602d11aea 2022-08-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `_f` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


