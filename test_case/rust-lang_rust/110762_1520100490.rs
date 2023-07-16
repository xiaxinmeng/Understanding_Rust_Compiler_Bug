plain
....F..........................................

failures:

---- [ui] tests/rustdoc-ui/issue-79468-too-big-current-arch.rs stdout ----


17               8: <rustc_session::session::Session>::time::<(), rustdoc::core::run_global_ctxt::{closure#1}>
18               9: rustdoc::core::run_global_ctxt
19              10: <rustc_session::session::Session>::time::<(rustdoc::clean::types::Crate, rustdoc::config::RenderOptions, rustdoc::formats::cache::Cache), rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
-              11: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
+              11: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
21              12: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}>
-              13: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}::{closure#0}>
-              14: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
-              15: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
-              16: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
-              18: start_thread
-              18: start_thread
-                         at ./nptl/pthread_create.c:442:8
-              19: clone3
-                         at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
+              13: <rustc_interface::interface::Compiler>::enter::<rustdoc::main_args::{closure#1}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
+              14: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}::{closure#0}>
+              15: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
+              16: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
+              17: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+              19: <unknown>
+              20: <unknown>
31            
32 
32 
33 note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

34 
- note: rustc 1.71.0-dev running on x86_64-unknown-linux-gnu
+ note: rustc 1.71.0-nightly (d23a87433 2023-04-24) running on x86_64-unknown-linux-gnu
36 
37 note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C debuginfo=0


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-79468-too-big-current-arch/issue-79468-too-big-current-arch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-79468-too-big-current-arch.rs`
error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issue-79468-too-big-current-arch.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-79468-too-big-current-arch" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-79468-too-big-current-arch/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: values of the type `[u8; 2305843009213693951]` are too big for the current architecture
  --> /checkout/tests/rustdoc-ui/issue-79468-too-big-current-arch.rs:6:1
   |
LL | static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
              2: rustc_hir_analysis::check::check::check_static_inhabited
              2: rustc_hir_analysis::check::check::check_static_inhabited
              3: rustc_hir_analysis::check::check::check_item_type
              4: rustc_hir_analysis::check::check::check_mod_item_types
              5: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
              6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
              7: <rustc_middle::hir::map::Map>::for_each_module::<rustdoc::core::run_global_ctxt::{closure#1}::{closure#0}>
              8: <rustc_session::session::Session>::time::<(), rustdoc::core::run_global_ctxt::{closure#1}>
              9: rustdoc::core::run_global_ctxt
             10: <rustc_session::session::Session>::time::<(rustdoc::clean::types::Crate, rustdoc::config::RenderOptions, rustdoc::formats::cache::Cache), rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
             11: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             12: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}::{closure#0}::{closure#0}>
             13: <rustc_interface::interface::Compiler>::enter::<rustdoc::main_args::{closure#1}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             14: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}::{closure#0}>
             15: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             16: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             17: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustdoc::main_args::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             19: <unknown>
             20: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (d23a87433 2023-04-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

