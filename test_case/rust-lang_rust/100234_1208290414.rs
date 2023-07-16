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

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_span/src/lib.rs:1750:13
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_span/src/lib.rs:1750:13
stack backtrace:
   0:     0x7f495726ef9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h032eb5434dc14ead
   1:     0x7f49572d59f8 - core::fmt::write::h999b05740866b428
   2:     0x7f495725f5b1 - std::io::Write::write_fmt::h080a70cd285685da
   3:     0x7f4957271f5e - std::panicking::default_hook::{{closure}}::h860d0529eff17555
   4:     0x7f4957271c1f - std::panicking::default_hook::hb9d7ab678a39eace
   5:     0x7f4957c2a834 - rustc_driver[6392feae8bcde39c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4957272712 - std::panicking::rust_panic_with_hook::h293a572a51a7652c
   7:     0x7f49572724f9 - std::panicking::begin_panic_handler::{{closure}}::hca0e7950fd2d9c1d
   8:     0x7f495726f514 - std::sys_common::backtrace::__rust_end_short_backtrace::h886ee160960fb76c
   9:     0x7f4957272202 - rust_begin_unwind
  10:     0x7f4957221e13 - core::panicking::panic_fmt::h33a891c18dfce6a5
  11:     0x7f4957221cdd - core::panicking::panic::h0db86117a1d5b4e1
  12:     0x7f495abb1131 - <rustc_span[af3927eb373ac1f8]::SourceFile>::lookup_file_pos_with_col_display
  13:     0x7f495ab9a55c - <rustc_span[af3927eb373ac1f8]::source_map::SourceMap>::lookup_char_pos
  14:     0x5557c944e4cd - <rustdoc[d6d8b57e9f81f45d]::visit_ast::Module as rustdoc[d6d8b57e9f81f45d]::clean::Clean<rustdoc[d6d8b57e9f81f45d]::clean::types::Item>>::clean
  15:     0x5557c93c9f5e - <core[f02b946876b79fae]::iter::adapters::map::Map<core[f02b946876b79fae]::slice::iter::Iter<rustdoc[d6d8b57e9f81f45d]::visit_ast::Module>, <rustdoc[d6d8b57e9f81f45d]::visit_ast::Module as rustdoc[d6d8b57e9f81f45d]::clean::Clean<rustdoc[d6d8b57e9f81f45d]::clean::types::Item>>::clean::{closure#1}> as core[f02b946876b79fae]::iter::traits::iterator::Iterator>::fold::<(), core[f02b946876b79fae]::iter::traits::iterator::Iterator::for_each::call<rustdoc[d6d8b57e9f81f45d]::clean::types::Item, <alloc[3c495757a0487a0c]::vec::Vec<rustdoc[d6d8b57e9f81f45d]::clean::types::Item> as alloc[3c495757a0487a0c]::vec::spec_extend::SpecExtend<rustdoc[d6d8b57e9f81f45d]::clean::types::Item, core[f02b946876b79fae]::iter::adapters::map::Map<core[f02b946876b79fae]::slice::iter::Iter<rustdoc[d6d8b57e9f81f45d]::visit_ast::Module>, <rustdoc[d6d8b57e9f81f45d]::visit_ast::Module as rustdoc[d6d8b57e9f81f45d]::clean::Clean<rustdoc[d6d8b57e9f81f45d]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>
  16:     0x5557c950ac03 - <alloc[3c495757a0487a0c]::vec::Vec<rustdoc[d6d8b57e9f81f45d]::clean::types::Item> as alloc[3c495757a0487a0c]::vec::spec_extend::SpecExtend<rustdoc[d6d8b57e9f81f45d]::clean::types::Item, core[f02b946876b79fae]::iter::adapters::map::Map<core[f02b946876b79fae]::slice::iter::Iter<rustdoc[d6d8b57e9f81f45d]::visit_ast::Module>, <rustdoc[d6d8b57e9f81f45d]::visit_ast::Module as rustdoc[d6d8b57e9f81f45d]::clean::Clean<rustdoc[d6d8b57e9f81f45d]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend
  17:     0x5557c944e261 - <rustdoc[d6d8b57e9f81f45d]::visit_ast::Module as rustdoc[d6d8b57e9f81f45d]::clean::Clean<rustdoc[d6d8b57e9f81f45d]::clean::types::Item>>::clean
  18:     0x5557c959aeaa - rustdoc[d6d8b57e9f81f45d]::clean::utils::krate
  19:     0x5557c9469984 - <rustc_session[89ee6a5c25589bfd]::session::Session>::time::<rustdoc[d6d8b57e9f81f45d]::clean::types::Crate, rustdoc[d6d8b57e9f81f45d]::core::run_global_ctxt::{closure#5}>
  20:     0x5557c92e830e - rustdoc[d6d8b57e9f81f45d]::core::run_global_ctxt
  21:     0x5557c9469df1 - <rustc_session[89ee6a5c25589bfd]::session::Session>::time::<(rustdoc[d6d8b57e9f81f45d]::clean::types::Crate, rustdoc[d6d8b57e9f81f45d]::config::RenderOptions, rustdoc[d6d8b57e9f81f45d]::formats::cache::Cache), rustdoc[d6d8b57e9f81f45d]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  22:     0x5557c93b976c - <rustc_interface[3e666c391a9b15e]::passes::QueryContext>::enter::<rustdoc[d6d8b57e9f81f45d]::main_options::{closure#0}::{closure#0}::{closure#1}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>
  23:     0x5557c929ff93 - <rustc_interface[3e666c391a9b15e]::interface::Compiler>::enter::<rustdoc[d6d8b57e9f81f45d]::main_options::{closure#0}::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>
  24:     0x5557c9525beb - rustc_span[af3927eb373ac1f8]::with_source_map::<core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>, rustc_interface[3e666c391a9b15e]::interface::create_compiler_and_run<core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>, rustdoc[d6d8b57e9f81f45d]::main_options::{closure#0}>::{closure#1}>
  25:     0x5557c9316fd1 - rustdoc[d6d8b57e9f81f45d]::main_options
  26:     0x5557c9374769 - <scoped_tls[828f4fb2180d5373]::ScopedKey<rustc_span[af3927eb373ac1f8]::SessionGlobals>>::set::<rustdoc[d6d8b57e9f81f45d]::main_args::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>
  27:     0x5557c942efa9 - std[f1315f45ef2c7437]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3e666c391a9b15e]::util::run_in_thread_pool_with_globals<rustdoc[d6d8b57e9f81f45d]::main_args::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>
  28:     0x5557c9584b4e - std[f1315f45ef2c7437]::panicking::try::<core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>, core[f02b946876b79fae]::panic::unwind_safe::AssertUnwindSafe<<std[f1315f45ef2c7437]::thread::Builder>::spawn_unchecked_<rustc_interface[3e666c391a9b15e]::util::run_in_thread_pool_with_globals<rustdoc[d6d8b57e9f81f45d]::main_args::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x5557c938d672 - <<std[f1315f45ef2c7437]::thread::Builder>::spawn_unchecked_<rustc_interface[3e666c391a9b15e]::util::run_in_thread_pool_with_globals<rustdoc[d6d8b57e9f81f45d]::main_args::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>::{closure#0}, core[f02b946876b79fae]::result::Result<(), rustc_errors[ddbd1470afa7767c]::ErrorGuaranteed>>::{closure#1} as core[f02b946876b79fae]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f495727d9e5 - std::sys::unix::thread::Thread::new::thread_start::h93d82730b367b541
  31:     0x7f4957176609 - start_thread
  32:     0x7f4956f4c133 - clone
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

