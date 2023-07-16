`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Item { id: Id("0:7:1529"), crate_id: 0, name: Some("builders"), span: Some(Span { filename: "./src/test/rustdoc/infinite-redirection.rs", begin: (20, 4), end: (22, 5) }), visibility: Public, docs: None, links: {}, attrs: ["#[doc(hidden)]"], deprecation: None, inner: Module(Module { is_crate: false, items: [Id("0:8")] }) }`,
 right: `Item { id: Id("0:7:1529"), crate_id: 0, name: Some("builders"), span: Some(Span { filename: "./src/test/rustdoc/infinite-redirection.rs", begin: (20, 4), end: (22, 5) }), visibility: Public, docs: None, links: {}, attrs: ["#[doc(hidden)]"], deprecation: None, inner: Module(Module { is_crate: false, items: [Id("0:5:1528")] }) }`', src/librustdoc/json/mod.rs:234:21
stack backtrace:
   0:     0x7f56f129e49d - std::backtrace_rs::backtrace::libunwind::trace::h6aafea5516e4bfb3
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f56f129e49d - std::backtrace_rs::backtrace::trace_unsynchronized::hbcdb3988957bd716
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f56f129e49d - std::sys_common::backtrace::_print_fmt::h5dee031312c45400
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f56f129e49d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd5d2448ca2750cd8
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f56f12fa2dc - core::fmt::write::he31ff040039f6999
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f56f128fb01 - std::io::Write::write_fmt::h75230ad9ee2f58d4
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/io/mod.rs:1654:15
   6:     0x7f56f12a1175 - std::sys_common::backtrace::_print::h6d256b189395961d
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f56f12a1175 - std::sys_common::backtrace::print::hf7a5cdb24da5efce
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f56f12a1175 - std::panicking::default_hook::{{closure}}::hefcd5ae4bc8144ef
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/panicking.rs:295:22
   9:     0x7f56f12a0e96 - std::panicking::default_hook::h31fefa94cf26c116
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/panicking.rs:314:9
  10:     0x7f56f1afbd21 - rustc_driver[474c9c938fff0173]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f56f12a184a - std::panicking::rust_panic_with_hook::hec596526dd12471b
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/panicking.rs:702:17
  12:     0x7f56f12a1687 - std::panicking::begin_panic_handler::{{closure}}::h13655ebbacf6e61b
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/panicking.rs:588:13
  13:     0x7f56f129e954 - std::sys_common::backtrace::__rust_end_short_backtrace::h3d5cf270ce171725
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f56f12a13b9 - rust_begin_unwind
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/panicking.rs:584:5
  15:     0x7f56f12662c3 - core::panicking::panic_fmt::hc96e8f9879b3be4d
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/core/src/panicking.rs:142:14
  16:     0x7f56f12f7088 - core::panicking::assert_failed_inner::hbb30d39562d7707e
  17:     0x55599bec9274 - core[3e71735526d4dd4b]::panicking::assert_failed::<rustdoc_json_types[71cfd09cfc6c9797]::Item, rustdoc_json_types[71cfd09cfc6c9797]::Item>
  18:     0x55599c1eb39b - <rustdoc[24247a081d39efee]::json::JsonRenderer as rustdoc[24247a081d39efee]::formats::renderer::FormatRenderer>::item
  19:     0x55599c1e50c9 - <rustdoc[24247a081d39efee]::json::JsonRenderer as rustdoc[24247a081d39efee]::formats::renderer::FormatRenderer>::item
  20:     0x55599c1671da - rustdoc[24247a081d39efee]::formats::renderer::run_format::<rustdoc[24247a081d39efee]::json::JsonRenderer>
  21:     0x55599bf244ae - rustdoc[24247a081d39efee]::run_renderer::<rustdoc[24247a081d39efee]::json::JsonRenderer>
  22:     0x55599c1f473b - <rustc_session[4c6539f8b6b5b057]::session::Session>::time::<core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>, rustdoc[24247a081d39efee]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  23:     0x55599c12ab57 - <rustc_interface[ee056990e706b6be]::passes::QueryContext>::enter::<rustdoc[24247a081d39efee]::main_options::{closure#0}::{closure#0}::{closure#1}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>
  24:     0x55599c02c16a - <rustc_interface[ee056990e706b6be]::interface::Compiler>::enter::<rustdoc[24247a081d39efee]::main_options::{closure#0}::{closure#0}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>
  25:     0x55599bedd62a - rustc_span[f5616531ecb15b4d]::with_source_map::<core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>, rustc_interface[ee056990e706b6be]::interface::create_compiler_and_run<core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>, rustdoc[24247a081d39efee]::main_options::{closure#0}>::{closure#1}>
  26:     0x55599c056001 - rustc_interface[ee056990e706b6be]::interface::create_compiler_and_run::<core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>, rustdoc[24247a081d39efee]::main_options::{closure#0}>
  27:     0x55599bee1b0f - <scoped_tls[d09f79adf346c7b8]::ScopedKey<rustc_span[f5616531ecb15b4d]::SessionGlobals>>::set::<rustdoc[24247a081d39efee]::main_args::{closure#0}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>
  28:     0x55599c05651f - std[a1a042396ca320bb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ee056990e706b6be]::util::run_in_thread_pool_with_globals<rustdoc[24247a081d39efee]::main_args::{closure#0}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>::{closure#0}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>
  29:     0x55599c147589 - <<std[a1a042396ca320bb]::thread::Builder>::spawn_unchecked_<rustc_interface[ee056990e706b6be]::util::run_in_thread_pool_with_globals<rustdoc[24247a081d39efee]::main_args::{closure#0}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>::{closure#0}, core[3e71735526d4dd4b]::result::Result<(), rustc_errors[792e4eb1ba22caeb]::ErrorGuaranteed>>::{closure#1} as core[3e71735526d4dd4b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f56f12ab783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb5e79c50ed46987d
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/alloc/src/boxed.rs:1951:9
  31:     0x7f56f12ab783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6e5b090d5a8c80f9
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/alloc/src/boxed.rs:1951:9
  32:     0x7f56f12ab783 - std::sys::unix::thread::Thread::new::thread_start::h2a064b2098f494b9
                               at /rustc/d40f24e956a698e47a209541031c4045acc5a684/library/std/src/sys/unix/thread.rs:108:17
  33:     0x7f56f107f54d - <unknown>
  34:     0x7f56f1104b14 - clone
  35:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (d40f24e95 2022-06-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options

query stack during panic:
end of query stack
