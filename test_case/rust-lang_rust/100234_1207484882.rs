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

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_span/src/lib.rs:1747:13
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_span/src/lib.rs:1747:13
stack backtrace:
   0:     0x7f746199ff9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h515db4be6ec8e293
   1:     0x7f7461a069f8 - core::fmt::write::h7df7c7798b577b79
   2:     0x7f74619905b1 - std::io::Write::write_fmt::h825e8503ef07569c
   3:     0x7f74619a2f5e - std::panicking::default_hook::{{closure}}::h8470d86fb1eef0fe
   4:     0x7f74619a2c1f - std::panicking::default_hook::h71ecb9c471232f59
   5:     0x7f746235b684 - rustc_driver[8ff18a7b2c4a5f1e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f74619a3712 - std::panicking::rust_panic_with_hook::h14677c46455965b8
   7:     0x7f74619a34f9 - std::panicking::begin_panic_handler::{{closure}}::h49f04d974ac1247e
   8:     0x7f74619a0514 - std::sys_common::backtrace::__rust_end_short_backtrace::hd5c31e404d522330
   9:     0x7f74619a3202 - rust_begin_unwind
  10:     0x7f7461952e13 - core::panicking::panic_fmt::hc03223e2121e0efc
  11:     0x7f7461952cdd - core::panicking::panic::h8edb7545bcb041a8
  12:     0x7f74652e289d - <rustc_span[ff81d54fa02a5b66]::SourceFile>::lookup_file_pos_with_col_display
  13:     0x7f74652cbd7c - <rustc_span[ff81d54fa02a5b66]::source_map::SourceMap>::lookup_char_pos
  14:     0x5617c9f6720d - <rustdoc[26aad51ab9b1bb6f]::visit_ast::Module as rustdoc[26aad51ab9b1bb6f]::clean::Clean<rustdoc[26aad51ab9b1bb6f]::clean::types::Item>>::clean
  15:     0x5617c9ee0e0e - <core[84dac936fcf30e7f]::iter::adapters::map::Map<core[84dac936fcf30e7f]::slice::iter::Iter<rustdoc[26aad51ab9b1bb6f]::visit_ast::Module>, <rustdoc[26aad51ab9b1bb6f]::visit_ast::Module as rustdoc[26aad51ab9b1bb6f]::clean::Clean<rustdoc[26aad51ab9b1bb6f]::clean::types::Item>>::clean::{closure#1}> as core[84dac936fcf30e7f]::iter::traits::iterator::Iterator>::fold::<(), core[84dac936fcf30e7f]::iter::traits::iterator::Iterator::for_each::call<rustdoc[26aad51ab9b1bb6f]::clean::types::Item, <alloc[39e8aacbe213dc7a]::vec::Vec<rustdoc[26aad51ab9b1bb6f]::clean::types::Item> as alloc[39e8aacbe213dc7a]::vec::spec_extend::SpecExtend<rustdoc[26aad51ab9b1bb6f]::clean::types::Item, core[84dac936fcf30e7f]::iter::adapters::map::Map<core[84dac936fcf30e7f]::slice::iter::Iter<rustdoc[26aad51ab9b1bb6f]::visit_ast::Module>, <rustdoc[26aad51ab9b1bb6f]::visit_ast::Module as rustdoc[26aad51ab9b1bb6f]::clean::Clean<rustdoc[26aad51ab9b1bb6f]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>
  16:     0x5617ca023953 - <alloc[39e8aacbe213dc7a]::vec::Vec<rustdoc[26aad51ab9b1bb6f]::clean::types::Item> as alloc[39e8aacbe213dc7a]::vec::spec_extend::SpecExtend<rustdoc[26aad51ab9b1bb6f]::clean::types::Item, core[84dac936fcf30e7f]::iter::adapters::map::Map<core[84dac936fcf30e7f]::slice::iter::Iter<rustdoc[26aad51ab9b1bb6f]::visit_ast::Module>, <rustdoc[26aad51ab9b1bb6f]::visit_ast::Module as rustdoc[26aad51ab9b1bb6f]::clean::Clean<rustdoc[26aad51ab9b1bb6f]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend
  17:     0x5617c9f66fa1 - <rustdoc[26aad51ab9b1bb6f]::visit_ast::Module as rustdoc[26aad51ab9b1bb6f]::clean::Clean<rustdoc[26aad51ab9b1bb6f]::clean::types::Item>>::clean
  18:     0x5617ca0b3b7a - rustdoc[26aad51ab9b1bb6f]::clean::utils::krate
  19:     0x5617c9f82634 - <rustc_session[907ab4c55084545c]::session::Session>::time::<rustdoc[26aad51ab9b1bb6f]::clean::types::Crate, rustdoc[26aad51ab9b1bb6f]::core::run_global_ctxt::{closure#5}>
  20:     0x5617c9e012ce - rustdoc[26aad51ab9b1bb6f]::core::run_global_ctxt
  21:     0x5617c9f82aa1 - <rustc_session[907ab4c55084545c]::session::Session>::time::<(rustdoc[26aad51ab9b1bb6f]::clean::types::Crate, rustdoc[26aad51ab9b1bb6f]::config::RenderOptions, rustdoc[26aad51ab9b1bb6f]::formats::cache::Cache), rustdoc[26aad51ab9b1bb6f]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  22:     0x5617c9ed248c - <rustc_interface[1858c51cf5faadb4]::passes::QueryContext>::enter::<rustdoc[26aad51ab9b1bb6f]::main_options::{closure#0}::{closure#0}::{closure#1}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>
  23:     0x5617c9db8f93 - <rustc_interface[1858c51cf5faadb4]::interface::Compiler>::enter::<rustdoc[26aad51ab9b1bb6f]::main_options::{closure#0}::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>
  24:     0x5617ca03e85b - rustc_span[ff81d54fa02a5b66]::with_source_map::<core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>, rustc_interface[1858c51cf5faadb4]::interface::create_compiler_and_run<core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>, rustdoc[26aad51ab9b1bb6f]::main_options::{closure#0}>::{closure#1}>
  25:     0x5617c9e2ff41 - rustdoc[26aad51ab9b1bb6f]::main_options
  26:     0x5617c9e8d599 - <scoped_tls[89929355d53e9069]::ScopedKey<rustc_span[ff81d54fa02a5b66]::SessionGlobals>>::set::<rustdoc[26aad51ab9b1bb6f]::main_args::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>
  27:     0x5617c9f47b19 - std[87ddf81d47b0c377]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1858c51cf5faadb4]::util::run_in_thread_pool_with_globals<rustdoc[26aad51ab9b1bb6f]::main_args::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>
  28:     0x5617ca0962be - std[87ddf81d47b0c377]::panicking::try::<core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>, core[84dac936fcf30e7f]::panic::unwind_safe::AssertUnwindSafe<<std[87ddf81d47b0c377]::thread::Builder>::spawn_unchecked_<rustc_interface[1858c51cf5faadb4]::util::run_in_thread_pool_with_globals<rustdoc[26aad51ab9b1bb6f]::main_args::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x5617c9ea64a2 - <<std[87ddf81d47b0c377]::thread::Builder>::spawn_unchecked_<rustc_interface[1858c51cf5faadb4]::util::run_in_thread_pool_with_globals<rustdoc[26aad51ab9b1bb6f]::main_args::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>::{closure#0}, core[84dac936fcf30e7f]::result::Result<(), rustc_errors[54f1c974ad75a3fb]::ErrorGuaranteed>>::{closure#1} as core[84dac936fcf30e7f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f74619ae9e5 - std::sys::unix::thread::Thread::new::thread_start::h08f429145e356694
  31:     0x7f74618a7609 - start_thread
  32:     0x7f746167d133 - clone
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

