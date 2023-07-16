
stack backtrace:
   1:     0x7f4dcc8166e4 - sys::backtrace::write::hb2ecc26860cf26a5N0C
   2:     0x7f4dcc83df90 - panicking::on_panic::h960f9523a4558f10fhJ
   3:     0x7f4dcc772f4a - rt::unwind::begin_unwind_inner::h6b71715c6cd07544OXI
   4:     0x7f4dc9c357bd - rt::unwind::begin_unwind::h10330668448694945308
   5:     0x7f4dc9c35763 - diagnostic::SpanHandler::span_bug::ha94d4327955c6b76FaB
   6:     0x7f4dca4c1053 - session::Session::span_bug::ha063ab5cf89afb7arTn
   7:     0x7f4dcbfa18d5 - trans::monomorphize::normalize_associated_type::h15578180277037779921
   8:     0x7f4dcbf9cec5 - trans::meth::emit_vtable_methods::closure.46262
   9:     0x7f4dcbf9c230 - vec::Vec<T>.FromIterator<T>::from_iter::h14757000865658806530
  10:     0x7f4dcbf9a8c1 - trans::meth::emit_vtable_methods::h869b133958115c379By
  11:     0x7f4dcbf9ba5a - iter::FlatMap<I, U, F>.Iterator::next::closure.46232
  12:     0x7f4dcbf9b6e8 - iter::FlatMap<I, U, F>.Iterator::next::h18664991822273943
  13:     0x7f4dcbf9afcc - vec::Vec<T>.FromIterator<T>::from_iter::h1954666775356190594
  14:     0x7f4dcbedc3eb - trans::meth::get_vtable::hcdbf30f6716b5c22Svy
  15:     0x7f4dcbed7f12 - trans::expr::unsized_info::h1026628279121771536
  16:     0x7f4dcbee34c3 - trans::expr::apply_adjustments::apply_autoref::hfb6d70e6aeef5555Txh
  17:     0x7f4dcbe9076d - trans::expr::trans::hbd4f36981c222183ifh
  18:     0x7f4dcbe8e3e4 - trans::expr::trans_into::h2fbdf169c8afb1f608g
  19:     0x7f4dcbed769c - trans::expr::trans_rvalue_stmt_unadjusted::h0b4eefee561d46b3wgi
  20:     0x7f4dcbe8e9bd - trans::expr::trans_into::h2fbdf169c8afb1f608g
  21:     0x7f4dcbe8da34 - trans::controlflow::trans_stmt_semi::hf010868e8995b5e144d
  22:     0x7f4dcbe8f510 - trans::controlflow::trans_block::h1a3e1fe31adb4417R5d
  23:     0x7f4dcbf5739b - trans::base::trans_closure::h3785eaa409045c3cv7s
  24:     0x7f4dcbe793f8 - trans::base::trans_fn::hb17787e2baa85f80oit
  25:     0x7f4dcbe74f31 - trans::base::trans_item::h9d97a4e78d8fd135gGt
  26:     0x7f4dcbf5efcc - trans::base::trans_crate::h317d2891eb44f3c8KCu
  27:     0x7f4dccea22a3 - driver::phase_4_translate_to_llvm::hf105818bf70a091fmOa
  28:     0x7f4dcce7cee9 - driver::compile_input::ha7a5cd31f19269c3Rba
  29:     0x7f4dccf357b2 - run_compiler::hffd60a8ae6171fbbz2b
  30:     0x7f4dccf335ec - thunk::F.Invoke<A, R>::invoke::h17211452014221736608
  31:     0x7f4dccf327a9 - rt::unwind::try::try_fn::h10298546463733922815
  32:     0x7f4dcc8bc118 - rust_try_inner
  33:     0x7f4dcc8bc105 - rust_try
  34:     0x7f4dccf32b29 - thunk::F.Invoke<A, R>::invoke::h16798335321014197657
  35:     0x7f4dcc82b0d8 - sys::thread::create::thread_start::h870bf041dfc81c1bDQH
  36:     0x7f4dc67ba373 - start_thread
  37:     0x7f4dcc3e927c - __clone
  38:                0x0 - <unknown>
