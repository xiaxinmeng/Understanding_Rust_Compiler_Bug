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

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_span/src/lib.rs:1723:47
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_span/src/lib.rs:1723:47
stack backtrace:
   0:     0x7ff207be1f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0abbb2bff8ef874a
   1:     0x7ff207c489f8 - core::fmt::write::h6600948b35f5fc2c
   2:     0x7ff207bd25b1 - std::io::Write::write_fmt::h00b65493021d9737
   3:     0x7ff207be4f5e - std::panicking::default_hook::{{closure}}::h087ab9e951c79787
   4:     0x7ff207be4c1f - std::panicking::default_hook::hfe726fe34e2f25f0
   5:     0x7ff20859c6c4 - rustc_driver[b88e6193ee8b092e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff207be5712 - std::panicking::rust_panic_with_hook::h73acba524f5f90d8
   7:     0x7ff207be5537 - std::panicking::begin_panic_handler::{{closure}}::h4565558e4aefc45a
   8:     0x7ff207be2514 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8761855c7aa160f
   9:     0x7ff207be5202 - rust_begin_unwind
  10:     0x7ff207b94e13 - core::panicking::panic_fmt::h19c9b84aeeb503ff
  11:     0x7ff207b94d52 - core::panicking::panic_bounds_check::h1b038a01b8bed6dd
  12:     0x7ff20b525892 - <rustc_span[4264e63caa8f548d]::SourceFile>::lookup_file_pos_with_col_display
  13:     0x7ff20b50ee7c - <rustc_span[4264e63caa8f548d]::source_map::SourceMap>::lookup_char_pos
  14:     0x5568119490ad - <rustdoc[b3819c7521d777f9]::visit_ast::Module as rustdoc[b3819c7521d777f9]::clean::Clean<rustdoc[b3819c7521d777f9]::clean::types::Item>>::clean
  15:     0x5568118c3c3e - <core[a940a7fbd128742]::iter::adapters::map::Map<core[a940a7fbd128742]::slice::iter::Iter<rustdoc[b3819c7521d777f9]::visit_ast::Module>, <rustdoc[b3819c7521d777f9]::visit_ast::Module as rustdoc[b3819c7521d777f9]::clean::Clean<rustdoc[b3819c7521d777f9]::clean::types::Item>>::clean::{closure#1}> as core[a940a7fbd128742]::iter::traits::iterator::Iterator>::fold::<(), core[a940a7fbd128742]::iter::traits::iterator::Iterator::for_each::call<rustdoc[b3819c7521d777f9]::clean::types::Item, <alloc[dd71ee534134e102]::vec::Vec<rustdoc[b3819c7521d777f9]::clean::types::Item> as alloc[dd71ee534134e102]::vec::spec_extend::SpecExtend<rustdoc[b3819c7521d777f9]::clean::types::Item, core[a940a7fbd128742]::iter::adapters::map::Map<core[a940a7fbd128742]::slice::iter::Iter<rustdoc[b3819c7521d777f9]::visit_ast::Module>, <rustdoc[b3819c7521d777f9]::visit_ast::Module as rustdoc[b3819c7521d777f9]::clean::Clean<rustdoc[b3819c7521d777f9]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>
  16:     0x556811a05953 - <alloc[dd71ee534134e102]::vec::Vec<rustdoc[b3819c7521d777f9]::clean::types::Item> as alloc[dd71ee534134e102]::vec::spec_extend::SpecExtend<rustdoc[b3819c7521d777f9]::clean::types::Item, core[a940a7fbd128742]::iter::adapters::map::Map<core[a940a7fbd128742]::slice::iter::Iter<rustdoc[b3819c7521d777f9]::visit_ast::Module>, <rustdoc[b3819c7521d777f9]::visit_ast::Module as rustdoc[b3819c7521d777f9]::clean::Clean<rustdoc[b3819c7521d777f9]::clean::types::Item>>::clean::{closure#1}>>>::spec_extend
  17:     0x556811948e41 - <rustdoc[b3819c7521d777f9]::visit_ast::Module as rustdoc[b3819c7521d777f9]::clean::Clean<rustdoc[b3819c7521d777f9]::clean::types::Item>>::clean
  18:     0x556811a95afa - rustdoc[b3819c7521d777f9]::clean::utils::krate
  19:     0x5568119644d4 - <rustc_session[b1e464b2a1448164]::session::Session>::time::<rustdoc[b3819c7521d777f9]::clean::types::Crate, rustdoc[b3819c7521d777f9]::core::run_global_ctxt::{closure#5}>
  20:     0x5568117e32ae - rustdoc[b3819c7521d777f9]::core::run_global_ctxt
  21:     0x556811964941 - <rustc_session[b1e464b2a1448164]::session::Session>::time::<(rustdoc[b3819c7521d777f9]::clean::types::Crate, rustdoc[b3819c7521d777f9]::config::RenderOptions, rustdoc[b3819c7521d777f9]::formats::cache::Cache), rustdoc[b3819c7521d777f9]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  22:     0x5568118b434c - <rustc_interface[89fc321abaf886be]::passes::QueryContext>::enter::<rustdoc[b3819c7521d777f9]::main_options::{closure#0}::{closure#0}::{closure#1}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>
  23:     0x55681179af93 - <rustc_interface[89fc321abaf886be]::interface::Compiler>::enter::<rustdoc[b3819c7521d777f9]::main_options::{closure#0}::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>
  24:     0x556811a207bb - rustc_span[4264e63caa8f548d]::with_source_map::<core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>, rustc_interface[89fc321abaf886be]::interface::create_compiler_and_run<core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>, rustdoc[b3819c7521d777f9]::main_options::{closure#0}>::{closure#1}>
  25:     0x556811811f21 - rustdoc[b3819c7521d777f9]::main_options
  26:     0x55681186f579 - <scoped_tls[a58f2e04e4b746e6]::ScopedKey<rustc_span[4264e63caa8f548d]::SessionGlobals>>::set::<rustdoc[b3819c7521d777f9]::main_args::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>
  27:     0x556811929b99 - std[de83072bc74570bc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[89fc321abaf886be]::util::run_in_thread_pool_with_globals<rustdoc[b3819c7521d777f9]::main_args::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>
  28:     0x556811a7f77e - std[de83072bc74570bc]::panicking::try::<core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>, core[a940a7fbd128742]::panic::unwind_safe::AssertUnwindSafe<<std[de83072bc74570bc]::thread::Builder>::spawn_unchecked_<rustc_interface[89fc321abaf886be]::util::run_in_thread_pool_with_globals<rustdoc[b3819c7521d777f9]::main_args::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x556811888482 - <<std[de83072bc74570bc]::thread::Builder>::spawn_unchecked_<rustc_interface[89fc321abaf886be]::util::run_in_thread_pool_with_globals<rustdoc[b3819c7521d777f9]::main_args::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>::{closure#0}, core[a940a7fbd128742]::result::Result<(), rustc_errors[55070c045f4d9650]::ErrorGuaranteed>>::{closure#1} as core[a940a7fbd128742]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7ff207bf09e5 - std::sys::unix::thread::Thread::new::thread_start::hbe7c2d331afa365d
  31:     0x7ff207ae9609 - start_thread
  32:     0x7ff2078bf133 - clone
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

