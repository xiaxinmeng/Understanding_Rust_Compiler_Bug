
thread 'rustc' panicked at 'assertion failed: lo <= hi', /home/mw/rust/src/libsyntax/codemap.rs:181

stack backtrace:
   1:     0x2b19ace8a8a1 - sys::backtrace::write::h3e9b4dca0513864dKrD
   2:     0x2b19acea7323 - panicking::on_panic::hcb89533c10dead95GJJ
   3:     0x2b19ace0602b - rt::unwind::begin_unwind_inner::ha5515b368a7d162eeqJ
   4:     0x2b19af553ccc - rt::unwind::begin_unwind::h7177973865999339449
   5:     0x2b19af623721 - parse::parser::Parser<'a>::mk_expr::h7e6012b214477e73kyI
   6:     0x2b19af62dd58 - parse::parser::Parser<'a>::parse_more_binops::hfd08f1c10b16b5abgoJ
   7:     0x2b19af62dff9 - parse::parser::Parser<'a>::parse_assign_expr::ha90692af103ba88bGsJ
   8:     0x2b19af626b18 - parse::parser::Parser<'a>::parse_bottom_expr::haeda66665b87a4b9qDI
   9:     0x2b19af62d59b - parse::parser::Parser<'a>::parse_prefix_expr::ha7f81e84568c81e9ZiJ
  10:     0x2b19af62dfec - parse::parser::Parser<'a>::parse_assign_expr::ha90692af103ba88bGsJ
  11:     0x2b19af63faf2 - parse::parser::Parser<'a>::parse_item_const::h73b6ec07b5cd7463PmL
  12:     0x2b19af631c07 - parse::parser::Parser<'a>::parse_item_::hb119daa3df77ffaeJRL
  13:     0x2b19af7b9e4e - ext::tt::macro_rules::ParserAnyMacro<'a>.MacResult::make_items::h21f35fae734f799fsyf
  14:     0x2b19af73fe93 - ext::expand::expand_item_mac::hc8036110974a2e865Ra
  15:     0x2b19af734af5 - ext::expand::expand_annotatable::hd63e48fa5a49f5f9Nlb
  16:     0x2b19af731ad0 - ext::expand::expand_item::h3e710c5693bd09f40La
  17:     0x2b19af73aec4 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::h5a856dd7f64966f3KDb
  18:     0x2b19af73ab70 - iter::FlatMap<I, U, F>.Iterator::next::h8620248382209828813
  19:     0x2b19af73a8ab - vec::Vec<T>.FromIterator<T>::from_iter::h17981381026392602979
  20:     0x2b19af73a548 - fold::noop_fold_mod::h13311583651119820058
  21:     0x2b19af736f47 - ext::expand::expand_item_underscore::hdbcfb7421bd9ad50GPa
  22:     0x2b19af777757 - fold::noop_fold_item_simple::h12913474026910056601
  23:     0x2b19af77744e - ptr::P<T>::map::h6853041154571815780
  24:     0x2b19af735491 - ext::expand::expand_annotatable::hd63e48fa5a49f5f9Nlb
  25:     0x2b19af731ad0 - ext::expand::expand_item::h3e710c5693bd09f40La
  26:     0x2b19af73aec4 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::h5a856dd7f64966f3KDb
  27:     0x2b19af73ab70 - iter::FlatMap<I, U, F>.Iterator::next::h8620248382209828813
  28:     0x2b19af73a8e2 - vec::Vec<T>.FromIterator<T>::from_iter::h17981381026392602979
  29:     0x2b19af73a548 - fold::noop_fold_mod::h13311583651119820058
  30:     0x2b19af736f47 - ext::expand::expand_item_underscore::hdbcfb7421bd9ad50GPa
  31:     0x2b19af777757 - fold::noop_fold_item_simple::h12913474026910056601
  32:     0x2b19af77744e - ptr::P<T>::map::h6853041154571815780
  33:     0x2b19af735491 - ext::expand::expand_annotatable::hd63e48fa5a49f5f9Nlb
  34:     0x2b19af731ad0 - ext::expand::expand_item::h3e710c5693bd09f40La
  35:     0x2b19af73aec4 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::h5a856dd7f64966f3KDb
  36:     0x2b19af77f2f3 - ext::expand::expand_crate::h6999d25ae521adf3bKb
  37:     0x2b19acabc113 - driver::phase_2_configure_and_expand::closure.18814
  38:     0x2b19aca7817c - driver::phase_2_configure_and_expand::h69f8943526788512Xsa
  39:     0x2b19aca69f28 - driver::compile_input::hbd3be055171b4244Rba
  40:     0x2b19acb0a049 - run_compiler::ha6b2ab1236d9b5f6S6b
  41:     0x2b19acb0825d - thunk::F.Invoke<A, R>::invoke::h12269788191574515370
  42:     0x2b19acb07835 - rt::unwind::try::try_fn::h17916825617625713907
  43:     0x2b19acf16c78 - rust_try_inner
  44:     0x2b19acf16c65 - rust_try
  45:     0x2b19acb07c43 - thunk::F.Invoke<A, R>::invoke::h724682906673806154
  46:     0x2b19ace9af89 - sys::thread::create::thread_start::h9e0cd7532d186bdd3iI
  47:     0x2b19b2c94181 - start_thread
  48:     0x2b19ad55f47c - __clone
  49:                0x0 - <unknown>
   1:     0x2b19ace8a8a1 - sys::backtrace::write::h3e9b4dca0513864dKrD
   2:     0x2b19acea7323 - panicking::on_panic::hcb89533c10dead95GJJ
   3:     0x2b19ace0602b - rt::unwind::begin_unwind_inner::ha5515b368a7d162eeqJ
   4:     0x2b19af553ccc - rt::unwind::begin_unwind::h7177973865999339449
   5:     0x2b19af623721 - parse::parser::Parser<'a>::mk_expr::h7e6012b214477e73kyI
   6:     0x2b19af62dd58 - parse::parser::Parser<'a>::parse_more_binops::hfd08f1c10b16b5abgoJ
   7:     0x2b19af62dff9 - parse::parser::Parser<'a>::parse_assign_expr::ha90692af103ba88bGsJ
   8:     0x2b19af626b18 - parse::parser::Parser<'a>::parse_bottom_expr::haeda66665b87a4b9qDI
   9:     0x2b19af62d59b - parse::parser::Parser<'a>::parse_prefix_expr::ha7f81e84568c81e9ZiJ
  10:     0x2b19af62dfec - parse::parser::Parser<'a>::parse_assign_expr::ha90692af103ba88bGsJ
  11:     0x2b19af63faf2 - parse::parser::Parser<'a>::parse_item_const::h73b6ec07b5cd7463PmL
  12:     0x2b19af631c07 - parse::parser::Parser<'a>::parse_item_::hb119daa3df77ffaeJRL
  13:     0x2b19af7b9e4e - ext::tt::macro_rules::ParserAnyMacro<'a>.MacResult::make_items::h21f35fae734f799fsyf
  14:     0x2b19af73fe93 - ext::expand::expand_item_mac::hc8036110974a2e865Ra
  15:     0x2b19af734af5 - ext::expand::expand_annotatable::hd63e48fa5a49f5f9Nlb
  16:     0x2b19af731ad0 - ext::expand::expand_item::h3e710c5693bd09f40La
  17:     0x2b19af73aec4 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::h5a856dd7f64966f3KDb
  18:     0x2b19af73ab70 - iter::FlatMap<I, U, F>.Iterator::next::h8620248382209828813
  19:     0x2b19af73a8ab - vec::Vec<T>.FromIterator<T>::from_iter::h17981381026392602979
  20:     0x2b19af73a548 - fold::noop_fold_mod::h13311583651119820058
  21:     0x2b19af736f47 - ext::expand::expand_item_underscore::hdbcfb7421bd9ad50GPa
  22:     0x2b19af777757 - fold::noop_fold_item_simple::h12913474026910056601
  23:     0x2b19af77744e - ptr::P<T>::map::h6853041154571815780
  24:     0x2b19af735491 - ext::expand::expand_annotatable::hd63e48fa5a49f5f9Nlb
  25:     0x2b19af731ad0 - ext::expand::expand_item::h3e710c5693bd09f40La
  26:     0x2b19af73aec4 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::h5a856dd7f64966f3KDb
  27:     0x2b19af73ab70 - iter::FlatMap<I, U, F>.Iterator::next::h8620248382209828813
  28:     0x2b19af73a8e2 - vec::Vec<T>.FromIterator<T>::from_iter::h17981381026392602979
  29:     0x2b19af73a548 - fold::noop_fold_mod::h13311583651119820058
  30:     0x2b19af736f47 - ext::expand::expand_item_underscore::hdbcfb7421bd9ad50GPa
  31:     0x2b19af777757 - fold::noop_fold_item_simple::h12913474026910056601
  32:     0x2b19af77744e - ptr::P<T>::map::h6853041154571815780
  33:     0x2b19af735491 - ext::expand::expand_annotatable::hd63e48fa5a49f5f9Nlb
  34:     0x2b19af731ad0 - ext::expand::expand_item::h3e710c5693bd09f40La
  35:     0x2b19af73aec4 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::h5a856dd7f64966f3KDb
  36:     0x2b19af77f2f3 - ext::expand::expand_crate::h6999d25ae521adf3bKb
  37:     0x2b19acabc113 - driver::phase_2_configure_and_expand::closure.18814
  38:     0x2b19aca7817c - driver::phase_2_configure_and_expand::h69f8943526788512Xsa
  39:     0x2b19aca69f28 - driver::compile_input::hbd3be055171b4244Rba
  40:     0x2b19acb0a049 - run_compiler::ha6b2ab1236d9b5f6S6b
  41:     0x2b19acb0825d - thunk::F.Invoke<A, R>::invoke::h12269788191574515370
  42:     0x2b19acb07835 - rt::unwind::try::try_fn::h17916825617625713907
  43:     0x2b19acf16c78 - rust_try_inner
  44:     0x2b19acf16c65 - rust_try
  45:     0x2b19acb07c43 - thunk::F.Invoke<A, R>::invoke::h724682906673806154
  46:     0x2b19ace9af89 - sys::thread::create::thread_start::h9e0cd7532d186bdd3iI
  47:     0x2b19b2c94181 - start_thread
  48:     0x2b19ad55f47c - __clone
  49:                0x0 - <unknown>
