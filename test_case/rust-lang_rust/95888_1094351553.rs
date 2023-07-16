plain

running 517 tests
i........................................................i.............................. 88/517
........................................................................................ 176/517
............F.........................F................................................. 264/517
.................................................................i...................... 440/517
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.............................................................................
failures:
failures:

---- [rustdoc] src/test/rustdoc/intra-doc/extern-inherent-impl.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/extern-inherent-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/extern-inherent-impl" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/extern-inherent-impl.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:976:16
stack backtrace:
   0:     0x7f23a98d7e5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0259eb7ea130c502
   1:     0x7f23a993b288 - core::fmt::write::h25faaa8b384afd90
   2:     0x7f23a98c81d1 - std::io::Write::write_fmt::h17f3e200c6859051
   3:     0x7f23a98db18e - std::panicking::default_hook::{{closure}}::h24abedac9c2955f7
   4:     0x7f23a98dadc4 - std::panicking::default_hook::hf0e8ad103916b99e
   5:     0x7f23aa431051 - rustc_driver[e0940eebf7173345]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f23a98db9bb - std::panicking::rust_panic_with_hook::hd532a47d814e4b19
   7:     0x7f23a98db7e7 - std::panicking::begin_panic_handler::{{closure}}::he1512fa7b0997b6a
   8:     0x7f23a98d8374 - std::sys_common::backtrace::__rust_end_short_backtrace::h9dc04897e7540c1f
   9:     0x7f23a98db4c9 - rust_begin_unwind
  10:     0x7f23a988ed73 - core::panicking::panic_fmt::h13203a458c0480b3
  11:     0x7f23a9937731 - core::panicking::panic_display::hd964c074a3409f26
  12:     0x7f23a99376db - core::panicking::panic_str::hf1a425be5f1386ba
  13:     0x7f23a988ebe6 - core::option::expect_failed::h2bb89b036003ffda
  14:     0x55b225e689c5 - rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::resolve_associated_trait_item
  15:     0x55b225e66ade - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
  16:     0x55b225e64228 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector>::resolve
  17:     0x55b225e6d1b1 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector>::resolve_link
  18:     0x55b225e690a8 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  19:     0x55b225e7814a - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_inner_recur
  20:     0x55b225e693fe - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  21:     0x55b225e780ca - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_inner_recur
  22:     0x55b225e69621 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  23:     0x55b225e628d6 - rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::collect_intra_doc_links
  24:     0x55b225ec0d05 - <rustc_session[2dd17ae08490524c]::session::Session>::time::<(rustdoc[e05fa81b99b89a0c]::clean::types::Crate, rustdoc[e05fa81b99b89a0c]::config::RenderOptions, rustdoc[e05fa81b99b89a0c]::formats::cache::Cache), rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  25:     0x55b225f10ae4 - <rustc_interface[928cb8e13c9650fd]::passes::QueryContext>::enter::<rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}::{closure#0}::{closure#1}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  26:     0x55b225e197e9 - <rustc_interface[928cb8e13c9650fd]::interface::Compiler>::enter::<rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  27:     0x55b225fb7f01 - rustc_span[2db60de15832a138]::with_source_map::<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, rustc_interface[928cb8e13c9650fd]::interface::create_compiler_and_run<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}>::{closure#1}>
  28:     0x55b225e345d1 - rustc_interface[928cb8e13c9650fd]::interface::create_compiler_and_run::<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}>
  29:     0x55b225d6343c - rustdoc[e05fa81b99b89a0c]::main_options
  30:     0x55b225fbab2c - <scoped_tls[2c01f44da361b9ad]::ScopedKey<rustc_span[2db60de15832a138]::SessionGlobals>>::set::<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  31:     0x55b225e34d19 - std[7a3eb7f2ac4b4d9b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[928cb8e13c9650fd]::util::run_in_thread_pool_with_globals<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  32:     0x55b225ec4991 - std[7a3eb7f2ac4b4d9b]::panicking::try::<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, core[8804424b9535ea4e]::panic::unwind_safe::AssertUnwindSafe<<std[7a3eb7f2ac4b4d9b]::thread::Builder>::spawn_unchecked_<rustc_interface[928cb8e13c9650fd]::util::run_in_thread_pool_with_globals<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x55b225f29be2 - <<std[7a3eb7f2ac4b4d9b]::thread::Builder>::spawn_unchecked_<rustc_interface[928cb8e13c9650fd]::util::run_in_thread_pool_with_globals<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#1} as core[8804424b9535ea4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f23a98e6753 - std::sys::unix::thread::Thread::new::thread_start::h702769e3b3171e19
  35:     0x7f23a97e7609 - start_thread
  36:     0x7f23a95bd163 - clone
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc] src/test/rustdoc/intra-doc/libstd-re-export.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/libstd-re-export.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:976:16
   0:     0x7f778faeae5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0259eb7ea130c502
   0:     0x7f778faeae5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0259eb7ea130c502
   1:     0x7f778fb4e288 - core::fmt::write::h25faaa8b384afd90
   2:     0x7f778fadb1d1 - std::io::Write::write_fmt::h17f3e200c6859051
   3:     0x7f778faee18e - std::panicking::default_hook::{{closure}}::h24abedac9c2955f7
   4:     0x7f778faeddc4 - std::panicking::default_hook::hf0e8ad103916b99e
   5:     0x7f7790644051 - rustc_driver[e0940eebf7173345]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f778faee9bb - std::panicking::rust_panic_with_hook::hd532a47d814e4b19
   7:     0x7f778faee7e7 - std::panicking::begin_panic_handler::{{closure}}::he1512fa7b0997b6a
   8:     0x7f778faeb374 - std::sys_common::backtrace::__rust_end_short_backtrace::h9dc04897e7540c1f
   9:     0x7f778faee4c9 - rust_begin_unwind
  10:     0x7f778faa1d73 - core::panicking::panic_fmt::h13203a458c0480b3
  11:     0x7f778fb4a731 - core::panicking::panic_display::hd964c074a3409f26
  12:     0x7f778fb4a6db - core::panicking::panic_str::hf1a425be5f1386ba
  13:     0x7f778faa1be6 - core::option::expect_failed::h2bb89b036003ffda
  14:     0x55ca8d8579c5 - rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::resolve_associated_trait_item
  15:     0x55ca8d855ade - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
  16:     0x55ca8d853228 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector>::resolve
  17:     0x55ca8d85c1b1 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector>::resolve_link
  18:     0x55ca8d8580a8 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  19:     0x55ca8d86714a - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_inner_recur
  20:     0x55ca8d8583fe - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  21:     0x55ca8d8670ca - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_inner_recur
  22:     0x55ca8d858621 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  23:     0x55ca8d8670ca - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_inner_recur
  24:     0x55ca8d858621 - <rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e05fa81b99b89a0c]::visit::DocVisitor>::visit_item
  25:     0x55ca8d8518d6 - rustdoc[e05fa81b99b89a0c]::passes::collect_intra_doc_links::collect_intra_doc_links
  26:     0x55ca8d8afd05 - <rustc_session[2dd17ae08490524c]::session::Session>::time::<(rustdoc[e05fa81b99b89a0c]::clean::types::Crate, rustdoc[e05fa81b99b89a0c]::config::RenderOptions, rustdoc[e05fa81b99b89a0c]::formats::cache::Cache), rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  27:     0x55ca8d8ffae4 - <rustc_interface[928cb8e13c9650fd]::passes::QueryContext>::enter::<rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}::{closure#0}::{closure#1}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  28:     0x55ca8d8087e9 - <rustc_interface[928cb8e13c9650fd]::interface::Compiler>::enter::<rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  29:     0x55ca8d9a6f01 - rustc_span[2db60de15832a138]::with_source_map::<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, rustc_interface[928cb8e13c9650fd]::interface::create_compiler_and_run<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}>::{closure#1}>
  30:     0x55ca8d8235d1 - rustc_interface[928cb8e13c9650fd]::interface::create_compiler_and_run::<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, rustdoc[e05fa81b99b89a0c]::main_options::{closure#0}>
  31:     0x55ca8d75243c - rustdoc[e05fa81b99b89a0c]::main_options
  32:     0x55ca8d9a9b2c - <scoped_tls[2c01f44da361b9ad]::ScopedKey<rustc_span[2db60de15832a138]::SessionGlobals>>::set::<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  33:     0x55ca8d823d19 - std[7a3eb7f2ac4b4d9b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[928cb8e13c9650fd]::util::run_in_thread_pool_with_globals<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>
  34:     0x55ca8d8b3991 - std[7a3eb7f2ac4b4d9b]::panicking::try::<core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>, core[8804424b9535ea4e]::panic::unwind_safe::AssertUnwindSafe<<std[7a3eb7f2ac4b4d9b]::thread::Builder>::spawn_unchecked_<rustc_interface[928cb8e13c9650fd]::util::run_in_thread_pool_with_globals<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x55ca8d918be2 - <<std[7a3eb7f2ac4b4d9b]::thread::Builder>::spawn_unchecked_<rustc_interface[928cb8e13c9650fd]::util::run_in_thread_pool_with_globals<rustdoc[e05fa81b99b89a0c]::main_args::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#0}, core[8804424b9535ea4e]::result::Result<(), rustc_errors[d13fae9e5f560f56]::ErrorGuaranteed>>::{closure#1} as core[8804424b9535ea4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f778faf9753 - std::sys::unix::thread::Thread::new::thread_start::h702769e3b3171e19
  37:     0x7f778f9fa609 - start_thread
  38:     0x7f778f7d0163 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

