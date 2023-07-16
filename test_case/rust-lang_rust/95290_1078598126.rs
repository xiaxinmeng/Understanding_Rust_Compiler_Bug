plain
---- [rustdoc] rustdoc/intra-doc/libstd-re-export.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/libstd-re-export.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:982:16
stack backtrace:
   0:     0x7f15a95c7c9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb46155a08188acdc
   1:     0x7f15a962f28f - core::fmt::write::h07966c189aa35091
   2:     0x7f15a95b7a71 - std::io::Write::write_fmt::h237d88c84d1c3cb7
   3:     0x7f15a95c7abb - std::sys_common::backtrace::print::h4b5b0545e596a045
   4:     0x7f15a95cb2a4 - std::panicking::default_hook::{{closure}}::h22d6fa936415f1f0
   5:     0x7f15a95cae5d - std::panicking::default_hook::h385d3a23348e4c6a
   6:     0x7f15aa06af11 - rustc_driver[322cdfc80470d5a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f15a95cb98b - std::panicking::rust_panic_with_hook::hd2fea976e3c69ff8
   8:     0x7f15a95cb7b7 - std::panicking::begin_panic_handler::{{closure}}::h45a3d1675de06059
   9:     0x7f15a95c81b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ece51d365c5afba
  10:     0x7f15a95cb499 - rust_begin_unwind
  11:     0x7f15a9579c03 - core::panicking::panic_fmt::hed73d002d24f3ad5
  12:     0x7f15a962b621 - core::panicking::panic_display::ha999c4156f487cf2
  13:     0x7f15a962b5cb - core::panicking::panic_str::h899e09e704e7d3a5
  14:     0x7f15a9579a76 - core::option::expect_failed::h64043cd3ae0e03a0
  15:     0x55d618ee0f6d - rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::resolve_associated_trait_item
  16:     0x55d618edf1b1 - <rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
  17:     0x55d618edc87d - <rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::LinkCollector>::resolve
  18:     0x55d618ee595f - <rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::LinkCollector>::resolve_link
  19:     0x55d618ee1763 - <rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::LinkCollector as rustdoc[24163f29015e045e]::visit::DocVisitor>::visit_item
  20:     0x55d618eeedee - <rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::LinkCollector as rustdoc[24163f29015e045e]::visit::DocVisitor>::visit_inner_recur
  21:     0x55d618ee1c13 - <rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::LinkCollector as rustdoc[24163f29015e045e]::visit::DocVisitor>::visit_item
  22:     0x55d618edb156 - rustdoc[24163f29015e045e]::passes::collect_intra_doc_links::collect_intra_doc_links
  23:     0x55d61902b090 - <rustc_session[4631a9c847ce1a67]::session::Session>::time::<rustdoc[24163f29015e045e]::clean::types::Crate, rustdoc[24163f29015e045e]::core::run_global_ctxt::{closure#8}>
  24:     0x55d618f8cc05 - rustdoc[24163f29015e045e]::core::run_global_ctxt
  25:     0x55d61902b34f - <rustc_session[4631a9c847ce1a67]::session::Session>::time::<(rustdoc[24163f29015e045e]::clean::types::Crate, rustdoc[24163f29015e045e]::config::RenderOptions, rustdoc[24163f29015e045e]::formats::cache::Cache), rustdoc[24163f29015e045e]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  26:     0x55d618f9c374 - <rustc_interface[a409d86d171b20d0]::passes::QueryContext>::enter::<rustdoc[24163f29015e045e]::main_options::{closure#0}::{closure#0}::{closure#1}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>
  27:     0x55d618efc60e - rustc_interface[a409d86d171b20d0]::interface::create_compiler_and_run::<core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>, rustdoc[24163f29015e045e]::main_options::{closure#0}>
  28:     0x55d618d63b24 - rustdoc[24163f29015e045e]::main_options
  29:     0x55d618e46cac - <scoped_tls[e10de3b1b457c586]::ScopedKey<rustc_span[4434ebff1cc39b17]::SessionGlobals>>::set::<rustdoc[24163f29015e045e]::main_args::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>
  30:     0x55d618f168c9 - std[85299461f12b33d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a409d86d171b20d0]::util::run_in_thread_pool_with_globals<rustdoc[24163f29015e045e]::main_args::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>
  31:     0x55d618f71911 - std[85299461f12b33d1]::panicking::try::<core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>, core[f664c2fe6850806b]::panic::unwind_safe::AssertUnwindSafe<<std[85299461f12b33d1]::thread::Builder>::spawn_unchecked_<rustc_interface[a409d86d171b20d0]::util::run_in_thread_pool_with_globals<rustdoc[24163f29015e045e]::main_args::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  32:     0x55d618fb6672 - <<std[85299461f12b33d1]::thread::Builder>::spawn_unchecked_<rustc_interface[a409d86d171b20d0]::util::run_in_thread_pool_with_globals<rustdoc[24163f29015e045e]::main_args::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>::{closure#0}, core[f664c2fe6850806b]::result::Result<(), rustc_errors[838ff62f62930a07]::ErrorGuaranteed>>::{closure#1} as core[f664c2fe6850806b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f15a95d6ef3 - std::sys::unix::thread::Thread::new::thread_start::h925419850d091f58
  34:     0x7f15a94d3609 - start_thread
  35:     0x7f15a92a9163 - clone
  36:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

