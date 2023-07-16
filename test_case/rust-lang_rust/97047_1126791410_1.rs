
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:11 ~ playground[c737]::{impl#0}::new) (NoSolution): could not prove Binder(ConstEvaluatable(WithOptConstParam { did: DefId(0:5 ~ playground[c737]::Changes::{constant#0}), const_param_did: None }, [Const { ty: &[&str], val: Param(CHANGES/#0) }]), [])
  [--> src/lib.rs:15:9
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)   |
15 | /         Self {
16 | |             changes: [0; CHANGES.len()],
17 | |         }
   | |_________^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:149:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1369:13
stack backtrace:
   0:     0x7f2991265fbd - std::backtrace_rs::backtrace::libunwind::trace::hfff13da9629808b3
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2991265fbd - std::backtrace_rs::backtrace::trace_unsynchronized::h574c052d104edbb2
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2991265fbd - std::sys_common::backtrace::_print_fmt::hdec7c42a9aaff178
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f2991265fbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7b437db464fe2eae
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f29912c1c4c - core::fmt::write::ha127ca1dd987811c
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f2991257731 - std::io::Write::write_fmt::haed364b23f647916
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/io/mod.rs:1654:15
   6:     0x7f2991268cd5 - std::sys_common::backtrace::_print::hb1db8a563e06b2cc
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f2991268cd5 - std::sys_common::backtrace::print::h16e06d5c2185a605
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f2991268cd5 - std::panicking::default_hook::{{closure}}::hc4512ccdd0ac08a5
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/panicking.rs:295:22
   9:     0x7f2991268949 - std::panicking::default_hook::hf0d6585cd7a26bd4
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/panicking.rs:314:9
  10:     0x7f2991a2dff1 - rustc_driver[8ee73bb5f36af3de]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f29912694a6 - std::panicking::rust_panic_with_hook::hb5d5e6f805719e75
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/panicking.rs:702:17
  12:     0x7f2992ba9cd1 - std[ac1b39edc8935404]::panicking::begin_panic::<rustc_errors[7358869d503cdbaa]::ExplicitBug>::{closure#0}
  13:     0x7f2992ba9c36 - std[ac1b39edc8935404]::sys_common::backtrace::__rust_end_short_backtrace::<std[ac1b39edc8935404]::panicking::begin_panic<rustc_errors[7358869d503cdbaa]::ExplicitBug>::{closure#0}, !>
  14:     0x7f2992bad906 - std[ac1b39edc8935404]::panicking::begin_panic::<rustc_errors[7358869d503cdbaa]::ExplicitBug>
  15:     0x7f2992bb19e6 - std[ac1b39edc8935404]::panic::panic_any::<rustc_errors[7358869d503cdbaa]::ExplicitBug>
  16:     0x7f2994370c41 - <rustc_errors[7358869d503cdbaa]::HandlerInner as core[295e1f7691ff5442]::ops::drop::Drop>::drop
  17:     0x7f2993ac4198 - core[295e1f7691ff5442]::ptr::drop_in_place::<rustc_session[ff20e138b897b605]::parse::ParseSess>
  18:     0x7f2993ac64b3 - <alloc[94313895c08d63e9]::rc::Rc<rustc_session[ff20e138b897b605]::session::Session> as core[295e1f7691ff5442]::ops::drop::Drop>::drop
  19:     0x7f2993ac306d - core[295e1f7691ff5442]::ptr::drop_in_place::<rustc_interface[d033f7e4bca3d7ee]::interface::Compiler>
  20:     0x7f2993ac29d4 - rustc_span[14222bfc8ecd5677]::with_source_map::<core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>, rustc_interface[d033f7e4bca3d7ee]::interface::create_compiler_and_run<core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>, rustc_driver[8ee73bb5f36af3de]::run_compiler::{closure#1}>::{closure#1}>
  21:     0x7f2993aae0c4 - rustc_interface[d033f7e4bca3d7ee]::interface::create_compiler_and_run::<core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>, rustc_driver[8ee73bb5f36af3de]::run_compiler::{closure#1}>
  22:     0x7f2993a98692 - <scoped_tls[2d60114f458dc383]::ScopedKey<rustc_span[14222bfc8ecd5677]::SessionGlobals>>::set::<rustc_interface[d033f7e4bca3d7ee]::interface::run_compiler<core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>, rustc_driver[8ee73bb5f36af3de]::run_compiler::{closure#1}>::{closure#0}, core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>>
  23:     0x7f2993aaf7bf - std[ac1b39edc8935404]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d033f7e4bca3d7ee]::util::run_in_thread_pool_with_globals<rustc_interface[d033f7e4bca3d7ee]::interface::run_compiler<core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>, rustc_driver[8ee73bb5f36af3de]::run_compiler::{closure#1}>::{closure#0}, core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>>::{closure#0}, core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>>
  24:     0x7f2993aaf8f9 - <<std[ac1b39edc8935404]::thread::Builder>::spawn_unchecked_<rustc_interface[d033f7e4bca3d7ee]::util::run_in_thread_pool_with_globals<rustc_interface[d033f7e4bca3d7ee]::interface::run_compiler<core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>, rustc_driver[8ee73bb5f36af3de]::run_compiler::{closure#1}>::{closure#0}, core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>>::{closure#0}, core[295e1f7691ff5442]::result::Result<(), rustc_errors[7358869d503cdbaa]::ErrorGuaranteed>>::{closure#1} as core[295e1f7691ff5442]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f29912733c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h54f496eb95d11bdf
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/alloc/src/boxed.rs:1872:9
  26:     0x7f29912733c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6d49434441582c00
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/alloc/src/boxed.rs:1872:9
  27:     0x7f29912733c3 - std::sys::unix::thread::Thread::new::thread_start::hbff7f546b16d10d6
                               at /rustc/f1f721e64014863f41c1a386b04af04c2de25321/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7f29911a3609 - start_thread
  29:     0x7f29910bc133 - clone
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (f1f721e64 2022-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
