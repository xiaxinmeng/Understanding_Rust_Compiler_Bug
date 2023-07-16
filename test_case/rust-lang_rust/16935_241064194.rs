
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'unhandled token in quote!', ../src/libsyntax/ext/quote.rs:707
stack backtrace:
   1:        0x111037e79 - std::sys::backtrace::tracing::imp::write::h482d45d91246faa2
   2:        0x111046b70 - std::panicking::default_hook::_{{closure}}::h89158f66286b674e
   3:        0x111044e9f - std::panicking::default_hook::h9e30d428ee3b0c43
   4:        0x1110455a6 - std::panicking::rust_panic_with_hook::h2224f33fb7bf2f4c
   5:        0x1107fd4d4 - std::panicking::begin_panic::h6da27a7ee15843ce
   6:        0x1109e95f3 - syntax::ext::quote::expr_mk_token::h7c3422b80bef3bd5
   7:        0x1109ebdb0 - syntax::ext::quote::statements_mk_tt::he0f33941b94e20bb
   8:        0x1108203af - _<core..iter..FlatMap<I, U, F> as core..iter..iterator..Iterator>::next::hb7943eed4d804bfc
   9:        0x11081918f - _<core..iter..Chain<A, B> as core..iter..iterator..Iterator>::next::hf66dec9ac7ebfd3e
  10:        0x11081e49a - _<collections..vec..Vec<T> as core..iter..traits..FromIterator<T>>::from_iter::hb6ba9a4116122593
  11:        0x1109ea625 - syntax::ext::quote::statements_mk_tt::he0f33941b94e20bb
  12:        0x1109ed8e2 - syntax::ext::quote::statements_mk_tts::hc942124e7cb85dbc
  13:        0x1109eda54 - syntax::ext::quote::expand_tts::h25510a66e2e03c00
  14:        0x1109eefa4 - syntax::ext::quote::expand_parse_call::h362dceaee0d8b0c6
  15:        0x1109e4c6a - syntax::ext::quote::expand_quote_expr::h7f4c2771cabcf421
  16:        0x1109b07d5 - _<F as syntax..ext..base..TTMacroExpander>::expand::h2cbfb95c85d07886
  17:        0x1109cb75b - syntax::ext::expand::expand_mac_invoc::mac_result::ha3a8dfd05d9eecb6
  18:        0x11096d850 - _<syntax..ptr..P<T>>::and_then::h53ce1d732b16c30f
  19:        0x11088da64 - syntax::fold::Folder::fold_exprs::haec82b8e9fe75416
  20:        0x1108f0234 - syntax::fold::noop_fold_expr::hab84161d928c2074
  21:        0x1109c6d85 - syntax::ext::expand::expand_expr::hd4de34daa0a5a90a
  22:        0x11096d963 - _<syntax..ptr..P<T>>::and_then::h53ce1d732b16c30f
  23:        0x1109cd0de - syntax::ext::expand::expand_stmt::hf1246bdbc447b35b
  24:        0x11083d0c2 - _<collections..vec..Vec<T> as syntax..util..move_map..MoveMap<T>>::move_flat_map::h95ab7180e0970e65
  25:        0x1109dc77f - _<syntax..ext..expand..MacroExpander<'a, 'b> as syntax..fold..Folder>::fold_block::h84ca09e1246a6f7f
  26:        0x1108b2c3f - syntax::fold::noop_fold_item_kind::he96b2686f10d5f9d
  27:        0x1108bee78 - syntax::fold::noop_fold_item_simple::hb58741bcf641e653
  28:        0x1109d0d40 - syntax::ext::expand::expand_annotatable::hd731ac0c7a5098e1
  29:        0x1109cc7cf - syntax::ext::expand::expand_item::h34a68666bcb77b42
  30:        0x1109dc20a - _<syntax..ext..expand..MacroExpander<'a, 'b> as syntax..fold..Folder>::fold_item::h11dc20d6fdb44d60
  31:        0x1108bd942 - syntax::fold::noop_fold_mod::h7ccd83e899e81c3c
  32:        0x1108b26b2 - syntax::fold::noop_fold_item_kind::he96b2686f10d5f9d
  33:        0x1108bee78 - syntax::fold::noop_fold_item_simple::hb58741bcf641e653
  34:        0x1109d102b - syntax::ext::expand::expand_annotatable::hd731ac0c7a5098e1
  35:        0x1109cc7cf - syntax::ext::expand::expand_item::h34a68666bcb77b42
  36:        0x1109dc342 - _<syntax..ext..expand..MacroExpander<'a, 'b> as syntax..fold..Folder>::fold_item::h11dc20d6fdb44d60
  37:        0x1109db30e - _<syntax..ext..expand..MacroExpander<'a, 'b> as syntax..fold..Folder>::fold_crate::hb0a12b0d8df69afd
  38:        0x1109dda1e - syntax::ext::expand::expand_crate_with_expander::h77575f1d1b34ae40
  39:        0x1109dd4ba - syntax::ext::expand::expand_crate::h72ec18c6565094c4
  40:        0x10c3d668b - rustc_driver::driver::phase_2_configure_and_expand::_{{closure}}::hae7f286174f41285
  41:        0x10c389133 - rustc_driver::driver::phase_2_configure_and_expand::h7a450d24678fc450
  42:        0x10c38089c - rustc_driver::driver::compile_input::hd9ecc57abd3cba85
  43:        0x10c3b2579 - rustc_driver::run_compiler::h184264500271cc39
  44:        0x10c2fdc94 - std::panicking::try::do_call::h17a7a17ad7240c5c
  45:        0x11104d8aa - __rust_maybe_catch_panic
  46:        0x10c317d57 - _<F as alloc..boxed..FnBox<A>>::call_box::h93f9128277b2964a
  47:        0x111043c65 - std::sys::thread::Thread::new::thread_start::he0bf102845911132
  48:     0x7fff89ecf99c - _pthread_body
  49:     0x7fff89ecf919 - _pthread_start
