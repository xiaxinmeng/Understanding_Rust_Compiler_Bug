
thread 'rustc' panicked at 'non-eager expansion without a parent scope', compiler/rustc_resolve/src/macros.rs:238:22
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::option::expect_failed
   3: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::resolve_macro_invocation
   4: rustc_expand::expand::MacroExpander::fully_expand_fragment
   5: rustc_expand::expand::MacroExpander::expand_crate
   6: rustc_session::utils::<impl rustc_session::session::Session>::time
   7: rustc_interface::passes::configure_and_expand_inner
   8: rustc_interface::passes::configure_and_expand::{{closure}}
   9: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  10: rustc_interface::passes::configure_and_expand
  11: rustc_interface::queries::Queries::expansion
  12: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  13: rustc_span::with_source_map
  14: rustc_interface::interface::create_compiler_and_run
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (158f8d034 2020-12-29) running on x86_64-apple-darwin

query stack during panic:
end of query stack
error: internal compiler error: expansion entered force mode without producing any errors
 --> src/main.rs:3:1
  |
3 | #[a = foo!()]
  | ^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_expand/src/expand.rs:459:34

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13
stack backtrace:
   0:        0x1021acfbc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha6f5ddd5259d3e16
   1:        0x102210d2d - core::fmt::write::hefe95a44532fe6ed
   2:        0x10219ee06 - std::io::Write::write_fmt::hf513c99961415bff
   3:        0x1021b0c49 - std::panicking::default_hook::{{closure}}::h70a3a940826edea5
   4:        0x1021b07d9 - std::panicking::default_hook::h9520f36dd50be056
   5:        0x1097cddf8 - rustc_driver::report_ice::h64178aa7cd01307c
   6:        0x1021b142e - std::panicking::rust_panic_with_hook::h102f8bee4e0ef4c7
   7:        0x1021b0f35 - std::panicking::begin_panic_handler::{{closure}}::hb72eee9aad2e147c
   8:        0x1021ad478 - std::sys_common::backtrace::__rust_end_short_backtrace::h372ff87ecb2667f3
   9:        0x1021b0e9a - _rust_begin_unwind
  10:        0x10223730b - std::panicking::begin_panic_fmt::hf0e714fa3e981e6e
  11:        0x10de5cab6 - rustc_errors::HandlerInner::flush_delayed::he9da53448a9bd660
  12:        0x10de58cfb - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h44af1dc5df0a1fa4
  13:        0x1097e855a - core::ptr::drop_in_place::hd03156b954b70984
  14:        0x1097eb7a8 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::hd4c9eabf652375d9
  15:        0x1097e48e2 - core::ptr::drop_in_place::h83698f86dd7e4a49
  16:        0x1097d07d5 - rustc_span::with_source_map::hb3cfcc431ed11000
  17:        0x109803f64 - rustc_interface::interface::create_compiler_and_run::he8e7b2281c1964ad
  18:        0x1097dc714 - std::sys_common::backtrace::__rust_begin_short_backtrace::h9bf0a320b59d4a76
  19:        0x109773889 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb43737fa064e4b63
  20:        0x1021be2cd - std::sys::unix::thread::Thread::new::thread_start::h93dd3097fa4fa219
  21:     0x7fff71e53109 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (158f8d034 2020-12-29) running on x86_64-apple-darwin

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
[1]    11925 illegal hardware instruction  RUST_BACKTRACE=1 rustc +nightly src/main.rs
