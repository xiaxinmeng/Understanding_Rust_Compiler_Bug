
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libcollections/vec.rs:1392
stack backtrace:
   1:        0x110cbf32c - std::sys::imp::backtrace::tracing::imp::write::hf587afb8e94ad165
   2:        0x110ccbd8e - std::panicking::default_hook::{{closure}}::haf3443cb412055ce
   3:        0x110ccb933 - std::panicking::default_hook::h742f925bfab3bbfa
   4:        0x110ccc247 - std::panicking::rust_panic_with_hook::h6f06ff8d28a94df6
   5:        0x110ccc0f4 - std::panicking::begin_panic::h7b9167ba3324cfae
   6:        0x110ccc012 - std::panicking::begin_panic_fmt::hb5f8f1fe0fe23e28
   7:        0x110ccbf77 - rust_begin_unwind
   8:        0x110d09bc0 - core::panicking::panic_fmt::he6eb92dab4407c61
   9:        0x110d09b38 - core::panicking::panic_bounds_check::h37b4772a417ae8c7
  10:        0x1104c03a3 - syntax::tokenstream::TokenTree::get_tt::he5c034e340a9b3c5
  11:        0x11044c55b - syntax::parse::parser::Parser::next_tok::h905448a1057a8fed
  12:        0x110455dd2 - syntax::parse::parser::Parser::bump::h440692046326abf3
  13:        0x11046d4e0 - syntax::parse::parser::Parser::parse_token_tree::he0cf0ff304b8e43c
  14:        0x11046d64f - syntax::parse::parser::Parser::parse_token_tree::he0cf0ff304b8e43c
  15:        0x11046d64f - syntax::parse::parser::Parser::parse_token_tree::he0cf0ff304b8e43c
  16:        0x110535132 - syntax::ext::tt::macro_parser::parse::he25612d1dc16302d
  17:        0x1104c05f7 - syntax::tokenstream::TokenTree::parse::h466bc5812b148d50
  18:        0x1105385ea - <syntax::ext::tt::macro_rules::MacroRulesMacroExpander as syntax::ext::base::TTMacroExpander>::expand::hc508dcca9558cc38
  19:        0x11050b5a9 - syntax::ext::expand::MacroExpander::expand_invoc::hcb8333bbfb4f6c60
  20:        0x1105082d9 - syntax::ext::expand::MacroExpander::expand::h5505c45c00137bb7
  21:        0x11050755f - syntax::ext::expand::MacroExpander::expand_crate::h44b347be1c8e3673
  22:        0x10bbea781 - rustc_driver::driver::phase_2_configure_and_expand::{{closure}}::h181e912e232702cc
  23:        0x10bbe290f - rustc_driver::driver::phase_2_configure_and_expand::h239efe12a958d8ec
  24:        0x10bbdc21f - rustc_driver::driver::compile_input::h550b23c65a0bc968
  25:        0x10bc2524e - rustc_driver::run_compiler::h65867e99dfb91e54
  26:        0x10bb3d0d8 - std::panicking::try::do_call::hdfa57184047b194c
  27:        0x110ccee5a - __rust_maybe_catch_panic
  28:        0x10bb65b33 - <F as alloc::boxed::FnBox<A>>::call_box::h7af5270adbc74627
  29:        0x110ccaeb4 - std::sys::imp::thread::Thread::new::thread_start::h4ad0b40513420e9c
  30:     0x7fffa73a3aaa - _pthread_body
  31:     0x7fffa73a39f6 - _pthread_start
