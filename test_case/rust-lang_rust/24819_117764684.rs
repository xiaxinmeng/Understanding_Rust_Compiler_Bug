
❯ RUST_BACKTRACE=1 cargo build                                                                                                                                                                                                                                         [32/4241]
   Compiling serde_codegen v0.4.2
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'index out of bounds: the len is 92 but the index is 529', ../src/libcollections/vec.rs:1362

stack backtrace:
   1:        0x10a1ae485 - sys::backtrace::write::ha69861d01c280e10Vns
   2:        0x10a1b6be3 - panicking::on_panic::he1db6e7d5ec9c9022Fw
   3:        0x10a172af2 - rt::unwind::begin_unwind_inner::h9f73b873795fcd2deow
   4:        0x10a173849 - rt::unwind::begin_unwind_fmt::h3999b4670fb43708knw
   5:        0x10a1b676c - rust_begin_unwind
   6:        0x10a20ae35 - panicking::panic_fmt::he89ea3094b3178e919B
   7:        0x10a2055b5 - panicking::panic_bounds_check::h8058af339f14343278B
   8:        0x10f375347 - util::interner::StrInterner::get::h33e23c9881d31a76Fpa
   9:        0x10ba9b224 - parse::token::get_name::hc8e6f23ae3b0497cwaT
  10:        0x10ba9b398 - parse::token::get_ident::h623c49ac14f4e12cNaT
  11:        0x10ba9b34c - str::ast..Ident.ToInternedString::to_interned_string::h226749af2201972apgd
  12:        0x10ba464b4 - lit::LitBuilder<F>::str::h2287811883313644798
  13:        0x10ba463f5 - expr::ExprBuilder<F>::str::h15103756226570193947
  14:        0x10ba46143 - mk_ident::h76e644649ecee0d6Yia
  15:        0x10ba4edb7 - expr_mk_token::h530e0aaf1ac4f9c2Cla
  16:        0x10ba53578 - statements_mk_tt::ha8ed870f505b201bBAa
  17:        0x10ba5bf8e - statements_mk_tt::closure.5782
  18:        0x10ba5bf1d - iter::FlatMap<I, U, F>.Iterator::next::closure.5777
  19:        0x10ba5be7d - option::Option<T>::map::h992756666428157757
  20:        0x10ba5bc0d - iter::FlatMap<I, U, F>.Iterator::next::h16461546956636534358
  21:        0x10ba5b9bd - iter::Chain<A, B>.Iterator::next::h15790673721037698405
  22:        0x10ba5b8a1 - iter::Chain<A, B>.Iterator::next::h13883406854348817918
  23:        0x10ba5cf34 - vec::Vec<T>::extend_desugared::h14308067113245803033
  24:        0x10ba5b561 - vec::Vec<T>.FromIterator<T>::from_iter::h4567733303014666736
  25:        0x10ba5b0db - iter::Iterator::collect::h15870988852878513640
  26:        0x10ba5449f - statements_mk_tt::ha8ed870f505b201bBAa
  27:        0x10ba4071d - statements_mk_tts::hf5a22e049c2a847cZJa
  28:        0x10ba228c2 - expand_tts::h4a76f356700220c2QKa
  29:        0x10ba3bacb - expand_parse_call::h847fdc0375aa33afHMa
  30:        0x10ba3cc23 - expand_quote_expr::hb1131b5a4ef58447Oba
  31:        0x10edf4bbe - ext::base::F.TTMacroExpander::expand::h16772666232880655435
  32:        0x107c39e8b - ext::expand::expand_expr::closure.64675
  33:        0x107c35510 - ext::expand::expand_expr::h99495f0a4050bb0eO3a
  34:        0x107c8e7de - ext::expand::expand_block_elts::closure.65395
  35:        0x107c8cb3e - ext::expand::expand_block_elts::closure.65376
  36:        0x107c48b8a - ext::expand::expand_block_elts::hae5d90e0cb6f3badrPb
  37:        0x107c8c552 - ext::expand::expand_block::h19a07d5850d44ec3MOb
  38:        0x107c3eb93 - fold::noop_fold_expr::h13944231768787436517
  39:        0x107c35712 - ext::expand::expand_expr::closure.64675
  40:        0x107c35510 - ext::expand::expand_expr::h99495f0a4050bb0eO3a
  41:        0x107c88130 - ext::expand::expand_arm::h362881c67b77a2bf5Jb
  42:        0x107c3e7d7 - fold::noop_fold_expr::h13944231768787436517
  43:        0x107c35712 - ext::expand::expand_expr::closure.64675
  44:        0x107c35510 - ext::expand::expand_expr::h99495f0a4050bb0eO3a
  45:        0x107c8e7de - ext::expand::expand_block_elts::closure.65395
  46:        0x107c8cb3e - ext::expand::expand_block_elts::closure.65376
  47:        0x107c48b8a - ext::expand::expand_block_elts::hae5d90e0cb6f3badrPb
  48:        0x107c8c552 - ext::expand::expand_block::h19a07d5850d44ec3MOb
  49:        0x107c4855c - ext::expand::expand_and_rename_fn_decl_and_block::h2f6d7bbfc095110aQic
  50:        0x107cb4677 - ext::expand::expand_and_rename_method::h146aa2e4f3025329akc
  51:        0x107cb963d - ext::expand::expand_impl_item::closure.65794
  52:        0x107cb5f05 - ext::expand::expand_impl_item::h624cadcd858560f3zfc
  53:        0x107c4c2ea - ext::expand::expand_annotatable::hd1384fdfcaa7365bNZb
  54:        0x107c5b91f - ext::expand::MacroExpander<'a, 'b>.Folder::fold_impl_item::h3c6c8bcc8f2aea45Gnc
  55:        0x107c5b39c - iter::FlatMap<I, U, F>.Iterator::next::h2176610863638934878
  56:        0x107c5b6c6 - vec::Vec<T>::extend_desugared::h3745718402942750841
  57:        0x107c5a3d7 - iter::Iterator::collect::h15651229304304986698
  58:        0x107c50e93 - ext::expand::expand_item_underscore::hc68401b64125c559Vsb
  59:        0x107cb3744 - fold::Folder::fold_item_simple::h6115586095879100014
  60:        0x107cb3313 - ptr::P<T>::map::h8648109792104747851
  61:        0x107c4cfcd - ext::expand::expand_annotatable::hd1384fdfcaa7365bNZb
  62:        0x107c48d11 - ext::expand::expand_item::h544d5c924b5556adksb
  63:        0x107c55c0a - iter::FlatMap<I, U, F>.Iterator::next::h10402144123647243192
  64:        0x107c54b17 - vec::Vec<T>.FromIterator<T>::from_iter::h2531848258214199605
  65:        0x107c542df - fold::noop_fold_mod::h11533148739362704925
  66:        0x107c5028e - ext::expand::expand_item_underscore::hc68401b64125c559Vsb
  67:        0x107cb3744 - fold::Folder::fold_item_simple::h6115586095879100014
  68:        0x107cb3313 - ptr::P<T>::map::h8648109792104747851
  69:        0x107c4da4d - ext::expand::expand_annotatable::hd1384fdfcaa7365bNZb
  70:        0x107c48d11 - ext::expand::expand_item::h544d5c924b5556adksb
  71:        0x107c612e4 - iter::FlatMap<I, U, F>.Iterator::next::h4604195285113091658
  72:        0x107c60357 - util::small_vector::SmallVector<T>.FromIterator<T>::from_iter::h15534074357520757775
  73:        0x107c5f1fb - ext::expand::expand_item_mac::h42d76a4eac0c1d4bmvb
  74:        0x107c4c9d7 - ext::expand::expand_annotatable::hd1384fdfcaa7365bNZb
  75:        0x107c48d11 - ext::expand::expand_item::h544d5c924b5556adksb
  76:        0x107c55c0a - iter::FlatMap<I, U, F>.Iterator::next::h10402144123647243192
  77:        0x107c54b17 - vec::Vec<T>.FromIterator<T>::from_iter::h2531848258214199605
  78:        0x107c542df - fold::noop_fold_mod::h11533148739362704925
  79:        0x107c5028e - ext::expand::expand_item_underscore::hc68401b64125c559Vsb
  80:        0x107cb3744 - fold::Folder::fold_item_simple::h6115586095879100014
  81:        0x107cb3313 - ptr::P<T>::map::h8648109792104747851
  82:        0x107c4da4d - ext::expand::expand_annotatable::hd1384fdfcaa7365bNZb
  83:        0x107c48d11 - ext::expand::expand_item::h544d5c924b5556adksb
  84:        0x107cbc9ef - ext::expand::expand_crate::he8ff7220990af98elrc
  85:        0x106693aa2 - driver::phase_2_configure_and_expand::closure.21445
  86:        0x106648bd2 - driver::phase_2_configure_and_expand::h154d7c2f9d5ff029ita
  87:        0x10663844d - driver::compile_input::hb76410fc800a2a41Tba
  88:        0x1067123af - run_compiler::hd724b5fef3441688y7b
  89:        0x10670fc63 - boxed::F.FnBox<A>::call_box::h16633087113713113418
  90:        0x10670f427 - rt::unwind::try::try_fn::h14742554275296927735
  91:        0x10a2430f8 - rust_try_inner
  92:        0x10a2430e5 - rust_try
  93:        0x10a1a09a5 - rt::unwind::try::inner_try::h2041d75ee62fcacd7jw
  94:        0x10670f658 - boxed::F.FnBox<A>::call_box::h11995599304104652074
  95:        0x10a1b566d - sys::thread::Thread::new::thread_start::hb06fec4578e7b89frIv
  96:     0x7fff95ccd2fb - _pthread_body
  97:     0x7fff95ccd278 - _pthread_start
