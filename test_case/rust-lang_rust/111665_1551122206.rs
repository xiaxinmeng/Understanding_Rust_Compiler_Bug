plain
failures:

---- [ui] tests/ui/offset-of/offset-of-dst-field.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/offset-of/offset-of-dst-field.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-dst-field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-dst-field/auxiliary"
stdout: none
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:36:5
   |
   |
LL |     offset_of!(Alpha, z); //~ ERROR the size for values of type
   |
   = help: the trait `Sized` is not implemented for `[u8]`
   = note: this error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:37:5
   |
LL |     offset_of!(Beta, z); //~ ERROR the size for values of type
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn Trait + 'static)`

error[E0277]: the size for values of type `Extern` cannot be known at compilation time
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:38:5
   |
   |
LL |     offset_of!(Gamma, z); //~ ERROR the size for values of type
   |
   = help: the trait `Sized` is not implemented for `Extern`
   = note: this error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:44:5
   |
LL |     offset_of!(Delta<dyn Trait>, z); //~ ERROR the size for values of type
   |
   = help: the trait `Sized` is not implemented for `dyn Trait`
   = note: this error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:43:5
   |
LL |     offset_of!(Delta<Alpha>, z); //~ ERROR the size for values of type
   |
   |
   = help: within `Alpha`, the trait `Sized` is not implemented for `[u8]`
note: required because it appears within the type `Alpha`
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:5:8
LL | struct Alpha {
   |        ^^^^^
   = note: this error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)


error: internal compiler error: compiler/rustc_hir_typeck/src/writeback.rs:697:17: writeback: `Delta<dyn Trait>` has inference variables
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:44:5
   |
