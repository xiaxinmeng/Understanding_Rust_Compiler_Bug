
$ RUST_BACKTRACE=1 rustc test.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: begin <= end', ../src/libcore/str/mod.rs:1443
stack backtrace:
   1:     0x7fe2eefe2940 - sys::backtrace::tracing::imp::write::he27ef2eb40e30753Plt
   2:     0x7fe2eefe9215 - panicking::log_panic::_<closure>::closure.39293
   3:     0x7fe2eefe8c85 - panicking::log_panic::h20a17353a34447751lx
   4:     0x7fe2eefad783 - sys_common::unwind::begin_unwind_inner::hf876a8bedc1a7d89gds
   5:     0x7fe2eefae0e8 - sys_common::unwind::begin_unwind_fmt::h69a2566a09092307mcs
   6:     0x7fe2eefe0801 - rust_begin_unwind
   7:     0x7fe2ef033a7f - panicking::panic_fmt::h4197e07a6c9f36b2dMK
   8:     0x7fe2ef02dfb8 - panicking::panic::h14dc0798a39fe65fKKK
   9:     0x7fe2ef0517ae - str::slice_error_fail::hf0d9e006a082073ap1S
  10:     0x7fe2e8d5fa3c - parse::lexer::comments::strip_doc_comment_decoration::h3b4315fd8c4f4986g1M
  11:     0x7fe2e8d5e6d3 - ast::_<impl>::get_tt::h771db6c9153c8483CKl
  12:     0x7fe2e8e4727f - ext::tt::transcribe::tt_next_token::h7848dc2a9c0e2bad3Pf
  13:     0x7fe2e8e5266d - parse::lexer::Reader::real_token::h17769928304857692945
  14:     0x7fe2e8de186a - parse::parser::_<impl>::new::h88a582d507a0205dPVE
  15:     0x7fe2e8d64e71 - ext::tt::macro_parser::parse::h844c6cd67c1fd9deXdg
  16:     0x7fe2e8d60d34 - ast::_<impl>::parse::h15e83bbaef90bc6cZOl
  17:     0x7fe2e902d2a4 - ext::tt::macro_rules::_<impl>::expand::hca8752198b608bcbUKg
  18:     0x7fe2e8f94af8 - ext::expand::expand_stmt::hd9da59f648bb2e0513b
  19:     0x7fe2e8fb887e - ext::expand::expand_block_elts::_<closure>::_<closure>::closure.69195
  20:     0x7fe2e8fb861d - iter::_<impl>::next::next::h16734081353555038292
  21:     0x7fe2e8fb7cdf - vec::_<impl>::from_iter::from_iter::h14097794162217787790
  22:     0x7fe2e8fb78dc - ext::expand::expand_block_elts::_<closure>::closure.69190
  23:     0x7fe2e8f80fe3 - ext::expand::expand_block_elts::h8d61523b3ef4a0eaVec
  24:     0x7fe2e8fb7578 - ext::expand::expand_block::h65394bbf166baee8gec
  25:     0x7fe2e8f80981 - ext::expand::expand_and_rename_fn_decl_and_block::h0d444361d4f5f425XCc
  26:     0x7fe2e8f86399 - ext::expand::expand_item_underscore::hc4243a869f3a877bGSb
  27:     0x7fe2e8fd798e - fold::noop_fold_item_simple::h14969237034223901466
  28:     0x7fe2e8fd74f0 - fold::noop_fold_item::h13324498876968949241
  29:     0x7fe2e8f84650 - ext::expand::expand_annotatable::hcdfa58fbdd2db5e45oc
  30:     0x7fe2e8f81178 - ext::expand::expand_item::h33ee80d897456d7c5Rb
  31:     0x7fe2e8f8c291 - iter::_<impl>::next::next::h6890117498453544242
  32:     0x7fe2e8f8be1e - vec::_<impl>::from_iter::from_iter::h2649464589074919154
  33:     0x7fe2e8f8bb33 - fold::noop_fold_mod::h8533001142484765576
  34:     0x7fe2e8f86c79 - ext::expand::expand_item_underscore::hc4243a869f3a877bGSb
  35:     0x7fe2e8fd798e - fold::noop_fold_item_simple::h14969237034223901466
  36:     0x7fe2e8fd74f0 - fold::noop_fold_item::h13324498876968949241
  37:     0x7fe2e8f85709 - ext::expand::expand_annotatable::hcdfa58fbdd2db5e45oc
  38:     0x7fe2e8f81178 - ext::expand::expand_item::h33ee80d897456d7c5Rb
  39:     0x7fe2e8fe0c5f - ext::expand::expand_crate::h9eb355faa0bd8654MNc
  40:     0x7fe2ef4efc9c - driver::phase_2_configure_and_expand::_<closure>::closure.27388
  41:     0x7fe2ef482e0d - driver::phase_2_configure_and_expand::h4e3b53043a6baad0Aua
  42:     0x7fe2ef470b03 - driver::compile_input::h403e292111b5d3da3ba
  43:     0x7fe2ef5cb51b - run_compiler::hc8f00af21d0b9395wsc
  44:     0x7fe2ef5c8da6 - boxed::_<impl>::call_box::call_box::h991646137014911779
  45:     0x7fe2ef5c86b4 - sys_common::unwind::try::try_fn::try_fn::h3743441614401701459
  46:     0x7fe2eefe0668 - __rust_try
  47:     0x7fe2eefd4deb - sys_common::unwind::try::inner_try::hbe46f56ef602f07cO9r
  48:     0x7fe2ef5c884e - boxed::_<impl>::call_box::call_box::h1940982206274010037
  49:     0x7fe2eefe7ce3 - sys::thread::_<impl>::new::thread_start::h13325138c3b56c4cdEw
  50:     0x7fe2e86256a9 - start_thread
  51:     0x7fe2eec6ceec - clone
  52:                0x0 - <unknown>
