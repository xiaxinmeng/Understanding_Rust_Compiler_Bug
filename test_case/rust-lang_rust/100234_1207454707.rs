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

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_span/src/lib.rs:1728:47
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_span/src/lib.rs:1728:47
stack backtrace:
   0:     0x7fef0c85df9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0636d14e55a6f3d2
   1:     0x7fef0c8c49f8 - core::fmt::write::hd3fba197dbabc517
   2:     0x7fef0c84e5b1 - std::io::Write::write_fmt::ha363b976d3fb5057
   3:     0x7fef0c860f5e - std::panicking::default_hook::{{closure}}::h9e82f904c97c1952
   4:     0x7fef0c860c1f - std::panicking::default_hook::h09cbbb2b0ca8776e
   5:     0x7fef0d218ea4 - rustc_driver[36a97d04a38c27f3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fef0c861712 - std::panicking::rust_panic_with_hook::h0b83d3fb3da13ea0
   7:     0x7fef0c861537 - std::panicking::begin_panic_handler::{{closure}}::h3f787a4565220a65
   8:     0x7fef0c85e514 - std::sys_common::backtrace::__rust_end_short_backtrace::h7244b3a08ddf4141
   9:     0x7fef0c861202 - rust_begin_unwind
  10:     0x7fef0c810e13 - core::panicking::panic_fmt::h6583a8fb921df579
  11:     0x7fef0c810d52 - core::panicking::panic_bounds_check::h970f7f12b4786894
  12:     0x7fef101a2cc2 - <rustc_span[aa28be8d9606d47]::SourceFile>::lookup_file_pos_with_col_display
  13:     0x7fef1018c29c - <rustc_span[aa28be8d9606d47]::source_map::SourceMap>::lookup_char_pos
  14:     0x564963d611dd - <rustdoc[c6650f89fb10e553]::visit_ast::Module as rustdoc[c6650f89fb10e553]::clean::Clean<rustdoc[c6650f89fb10e553]::clean::types::Item>>::clean
  15:     0x564963cdcbde - <core[df274439a79c4c7]::iter::adapters::map::Map<core[df274439a79c4c7]::slice::iter::Iter<rustdoc[c6650f89fb10e553]::visit_ast::Module>, <rustdoc[c6650f89fb10e553]::visit_ast::Module as rustdoc[c6650f89fb10e553]::clean::Clean<rustdoc[c6650f89fb10e553]::clean::types::Item>>::clean::{closure#1}> as core[df274439a79c4c7]::iter::traits::iterator::Iterator>::fold::<(), core[df274439a79c4c7]::iter::traits::iterator::Iterator::for_each::call<rustdoc[c6650f89fb10e553]::clean::types::Item, <alloc[d2dd953347bac8db]::vec::Vec<rustdoc[c6650f89fb10e553]::clean::types::Item> as alloc[d2dd953347bac8db]::vec::spec_extend::SpecExtend<rustdoc[c6650f89fb10e553]::clean::types::Item, core[df274439a79c4c7]::iter::adapters::map::Map<core[df274439a79c4c7]::slice::iter::Iter<rustdoc[c6650f89fb10e553]::visit_ast::Module>, <rustdoc[c6650f89fb10e553]::visit_ast::Module as rustdoc[c6650f89fb10e553]::clean::Clean<rustdoc[c6650f89fb10e553]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>
  16:     0x564963e1db93 - <alloc[d2dd953347bac8db]::vec::Vec<rustdoc[c6650f89fb10e553]::clean::types::Item> as alloc[d2dd953347bac8db]::vec::spec_extend::SpecExtend<rustdoc[c6650f89fb10e553]::clean::types::Item, core[df274439a79c4c7]::iter::adapters::map::Map<core[df274439a79c4c7]::slice::iter::Iter<rustdoc[c6650f89fb10e553]::visit_ast::Module>, <rustdoc[c6650f89fb10e553]::visit_ast::Module as rustdoc[c6650f89fb10e553]::clean::Clean<rustdoc[c6650f89fb10e553]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend
  17:     0x564963d60f71 - <rustdoc[c6650f89fb10e553]::visit_ast::Module as rustdoc[c6650f89fb10e553]::clean::Clean<rustdoc[c6650f89fb10e553]::clean::types::Item>>::clean
  18:     0x564963eade4a - rustdoc[c6650f89fb10e553]::clean::utils::krate
  19:     0x564963d7c674 - <rustc_session[3f22b2ca98c8b1d4]::session::Session>::time::<rustdoc[c6650f89fb10e553]::clean::types::Crate, rustdoc[c6650f89fb10e553]::core::run_global_ctxt::{closure#5}>
  20:     0x564963bfb2de - rustdoc[c6650f89fb10e553]::core::run_global_ctxt
  21:     0x564963d7cae1 - <rustc_session[3f22b2ca98c8b1d4]::session::Session>::time::<(rustdoc[c6650f89fb10e553]::clean::types::Crate, rustdoc[c6650f89fb10e553]::config::RenderOptions, rustdoc[c6650f89fb10e553]::formats::cache::Cache), rustdoc[c6650f89fb10e553]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  22:     0x564963ccc45c - <rustc_interface[b6984fa4fc40cc5a]::passes::QueryContext>::enter::<rustdoc[c6650f89fb10e553]::main_options::{closure#0}::{closure#0}::{closure#1}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>
  23:     0x564963bb2f93 - <rustc_interface[b6984fa4fc40cc5a]::interface::Compiler>::enter::<rustdoc[c6650f89fb10e553]::main_options::{closure#0}::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>
  24:     0x564963e38b0b - rustc_span[aa28be8d9606d47]::with_source_map::<core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>, rustc_interface[b6984fa4fc40cc5a]::interface::create_compiler_and_run<core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>, rustdoc[c6650f89fb10e553]::main_options::{closure#0}>::{closure#1}>
  25:     0x564963c29f51 - rustdoc[c6650f89fb10e553]::main_options
  26:     0x564963c87579 - <scoped_tls[23085760e69dabc3]::ScopedKey<rustc_span[aa28be8d9606d47]::SessionGlobals>>::set::<rustdoc[c6650f89fb10e553]::main_args::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>
  27:     0x564963d41bf9 - std[1112309c92fed1ea]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b6984fa4fc40cc5a]::util::run_in_thread_pool_with_globals<rustdoc[c6650f89fb10e553]::main_args::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>
  28:     0x564963e9689e - std[1112309c92fed1ea]::panicking::try::<core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>, core[df274439a79c4c7]::panic::unwind_safe::AssertUnwindSafe<<std[1112309c92fed1ea]::thread::Builder>::spawn_unchecked_<rustc_interface[b6984fa4fc40cc5a]::util::run_in_thread_pool_with_globals<rustdoc[c6650f89fb10e553]::main_args::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x564963ca0482 - <<std[1112309c92fed1ea]::thread::Builder>::spawn_unchecked_<rustc_interface[b6984fa4fc40cc5a]::util::run_in_thread_pool_with_globals<rustdoc[c6650f89fb10e553]::main_args::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>::{closure#0}, core[df274439a79c4c7]::result::Result<(), rustc_errors[320613d66cc559e4]::ErrorGuaranteed>>::{closure#1} as core[df274439a79c4c7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7fef0c86c9e5 - std::sys::unix::thread::Thread::new::thread_start::hc56f0c58a27ecc76
  31:     0x7fef0c765609 - start_thread
  32:     0x7fef0c53b133 - clone
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

