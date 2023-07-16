plain
To only update this specific test, also pass `--test-args panics/default-backtrace-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/panics/default-backtrace-ice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice/auxiliary" "-Z" "treat-err-as-bug=1"
stdout: none
error[E0425]: cannot find value `missing_ident` in this scope
  --> fake-test-src-base/panics/default-backtrace-ice.rs:12:13
   |
   |
LL | fn main() { missing_ident; }


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1704:30
   0:     0x7fb1347f0444 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2d5b46917fd7c1ed
   1:     0x7fb1348578d8 - core::fmt::write::h0326bea729dd2e4f
   2:     0x7fb1347e4b71 - std::io::Write::write_fmt::hde6155503cbac18a
   3:     0x7fb1347f0251 - std::sys_common::backtrace::print::hc54536986b35dc03
   3:     0x7fb1347f0251 - std::sys_common::backtrace::print::hc54536986b35dc03
   4:     0x7fb1347f331a - std::panicking::default_hook::{{closure}}::h3c55f2cdb17c82a4
   5:     0x7fb1347f2ffc - std::panicking::default_hook::h9cb69558157953b5
   6:     0x7fb1352b2025 - rustc_driver_impl[41e4a4b88c7cd28a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb1347f3a27 - std::panicking::rust_panic_with_hook::ha9f525eeccc654a4
   8:     0x7fb1347f3769 - std::panicking::begin_panic_handler::{{closure}}::h353586815c3a5961
   9:     0x7fb1347f0926 - std::sys_common::backtrace::__rust_end_short_backtrace::h6516934dfb0c0d26
  10:     0x7fb1347f3497 - rust_begin_unwind
  11:     0x7fb1347a80c3 - core::panicking::panic_fmt::hf31168387c77c577
  12:     0x7fb13821afd7 - <rustc_errors[ad71752c5da4a063]::HandlerInner>::panic_if_treat_err_as_bug
  13:     0x7fb13821a46c - <rustc_errors[ad71752c5da4a063]::HandlerInner>::emit_diagnostic::{closure#2}
  14:     0x7fb1353d6d8e - rustc_interface[4a020681163ff955]::callbacks::track_diagnostic
  15:     0x7fb138219c27 - <rustc_errors[ad71752c5da4a063]::HandlerInner>::emit_diagnostic
  16:     0x7fb138218c2e - <rustc_errors[ad71752c5da4a063]::Handler>::emit_diagnostic
  17:     0x7fb13822380a - <rustc_span[8626ceb64d14dea3]::ErrorGuaranteed as rustc_errors[ad71752c5da4a063]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7fb136374959 - <rustc_resolve[fbd0decce00f8d6d]::Resolver>::report_errors
  19:     0x7fb13633964e - <rustc_session[3b30dbb5a3409505]::session::Session>::time::<(), <rustc_resolve[fbd0decce00f8d6d]::Resolver>::resolve_crate::{closure#0}>
  20:     0x7fb1363a038d - <rustc_resolve[fbd0decce00f8d6d]::Resolver>::resolve_crate
  21:     0x7fb13539b12f - rustc_interface[4a020681163ff955]::passes::resolver_for_lowering
  22:     0x7fb137014d62 - rustc_query_system[cb9f4236f8efd56c]::query::plumbing::try_execute_query::<rustc_query_impl[24b994aa85de3ed1]::queries::resolver_for_lowering, rustc_query_impl[24b994aa85de3ed1]::plumbing::QueryCtxt>
  23:     0x7fb136e08dcb - rustc_query_impl[24b994aa85de3ed1]::get_query::resolver_for_lowering
  24:     0x7fb1352ce78a - <rustc_middle[19f3f973d0ca6bed]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[b5ade3da13c18b6e]::steal::Steal<(rustc_middle[19f3f973d0ca6bed]::ty::ResolverAstLowering, alloc[a5d2b40a1d2b5b77]::rc::Rc<rustc_ast[d41a580110214da]::ast::Crate>)>>
  25:     0x7fb1352fd330 - <rustc_interface[4a020681163ff955]::interface::Compiler>::enter::<rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}::{closure#2}, core[b4036e1c62e3930]::result::Result<core[b4036e1c62e3930]::option::Option<rustc_interface[4a020681163ff955]::queries::Linker>, rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>
  26:     0x7fb1352cd740 - rustc_span[8626ceb64d14dea3]::set_source_map::<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, rustc_interface[4a020681163ff955]::interface::run_compiler<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7fb1352bf879 - <scoped_tls[be192a3cd81f6d6c]::ScopedKey<rustc_span[8626ceb64d14dea3]::SessionGlobals>>::set::<rustc_interface[4a020681163ff955]::interface::run_compiler<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}>::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>
  28:     0x7fb1352d7d06 - std[5bb3ae564779d704]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4a020681163ff955]::util::run_in_thread_pool_with_globals<rustc_interface[4a020681163ff955]::interface::run_compiler<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}>::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>
  29:     0x7fb135315898 - std[5bb3ae564779d704]::panicking::try::<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, core[b4036e1c62e3930]::panic::unwind_safe::AssertUnwindSafe<<std[5bb3ae564779d704]::thread::Builder>::spawn_unchecked_<rustc_interface[4a020681163ff955]::util::run_in_thread_pool_with_globals<rustc_interface[4a020681163ff955]::interface::run_compiler<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}>::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x7fb1352c59fd - <<std[5bb3ae564779d704]::thread::Builder>::spawn_unchecked_<rustc_interface[4a020681163ff955]::util::run_in_thread_pool_with_globals<rustc_interface[4a020681163ff955]::interface::run_compiler<core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>, rustc_driver_impl[41e4a4b88c7cd28a]::run_compiler::{closure#1}>::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4036e1c62e3930]::result::Result<(), rustc_span[8626ceb64d14dea3]::ErrorGuaranteed>>::{closure#1} as core[b4036e1c62e3930]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7fb1348003de - std::sys::unix::thread::Thread::new::thread_start::h6025348d5d8f9633
  32:     0x7fb134598b43 - <unknown>
  33:     0x7fb13462aa00 - <unknown>
  34:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (7b39875b3 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
------------------------------------------
