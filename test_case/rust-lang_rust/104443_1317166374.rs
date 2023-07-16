plain
Successfully built a1e0dd5c6ca7
Successfully tagged rust-ci:latest
Built container sha256:a1e0dd5c6ca70855d8cf24e8f339da607a0611db72419e3dccf3d8d03692e159
Uploading finished image to https://ci-caches.rust-lang.org/docker/228fe0c973fd83bc0e2087bfb623d368d5127cb8dda872f460580ccb0be6ba22eb1cc9564f9975f532055e823f397e13c814d819a32b966657f03e4507c5de65
upload failed: - to s3://rust-lang-ci-sccache2/docker/228fe0c973fd83bc0e2087bfb623d368d5127cb8dda872f460580ccb0be6ba22eb1cc9564f9975f532055e823f397e13c814d819a32b966657f03e4507c5de65 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
...................................................i...........ii....................... 1848/13849
........................................................................................ 1936/13849
........................................................................................ 2024/13849
......................................i................................................. 2112/13849
..................F..F.................................................................. 2200/13849
...........................................................FF........................... 2288/13849
...........................F........................................................FF.. 2376/13849
...F.....F.....................F........................................F............... 2464/13849
......................................F................................................. 2552/13849
........................................................................................ 2728/13849
........................................................................................ 2816/13849
........................................................................................ 2904/13849
........................................................................................ 2992/13849
---
failures:

---- [ui] src/test/ui/const-generics/const-param-elided-lifetime.rs#full stdout ----

