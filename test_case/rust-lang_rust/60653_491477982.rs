
$ RUST_BACKTRACE=full rustc +devel file.rs --edition 2018 -Ztreat-err-as-bug
error: expected `!`, found `{`
 --> file.rs:2:13
  |
2 | match await { await => () }
  | -----       ^ expected `!`
  | |
  | while parsing this match expression

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:534:13
stack backtrace:
   0:        0x10c9fd69f - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h8a6db83f7a75af33
   1:        0x10c9ddebb - std::sys_common::backtrace::print::hf2a00a08498348a8
   2:        0x10c9e13b6 - std::panicking::default_hook::{{closure}}::h4e239a3e750f0ac0
   3:        0x10c9e113a - std::panicking::default_hook::hff4131d917056e24
   4:        0x109f7c842 - rustc::util::common::panic_hook::h2baf3bde54671dd9
   5:        0x10c9e1ab1 - std::panicking::rust_panic_with_hook::hcf602bb84d4497b1
   6:        0x10c66d264 - std::panicking::begin_panic::he374428343d5a254
   7:        0x10c651506 - rustc_errors::Handler::emit_db::h79e71562dd360559
   8:        0x10c65bddc - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h45b9b354b10ba5c3
   9:        0x10be47b34 - syntax::parse::parser::Parser::parse_block_tail::h688c940f2b491872
  10:        0x10be478ed - syntax::parse::parser::Parser::parse_inner_attrs_and_block::ha73c7b7cce9c9007
  11:        0x10be4f968 - syntax::parse::parser::Parser::parse_item_fn::haf7d87d0053e50ad
  12:        0x10be5b9e0 - syntax::parse::parser::Parser::parse_item_implementation::h89329ee2e320d99c
  13:        0x10bd00d06 - syntax::parse::parser::Parser::collect_tokens::ha8308cb945828795
  14:        0x10be596a0 - syntax::parse::parser::Parser::parse_item_::hb6489be270d2b09c
  15:        0x10be57302 - syntax::parse::parser::Parser::parse_mod_items::h67ff38dc4843fbef
  16:        0x10be6d81c - syntax::parse::parser::Parser::parse_crate_mod::hb03b42ac8b02b528
  17:        0x10bdc7fde - syntax::parse::parse_crate_from_file::h474a7b1e8f7728c2
  18:        0x106474442 - rustc_interface::passes::parse::{{closure}}::hb2dc9bc13f902418
  19:        0x10646ed9a - rustc::util::common::time::h4090d8453a2a1287
  20:        0x106436709 - rustc_interface::passes::parse::hc5fc8f6a5bdc5374
  21:        0x106387290 - rustc_interface::queries::Query<T>::compute::h5010fd859f8c1555
  22:        0x1064c297b - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::parse::he375b870c47ab621
  23:        0x106105aab - rustc_interface::interface::run_compiler_in_existing_thread_pool::h75863db245a1f7c4
  24:        0x1060f67f6 - std::thread::local::LocalKey<T>::with::ha68537697b1d9847
  25:        0x1060f95c5 - scoped_tls::ScopedKey<T>::set::h6c351d6afaba1b0d
  26:        0x1060c0e72 - syntax::with_globals::h6be6f8bb8a5c44d4
  27:        0x10619a4d8 - std::sys_common::backtrace::__rust_begin_short_backtrace::h2bc98a50247a92df
  28:        0x10ca1fb9e - __rust_maybe_catch_panic
  29:        0x10618e8a8 - std::panicking::try::h66c2ea20a72fdf6c
  30:        0x1060f5cbb - core::ops::function::FnOnce::call_once{{vtable.shim}}::h44525c41b28abf99
  31:        0x10c9fa06d - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h2847ca8ac1b9af59
  32:        0x10c9ea1cd - std::sys_common::thread::start_thread::hbb0a7ccbb853d748
  33:        0x10c9fad28 - std::sys::unix::thread::Thread::new::thread_start::h8519a4aa16b7a55f
  34:     0x7fff6970f2ea - _pthread_body
  35:     0x7fff69712248 - _pthread_start
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z treat-err-as-bug