LL |     offset_of!(Delta<dyn Trait>, z); //~ ERROR the size for values of type
   |
   = note: this error: internal compiler error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:994:33
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:994:33
stack backtrace:
   0:     0x7fb5735d9494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd2d8e2309d1fed43
   1:     0x7fb573640148 - core::fmt::write::h6d7af34a838da8d4
   2:     0x7fb5735cdba1 - std::io::Write::write_fmt::h9a034920726890be
   3:     0x7fb5735d92a1 - std::sys_common::backtrace::print::h4adf185dae0f6169
   4:     0x7fb5735dc41a - std::panicking::default_hook::{{closure}}::ha2872a2590c8ebf3
   5:     0x7fb5735dc0fc - std::panicking::default_hook::h981bc07c11c4931b
   6:     0x7fb57408922b - rustc_driver_impl[40b6097c0ea8d6ac]::install_ice_hook::{closure#0}
   7:     0x7fb5735dcb27 - std::panicking::rust_panic_with_hook::he9f338882ab0bc02
   8:     0x7fb574681a73 - std[88ff7485e6789050]::panicking::begin_panic::<rustc_errors[6c11ecac5352b63e]::ExplicitBug>::{closure#0}
   9:     0x7fb574671016 - std[88ff7485e6789050]::sys_common::backtrace::__rust_end_short_backtrace::<std[88ff7485e6789050]::panicking::begin_panic<rustc_errors[6c11ecac5352b63e]::ExplicitBug>::{closure#0}, !>
  10:     0x7fb573e491b6 - std[88ff7485e6789050]::panicking::begin_panic::<rustc_errors[6c11ecac5352b63e]::ExplicitBug>
  11:     0x7fb574731461 - <rustc_errors[6c11ecac5352b63e]::HandlerInner>::span_bug::<rustc_span[150ea94e489d3a47]::span_encoding::Span, alloc[9141ddfa543d6991]::string::String>
  12:     0x7fb574731140 - <rustc_errors[6c11ecac5352b63e]::Handler>::span_bug::<rustc_span[150ea94e489d3a47]::span_encoding::Span, alloc[9141ddfa543d6991]::string::String>
  13:     0x7fb5746998b8 - rustc_middle[faf83cb2799b7ed6]::util::bug::opt_span_bug_fmt::<rustc_span[150ea94e489d3a47]::span_encoding::Span>::{closure#0}
  14:     0x7fb57469990c - rustc_middle[faf83cb2799b7ed6]::ty::context::tls::with_opt::<rustc_middle[faf83cb2799b7ed6]::util::bug::opt_span_bug_fmt<rustc_span[150ea94e489d3a47]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7fb574692ee4 - rustc_middle[faf83cb2799b7ed6]::ty::context::tls::with_context_opt::<rustc_middle[faf83cb2799b7ed6]::ty::context::tls::with_opt<rustc_middle[faf83cb2799b7ed6]::util::bug::opt_span_bug_fmt<rustc_span[150ea94e489d3a47]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7fb573e42a87 - rustc_middle[faf83cb2799b7ed6]::util::bug::span_bug_fmt::<rustc_span[150ea94e489d3a47]::span_encoding::Span>
  17:     0x7fb574642ec7 - <rustc_hir_typeck[5c35d48b0356938b]::writeback::WritebackCx>::visit_offset_of_container_types
  18:     0x7fb5745cd8bc - <rustc_hir_typeck[5c35d48b0356938b]::fn_ctxt::FnCtxt>::resolve_type_vars_in_body
  19:     0x7fb574665918 - rustc_hir_typeck[5c35d48b0356938b]::typeck
  20:     0x7fb575d56548 - rustc_query_system[ef73c22083ab44c7]::query::plumbing::try_execute_query::<rustc_query_impl[62a88670161651c2]::DynamicConfig<rustc_query_system[ef73c22083ab44c7]::query::caches::VecCache<rustc_span[150ea94e489d3a47]::def_id::LocalDefId, rustc_middle[faf83cb2799b7ed6]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[62a88670161651c2]::plumbing::QueryCtxt, false>
  21:     0x7fb575b474ca - rustc_query_impl[62a88670161651c2]::get_query_non_incr::typeck
  22:     0x7fb57464f039 - rustc_middle[faf83cb2799b7ed6]::ty::query::query_get_at::<rustc_query_system[ef73c22083ab44c7]::query::caches::VecCache<rustc_span[150ea94e489d3a47]::def_id::LocalDefId, rustc_middle[faf83cb2799b7ed6]::query::erase::Erased<[u8; 8usize]>>>
  23:     0x7fb574664bc2 - rustc_hir_typeck[5c35d48b0356938b]::used_trait_imports
  24:     0x7fb575d56548 - rustc_query_system[ef73c22083ab44c7]::query::plumbing::try_execute_query::<rustc_query_impl[62a88670161651c2]::DynamicConfig<rustc_query_system[ef73c22083ab44c7]::query::caches::VecCache<rustc_span[150ea94e489d3a47]::def_id::LocalDefId, rustc_middle[faf83cb2799b7ed6]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[62a88670161651c2]::plumbing::QueryCtxt, false>
  25:     0x7fb575b481ea - rustc_query_impl[62a88670161651c2]::get_query_non_incr::used_trait_imports
  26:     0x7fb574950de9 - rustc_middle[faf83cb2799b7ed6]::ty::query::query_get_at::<rustc_query_system[ef73c22083ab44c7]::query::caches::VecCache<rustc_span[150ea94e489d3a47]::def_id::LocalDefId, rustc_middle[faf83cb2799b7ed6]::query::erase::Erased<[u8; 8usize]>>>
  27:     0x7fb57496730f - rustc_hir_analysis[2d80267229b5d511]::check_crate
  28:     0x7fb57417d369 - rustc_interface[6d78e37740b0e5ff]::passes::analysis
  29:     0x7fb575e2aea7 - <rustc_query_impl[62a88670161651c2]::dynamic_query::analysis::{closure#2} as core[85c62f5f18d12f5e]::ops::function::FnOnce<(rustc_middle[faf83cb2799b7ed6]::ty::context::TyCtxt, ())>>::call_once
  30:     0x7fb575cc82a2 - rustc_query_system[ef73c22083ab44c7]::query::plumbing::try_execute_query::<rustc_query_impl[62a88670161651c2]::DynamicConfig<rustc_query_system[ef73c22083ab44c7]::query::caches::SingleCache<rustc_middle[faf83cb2799b7ed6]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[62a88670161651c2]::plumbing::QueryCtxt, false>
  31:     0x7fb575b237a6 - rustc_query_impl[62a88670161651c2]::get_query_non_incr::analysis
  32:     0x7fb5740a57ad - <rustc_middle[faf83cb2799b7ed6]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>
  33:     0x7fb5740db888 - <rustc_interface[6d78e37740b0e5ff]::interface::Compiler>::enter::<rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}::{closure#2}, core[85c62f5f18d12f5e]::result::Result<core[85c62f5f18d12f5e]::option::Option<rustc_interface[6d78e37740b0e5ff]::queries::Linker>, rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>
  34:     0x7fb5740a4400 - rustc_span[150ea94e489d3a47]::set_source_map::<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, rustc_interface[6d78e37740b0e5ff]::interface::run_compiler<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7fb574096ae9 - <scoped_tls[ea298804ec9fbff7]::ScopedKey<rustc_span[150ea94e489d3a47]::SessionGlobals>>::set::<rustc_interface[6d78e37740b0e5ff]::interface::run_compiler<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}>::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>
  36:     0x7fb5740ae856 - std[88ff7485e6789050]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6d78e37740b0e5ff]::util::run_in_thread_pool_with_globals<rustc_interface[6d78e37740b0e5ff]::interface::run_compiler<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}>::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>
  37:     0x7fb5740dcd56 - std[88ff7485e6789050]::panicking::try::<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, core[85c62f5f18d12f5e]::panic::unwind_safe::AssertUnwindSafe<<std[88ff7485e6789050]::thread::Builder>::spawn_unchecked_<rustc_interface[6d78e37740b0e5ff]::util::run_in_thread_pool_with_globals<rustc_interface[6d78e37740b0e5ff]::interface::run_compiler<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}>::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7fb57409ccbd - <<std[88ff7485e6789050]::thread::Builder>::spawn_unchecked_<rustc_interface[6d78e37740b0e5ff]::util::run_in_thread_pool_with_globals<rustc_interface[6d78e37740b0e5ff]::interface::run_compiler<core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>, rustc_driver_impl[40b6097c0ea8d6ac]::run_compiler::{closure#1}>::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c62f5f18d12f5e]::result::Result<(), rustc_span[150ea94e489d3a47]::ErrorGuaranteed>>::{closure#1} as core[85c62f5f18d12f5e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7fb5735e945e - std::sys::unix::thread::Thread::new::thread_start::h04412f818bdd9614
  40:     0x7fb573384b43 - <unknown>
  41:     0x7fb573416a00 - <unknown>
  42:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (e12d62e50 2023-05-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `delta`
#1 [used_trait_imports] finding used_trait_imports `delta`
#2 [analysis] running analysis passes on this crate
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
