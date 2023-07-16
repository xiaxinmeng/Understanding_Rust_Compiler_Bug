plain
...............................................

failures:

---- [ui] tests/rustdoc-ui/ice-bug-report-url.rs stdout ----

6 
6 
7 thread panicked at 'aborting due to `-Z treat-err-as-bug`'
-                                           
+                                         
10 error: the compiler unexpectedly panicked. this is a bug.
11 
11 
12 note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-rustdoc&template=ice.md

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/ice-bug-report-url/ice-bug-report-url.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args ice-bug-report-url.rs`
error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/ice-bug-report-url.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/ice-bug-report-url" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/ice-bug-report-url/auxiliary" "-Ztreat-err-as-bug"
stdout: none
--- stderr -------------------------------
error: expected one of `->`, `where`, or `{`, found `<eof>`
  --> /checkout/tests/rustdoc-ui/ice-bug-report-url.rs:13:10
LL | fn wrong()
LL | fn wrong()
   |          ^ expected one of `->`, `where`, or `{`

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1704:30
   0:     0x7f33f8b0e444 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5135ec782315f01c
   1:     0x7f33f8b758d8 - core::fmt::write::h1f03989bc495ebb0
   2:     0x7f33f8b02ad1 - std::io::Write::write_fmt::h8a0f275b422a3529
   3:     0x7f33f8b0e251 - std::sys_common::backtrace::print::hfd58df1fb131e0ae
   3:     0x7f33f8b0e251 - std::sys_common::backtrace::print::hfd58df1fb131e0ae
   4:     0x7f33f8b1131a - std::panicking::default_hook::{{closure}}::h58a2b073cfc6aba5
   5:     0x7f33f8b10ffc - std::panicking::default_hook::ha7bcec9f9d7e05b1
   6:     0x7f33f95da9a7 - rustc_driver_impl[7435884ac0a8d547]::install_ice_hook::{closure#0}
   7:     0x7f33f8b11a27 - std::panicking::rust_panic_with_hook::hf5bef3f0e3391b07
   8:     0x7f33f8b11769 - std::panicking::begin_panic_handler::{{closure}}::hb19443e7e8bcda6e
   9:     0x7f33f8b0e926 - std::sys_common::backtrace::__rust_end_short_backtrace::h1bdc685b70dd6114
  10:     0x7f33f8b11497 - rust_begin_unwind
  11:     0x7f33f8ac60c3 - core::panicking::panic_fmt::hcf60805b8331c793
  12:     0x7f33fc5a54c7 - <rustc_errors[6ab3cdf1e73cc7ab]::HandlerInner>::panic_if_treat_err_as_bug
  13:     0x7f33fc5a495c - <rustc_errors[6ab3cdf1e73cc7ab]::HandlerInner>::emit_diagnostic::{closure#2}
  14:     0x7f33fc5a4117 - <rustc_errors[6ab3cdf1e73cc7ab]::HandlerInner>::emit_diagnostic
  15:     0x7f33fc5a311e - <rustc_errors[6ab3cdf1e73cc7ab]::Handler>::emit_diagnostic
  16:     0x7f33fc5adcfa - <rustc_span[e97969e431f9bf3a]::ErrorGuaranteed as rustc_errors[6ab3cdf1e73cc7ab]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  17:     0x7f33f97715a6 - <rustc_interface[d821def2a3da0340]::queries::Queries>::parse
  18:     0x7f33f97717b8 - <rustc_interface[d821def2a3da0340]::queries::Queries>::pre_configure
  19:     0x7f33f97722b1 - <rustc_interface[d821def2a3da0340]::queries::Queries>::crate_name
  20:     0x7f33f97725e4 - <rustc_interface[d821def2a3da0340]::queries::Queries>::global_ctxt
  21:     0x55e148b47a4f - <rustc_interface[d821def2a3da0340]::interface::Compiler>::enter::<rustdoc[145bdc2920d56c81]::main_args::{closure#1}::{closure#0}, core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>>
  22:     0x55e148a727dc - rustc_span[e97969e431f9bf3a]::set_source_map::<core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>, rustc_interface[d821def2a3da0340]::interface::run_compiler<core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>, rustdoc[145bdc2920d56c81]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  23:     0x55e148a3a6e5 - <scoped_tls[6c4c56a4329163a1]::ScopedKey<rustc_span[e97969e431f9bf3a]::SessionGlobals>>::set::<rustc_interface[d821def2a3da0340]::interface::run_compiler<core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>, rustdoc[145bdc2920d56c81]::main_args::{closure#1}>::{closure#0}, core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>>
  24:     0x55e14890b7e6 - std[b2d3a7e015c59133]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d821def2a3da0340]::util::run_in_thread_pool_with_globals<rustc_interface[d821def2a3da0340]::interface::run_compiler<core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>, rustdoc[145bdc2920d56c81]::main_args::{closure#1}>::{closure#0}, core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>>
  25:     0x55e148a04179 - <<std[b2d3a7e015c59133]::thread::Builder>::spawn_unchecked_<rustc_interface[d821def2a3da0340]::util::run_in_thread_pool_with_globals<rustc_interface[d821def2a3da0340]::interface::run_compiler<core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>, rustdoc[145bdc2920d56c81]::main_args::{closure#1}>::{closure#0}, core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[d8b910e1c7170c38]::result::Result<(), rustc_span[e97969e431f9bf3a]::ErrorGuaranteed>>::{closure#1} as core[d8b910e1c7170c38]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7f33f8b1e3de - std::sys::unix::thread::Thread::new::thread_start::hb37e87cef420cc72
  27:     0x7f33f87adb43 - <unknown>
  28:     0x7f33f883fa00 - <unknown>
  29:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-rustdoc&template=ice.md
note: rustc 1.71.0-nightly (edcd1b41d 2023-04-29) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
end of query stack
------------------------------------------

