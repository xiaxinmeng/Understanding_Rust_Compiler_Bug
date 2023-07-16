
nest! { S (
S (
S (
S (
S (
S (
S (
S (
S (
S (
S (
S (
S (
S ( S ( S ( S ( S ( S ( S ( S ( S ( S ( S ( S ( S ( 0 ) ) ) ) ) ) ) ) ) ) ) )
) ) ) ) ) ) ) ) ) ) ) ) ) ) }
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: (self.right != self.left)', ../src/libsyntax/print/pp.rs:447
stack backtrace:
   1:     0x7f23bc4a6670 - std::sys::backtrace::tracing::imp::write::h714760a4c8c0cdd8
   2:     0x7f23bc4b3dbb - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hff309ab1d83ffd90
   3:     0x7f23bc4b395c - std::panicking::default_hook::h08ad3bb09872855b
   4:     0x7f23bc47891f - std::sys_common::unwind::begin_unwind_inner::h406d5f1a330b854b
   5:     0x7f23b4ad42bf - std::sys_common::unwind::begin_unwind::h51df842978a11a26
   6:     0x7f23b4d0df87 - syntax::print::pp::Printer::pretty_print::hdaa42f3f0ec4f3c0
   7:     0x7f23b4d2803a - syntax::print::pprust::State::commasep_cmnt::hce4516d3bc68a067
   8:     0x7f23b4d33c3c - syntax::print::pprust::State::print_call_post::h3a0506ed92a97f5d
   9:     0x7f23b4d2e760 - syntax::print::pprust::State::print_expr_outer_attr_style::hd57493886e43b671
  10:     0x7f23b4d19d2a - syntax::print::pprust::to_string::hac387e51e4c4f772
  11:     0x7f23b4bb56e9 - syntax::print::pprust::token_to_string::hcfee81f1ef95dcca
  12:     0x7f23b4d1a71d - syntax::print::pprust::State::print_tt::hd363fc759894cd15
  13:     0x7f23b4d1b32d - syntax::print::pprust::State::print_tts::h3dcdbffe6b275dd8
  14:     0x7f23b4d1aebb - syntax::print::pprust::tts_to_string::hb290d011456f08bb
  15:     0x7f23b4e1a130 - _<ext..tt..macro_rules..MacroRulesMacroExpander as ext..base..TTMacroExpander>::expand::hb64d8291a0f495a6
  16:     0x7f23b4d9a41c - syntax::ext::expand::expand_stmt::h08d1d2bcde0cefaf
  17:     0x7f23b4d9d8cf - _<std..iter..FlatMap<I, U, F> as std..iter..Iterator>::next::hdfdae752d618f96a
  18:     0x7f23b4d9d05a - _<util..small_vector..SmallVector<T> as std..iter..FromIterator<T>>::from_iter::hd2d8e6287dd22630
  19:     0x7f23b4d9bac3 - syntax::ext::expand::expand_stmt::h08d1d2bcde0cefaf
  20:     0x7f23b4da481b - syntax::ext::expand::expand_block_elts::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h4e06053ab0d2f583
  21:     0x7f23b4da4324 - _<std..iter..FlatMap<I, U, F> as std..iter..Iterator>::next::hc77aa4875ddb4f42
  22:     0x7f23b4da3a8e - core::iter::Iterator::collect::h1a65a961106a7fbd
  23:     0x7f23b4da37d4 - _<ptr..P<T>>::map::hfc5f47b235cd94b9
  24:     0x7f23b4da35fa - syntax::ext::expand::expand_block::hff1bc444b208e8d3
  25:     0x7f23b4d7985b - syntax::ext::expand::expand_and_rename_fn_decl_and_block::hb764af6d140666f3
  26:     0x7f23b4d89354 - syntax::ext::expand::expand_item_kind::ha057f5cdbbf45776
  27:     0x7f23b4dc8c69 - syntax::fold::Folder::fold_item_simple::h7c85ad9e72031ec6
  28:     0x7f23b4dc88b6 - _<ptr..P<T>>::map::h822ea3fca75ed010
  29:     0x7f23b4d85895 - syntax::ext::expand::expand_annotatable::hd0129691b0f8b93d
  30:     0x7f23b4d825a0 - syntax::ext::expand::expand_item::h56ee539b7cde8407
  31:     0x7f23b4d908c1 - _<ext..expand..MacroExpander<'a, 'b> as fold..Folder>::fold_item::hdce1cac1401f2259
  32:     0x7f23b4d9057f - syntax::fold::noop_fold_mod::h0679922c15e93846
  33:     0x7f23b4d8991c - syntax::ext::expand::expand_item_kind::ha057f5cdbbf45776
  34:     0x7f23b4dc8c69 - syntax::fold::Folder::fold_item_simple::h7c85ad9e72031ec6
  35:     0x7f23b4dc88b6 - _<ptr..P<T>>::map::h822ea3fca75ed010
  36:     0x7f23b4d869a8 - syntax::ext::expand::expand_annotatable::hd0129691b0f8b93d
  37:     0x7f23b4d825a0 - syntax::ext::expand::expand_item::h56ee539b7cde8407
  38:     0x7f23b4d90aa4 - _<ext..expand..MacroExpander<'a, 'b> as fold..Folder>::fold_item::hdce1cac1401f2259
  39:     0x7f23b4dd63e2 - _<ext..expand..MacroExpander<'a, 'b> as fold..Folder>::fold_crate::he184cdd28c91132e
  40:     0x7f23b4dd775a - syntax::ext::expand::expand_crate::h9521fe412e412c78
  41:     0x7f23bca3464b - rustc_driver::driver::phase_2_configure_and_expand::_$u7b$$u7b$closure$u7d$$u7d$::ha17e10ff1fe4303c
  42:     0x7f23bc9df0f1 - rustc_driver::driver::phase_2_configure_and_expand::hd8abf650d2d5e04d
  43:     0x7f23bc9c2ffa - rustc_driver::driver::compile_input::h650fe5b01cb8d74d
  44:     0x7f23bc9ab464 - rustc_driver::run_compiler::h68d23e0e9b7b247d
  45:     0x7f23bc9a88c1 - std::sys_common::unwind::try::try_fn::h67fde221a73148bc
  46:     0x7f23bc4a3e0b - __rust_try
  47:     0x7f23bc4a3d9d - std::sys_common::unwind::inner_try::h4e97625a08807651
  48:     0x7f23bc9a910a - _<F as std..boxed..FnBox<A>>::call_box::hc8936fa120642c49
  49:     0x7f23bc4b1f54 - std::sys::thread::Thread::new::thread_start::h74af400293164137
  50:     0x7f23b439a423 - start_thread
  51:     0x7f23bc110cbc - clone
  52:                0x0 - <unknown>

playpen: application terminated with error code 101
