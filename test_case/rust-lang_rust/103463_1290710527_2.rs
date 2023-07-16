
 Documenting A v0.1.0 (/extra/Projects/MCVE/A)
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:781:16
stack backtrace:
   0:     0x7fbe9216c810 - std::backtrace_rs::backtrace::libunwind::trace::haefc0dd0a0a62b67
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7fbe9216c810 - std::backtrace_rs::backtrace::trace_unsynchronized::h231f73070a372c1c
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fbe9216c810 - std::sys_common::backtrace::_print_fmt::haa15dcb5c660c296
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fbe9216c810 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfcfc314fff15b0e4
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fbe921c86ce - core::fmt::write::h2e893235039ae031
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fbe9215c985 - std::io::Write::write_fmt::h310687acf0f328ac
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/io/mod.rs:1682:15
   6:     0x7fbe9216c5d5 - std::sys_common::backtrace::_print::h7594904cd4694801
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fbe9216c5d5 - std::sys_common::backtrace::print::hc63fb4b3dff70d7e
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fbe9216f3df - std::panicking::default_hook::{{closure}}::h6ab59645549c33df
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:267:22
   9:     0x7fbe9216f11a - std::panicking::default_hook::h9d471aa0319ef778
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:286:9
  10:     0x7fbe9216fbe8 - std::panicking::rust_panic_with_hook::hedb82473dd3f5a0d
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:688:13
  11:     0x7fbe9216f987 - std::panicking::begin_panic_handler::{{closure}}::h7e1d229967107c2d
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:579:13
  12:     0x7fbe9216ccbc - std::sys_common::backtrace::__rust_end_short_backtrace::he5492151b8b5b3a1
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7fbe9216f6a2 - rust_begin_unwind
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:575:5
  14:     0x7fbe921c50b3 - core::panicking::panic_fmt::hf762b822ffcd739c
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/core/src/panicking.rs:65:14
  15:     0x7fbe921c5201 - core::panicking::panic_display::h6b3e06ee369cb282
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/core/src/panicking.rs:139:5
  16:     0x7fbe921c51ab - core::panicking::panic_str::heaf5e09d79be0e66
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/core/src/panicking.rs:123:5
  17:     0x7fbe921c4e26 - core::option::expect_failed::h581bb50ba961001e
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/core/src/option.rs:1876:5
  18:     0x55d9648fe9d4 - rustdoc[e6b092b8ebcfcef]::passes::collect_intra_doc_links::resolve_associated_trait_item
  19:     0x55d9648fd671 - <rustdoc[e6b092b8ebcfcef]::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
  20:     0x55d9648fb711 - <rustdoc[e6b092b8ebcfcef]::passes::collect_intra_doc_links::LinkCollector>::resolve
  21:     0x55d9649007a8 - <rustdoc[e6b092b8ebcfcef]::passes::collect_intra_doc_links::LinkCollector as rustdoc[e6b092b8ebcfcef]::visit::DocVisitor>::visit_item
  22:     0x55d9648f9ec1 - rustdoc[e6b092b8ebcfcef]::passes::collect_intra_doc_links::collect_intra_doc_links
  23:     0x55d964ae648d - <rustc_session[4ded857a98b896bc]::session::Session>::time::<rustdoc[e6b092b8ebcfcef]::clean::types::Crate, rustdoc[e6b092b8ebcfcef]::core::run_global_ctxt::{closure#7}>
  24:     0x55d9648df769 - rustdoc[e6b092b8ebcfcef]::core::run_global_ctxt
  25:     0x55d964ae6769 - <rustc_session[4ded857a98b896bc]::session::Session>::time::<(rustdoc[e6b092b8ebcfcef]::clean::types::Crate, rustdoc[e6b092b8ebcfcef]::config::RenderOptions, rustdoc[e6b092b8ebcfcef]::formats::cache::Cache), rustdoc[e6b092b8ebcfcef]::main_args::{closure#1}::{closure#0}::{closure#1}::{closure#0}>
  26:     0x55d964b25741 - <rustc_interface[a64a4e4385c65420]::passes::QueryContext>::enter::<rustdoc[e6b092b8ebcfcef]::main_args::{closure#1}::{closure#0}::{closure#1}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  27:     0x55d964a42844 - <rustc_interface[a64a4e4385c65420]::interface::Compiler>::enter::<rustdoc[e6b092b8ebcfcef]::main_args::{closure#1}::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  28:     0x55d9648bcf23 - std[aff2bf53d717ad31]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a64a4e4385c65420]::util::run_in_thread_pool_with_globals<rustc_interface[a64a4e4385c65420]::interface::run_compiler<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustdoc[e6b092b8ebcfcef]::main_args::{closure#1}>::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  29:     0x55d964b9149a - <<std[aff2bf53d717ad31]::thread::Builder>::spawn_unchecked_<rustc_interface[a64a4e4385c65420]::util::run_in_thread_pool_with_globals<rustc_interface[a64a4e4385c65420]::interface::run_compiler<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustdoc[e6b092b8ebcfcef]::main_args::{closure#1}>::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>::{closure#1} as core[1359fc6eb4466362]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7fbe921796a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h27f3134faf8b42b4
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/alloc/src/boxed.rs:1987:9
  31:     0x7fbe921796a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h46d82bf285dffad3
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/alloc/src/boxed.rs:1987:9
  32:     0x7fbe921796a3 - std::sys::unix::thread::Thread::new::thread_start::hdb1ca70a964535d0
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys/unix/thread.rs:108:17
  33:     0x7fbe91edc8fd - <unknown>
  34:     0x7fbe91f5ea60 - <unknown>
  35:                0x0 - <unknown>
error: could not document `A`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2021 --crate-type bin --crate-name A src/main.rs -o /extra/Projects/MCVE/A/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --document-private-items '-Arustdoc::private-intra-doc-links' -C metadata=8650d43ce013f66f -L dependency=/extra/Projects/MCVE/A/target/debug/deps --extern B=/extra/Projects/MCVE/A/target/debug/deps/libB-a432d16f3e36888e.rmeta --crate-version 0.1.0` (exit status: 101)
