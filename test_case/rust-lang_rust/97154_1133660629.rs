plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 68 tests
..............................F...FF.F.F..FFFFFF..F.........FF.F....
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs stdout ----
diff of stderr:
diff of stderr:

+ thread 'rustc' panicked at '`""` is not a valid identifier', compiler/rustc_expand/src/proc_macro_server.rs:332:13
+ stack backtrace:
+    0:     0x55d8ffa3e9cf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
+    1:     0x55d8fcf932f8 - core::fmt::write::hd71271a06e52550f
+    2:     0x55d8ffa39851 - std::io::Write::write_fmt::hd136a57582c4beec
+    3:     0x55d8ffa40e1e - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
+    4:     0x55d8ffa40b2c - std::panicking::default_hook::h86bb23435e1a6c97
+    5:     0x55d8fdab01e1 - rustc_driver[6000cff12f68d0f4]::DEFAULT_HOOK::{closure#0}::{closure#0}
+    6:     0x55d8ffa41463 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
+    7:     0x55d8ffa41294 - std::panicking::begin_panic_handler::{{closure}}::h68ac96524895d5a6
+    8:     0x55d8ffa3ee76 - std::sys_common::backtrace::__rust_end_short_backtrace::h4d56321a028e398a
+    9:     0x55d8ffa40fa2 - rust_begin_unwind
+   10:     0x55d8fcbe4cc3 - core::panicking::panic_fmt::h7cc45c490badb44f
+   11:     0x55d8fdbc790d - <rustc_expand[69c1f71429d16d3b]::proc_macro_server::Ident>::new
+   12:     0x55d8fdc356c5 - <proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc> as proc_macro[baa8798d495fc3ea]::bridge::server::Ident>::new
+   13:     0x55d8fdc66cd9 - <core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>> as proc_macro[baa8798d495fc3ea]::bridge::server::DispatcherTrait>::dispatch::{closure#33}> as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once
+   14:     0x55d8fdc4cfd8 - std[2967767f1d0f7689]::panicking::try::<proc_macro[baa8798d495fc3ea]::bridge::Marked<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Ident, proc_macro[baa8798d495fc3ea]::bridge::client::Ident>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>> as proc_macro[baa8798d495fc3ea]::bridge::server::DispatcherTrait>::dispatch::{closure#33}>>
+   15:     0x55d8fdc36345 - <proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>> as proc_macro[baa8798d495fc3ea]::bridge::server::DispatcherTrait>::dispatch
+   16:     0x55d8fdbd0c10 - <proc_macro[baa8798d495fc3ea]::bridge::closure::Closure<_, _> as core[80e3980e51bbab23]::convert::From<&mut _>>::from::call::<proc_macro[baa8798d495fc3ea]::bridge::buffer::Buffer<u8>, proc_macro[baa8798d495fc3ea]::bridge::buffer::Buffer<u8>, <proc_macro[baa8798d495fc3ea]::bridge::server::SameThread as proc_macro[baa8798d495fc3ea]::bridge::server::ExecutionStrategy>::run_bridge_and_client<fn(proc_macro[baa8798d495fc3ea]::TokenStream, proc_macro[baa8798d495fc3ea]::TokenStream) -> proc_macro[baa8798d495fc3ea]::TokenStream, proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>>>::{closure#0}>
+   17:     0x7ff15020e655 - proc_macro::bridge::client::Ident::new::h8a09b3172b6a8dee
+   18:     0x7ff1501dc530 - proc_macro_panic::panic_in_libproc_macro::hc269d2b211a94aaa
+   19:     0x7ff1501da86b - proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>::expand1::run::h0c694a83abe5b6cf
+   20:     0x55d8fdc2ce3d - <proc_macro[baa8798d495fc3ea]::bridge::client::Client<fn(proc_macro[baa8798d495fc3ea]::TokenStream) -> proc_macro[baa8798d495fc3ea]::TokenStream>>::run::<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc, proc_macro[baa8798d495fc3ea]::bridge::server::SameThread>
+   21:     0x55d8fdbd6e33 - <rustc_expand[69c1f71429d16d3b]::proc_macro::BangProcMacro as rustc_expand[69c1f71429d16d3b]::base::ProcMacro>::expand
+   22:     0x55d8fdc1f226 - <rustc_expand[69c1f71429d16d3b]::expand::MacroExpander>::fully_expand_fragment
+   23:     0x55d8fdc1dcf7 - <rustc_expand[69c1f71429d16d3b]::expand::MacroExpander>::expand_crate
+   24:     0x55d8fded6607 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand::{closure#1}>
+   25:     0x55d8fdeb8c55 - rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand
+   26:     0x55d8fdee545f - <rustc_interface[77c6dd46ea15cc59]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
+   27:     0x55d8fdea8253 - <rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion
+   28:     0x55d8fda4155c - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
+   29:     0x55d8fda2431f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
+   30:     0x55d8fda42929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
+   31:     0x55d8fda97bb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
+   32:     0x55d8fda43951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
+   33:     0x55d8fda988ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   34:     0x55d8ffa46f33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
+   35:     0x7ff153e7f609 - start_thread
+   36:     0x7ff153c53133 - clone
+   37:                0x0 - <unknown>
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.63.0-nightly (4b119d2eb 2022-05-21) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
+ query stack during panic:
+ end of query stack
1 error: proc macro panicked
2   --> $DIR/issue-76270-panic-in-libproc-macro.rs:15:1
---
To only update this specific test, also pass `--test-args issue-76270-panic-in-libproc-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`""` is not a valid identifier', compiler/rustc_expand/src/proc_macro_server.rs:332:13
stack backtrace:
   0:     0x55d8ffa3e9cf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x55d8fcf932f8 - core::fmt::write::hd71271a06e52550f
   2:     0x55d8ffa39851 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x55d8ffa40e1e - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x55d8ffa40b2c - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x55d8fdab01e1 - rustc_driver[6000cff12f68d0f4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x55d8ffa41463 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   7:     0x55d8ffa41294 - std::panicking::begin_panic_handler::{{closure}}::h68ac96524895d5a6
   8:     0x55d8ffa3ee76 - std::sys_common::backtrace::__rust_end_short_backtrace::h4d56321a028e398a
   9:     0x55d8ffa40fa2 - rust_begin_unwind
  10:     0x55d8fcbe4cc3 - core::panicking::panic_fmt::h7cc45c490badb44f
  11:     0x55d8fdbc790d - <rustc_expand[69c1f71429d16d3b]::proc_macro_server::Ident>::new
  12:     0x55d8fdc356c5 - <proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc> as proc_macro[baa8798d495fc3ea]::bridge::server::Ident>::new
  13:     0x55d8fdc66cd9 - <core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>> as proc_macro[baa8798d495fc3ea]::bridge::server::DispatcherTrait>::dispatch::{closure#33}> as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once
  14:     0x55d8fdc4cfd8 - std[2967767f1d0f7689]::panicking::try::<proc_macro[baa8798d495fc3ea]::bridge::Marked<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Ident, proc_macro[baa8798d495fc3ea]::bridge::client::Ident>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>> as proc_macro[baa8798d495fc3ea]::bridge::server::DispatcherTrait>::dispatch::{closure#33}>>
  15:     0x55d8fdc36345 - <proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>> as proc_macro[baa8798d495fc3ea]::bridge::server::DispatcherTrait>::dispatch
  16:     0x55d8fdbd0c10 - <proc_macro[baa8798d495fc3ea]::bridge::closure::Closure<_, _> as core[80e3980e51bbab23]::convert::From<&mut _>>::from::call::<proc_macro[baa8798d495fc3ea]::bridge::buffer::Buffer<u8>, proc_macro[baa8798d495fc3ea]::bridge::buffer::Buffer<u8>, <proc_macro[baa8798d495fc3ea]::bridge::server::SameThread as proc_macro[baa8798d495fc3ea]::bridge::server::ExecutionStrategy>::run_bridge_and_client<fn(proc_macro[baa8798d495fc3ea]::TokenStream, proc_macro[baa8798d495fc3ea]::TokenStream) -> proc_macro[baa8798d495fc3ea]::TokenStream, proc_macro[baa8798d495fc3ea]::bridge::server::Dispatcher<proc_macro[baa8798d495fc3ea]::bridge::server::MarkedTypes<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc>>>::{closure#0}>
  17:     0x7ff15020e655 - proc_macro::bridge::client::Ident::new::h8a09b3172b6a8dee
  18:     0x7ff1501dc530 - proc_macro_panic::panic_in_libproc_macro::hc269d2b211a94aaa
  19:     0x7ff1501da86b - proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>::expand1::run::h0c694a83abe5b6cf
  20:     0x55d8fdc2ce3d - <proc_macro[baa8798d495fc3ea]::bridge::client::Client<fn(proc_macro[baa8798d495fc3ea]::TokenStream) -> proc_macro[baa8798d495fc3ea]::TokenStream>>::run::<rustc_expand[69c1f71429d16d3b]::proc_macro_server::Rustc, proc_macro[baa8798d495fc3ea]::bridge::server::SameThread>
  21:     0x55d8fdbd6e33 - <rustc_expand[69c1f71429d16d3b]::proc_macro::BangProcMacro as rustc_expand[69c1f71429d16d3b]::base::ProcMacro>::expand
  22:     0x55d8fdc1f226 - <rustc_expand[69c1f71429d16d3b]::expand::MacroExpander>::fully_expand_fragment
  23:     0x55d8fdc1dcf7 - <rustc_expand[69c1f71429d16d3b]::expand::MacroExpander>::expand_crate
  24:     0x55d8fded6607 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand::{closure#1}>
  25:     0x55d8fdeb8c55 - rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand
  26:     0x55d8fdee545f - <rustc_interface[77c6dd46ea15cc59]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  27:     0x55d8fdea8253 - <rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion
  28:     0x55d8fda4155c - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  29:     0x55d8fda2431f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x55d8fda42929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  31:     0x55d8fda97bb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  32:     0x55d8fda43951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x55d8fda988ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x55d8ffa46f33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  35:     0x7ff153e7f609 - start_thread
  36:     0x7ff153c53133 - clone
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (4b119d2eb 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: proc macro panicked
  --> /checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs:15:1
  --> /checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs:15:1
   |
LL | proc_macro_panic::panic_in_libproc_macro!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: `""` is not a valid identifier
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "lint-me" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f836387697c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7f83638dcfd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7f8363866761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7f83638799ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7f83638795fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7f836387a181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7f835a8b5121 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7f835a8b4b64 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7f83571e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7f835a8bf291 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7f83571f310a - <lint_group_plugin_test::Pass as rustc_lint::passes::LateLintPass>::check_item::h7d37a641d36fbad1
  11:     0x55ece94b60f0 - <rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::LateLintPass>::check_item
  12:     0x55ece94401f1 - <rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects> as rustc_hir[673684864e30ed17]::intravisit::Visitor>::visit_nested_item
  13:     0x55ece944569c - rustc_hir[673684864e30ed17]::intravisit::walk_mod::<rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>>
  14:     0x55ece943b65e - rustc_lint[c9bf7f2aed3181d1]::late::late_lint_pass_crate::<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>
  15:     0x55ece943ac75 - rustc_lint[c9bf7f2aed3181d1]::late::late_lint_crate::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass>
  16:     0x55ece93dc6eb - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  17:     0x55ece93cea32 - rustc_data_structures[a74b9d04bae6537e]::sync::join::<rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  18:     0x55ece93e02e6 - std[2967767f1d0f7689]::panicking::try::<(), core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  19:     0x55ece93a9a2b - <core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}> as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once
  20:     0x55ece93de146 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}>
  21:     0x55ece93c2fc0 - rustc_interface[77c6dd46ea15cc59]::passes::analysis
  22:     0x55ece9f5a869 - rustc_query_system[4d591fe21c4d62dd]::query::plumbing::try_execute_query::<rustc_query_impl[17b79062a2d83ba3]::plumbing::QueryCtxt, rustc_query_system[4d591fe21c4d62dd]::query::caches::DefaultCache<(), core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>>
  23:     0x55ecea0354c2 - rustc_query_system[4d591fe21c4d62dd]::query::plumbing::get_query::<rustc_query_impl[17b79062a2d83ba3]::queries::analysis, rustc_query_impl[17b79062a2d83ba3]::plumbing::QueryCtxt>
  24:     0x55ecea0d567d - <rustc_query_impl[17b79062a2d83ba3]::Queries as rustc_middle[58e7890e65380cbc]::ty::query::QueryEngine>::analysis
  25:     0x55ece8f98f23 - <rustc_interface[77c6dd46ea15cc59]::passes::QueryContext>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  26:     0x55ece8f466f8 - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  27:     0x55ece8f2931f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  28:     0x55ece8f47929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  29:     0x55ece8f9cbb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  30:     0x55ece8f48951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x55ece8f9d8ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x55eceaf4bf33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  33:     0x7f8364753609 - start_thread
  34:     0x7f8364527133 - clone
  35:                0x0 - <unknown>
panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-group-plugin.rs stdout ----
---- [ui] src/test/ui-fulldeps/lint-group-plugin.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7fa8349f397c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7fa834a59fd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7fa8349e3761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7fa8349f69ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7fa8349f65fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7fa8349f7181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7fa82e8b5121 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7fa82e8b4b64 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7fa82b1e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7fa82e8bf291 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7fa82b1f310a - <lint_group_plugin_test::Pass as rustc_lint::passes::LateLintPass>::check_item::h7d37a641d36fbad1
  11:     0x55adfef920f0 - <rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::LateLintPass>::check_item
  12:     0x55adfef1c1f1 - <rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects> as rustc_hir[673684864e30ed17]::intravisit::Visitor>::visit_nested_item
  13:     0x55adfef2169c - rustc_hir[673684864e30ed17]::intravisit::walk_mod::<rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>>
  14:     0x55adfef1765e - rustc_lint[c9bf7f2aed3181d1]::late::late_lint_pass_crate::<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>
  15:     0x55adfef16c75 - rustc_lint[c9bf7f2aed3181d1]::late::late_lint_crate::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass>
  16:     0x55adfeeb86eb - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  17:     0x55adfeeaaa32 - rustc_data_structures[a74b9d04bae6537e]::sync::join::<rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  18:     0x55adfeebc2e6 - std[2967767f1d0f7689]::panicking::try::<(), core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  19:     0x55adfee85a2b - <core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}> as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once
  20:     0x55adfeeba146 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}>
  21:     0x55adfee9efc0 - rustc_interface[77c6dd46ea15cc59]::passes::analysis
  22:     0x55adffa36869 - rustc_query_system[4d591fe21c4d62dd]::query::plumbing::try_execute_query::<rustc_query_impl[17b79062a2d83ba3]::plumbing::QueryCtxt, rustc_query_system[4d591fe21c4d62dd]::query::caches::DefaultCache<(), core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>>
  23:     0x55adffb114c2 - rustc_query_system[4d591fe21c4d62dd]::query::plumbing::get_query::<rustc_query_impl[17b79062a2d83ba3]::queries::analysis, rustc_query_impl[17b79062a2d83ba3]::plumbing::QueryCtxt>
  24:     0x55adffbb167d - <rustc_query_impl[17b79062a2d83ba3]::Queries as rustc_middle[58e7890e65380cbc]::ty::query::QueryEngine>::analysis
  25:     0x55adfea74f23 - <rustc_interface[77c6dd46ea15cc59]::passes::QueryContext>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  26:     0x55adfea226f8 - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  27:     0x55adfea0531f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  28:     0x55adfea23929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  29:     0x55adfea78bb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  30:     0x55adfea24951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x55adfea798ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x55ae00a27f33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  33:     0x7fa8358d0609 - start_thread
  34:     0x7fa8356a4133 - clone
  35:                0x0 - <unknown>
panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
---- [ui] src/test/ui-fulldeps/lint-plugin-deny-attr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7fbb912ca97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7fbb91330fd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7fbb912ba761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7fbb912cd9ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7fbb912cd5fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7fbb912ce181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7fbb8a8b4ee1 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7fbb8a8b4924 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7fbb871e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7fbb8a8bf051 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7fbb871f303d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::hdd43f31834213431
  11:     0x562e59a95f10 - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::EarlyLintPass>::check_item
  12:     0x562e599949ff - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects> as rustc_ast[8f0a14d5061c8a1c]::visit::Visitor>::visit_item
  13:     0x562e599e7f0f - rustc_ast[8f0a14d5061c8a1c]::visit::walk_crate::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects>>
  14:     0x562e59993798 - rustc_lint[c9bf7f2aed3181d1]::early::early_lint_node::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  15:     0x562e599921ac - rustc_lint[c9bf7f2aed3181d1]::early::check_ast_node::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedEarlyLintPass, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  16:     0x562e599be001 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand::{closure#8}>
  17:     0x562e5999f81a - rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand
  18:     0x562e599cb45f - <rustc_interface[77c6dd46ea15cc59]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  19:     0x562e5998e253 - <rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion
  20:     0x562e5952755c - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  21:     0x562e5950a31f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  22:     0x562e59528929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  23:     0x562e5957dbb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  24:     0x562e59529951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  25:     0x562e5957e8ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x562e5b52cf33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  27:     0x7fbb921a7609 - start_thread
  28:     0x7fbb91f7b133 - clone
  29:                0x0 - <unknown>

panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
panicked after panic::always_abort(), aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f37ccfc597c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7f37cd02bfd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7f37ccfb5761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7f37ccfc89ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7f37ccfc85fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7f37ccfc9181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7f37c68b4ee1 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7f37c68b4924 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7f37c31e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7f37c68bf051 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7f37c31f303d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::hdd43f31834213431
  11:     0x562e09261f10 - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::EarlyLintPass>::check_item
  12:     0x562e091609ff - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects> as rustc_ast[8f0a14d5061c8a1c]::visit::Visitor>::visit_item
  13:     0x562e091b3f0f - rustc_ast[8f0a14d5061c8a1c]::visit::walk_crate::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects>>
  14:     0x562e0915f798 - rustc_lint[c9bf7f2aed3181d1]::early::early_lint_node::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  15:     0x562e0915e1ac - rustc_lint[c9bf7f2aed3181d1]::early::check_ast_node::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedEarlyLintPass, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  16:     0x562e0918a001 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand::{closure#8}>
  17:     0x562e0916b81a - rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand
  18:     0x562e0919745f - <rustc_interface[77c6dd46ea15cc59]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  19:     0x562e0915a253 - <rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion
  20:     0x562e08cf355c - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  21:     0x562e08cd631f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  22:     0x562e08cf4929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  23:     0x562e08d49bb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  24:     0x562e08cf5951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  25:     0x562e08d4a8ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x562e0acf8f33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  27:     0x7f37cdea2609 - start_thread
  28:     0x7f37cdc76133 - clone
  29:                0x0 - <unknown>

panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
panicked after panic::always_abort(), aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "crate-attr=plugin(lint_plugin_test)" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> <crate attribute>:1:1
   |
LL | plugin(lint_plugin_test)
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f0bcb63897c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7f0bcb69efd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7f0bcb628761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7f0bcb63b9ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7f0bcb63b5fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7f0bcb63c181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7f0bc28b4ee1 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7f0bc28b4924 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7f0bbf1e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7f0bc28bf051 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7f0bbf1f303d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::hdd43f31834213431
  11:     0x555845986f10 - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::EarlyLintPass>::check_item
  12:     0x5558458859ff - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects> as rustc_ast[8f0a14d5061c8a1c]::visit::Visitor>::visit_item
  13:     0x5558458d8f0f - rustc_ast[8f0a14d5061c8a1c]::visit::walk_crate::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects>>
  14:     0x555845884798 - rustc_lint[c9bf7f2aed3181d1]::early::early_lint_node::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  15:     0x5558458831ac - rustc_lint[c9bf7f2aed3181d1]::early::check_ast_node::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedEarlyLintPass, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  16:     0x5558458af001 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand::{closure#8}>
  17:     0x55584589081a - rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand
  18:     0x5558458bc45f - <rustc_interface[77c6dd46ea15cc59]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  19:     0x55584587f253 - <rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion
  20:     0x55584541855c - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  21:     0x5558453fb31f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  22:     0x555845419929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  23:     0x55584546ebb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  24:     0x55584541a951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  25:     0x55584546f8ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x55584741df33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  27:     0x7f0bcc515609 - start_thread
  28:     0x7f0bcc2e9133 - clone
  29:                0x0 - <unknown>

panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
panicked after panic::always_abort(), aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/issue-40001.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-40001.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(issue_40001_plugin)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f5627ec197c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7f5627f27fd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7f5627eb1761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7f5627ec49ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7f5627ec45fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7f5627ec5181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7f561e8b5501 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7f561e8b4f44 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7f561b1e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7f561e8bf671 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7f561e89eab8 - <rustc_span[ca43e1b058343f42]::symbol::IdentPrinter as core[80e3980e51bbab23]::fmt::Display>::fmt
  11:     0x7f561e7f4d54 - <rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::State as rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::PrintState>::print_ident
  12:     0x7f561e7f2b92 - <rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::State as rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::PrintState>::print_path
  13:     0x7f561e7f214e - <rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::State as rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::PrintState>::print_attr_item
  14:     0x7f561e7f204a - <rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::State as rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::PrintState>::print_attribute_inline
  15:     0x7f561e7f4067 - <rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::State as rustc_ast_pretty[31c7b4af11e440f8]::pprust::state::PrintState>::attribute_to_string
  16:     0x7f561e81fa5c - rustc_ast_pretty[31c7b4af11e440f8]::pprust::attribute_to_string
  17:     0x7f561b1f315c - <issue_40001_plugin::MissingAllowedAttrPass as rustc_lint::passes::LateLintPass>::check_fn::h3baabd3bc6506634
  18:     0x56526611172b - <rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::LateLintPass>::check_fn
  19:     0x56526609a766 - <rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects> as rustc_hir[673684864e30ed17]::intravisit::Visitor>::visit_fn
  20:     0x5652660a1978 - rustc_hir[673684864e30ed17]::intravisit::walk_item::<rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>>
  21:     0x56526609b1fc - <rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects> as rustc_hir[673684864e30ed17]::intravisit::Visitor>::visit_nested_item
  22:     0x5652660a069c - rustc_hir[673684864e30ed17]::intravisit::walk_mod::<rustc_lint[c9bf7f2aed3181d1]::late::LateContextAndPass<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>>
  23:     0x56526609665e - rustc_lint[c9bf7f2aed3181d1]::late::late_lint_pass_crate::<rustc_lint[c9bf7f2aed3181d1]::late::LateLintPassObjects>
  24:     0x565266095c75 - rustc_lint[c9bf7f2aed3181d1]::late::late_lint_crate::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass>
  25:     0x5652660376eb - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  26:     0x565266029a32 - rustc_data_structures[a74b9d04bae6537e]::sync::join::<rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[c9bf7f2aed3181d1]::late::check_crate<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedLateLintPass, rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  27:     0x56526603b2e6 - std[2967767f1d0f7689]::panicking::try::<(), core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  28:     0x565266004a2b - <core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}::{closure#1}> as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once
  29:     0x565266039146 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::analysis::{closure#5}>
  30:     0x56526601dfc0 - rustc_interface[77c6dd46ea15cc59]::passes::analysis
  31:     0x565266bb5869 - rustc_query_system[4d591fe21c4d62dd]::query::plumbing::try_execute_query::<rustc_query_impl[17b79062a2d83ba3]::plumbing::QueryCtxt, rustc_query_system[4d591fe21c4d62dd]::query::caches::DefaultCache<(), core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>>
  32:     0x565266c904c2 - rustc_query_system[4d591fe21c4d62dd]::query::plumbing::get_query::<rustc_query_impl[17b79062a2d83ba3]::queries::analysis, rustc_query_impl[17b79062a2d83ba3]::plumbing::QueryCtxt>
  33:     0x565266d3067d - <rustc_query_impl[17b79062a2d83ba3]::Queries as rustc_middle[58e7890e65380cbc]::ty::query::QueryEngine>::analysis
  34:     0x565265bf3f23 - <rustc_interface[77c6dd46ea15cc59]::passes::QueryContext>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  35:     0x565265ba16f8 - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  36:     0x565265b8431f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x565265ba2929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  38:     0x565265bf7bb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  39:     0x565265ba3951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x565265bf88ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x565267ba6f33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  42:     0x7f5628d9e609 - start_thread
  43:     0x7f5628b72133 - clone
  44:                0x0 - <unknown>
panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
---- [ui] src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f3bb944897c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ec473e9c5e7550
   1:     0x7f3bb94aefd8 - core::fmt::write::hd71271a06e52550f
   2:     0x7f3bb9438761 - std::io::Write::write_fmt::hd136a57582c4beec
   3:     0x7f3bb944b9ce - std::panicking::default_hook::{{closure}}::h2ce79bccd8f23743
   4:     0x7f3bb944b5fc - std::panicking::default_hook::h86bb23435e1a6c97
   5:     0x7f3bb944c181 - std::panicking::rust_panic_with_hook::hc45eb5dee527bf18
   6:     0x7f3bb28b4ee1 - std[2967767f1d0f7689]::panicking::begin_panic::<&str>::{closure#0}
   7:     0x7f3bb28b4924 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_end_short_backtrace::<std[2967767f1d0f7689]::panicking::begin_panic<&str>::{closure#0}, !>
   8:     0x7f3baf1e908c - std[2967767f1d0f7689]::panicking::begin_panic::<&str>
   9:     0x7f3bb28bf051 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::with::<<rustc_span[ca43e1b058343f42]::symbol::Symbol>::as_str::{closure#0}, &str>
  10:     0x7f3baf1f303d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::hdd43f31834213431
  11:     0x5575825a7f10 - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects as rustc_lint[c9bf7f2aed3181d1]::passes::EarlyLintPass>::check_item
  12:     0x5575824a69ff - <rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects> as rustc_ast[8f0a14d5061c8a1c]::visit::Visitor>::visit_item
  13:     0x5575824f9f0f - rustc_ast[8f0a14d5061c8a1c]::visit::walk_crate::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyContextAndPass<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects>>
  14:     0x5575824a5798 - rustc_lint[c9bf7f2aed3181d1]::early::early_lint_node::<rustc_lint[c9bf7f2aed3181d1]::early::EarlyLintPassObjects, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  15:     0x5575824a41ac - rustc_lint[c9bf7f2aed3181d1]::early::check_ast_node::<rustc_lint[c9bf7f2aed3181d1]::BuiltinCombinedEarlyLintPass, &rustc_ast[8f0a14d5061c8a1c]::ast::Crate>
  16:     0x5575824d0001 - <rustc_session[2d3cd4a3d329c51e]::session::Session>::time::<(), rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand::{closure#8}>
  17:     0x5575824b181a - rustc_interface[77c6dd46ea15cc59]::passes::configure_and_expand
  18:     0x5575824dd45f - <rustc_interface[77c6dd46ea15cc59]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[80e3980e51bbab23]::result::Result<rustc_ast[8f0a14d5061c8a1c]::ast::Crate, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  19:     0x5575824a0253 - <rustc_interface[77c6dd46ea15cc59]::queries::Queries>::expansion
  20:     0x55758203955c - <rustc_interface[77c6dd46ea15cc59]::interface::Compiler>::enter::<rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}::{closure#2}, core[80e3980e51bbab23]::result::Result<core[80e3980e51bbab23]::option::Option<rustc_interface[77c6dd46ea15cc59]::queries::Linker>, rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  21:     0x55758201c31f - rustc_span[ca43e1b058343f42]::with_source_map::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_interface[77c6dd46ea15cc59]::interface::create_compiler_and_run<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#1}>
  22:     0x55758203a929 - <scoped_tls[51dd54f6fcd03674]::ScopedKey<rustc_span[ca43e1b058343f42]::SessionGlobals>>::set::<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  23:     0x55758208fbb6 - std[2967767f1d0f7689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>
  24:     0x55758203b951 - std[2967767f1d0f7689]::panicking::try::<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, core[80e3980e51bbab23]::panic::unwind_safe::AssertUnwindSafe<<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  25:     0x5575820908ad - <<std[2967767f1d0f7689]::thread::Builder>::spawn_unchecked_<rustc_interface[77c6dd46ea15cc59]::util::run_in_thread_pool_with_globals<rustc_interface[77c6dd46ea15cc59]::interface::run_compiler<core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>, rustc_driver[6000cff12f68d0f4]::run_compiler::{closure#1}>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#0}, core[80e3980e51bbab23]::result::Result<(), rustc_errors[da621a2e06366e16]::ErrorGuaranteed>>::{closure#1} as core[80e3980e51bbab23]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x55758403ef33 - std::sys::unix::thread::Thread::new::thread_start::h64976e4d5164c806
  27:     0x7f3bba325609 - start_thread
  28:     0x7f3bba0f9133 - clone
  29:                0x0 - <unknown>

panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
panicked after panic::always_abort(), aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
