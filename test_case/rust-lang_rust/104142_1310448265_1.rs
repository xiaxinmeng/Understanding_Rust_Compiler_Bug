
<output>
   Compiling tr v0.1.0 (/home/stephan/rust1/tr)
thread '<unnamed>' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', compiler/rustc_span/src/lib.rs:1700:17
stack backtrace:
   0:     0x7fab753f63e0 - std::backtrace_rs::backtrace::libunwind::trace::h475e6d14c20ac628
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fab753f63e0 - std::backtrace_rs::backtrace::trace_unsynchronized::h782386969d170809
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fab753f63e0 - std::sys_common::backtrace::_print_fmt::h2cff96f1339c9fb3
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fab753f63e0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h65381e912452bd50
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fab754584fe - core::fmt::write::hddf5212808edda8e
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fab753e6555 - std::io::Write::write_fmt::h86ff774c1fbcc56f
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/io/mod.rs:1682:15
   6:     0x7fab753f61a5 - std::sys_common::backtrace::_print::hac5746116d4b1148
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fab753f61a5 - std::sys_common::backtrace::print::h989584698b4383b5
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fab753f8f7f - std::panicking::default_hook::{{closure}}::hdf7100a7f19aaac7
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/panicking.rs:267:22
   9:     0x7fab753f8cba - std::panicking::default_hook::hd54c9e09261f65ab
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/panicking.rs:286:9
  10:     0x7fab753f978c - std::panicking::rust_panic_with_hook::h6cb13ee30fd78212
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/panicking.rs:688:13
  11:     0x7fab753f94e1 - std::panicking::begin_panic_handler::{{closure}}::h81e7b2a35760b48f
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/panicking.rs:577:13
  12:     0x7fab753f688c - std::sys_common::backtrace::__rust_end_short_backtrace::h74bbf3f6d71875f1
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7fab753f9242 - rust_begin_unwind
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/panicking.rs:575:5
  14:     0x7fab75454ee3 - core::panicking::panic_fmt::h89bd6f1292ecd482
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/core/src/panicking.rs:65:14
  15:     0x7fab75454fbd - core::panicking::panic::h8621bfb166577b81
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/core/src/panicking.rs:115:5
  16:     0x7fab76e6bda8 - <rustc_span[2e77502f23847349]::source_map::SourceMap>::lookup_char_pos
  17:     0x7fab77c5e132 - <rustc_errors[2bcb93b13263f24b]::emitter::EmitterWriter>::get_multispan_max_line_num
  18:     0x7fab77c55ceb - <rustc_errors[2bcb93b13263f24b]::emitter::EmitterWriter as rustc_errors[2bcb93b13263f24b]::emitter::Emitter>::emit_diagnostic
  19:     0x7fab77c53b09 - <rustc_errors[2bcb93b13263f24b]::json::Diagnostic>::from_errors_diagnostic
  20:     0x7fab77c530bc - <rustc_errors[2bcb93b13263f24b]::json::JsonEmitter as rustc_errors[2bcb93b13263f24b]::emitter::Emitter>::emit_diagnostic
  21:     0x7fab768c9c0a - <rustc_errors[2bcb93b13263f24b]::HandlerInner>::emit_diagnostic
  22:     0x7fab77415091 - <rustc_errors[2bcb93b13263f24b]::Handler>::emit_diagnostic
  23:     0x7fab773d198d - <rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed as rustc_errors[2bcb93b13263f24b]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  24:     0x7fab779de51f - rustc_builtin_macros[27fc18700f4ab977]::format::make_format_args
  25:     0x7fab779d3829 - rustc_builtin_macros[27fc18700f4ab977]::format::expand_format_args_impl
  26:     0x7fab7704f985 - <rustc_expand[222b844153769ae4]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fab778ed2e7 - <rustc_expand[222b844153769ae4]::expand::MacroExpander>::expand_crate
  28:     0x7fab778ec053 - <rustc_session[6a0929c18a48d200]::session::Session>::time::<core[7d0df67402c3c815]::result::Result<rustc_ast[e734813f4b1203b7]::ast::Crate, rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>, rustc_interface[2d69c3ee45228f8a]::passes::configure_and_expand::{closure#1}>
  29:     0x7fab778b3086 - rustc_interface[2d69c3ee45228f8a]::passes::configure_and_expand
  30:     0x7fab778ab934 - <rustc_interface[2d69c3ee45228f8a]::queries::Queries>::expansion
  31:     0x7fab778a927c - <rustc_interface[2d69c3ee45228f8a]::interface::Compiler>::enter::<rustc_driver[41f9f2ea3541ee7a]::run_compiler::{closure#1}::{closure#2}, core[7d0df67402c3c815]::result::Result<core[7d0df67402c3c815]::option::Option<rustc_interface[2d69c3ee45228f8a]::queries::Linker>, rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>>
  32:     0x7fab778a43c2 - rustc_span[2e77502f23847349]::with_source_map::<core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>, rustc_interface[2d69c3ee45228f8a]::interface::run_compiler<core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>, rustc_driver[41f9f2ea3541ee7a]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7fab778a3e9c - <scoped_tls[5e9cb458eb9b4943]::ScopedKey<rustc_span[2e77502f23847349]::SessionGlobals>>::set::<rustc_interface[2d69c3ee45228f8a]::interface::run_compiler<core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>, rustc_driver[41f9f2ea3541ee7a]::run_compiler::{closure#1}>::{closure#0}, core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>>
  34:     0x7fab778a3488 - std[15847fe8052079fc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d69c3ee45228f8a]::util::run_in_thread_pool_with_globals<rustc_interface[2d69c3ee45228f8a]::interface::run_compiler<core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>, rustc_driver[41f9f2ea3541ee7a]::run_compiler::{closure#1}>::{closure#0}, core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>>
  35:     0x7fab778a31ac - <<std[15847fe8052079fc]::thread::Builder>::spawn_unchecked_<rustc_interface[2d69c3ee45228f8a]::util::run_in_thread_pool_with_globals<rustc_interface[2d69c3ee45228f8a]::interface::run_compiler<core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>, rustc_driver[41f9f2ea3541ee7a]::run_compiler::{closure#1}>::{closure#0}, core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7d0df67402c3c815]::result::Result<(), rustc_errors[2bcb93b13263f24b]::ErrorGuaranteed>>::{closure#1} as core[7d0df67402c3c815]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fab79343953 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9a068bdd0d9f070e
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/alloc/src/boxed.rs:2000:9
  37:     0x7fab79343953 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h86d7b043c0b02a10
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/alloc/src/boxed.rs:2000:9
  38:     0x7fab79343953 - std::sys::unix::thread::Thread::new::thread_start::hfaaa856b3ec2dce6
                               at /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/library/std/src/sys/unix/thread.rs:108:17
  39:     0x7fab752bc609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  40:     0x7fab751df133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  41:                0x0 - <unknown>
error: could not compile `tr`


