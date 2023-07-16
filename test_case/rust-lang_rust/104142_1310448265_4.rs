
<output>

  Compiling tr v0.1.0 (/home/stephan/rust1/tr)
thread '<unnamed>' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', compiler/rustc_span/src/lib.rs:1700:17
stack backtrace:
   0:     0x7f27394759a0 - std::backtrace_rs::backtrace::libunwind::trace::h2df8645d586d9354
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f27394759a0 - std::backtrace_rs::backtrace::trace_unsynchronized::h8ccc7432adfc32a8
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f27394759a0 - std::sys_common::backtrace::_print_fmt::h6b5315b3e6c5b112
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f27394759a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1196a40d5ac35d56
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f27394d198e - core::fmt::write::hb5395aee3db34e90
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f2739465c15 - std::io::Write::write_fmt::hf1b72fac8a3acb27
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/io/mod.rs:1682:15
   6:     0x7f2739475765 - std::sys_common::backtrace::_print::h805a3d8a840d4dd9
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f2739475765 - std::sys_common::backtrace::print::h9dc5789e99bcd646
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f273947856f - std::panicking::default_hook::{{closure}}::h79c2a100d70e0a69
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:267:22
   9:     0x7f27394782aa - std::panicking::default_hook::h9d54a6e2e183091b
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:286:9
  10:     0x7f2739478d78 - std::panicking::rust_panic_with_hook::h66c09c3756b56830
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:688:13
  11:     0x7f2739478ad1 - std::panicking::begin_panic_handler::{{closure}}::haacf5fe4954e1592
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:577:13
  12:     0x7f2739475e4c - std::sys_common::backtrace::__rust_end_short_backtrace::h3f18dbb82ab7bb81
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7f2739478832 - rust_begin_unwind
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:575:5
  14:     0x7f27394ce373 - core::panicking::panic_fmt::h44028ba00aff6a3b
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/core/src/panicking.rs:65:14
  15:     0x7f27394ce44d - core::panicking::panic::h071343d94407099c
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/core/src/panicking.rs:115:5
  16:     0x7f273ad262dd - <rustc_span[85bea1f81190796e]::source_map::SourceMap>::lookup_char_pos
  17:     0x7f273bc8f352 - <rustc_errors[8289f13c10dc8783]::emitter::EmitterWriter>::get_multispan_max_line_num
  18:     0x7f273bc86f18 - <rustc_errors[8289f13c10dc8783]::emitter::EmitterWriter as rustc_errors[8289f13c10dc8783]::emitter::Emitter>::emit_diagnostic
  19:     0x7f273bc84c11 - <rustc_errors[8289f13c10dc8783]::json::Diagnostic>::from_errors_diagnostic
  20:     0x7f273bc841bc - <rustc_errors[8289f13c10dc8783]::json::JsonEmitter as rustc_errors[8289f13c10dc8783]::emitter::Emitter>::emit_diagnostic
  21:     0x7f273b6bb28a - <rustc_errors[8289f13c10dc8783]::HandlerInner>::emit_diagnostic
  22:     0x7f273b6bac41 - <rustc_errors[8289f13c10dc8783]::Handler>::emit_diagnostic
  23:     0x7f273b676e8d - <rustc_errors[8289f13c10dc8783]::ErrorGuaranteed as rustc_errors[8289f13c10dc8783]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  24:     0x7f273b92a573 - rustc_builtin_macros[ad802a20e964a19a]::format::make_format_args
  25:     0x7f273b91f719 - rustc_builtin_macros[ad802a20e964a19a]::format::expand_format_args_impl
  26:     0x7f273b06b864 - <rustc_expand[16ea2ac5f3917bd2]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7f273b9c87bd - <rustc_expand[16ea2ac5f3917bd2]::expand::MacroExpander>::expand_crate
  28:     0x7f273b9c7504 - <rustc_session[638750f383577479]::session::Session>::time::<core[6f75f940baff7b16]::result::Result<rustc_ast[3c7fc2053f8ae4a5]::ast::Crate, rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_interface[e10a04643064e0c3]::passes::configure_and_expand::{closure#1}>
  29:     0x7f273b98d986 - rustc_interface[e10a04643064e0c3]::passes::configure_and_expand
  30:     0x7f273b986154 - <rustc_interface[e10a04643064e0c3]::queries::Queries>::expansion
  31:     0x7f273b983a9c - <rustc_interface[e10a04643064e0c3]::interface::Compiler>::enter::<rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}::{closure#2}, core[6f75f940baff7b16]::result::Result<core[6f75f940baff7b16]::option::Option<rustc_interface[e10a04643064e0c3]::queries::Linker>, rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  32:     0x7f273b97b6f2 - rustc_span[85bea1f81190796e]::with_source_map::<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7f273b97b1e9 - <scoped_tls[df6a59750083ca9d]::ScopedKey<rustc_span[85bea1f81190796e]::SessionGlobals>>::set::<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  34:     0x7f273b97a7f8 - std[6a238fb43aa4c274]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e10a04643064e0c3]::util::run_in_thread_pool_with_globals<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  35:     0x7f273b97a51c - <<std[6a238fb43aa4c274]::thread::Builder>::spawn_unchecked_<rustc_interface[e10a04643064e0c3]::util::run_in_thread_pool_with_globals<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#1} as core[6f75f940baff7b16]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f273d31dbc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h23aa6e7db304ed51
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:1987:9
  37:     0x7f273d31dbc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0482f1835c06f38d
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:1987:9
  38:     0x7f273d31dbc3 - std::sys::unix::thread::Thread::new::thread_start::h5213f0bce91e8f3e
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys/unix/thread.rs:108:17
  39:     0x7f2739344609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  40:     0x7f2739267133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  41:                0x0 - <unknown>
error: could not compile `tr`

