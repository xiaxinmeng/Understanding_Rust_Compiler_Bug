plain
---- [rustdoc] src/test/rustdoc/issue-26995.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26995/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26995" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-26995.rs" "--no-defaults"
stdout: none
--- stderr -------------------------------
warning: the `no-defaults` flag no longer functions
  = note: see issue #44136 <https://github.com/rust-lang/rust/issues/44136> for more information
  = help: you may want to use --document-private-items

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_span/src/lib.rs:1710:17
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_span/src/lib.rs:1710:17
stack backtrace:
   0:     0x7f0dc33b8f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha834d6660b6479e4
   1:     0x7f0dc341f9f8 - core::fmt::write::h268d4041b273060a
   2:     0x7f0dc33a9651 - std::io::Write::write_fmt::he6aad91c1a4d3f5c
   3:     0x7f0dc33bbf5e - std::panicking::default_hook::{{closure}}::h5e13139614ba38ae
   4:     0x7f0dc33bbc1f - std::panicking::default_hook::h017f998477ad4491
   5:     0x7f0dc3d74844 - rustc_driver[d045b0d5b586cb19]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0dc33bc712 - std::panicking::rust_panic_with_hook::h1197b86161817713
   7:     0x7f0dc33bc4f9 - std::panicking::begin_panic_handler::{{closure}}::h3c7aa41611835c92
   8:     0x7f0dc33b9514 - std::sys_common::backtrace::__rust_end_short_backtrace::h781db4f432b23754
   9:     0x7f0dc33bc202 - rust_begin_unwind
  10:     0x7f0dc336be13 - core::panicking::panic_fmt::hdcc0ee1d31398730
  11:     0x7f0dc336bcdd - core::panicking::panic::h1602179f272845ca
  12:     0x7f0dc6cf780e - <rustc_span[315b5a5495f1c7d4]::SourceFile>::lookup_file_pos
  13:     0x7f0dc6cf79d7 - <rustc_span[315b5a5495f1c7d4]::SourceFile>::lookup_file_pos_with_col_display
  14:     0x7f0dc6ce152c - <rustc_span[315b5a5495f1c7d4]::source_map::SourceMap>::lookup_char_pos
  15:     0x55740191022d - <rustdoc[c0a6412cb0be4570]::visit_ast::Module as rustdoc[c0a6412cb0be4570]::clean::Clean<rustdoc[c0a6412cb0be4570]::clean::types::Item>>::clean
  16:     0x55740188b03e - <core[66423f074c679c51]::iter::adapters::map::Map<core[66423f074c679c51]::slice::iter::Iter<rustdoc[c0a6412cb0be4570]::visit_ast::Module>, <rustdoc[c0a6412cb0be4570]::visit_ast::Module as rustdoc[c0a6412cb0be4570]::clean::Clean<rustdoc[c0a6412cb0be4570]::clean::types::Item>>::clean::{closure#1}> as core[66423f074c679c51]::iter::traits::iterator::Iterator>::fold::<(), core[66423f074c679c51]::iter::traits::iterator::Iterator::for_each::call<rustdoc[c0a6412cb0be4570]::clean::types::Item, <alloc[c596d4c77513598b]::vec::Vec<rustdoc[c0a6412cb0be4570]::clean::types::Item> as alloc[c596d4c77513598b]::vec::spec_extend::SpecExtend<rustdoc[c0a6412cb0be4570]::clean::types::Item, core[66423f074c679c51]::iter::adapters::map::Map<core[66423f074c679c51]::slice::iter::Iter<rustdoc[c0a6412cb0be4570]::visit_ast::Module>, <rustdoc[c0a6412cb0be4570]::visit_ast::Module as rustdoc[c0a6412cb0be4570]::clean::Clean<rustdoc[c0a6412cb0be4570]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>
  17:     0x5574019ccab3 - <alloc[c596d4c77513598b]::vec::Vec<rustdoc[c0a6412cb0be4570]::clean::types::Item> as alloc[c596d4c77513598b]::vec::spec_extend::SpecExtend<rustdoc[c0a6412cb0be4570]::clean::types::Item, core[66423f074c679c51]::iter::adapters::map::Map<core[66423f074c679c51]::slice::iter::Iter<rustdoc[c0a6412cb0be4570]::visit_ast::Module>, <rustdoc[c0a6412cb0be4570]::visit_ast::Module as rustdoc[c0a6412cb0be4570]::clean::Clean<rustdoc[c0a6412cb0be4570]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend
  18:     0x55740190ffc1 - <rustdoc[c0a6412cb0be4570]::visit_ast::Module as rustdoc[c0a6412cb0be4570]::clean::Clean<rustdoc[c0a6412cb0be4570]::clean::types::Item>>::clean
  19:     0x557401a5ca9a - rustdoc[c0a6412cb0be4570]::clean::utils::krate
  20:     0x55740192b654 - <rustc_session[9c277dba11350be5]::session::Session>::time::<rustdoc[c0a6412cb0be4570]::clean::types::Crate, rustdoc[c0a6412cb0be4570]::core::run_global_ctxt::{closure#5}>
  21:     0x5574017aa2ce - rustdoc[c0a6412cb0be4570]::core::run_global_ctxt
  22:     0x55740192bac1 - <rustc_session[9c277dba11350be5]::session::Session>::time::<(rustdoc[c0a6412cb0be4570]::clean::types::Crate, rustdoc[c0a6412cb0be4570]::config::RenderOptions, rustdoc[c0a6412cb0be4570]::formats::cache::Cache), rustdoc[c0a6412cb0be4570]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  23:     0x55740187b4cc - <rustc_interface[7a03c30678afae11]::passes::QueryContext>::enter::<rustdoc[c0a6412cb0be4570]::main_options::{closure#0}::{closure#0}::{closure#1}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>
  24:     0x557401761f93 - <rustc_interface[7a03c30678afae11]::interface::Compiler>::enter::<rustdoc[c0a6412cb0be4570]::main_options::{closure#0}::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>
  25:     0x5574019e777b - rustc_span[315b5a5495f1c7d4]::with_source_map::<core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>, rustc_interface[7a03c30678afae11]::interface::create_compiler_and_run<core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>, rustdoc[c0a6412cb0be4570]::main_options::{closure#0}>::{closure#1}>
  26:     0x5574017d8f41 - rustdoc[c0a6412cb0be4570]::main_options
  27:     0x5574018365d9 - <scoped_tls[bb41e43d591aab10]::ScopedKey<rustc_span[315b5a5495f1c7d4]::SessionGlobals>>::set::<rustdoc[c0a6412cb0be4570]::main_args::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>
  28:     0x5574018f0cf9 - std[94e631a80f410e35]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a03c30678afae11]::util::run_in_thread_pool_with_globals<rustdoc[c0a6412cb0be4570]::main_args::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>
  29:     0x557401a4569e - std[94e631a80f410e35]::panicking::try::<core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>, core[66423f074c679c51]::panic::unwind_safe::AssertUnwindSafe<<std[94e631a80f410e35]::thread::Builder>::spawn_unchecked_<rustc_interface[7a03c30678afae11]::util::run_in_thread_pool_with_globals<rustdoc[c0a6412cb0be4570]::main_args::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x55740184f4e2 - <<std[94e631a80f410e35]::thread::Builder>::spawn_unchecked_<rustc_interface[7a03c30678afae11]::util::run_in_thread_pool_with_globals<rustdoc[c0a6412cb0be4570]::main_args::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>::{closure#0}, core[66423f074c679c51]::result::Result<(), rustc_errors[b5a8abfca36c583f]::ErrorGuaranteed>>::{closure#1} as core[66423f074c679c51]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f0dc33c79e5 - std::sys::unix::thread::Thread::new::thread_start::hc2257a089f198ccd
  32:     0x7f0dc32c2609 - start_thread
  33:     0x7f0dc3096133 - clone
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

