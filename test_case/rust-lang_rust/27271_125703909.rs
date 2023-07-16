
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'index out of bounds: the len is 60 but the index is 65', ../src/libcollections/vec.rs:1246

stack backtrace:
   1:        0x11070c5a5 - sys::backtrace::write::h8d57f899e3fda16elus
   2:        0x11071560e - panicking::on_panic::h5ba36379bfe7cb23XUw
   3:        0x1106cf062 - rt::unwind::begin_unwind_inner::hfdefe03d89c9e15c8Cw
   4:        0x1106cff0c - rt::unwind::begin_unwind_fmt::h4abe28056f41fed5eCw
   5:        0x11071518c - rust_begin_unwind
   6:        0x11076b2f5 - panicking::panic_fmt::h13125e855ad0c512EhC
   7:        0x110765a35 - panicking::panic_bounds_check::h930022a157a2365dKgC
   8:        0x1157d65f7 - util::interner::StrInterner::get::hd168b7b16d7f039aFpa
   9:        0x110be2aa4 - parse::token::get_name::hd9e2929ac0706451apT
  10:        0x110be29d8 - parse::token::get_ident::h8eabd2f158c72c5frpT
  11:        0x110bdeeb3 - expand_rn::h7069071bf5512a5esaa
  12:        0x11521066e - ext::base::F.TTMacroExpander::expand::h13800486614208293857
  13:        0x10e115fcd - ext::expand::expand_expr::closure.66042
  14:        0x10e1103da - ext::expand::expand_expr::h10d1ec6bb3acc3e4rlb
  15:        0x10e0bf88b - ext::base::get_exprs_from_tts::h3230de70dec5bb6fTC5
  16:        0x10e0b848c - ext::concat::expand_syntax_ext::h6d30518a25606a73bi7
  17:        0x10e0bcc0e - ext::base::F.TTMacroExpander::expand::h17916794981785300046
  18:        0x10e115fcd - ext::expand::expand_expr::closure.66042
  19:        0x10e1103da - ext::expand::expand_expr::h10d1ec6bb3acc3e4rlb
  20:        0x10e1b41d8 - ext::format::expand_preparsed_format_args::h39749fc855abdc45vGd
  21:        0x10e0b5f57 - ext::format::expand_format_args::h2034aa489443eeeaIFd
  22:        0x10e0b640e - ext::base::F.TTMacroExpander::expand::h1414252957907639972
  23:        0x10e115fcd - ext::expand::expand_expr::closure.66042
  24:        0x10e1103da - ext::expand::expand_expr::h10d1ec6bb3acc3e4rlb
  25:        0x10e11abcf - fold::noop_fold_expr::h8234447633789036141
  26:        0x10e1105f3 - ext::expand::expand_expr::closure.66042
  27:        0x10e1103da - ext::expand::expand_expr::h10d1ec6bb3acc3e4rlb
  28:        0x10e158fc1 - fold::noop_fold_stmt::h6333715652680442577
  29:        0x10e139d30 - ext::expand::expand_stmt::h8c4bd9703af6a2f7b4b
  30:        0x10e13cc8c - iter::FlatMap<I, U, F>.Iterator::next::h10170223740143956055
  31:        0x10e13c647 - util::small_vector::SmallVector<T>.FromIterator<T>::from_iter::h3322216669002410848
  32:        0x10e13b307 - ext::expand::expand_stmt::h8c4bd9703af6a2f7b4b
  33:        0x10e13cc8c - iter::FlatMap<I, U, F>.Iterator::next::h10170223740143956055
  34:        0x10e13c647 - util::small_vector::SmallVector<T>.FromIterator<T>::from_iter::h3322216669002410848
  35:        0x10e13b307 - ext::expand::expand_stmt::h8c4bd9703af6a2f7b4b
  36:        0x10e15ffe3 - ext::expand::expand_block_elts::closure.66799
  37:        0x10e15fd15 - iter::FlatMap<I, U, F>.Iterator::next::h12880663052903539887
  38:        0x10e15f3d4 - vec::Vec<T>.FromIterator<T>::from_iter::h11897925576472944830
  39:        0x10e15efc7 - ext::expand::expand_block_elts::closure.66792
  40:        0x10e124529 - ext::expand::expand_block_elts::h57e4fa0ef3fb82095ec
  41:        0x10e15ebf2 - ext::expand::expand_block::hebf31e0d54524c16qec
  42:        0x10e124053 - ext::expand::expand_and_rename_fn_decl_and_block::h9735bb96b0f3ef77uIc
  43:        0x10e12af97 - ext::expand::expand_item_underscore::hf55f46424c3923c8BSb
  44:        0x10e1806e7 - fold::noop_fold_item_simple::h13360826160893592273
  45:        0x10e180246 - fold::noop_fold_item::h6178621647060960707
  46:        0x10e128d77 - ext::expand::expand_annotatable::h196830a9bb0064b9rpc
  47:        0x10e1246b9 - ext::expand::expand_item::h20b8951f3ce4e5750Rb
  48:        0x10e130a95 - iter::FlatMap<I, U, F>.Iterator::next::h16067873143848592029
  49:        0x10e13056c - vec::Vec<T>.FromIterator<T>::from_iter::h5969725813415914631
  50:        0x10e130267 - fold::noop_fold_mod::h15221366836033915591
  51:        0x10e12b8d6 - ext::expand::expand_item_underscore::hf55f46424c3923c8BSb
  52:        0x10e1806e7 - fold::noop_fold_item_simple::h13360826160893592273
  53:        0x10e180246 - fold::noop_fold_item::h6178621647060960707
  54:        0x10e1299ad - ext::expand::expand_annotatable::h196830a9bb0064b9rpc
  55:        0x10e1246b9 - ext::expand::expand_item::h20b8951f3ce4e5750Rb
  56:        0x10e18837c - ext::expand::expand_crate::h3f3f15780cfbd68bhRc
  57:        0x10cbb3a70 - driver::phase_2_configure_and_expand::closure.22339
  58:        0x10cb68430 - driver::phase_2_configure_and_expand::h39dfa417ce3247ddita
  59:        0x10cb58978 - driver::compile_input::h87a640b6a6c7679fTba
  60:        0x10cc3bb4b - run_compiler::h1988d61cd748f5f6A7b
  61:        0x10cc393c0 - boxed::F.FnBox<A>::call_box::h8812244964073324717
  62:        0x10cc38c37 - rt::unwind::try::try_fn::h15965772982377226605
  63:        0x1107150bf - __rust_try_inner
  64:        0x1107150fa - __rust_try
  65:        0x1106feb45 - rt::unwind::try::inner_try::h211b654f88b1ec3f1yw
  66:        0x10cc38e79 - boxed::F.FnBox<A>::call_box::h6462970046057749915
  67:        0x11071401d - sys::thread::Thread::new::thread_start::h9dddbcf4515946ba1Wv
  68:     0x7fff904dc267 - _pthread_body
  69:     0x7fff904dc1e4 - _pthread_start
