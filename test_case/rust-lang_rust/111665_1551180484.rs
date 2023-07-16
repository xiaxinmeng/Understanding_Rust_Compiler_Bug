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
   0:     0x7fc9756d2494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h83c4d273a293937a
   1:     0x7fc975739148 - core::fmt::write::h4944797bfd0ec333
   2:     0x7fc9756c6c41 - std::io::Write::write_fmt::hdcc1f740d69f3926
   3:     0x7fc9756d22a1 - std::sys_common::backtrace::print::h073f86496844392b
   4:     0x7fc9756d541a - std::panicking::default_hook::{{closure}}::h05a9bc928a1853ef
   5:     0x7fc9756d50fc - std::panicking::default_hook::h3f4a2e90430e7afd
   6:     0x7fc97618222b - rustc_driver_impl[3f6d7dedc3cd2469]::install_ice_hook::{closure#0}
   7:     0x7fc9756d5b27 - std::panicking::rust_panic_with_hook::hcab7ffc533d49d69
   8:     0x7fc97677a7f3 - std[6e36f31dabee1d53]::panicking::begin_panic::<rustc_errors[eb4320e7b0a9c594]::ExplicitBug>::{closure#0}
   9:     0x7fc976769c86 - std[6e36f31dabee1d53]::sys_common::backtrace::__rust_end_short_backtrace::<std[6e36f31dabee1d53]::panicking::begin_panic<rustc_errors[eb4320e7b0a9c594]::ExplicitBug>::{closure#0}, !>
  10:     0x7fc975f421e6 - std[6e36f31dabee1d53]::panicking::begin_panic::<rustc_errors[eb4320e7b0a9c594]::ExplicitBug>
  11:     0x7fc97682a1f1 - <rustc_errors[eb4320e7b0a9c594]::HandlerInner>::span_bug::<rustc_span[e7b4b05131986322]::span_encoding::Span, alloc[dfd73537032ceef3]::string::String>
  12:     0x7fc976829ed0 - <rustc_errors[eb4320e7b0a9c594]::Handler>::span_bug::<rustc_span[e7b4b05131986322]::span_encoding::Span, alloc[dfd73537032ceef3]::string::String>
  13:     0x7fc976792648 - rustc_middle[dff2417f09c5d1c7]::util::bug::opt_span_bug_fmt::<rustc_span[e7b4b05131986322]::span_encoding::Span>::{closure#0}
  14:     0x7fc97679269c - rustc_middle[dff2417f09c5d1c7]::ty::context::tls::with_opt::<rustc_middle[dff2417f09c5d1c7]::util::bug::opt_span_bug_fmt<rustc_span[e7b4b05131986322]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7fc97678bc74 - rustc_middle[dff2417f09c5d1c7]::ty::context::tls::with_context_opt::<rustc_middle[dff2417f09c5d1c7]::ty::context::tls::with_opt<rustc_middle[dff2417f09c5d1c7]::util::bug::opt_span_bug_fmt<rustc_span[e7b4b05131986322]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7fc975f3bab7 - rustc_middle[dff2417f09c5d1c7]::util::bug::span_bug_fmt::<rustc_span[e7b4b05131986322]::span_encoding::Span>
  17:     0x7fc97673bc47 - <rustc_hir_typeck[5e8ba42dccce2c8d]::writeback::WritebackCx>::visit_offset_of_container_types
  18:     0x7fc9766c6e2c - <rustc_hir_typeck[5e8ba42dccce2c8d]::fn_ctxt::FnCtxt>::resolve_type_vars_in_body
  19:     0x7fc97675e698 - rustc_hir_typeck[5e8ba42dccce2c8d]::typeck
  20:     0x7fc977e4d8e8 - rustc_query_system[eec62171ae950562]::query::plumbing::try_execute_query::<rustc_query_impl[4d8232e64bd838e8]::DynamicConfig<rustc_query_system[eec62171ae950562]::query::caches::VecCache<rustc_span[e7b4b05131986322]::def_id::LocalDefId, rustc_middle[dff2417f09c5d1c7]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4d8232e64bd838e8]::plumbing::QueryCtxt, false>
  21:     0x7fc977c3e75a - rustc_query_impl[4d8232e64bd838e8]::get_query_non_incr::typeck
  22:     0x7fc976747db9 - rustc_middle[dff2417f09c5d1c7]::ty::query::query_get_at::<rustc_query_system[eec62171ae950562]::query::caches::VecCache<rustc_span[e7b4b05131986322]::def_id::LocalDefId, rustc_middle[dff2417f09c5d1c7]::query::erase::Erased<[u8; 8usize]>>>
  23:     0x7fc97675d942 - rustc_hir_typeck[5e8ba42dccce2c8d]::used_trait_imports
  24:     0x7fc977e4d8e8 - rustc_query_system[eec62171ae950562]::query::plumbing::try_execute_query::<rustc_query_impl[4d8232e64bd838e8]::DynamicConfig<rustc_query_system[eec62171ae950562]::query::caches::VecCache<rustc_span[e7b4b05131986322]::def_id::LocalDefId, rustc_middle[dff2417f09c5d1c7]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4d8232e64bd838e8]::plumbing::QueryCtxt, false>
  25:     0x7fc977c3f47a - rustc_query_impl[4d8232e64bd838e8]::get_query_non_incr::used_trait_imports
  26:     0x7fc976a4a459 - rustc_middle[dff2417f09c5d1c7]::ty::query::query_get_at::<rustc_query_system[eec62171ae950562]::query::caches::VecCache<rustc_span[e7b4b05131986322]::def_id::LocalDefId, rustc_middle[dff2417f09c5d1c7]::query::erase::Erased<[u8; 8usize]>>>
  27:     0x7fc976a6097f - rustc_hir_analysis[d1318f2d0275e338]::check_crate
  28:     0x7fc976276229 - rustc_interface[2e0b07c19739ca13]::passes::analysis
  29:     0x7fc977f22227 - <rustc_query_impl[4d8232e64bd838e8]::dynamic_query::analysis::{closure#2} as core[889acdda7fcfab05]::ops::function::FnOnce<(rustc_middle[dff2417f09c5d1c7]::ty::context::TyCtxt, ())>>::call_once
  30:     0x7fc977dbf652 - rustc_query_system[eec62171ae950562]::query::plumbing::try_execute_query::<rustc_query_impl[4d8232e64bd838e8]::DynamicConfig<rustc_query_system[eec62171ae950562]::query::caches::SingleCache<rustc_middle[dff2417f09c5d1c7]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[4d8232e64bd838e8]::plumbing::QueryCtxt, false>
  31:     0x7fc977c1aa36 - rustc_query_impl[4d8232e64bd838e8]::get_query_non_incr::analysis
  32:     0x7fc97619e89d - <rustc_middle[dff2417f09c5d1c7]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>
  33:     0x7fc9761d4888 - <rustc_interface[2e0b07c19739ca13]::interface::Compiler>::enter::<rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}::{closure#2}, core[889acdda7fcfab05]::result::Result<core[889acdda7fcfab05]::option::Option<rustc_interface[2e0b07c19739ca13]::queries::Linker>, rustc_span[e7b4b05131986322]::ErrorGuaranteed>>
  34:     0x7fc97619d4f0 - rustc_span[e7b4b05131986322]::set_source_map::<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, rustc_interface[2e0b07c19739ca13]::interface::run_compiler<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7fc97618fae9 - <scoped_tls[85c7688d238b0262]::ScopedKey<rustc_span[e7b4b05131986322]::SessionGlobals>>::set::<rustc_interface[2e0b07c19739ca13]::interface::run_compiler<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}>::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>
  36:     0x7fc9761a78c6 - std[6e36f31dabee1d53]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e0b07c19739ca13]::util::run_in_thread_pool_with_globals<rustc_interface[2e0b07c19739ca13]::interface::run_compiler<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}>::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>
  37:     0x7fc9761d5d56 - std[6e36f31dabee1d53]::panicking::try::<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, core[889acdda7fcfab05]::panic::unwind_safe::AssertUnwindSafe<<std[6e36f31dabee1d53]::thread::Builder>::spawn_unchecked_<rustc_interface[2e0b07c19739ca13]::util::run_in_thread_pool_with_globals<rustc_interface[2e0b07c19739ca13]::interface::run_compiler<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}>::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7fc976195cbd - <<std[6e36f31dabee1d53]::thread::Builder>::spawn_unchecked_<rustc_interface[2e0b07c19739ca13]::util::run_in_thread_pool_with_globals<rustc_interface[2e0b07c19739ca13]::interface::run_compiler<core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>, rustc_driver_impl[3f6d7dedc3cd2469]::run_compiler::{closure#1}>::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[889acdda7fcfab05]::result::Result<(), rustc_span[e7b4b05131986322]::ErrorGuaranteed>>::{closure#1} as core[889acdda7fcfab05]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7fc9756e245e - std::sys::unix::thread::Thread::new::thread_start::h4ca786d16292e1e9
  40:     0x7fc97547db43 - <unknown>
  41:     0x7fc97550fa00 - <unknown>
  42:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (7dc93f958 2023-05-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `delta`
#1 [used_trait_imports] finding used_trait_imports `delta`
#2 [analysis] running analysis passes on this crate
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