error in revision `full`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | struct A<const N: &u8>;
   |                   ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | impl<const N: &u8> A<N> {
   |               ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL |     fn foo<const M: &u8>(&self) {}
   |                     ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | impl<const N: &u8> B for A<N> {}
   |               ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | fn bar<const N: &u8>() {}
   |                 ^ explicit lifetime name needed here

error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:536:17: cannot relate region: LUB(ReFree(DefId(0:9 ~ const_param_elided_lifetime[13ea]::{impl#0}::foo), BrNamed(DefId(0:17 ~ const_param_elided_lifetime[13ea]::{impl#0}::foo::'_), '_)), ReErased)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
   0:     0x7fb1f92ead8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   0:     0x7fb1f92ead8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   1:     0x7fb1f935a628 - core::fmt::write::h49960c28f6893d6a
   2:     0x7fb1f92dcb51 - std::io::Write::write_fmt::h102a2b4e994607c6
   3:     0x7fb1f92eab91 - std::sys_common::backtrace::print::h98a61b07d2a493a8
   4:     0x7fb1f92edef4 - std::panicking::default_hook::{{closure}}::h8265366696d19d7d
   5:     0x7fb1f92edbb9 - std::panicking::default_hook::h758ba32b28879bcc
   6:     0x7fb1f9cfc4e4 - rustc_driver[449791d1a40e792e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb1f92ee644 - std::panicking::rust_panic_with_hook::h29df597bad3a0157
   8:     0x7fb1fc945843 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}
   9:     0x7fb1fc93e7f6 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_end_short_backtrace::<std[41ffe0caf4ddeafb]::panicking::begin_panic<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}, !>
  10:     0x7fb1f9ca0f56 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  11:     0x7fb1fc9ff0e6 - std[41ffe0caf4ddeafb]::panic::panic_any::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  12:     0x7fb1fc9f6b37 - <rustc_errors[26aae3f62dab3f62]::HandlerInner>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  13:     0x7fb1fc9f2bb0 - <rustc_errors[26aae3f62dab3f62]::Handler>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  14:     0x7fb1fcac346e - rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_context_opt::<rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_opt<rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt<rustc_span[d0c85c35704fd34a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fb1fcac4b99 - rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt::<rustc_span[d0c85c35704fd34a]::span_encoding::Span>
  16:     0x7fb1f9ca8848 - rustc_middle[2861a5a2647faf6f]::util::bug::bug_fmt
  17:     0x7fb1fc8925ee - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  18:     0x7fb1fc891c16 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::sub_concrete_regions
  19:     0x7fb1fc888390 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
  20:     0x7fb1fc891477 - rustc_infer[be35d10197890244]::infer::lexical_region_resolve::resolve
  21:     0x7fb1fc8b86a9 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::resolve_regions
  22:     0x7fb1fc8e2bdf - <rustc_infer[be35d10197890244]::infer::error_reporting::TypeErrCtxt>::resolve_regions_and_report_errors
  23:     0x7fb1fc8b2fa2 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::check_region_obligations_and_report_errors
  24:     0x7fb1fa5bc6e0 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_associated_item
  25:     0x7fb1fa5a9692 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_well_formed
  26:     0x7fb1fba983f2 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId, ()>>
  27:     0x7fb1fbb91caa - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_well_formed, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  28:     0x7fb1fb7649c0 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_well_formed
  29:     0x7fb1fa5d60e5 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ImplItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  30:     0x7fb1fa4fdbb6 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ImplItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>>
  31:     0x7fb1fa675e8d - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir::ImplItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  32:     0x7fb1fa5b9c50 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf
  33:     0x7fb1fbaa6362 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_span[d0c85c35704fd34a]::def_id::LocalDefId, ()>>
  34:     0x7fb1fbb91b8a - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_mod_type_wf, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  35:     0x7fb1fb739930 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7fb1fa5d6725 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  37:     0x7fb1fa4fdc36 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7fb1fa67644d - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7fb1fa4f8abd - <rustc_session[de9ea2a484ed4989]::session::Session>::track_errors::<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}, ()>
  40:     0x7fb1fa68d233 - rustc_hir_analysis[b8d4af9d9079fbae]::check_crate
  41:     0x7fb1f9e54cd1 - rustc_interface[39e98fe1b5933e6b]::passes::analysis
  42:     0x7fb1fbae716f - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<(), core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>>
  43:     0x7fb1fbbd18eb - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::analysis, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  44:     0x7fb1fb711f8a - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::analysis
  45:     0x7fb1f9d57b19 - <rustc_interface[39e98fe1b5933e6b]::passes::QueryContext>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  46:     0x7fb1f9d6ca0a - <rustc_interface[39e98fe1b5933e6b]::interface::Compiler>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}, core[9932f27d9333ac31]::result::Result<core[9932f27d9333ac31]::option::Option<rustc_interface[39e98fe1b5933e6b]::queries::Linker>, rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  47:     0x7fb1f9cfdc2e - rustc_span[d0c85c35704fd34a]::with_source_map::<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  48:     0x7fb1f9d5f76c - <scoped_tls[77ce614b43bc26b9]::ScopedKey<rustc_span[d0c85c35704fd34a]::SessionGlobals>>::set::<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  49:     0x7fb1f9d5b189 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  50:     0x7fb1f9d60876 - std[41ffe0caf4ddeafb]::panic::catch_unwind::<core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  51:     0x7fb1f9d0cd4a - <<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1} as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7fb1f92fad7e - std::sys::unix::thread::Thread::new::thread_start::hd199a9f43fbdddea
  53:     0x7fb1f9091b43 - <unknown>
  54:     0x7fb1f9123a00 - <unknown>
  55:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (90f960f09 2022-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `<impl at /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:14:1: 14:24>::foo` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0637`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/const-param-elided-lifetime.rs#min stdout ----

error in revision `min`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.min" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.min/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | struct A<const N: &u8>;
   |                   ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | impl<const N: &u8> A<N> {
   |               ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL |     fn foo<const M: &u8>(&self) {}
   |                     ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | impl<const N: &u8> B for A<N> {}
   |               ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | fn bar<const N: &u8>() {}
   |                 ^ explicit lifetime name needed here

error: `&'static u8` is forbidden as the type of a const generic parameter
   |
   |
LL | struct A<const N: &u8>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&'static u8` is forbidden as the type of a const generic parameter
   |
   |
LL | impl<const N: &u8> A<N> {
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&'static u8` is forbidden as the type of a const generic parameter
   |
   |
LL | impl<const N: &u8> B for A<N> {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&'static u8` is forbidden as the type of a const generic parameter
   |
   |
LL | fn bar<const N: &u8>() {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:536:17: cannot relate region: LUB(ReFree(DefId(0:9 ~ const_param_elided_lifetime[13ea]::{impl#0}::foo), BrNamed(DefId(0:17 ~ const_param_elided_lifetime[13ea]::{impl#0}::foo::'_), '_)), ReErased)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
   0:     0x7f60bef9fd8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   0:     0x7f60bef9fd8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   1:     0x7f60bf00f628 - core::fmt::write::h49960c28f6893d6a
   2:     0x7f60bef91b51 - std::io::Write::write_fmt::h102a2b4e994607c6
   3:     0x7f60bef9fb91 - std::sys_common::backtrace::print::h98a61b07d2a493a8
   4:     0x7f60befa2ef4 - std::panicking::default_hook::{{closure}}::h8265366696d19d7d
   5:     0x7f60befa2bb9 - std::panicking::default_hook::h758ba32b28879bcc
   6:     0x7f60bf9b14e4 - rustc_driver[449791d1a40e792e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f60befa3644 - std::panicking::rust_panic_with_hook::h29df597bad3a0157
   8:     0x7f60c25fa843 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}
   9:     0x7f60c25f37f6 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_end_short_backtrace::<std[41ffe0caf4ddeafb]::panicking::begin_panic<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}, !>
  10:     0x7f60bf955f56 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  11:     0x7f60c26b40e6 - std[41ffe0caf4ddeafb]::panic::panic_any::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  12:     0x7f60c26abb37 - <rustc_errors[26aae3f62dab3f62]::HandlerInner>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  13:     0x7f60c26a7bb0 - <rustc_errors[26aae3f62dab3f62]::Handler>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  14:     0x7f60c277846e - rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_context_opt::<rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_opt<rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt<rustc_span[d0c85c35704fd34a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f60c2779b99 - rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt::<rustc_span[d0c85c35704fd34a]::span_encoding::Span>
  16:     0x7f60bf95d848 - rustc_middle[2861a5a2647faf6f]::util::bug::bug_fmt
  17:     0x7f60c25475ee - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  18:     0x7f60c2546c16 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::sub_concrete_regions
  19:     0x7f60c253d390 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
  20:     0x7f60c2546477 - rustc_infer[be35d10197890244]::infer::lexical_region_resolve::resolve
  21:     0x7f60c256d6a9 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::resolve_regions
  22:     0x7f60c2597bdf - <rustc_infer[be35d10197890244]::infer::error_reporting::TypeErrCtxt>::resolve_regions_and_report_errors
  23:     0x7f60c2567fa2 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::check_region_obligations_and_report_errors
  24:     0x7f60c02716e0 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_associated_item
  25:     0x7f60c025e692 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_well_formed
  26:     0x7f60c174d3f2 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId, ()>>
  27:     0x7f60c1846caa - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_well_formed, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  28:     0x7f60c14199c0 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_well_formed
  29:     0x7f60c028b0e5 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ImplItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  30:     0x7f60c01b2bb6 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ImplItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>>
  31:     0x7f60c032ae8d - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir::ImplItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  32:     0x7f60c026ec50 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf
  33:     0x7f60c175b362 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_span[d0c85c35704fd34a]::def_id::LocalDefId, ()>>
  34:     0x7f60c1846b8a - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_mod_type_wf, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  35:     0x7f60c13ee930 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7f60c028b725 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  37:     0x7f60c01b2c36 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7f60c032b44d - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7f60c01adabd - <rustc_session[de9ea2a484ed4989]::session::Session>::track_errors::<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}, ()>
  40:     0x7f60c0342233 - rustc_hir_analysis[b8d4af9d9079fbae]::check_crate
  41:     0x7f60bfb09cd1 - rustc_interface[39e98fe1b5933e6b]::passes::analysis
  42:     0x7f60c179c16f - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<(), core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>>
  43:     0x7f60c18868eb - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::analysis, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  44:     0x7f60c13c6f8a - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::analysis
  45:     0x7f60bfa0cb19 - <rustc_interface[39e98fe1b5933e6b]::passes::QueryContext>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  46:     0x7f60bfa21a0a - <rustc_interface[39e98fe1b5933e6b]::interface::Compiler>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}, core[9932f27d9333ac31]::result::Result<core[9932f27d9333ac31]::option::Option<rustc_interface[39e98fe1b5933e6b]::queries::Linker>, rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  47:     0x7f60bf9b2c2e - rustc_span[d0c85c35704fd34a]::with_source_map::<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  48:     0x7f60bfa1476c - <scoped_tls[77ce614b43bc26b9]::ScopedKey<rustc_span[d0c85c35704fd34a]::SessionGlobals>>::set::<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  49:     0x7f60bfa10189 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  50:     0x7f60bfa15876 - std[41ffe0caf4ddeafb]::panic::catch_unwind::<core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  51:     0x7f60bf9c1d4a - <<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1} as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f60befafd7e - std::sys::unix::thread::Thread::new::thread_start::hd199a9f43fbdddea
  53:     0x7f60bed46b43 - <unknown>
  54:     0x7f60bedd8a00 - <unknown>
  55:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (90f960f09 2022-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `<impl at /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:14:1: 14:24>::foo` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0637`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information
   = note: `#[warn(incomplete_features)]` on by default


warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information


error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:536:17: cannot relate region: LUB(ReErased, ReErased)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
   0:     0x7f3098f17d8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   0:     0x7f3098f17d8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   1:     0x7f3098f87628 - core::fmt::write::h49960c28f6893d6a
   2:     0x7f3098f09b51 - std::io::Write::write_fmt::h102a2b4e994607c6
   3:     0x7f3098f17b91 - std::sys_common::backtrace::print::h98a61b07d2a493a8
   4:     0x7f3098f1aef4 - std::panicking::default_hook::{{closure}}::h8265366696d19d7d
   5:     0x7f3098f1abb9 - std::panicking::default_hook::h758ba32b28879bcc
   6:     0x7f30999294e4 - rustc_driver[449791d1a40e792e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3098f1b644 - std::panicking::rust_panic_with_hook::h29df597bad3a0157
   8:     0x7f309c572843 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}
   9:     0x7f309c56b7f6 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_end_short_backtrace::<std[41ffe0caf4ddeafb]::panicking::begin_panic<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}, !>
  10:     0x7f30998cdf56 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  11:     0x7f309c62c0e6 - std[41ffe0caf4ddeafb]::panic::panic_any::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  12:     0x7f309c623b37 - <rustc_errors[26aae3f62dab3f62]::HandlerInner>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  13:     0x7f309c61fbb0 - <rustc_errors[26aae3f62dab3f62]::Handler>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  14:     0x7f309c6f046e - rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_context_opt::<rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_opt<rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt<rustc_span[d0c85c35704fd34a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f309c6f1b99 - rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt::<rustc_span[d0c85c35704fd34a]::span_encoding::Span>
  16:     0x7f30998d5848 - rustc_middle[2861a5a2647faf6f]::util::bug::bug_fmt
  17:     0x7f309c4bf5ee - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  18:     0x7f309c4bec16 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::sub_concrete_regions
  19:     0x7f309c4b5390 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
  20:     0x7f309c4be477 - rustc_infer[be35d10197890244]::infer::lexical_region_resolve::resolve
  21:     0x7f309c4e56a9 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::resolve_regions
  22:     0x7f309c50fbdf - <rustc_infer[be35d10197890244]::infer::error_reporting::TypeErrCtxt>::resolve_regions_and_report_errors
  23:     0x7f309c4dffa2 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::check_region_obligations_and_report_errors
  24:     0x7f309a1d8440 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_well_formed
  25:     0x7f309b6c53f2 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId, ()>>
  26:     0x7f309b7becaa - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_well_formed, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  27:     0x7f309b3919c0 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_well_formed
  28:     0x7f309a203595 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  29:     0x7f309a12ac16 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  30:     0x7f309a2a32dd - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir::ItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  31:     0x7f309a1e6c39 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf
  32:     0x7f309b6d3362 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_span[d0c85c35704fd34a]::def_id::LocalDefId, ()>>
  33:     0x7f309b7beb8a - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_mod_type_wf, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  34:     0x7f309b366930 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_mod_type_wf
  35:     0x7f309a203725 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  36:     0x7f309a12ac36 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  37:     0x7f309a2a344d - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  38:     0x7f309a125abd - <rustc_session[de9ea2a484ed4989]::session::Session>::track_errors::<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}, ()>
  39:     0x7f309a2ba233 - rustc_hir_analysis[b8d4af9d9079fbae]::check_crate
  40:     0x7f3099a81cd1 - rustc_interface[39e98fe1b5933e6b]::passes::analysis
  41:     0x7f309b71416f - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<(), core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>>
  42:     0x7f309b7fe8eb - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::analysis, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  43:     0x7f309b33ef8a - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::analysis
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  44:     0x7f3099984b19 - <rustc_interface[39e98fe1b5933e6b]::passes::QueryContext>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  45:     0x7f3099999a0a - <rustc_interface[39e98fe1b5933e6b]::interface::Compiler>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}, core[9932f27d9333ac31]::result::Result<core[9932f27d9333ac31]::option::Option<rustc_interface[39e98fe1b5933e6b]::queries::Linker>, rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  46:     0x7f309992ac2e - rustc_span[d0c85c35704fd34a]::with_source_map::<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  47:     0x7f309998c76c - <scoped_tls[77ce614b43bc26b9]::ScopedKey<rustc_span[d0c85c35704fd34a]::SessionGlobals>>::set::<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  48:     0x7f3099988189 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  49:     0x7f309998d876 - std[41ffe0caf4ddeafb]::panic::catch_unwind::<core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  50:     0x7f3099939d4a - <<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1} as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7f3098f27d7e - std::sys::unix::thread::Thread::new::thread_start::hd199a9f43fbdddea
  52:     0x7f3098cbeb43 - <unknown>
  53:     0x7f3098d50a00 - <unknown>
  54:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (90f960f09 2022-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `<impl at /checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs:14:1: 14:62>` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information
   = note: `#[warn(incomplete_features)]` on by default


warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information


error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:536:17: cannot relate region: LUB(ReErased, ReErased)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
   0:     0x7f023d9a7d8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   0:     0x7f023d9a7d8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   1:     0x7f023da17628 - core::fmt::write::h49960c28f6893d6a
   2:     0x7f023d999b51 - std::io::Write::write_fmt::h102a2b4e994607c6
   3:     0x7f023d9a7b91 - std::sys_common::backtrace::print::h98a61b07d2a493a8
   4:     0x7f023d9aaef4 - std::panicking::default_hook::{{closure}}::h8265366696d19d7d
   5:     0x7f023d9aabb9 - std::panicking::default_hook::h758ba32b28879bcc
   6:     0x7f023e3b94e4 - rustc_driver[449791d1a40e792e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f023d9ab644 - std::panicking::rust_panic_with_hook::h29df597bad3a0157
   8:     0x7f0241002843 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}
   9:     0x7f0240ffb7f6 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_end_short_backtrace::<std[41ffe0caf4ddeafb]::panicking::begin_panic<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}, !>
  10:     0x7f023e35df56 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  11:     0x7f02410bc0e6 - std[41ffe0caf4ddeafb]::panic::panic_any::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  12:     0x7f02410b3b37 - <rustc_errors[26aae3f62dab3f62]::HandlerInner>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  13:     0x7f02410afbb0 - <rustc_errors[26aae3f62dab3f62]::Handler>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  14:     0x7f024118046e - rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_context_opt::<rustc_middle[2861a5a2647faf6f]::ty::context::tls::with_opt<rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt<rustc_span[d0c85c35704fd34a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f0241181b99 - rustc_middle[2861a5a2647faf6f]::util::bug::opt_span_bug_fmt::<rustc_span[d0c85c35704fd34a]::span_encoding::Span>
  16:     0x7f023e365848 - rustc_middle[2861a5a2647faf6f]::util::bug::bug_fmt
  17:     0x7f0240f4f5ee - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  18:     0x7f0240f4ec16 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::sub_concrete_regions
  19:     0x7f0240f45390 - <rustc_infer[be35d10197890244]::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
  20:     0x7f0240f4e477 - rustc_infer[be35d10197890244]::infer::lexical_region_resolve::resolve
  21:     0x7f0240f756a9 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::resolve_regions
  22:     0x7f0240f9fbdf - <rustc_infer[be35d10197890244]::infer::error_reporting::TypeErrCtxt>::resolve_regions_and_report_errors
  23:     0x7f0240f6ffa2 - <rustc_infer[be35d10197890244]::infer::InferCtxt>::check_region_obligations_and_report_errors
  24:     0x7f023ec68440 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_well_formed
  25:     0x7f02401553f2 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId, ()>>
  26:     0x7f024024ecaa - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_well_formed, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  27:     0x7f023fe219c0 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_well_formed
  28:     0x7f023ec93595 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  29:     0x7f023ebbac16 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir::ItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  30:     0x7f023ed332dd - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir::ItemId], <rustc_middle[2861a5a2647faf6f]::hir::ModuleItems>::par_items<rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  31:     0x7f023ec76c39 - rustc_hir_analysis[b8d4af9d9079fbae]::check::wfcheck::check_mod_type_wf
  32:     0x7f0240163362 - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<rustc_span[d0c85c35704fd34a]::def_id::LocalDefId, ()>>
  33:     0x7f024024eb8a - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::check_mod_type_wf, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  34:     0x7f023fdf6930 - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::check_mod_type_wf
  35:     0x7f023ec93725 - <core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once
  36:     0x7f023ebbac36 - std[41ffe0caf4ddeafb]::panicking::try::<(), core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  37:     0x7f023ed3344d - rustc_data_structures[6602775ae3001da6]::sync::par_for_each_in::<&[rustc_hir[a9c426c6c0f7c511]::hir_id::OwnerId], <rustc_middle[2861a5a2647faf6f]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  38:     0x7f023ebb5abd - <rustc_session[de9ea2a484ed4989]::session::Session>::track_errors::<rustc_hir_analysis[b8d4af9d9079fbae]::check_crate::{closure#5}, ()>
  39:     0x7f023ed4a233 - rustc_hir_analysis[b8d4af9d9079fbae]::check_crate
  40:     0x7f023e511cd1 - rustc_interface[39e98fe1b5933e6b]::passes::analysis
  41:     0x7f02401a416f - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::try_execute_query::<rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt, rustc_query_system[9cac7bf20ebe7ce9]::query::caches::DefaultCache<(), core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>>
  42:     0x7f024028e8eb - rustc_query_system[9cac7bf20ebe7ce9]::query::plumbing::get_query::<rustc_query_impl[30bb6a53955cf19f]::queries::analysis, rustc_query_impl[30bb6a53955cf19f]::plumbing::QueryCtxt>
  43:     0x7f023fdcef8a - <rustc_query_impl[30bb6a53955cf19f]::Queries as rustc_middle[2861a5a2647faf6f]::ty::query::QueryEngine>::analysis
  44:     0x7f023e414b19 - <rustc_interface[39e98fe1b5933e6b]::passes::QueryContext>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  45:     0x7f023e429a0a - <rustc_interface[39e98fe1b5933e6b]::interface::Compiler>::enter::<rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}::{closure#2}, core[9932f27d9333ac31]::result::Result<core[9932f27d9333ac31]::option::Option<rustc_interface[39e98fe1b5933e6b]::queries::Linker>, rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  46:     0x7f023e3bac2e - rustc_span[d0c85c35704fd34a]::with_source_map::<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  47:     0x7f023e41c76c - <scoped_tls[77ce614b43bc26b9]::ScopedKey<rustc_span[d0c85c35704fd34a]::SessionGlobals>>::set::<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  48:     0x7f023e418189 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  49:     0x7f023e41d876 - std[41ffe0caf4ddeafb]::panic::catch_unwind::<core[9932f27d9333ac31]::panic::unwind_safe::AssertUnwindSafe<<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>
  50:     0x7f023e3c9d4a - <<std[41ffe0caf4ddeafb]::thread::Builder>::spawn_unchecked_<rustc_interface[39e98fe1b5933e6b]::util::run_in_thread_pool_with_globals<rustc_interface[39e98fe1b5933e6b]::interface::run_compiler<core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>, rustc_driver[449791d1a40e792e]::run_compiler::{closure#1}>::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9932f27d9333ac31]::result::Result<(), rustc_errors[26aae3f62dab3f62]::ErrorGuaranteed>>::{closure#1} as core[9932f27d9333ac31]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7f023d9b7d7e - std::sys::unix::thread::Thread::new::thread_start::hd199a9f43fbdddea
  52:     0x7f023d74eb43 - <unknown>
  53:     0x7f023d7e0a00 - <unknown>
  54:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (90f960f09 2022-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `<impl at /checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs:14:1: 14:62>` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-100313.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-100313.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-100313" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-100313/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:536:17: cannot relate region: LUB(ReFree(DefId(0:8 ~ issue_100313[00b2]::{impl#0}::set_false), BrNamed(DefId(0:13 ~ issue_100313[00b2]::{impl#0}::set_false::'_), '_)), ReErased)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
   0:     0x7f53b083ad8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   0:     0x7f53b083ad8e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h97d87316d1a510bf
   1:     0x7f53b08aa628 - core::fmt::write::h49960c28f6893d6a
   2:     0x7f53b082cb51 - std::io::Write::write_fmt::h102a2b4e994607c6
   3:     0x7f53b083ab91 - std::sys_common::backtrace::print::h98a61b07d2a493a8
   4:     0x7f53b083def4 - std::panicking::default_hook::{{closure}}::h8265366696d19d7d
   5:     0x7f53b083dbb9 - std::panicking::default_hook::h758ba32b28879bcc
   6:     0x7f53b124c4e4 - rustc_driver[449791d1a40e792e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f53b083e644 - std::panicking::rust_panic_with_hook::h29df597bad3a0157
   8:     0x7f53b3e95843 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}
   9:     0x7f53b3e8e7f6 - std[41ffe0caf4ddeafb]::sys_common::backtrace::__rust_end_short_backtrace::<std[41ffe0caf4ddeafb]::panicking::begin_panic<rustc_errors[26aae3f62dab3f62]::ExplicitBug>::{closure#0}, !>
  10:     0x7f53b11f0f56 - std[41ffe0caf4ddeafb]::panicking::begin_panic::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  11:     0x7f53b3f4f0e6 - std[41ffe0caf4ddeafb]::panic::panic_any::<rustc_errors[26aae3f62dab3f62]::ExplicitBug>
  12:     0x7f53b3f46b37 - <rustc_errors[26aae3f62dab3f62]::HandlerInner>::bug::<&alloc[8ddb1744edabbe8]::string::String>
  13:     0x7f53b3f42bb0 - <rustc_errors[26aae3f62dab3f62]::Handler>::bug::<&alloc[8ddb1744edabbe8]::string::String>
