
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /home/munksgaard/src/rust/src/librbml/lib.rs:265

stack backtrace:
   1:     0x7f39e1806870 - sys::backtrace::write::hc08edaab92ea5f11fmB
   2:     0x7f39e182f460 - panicking::on_panic::h0aad656adb1a5494TOK
   3:     0x7f39e1775c50 - rt::unwind::begin_unwind_inner::h2d8760d6e540a908etK
   4:     0x7f39dab90590 - rt::unwind::begin_unwind::h10945465982621760511
   5:     0x7f39dab8f2c0 - reader::get_doc::h3b375501de21dc11nLa
   6:     0x7f39dfa596a0 - metadata::decoder::item_type::h0bcc5bc8041b3f42ggk
   7:     0x7f39dfa69ce0 - metadata::decoder::get_type::hc5d4bd2cb40817bbqtk
   8:     0x7f39df8bb890 - middle::ty::lookup_item_type::h96595a9a15791cf9lx9
   9:     0x7f39e0ffd050 - collect::CollectCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::h8fd7705c7c9341cflEw
  10:     0x7f39e0feb990 - astconv::ast_path_to_ty::hc64524103586bf24pfv
  11:     0x7f39e0fecf40 - astconv::ast_ty_to_ty::closure.34235
  12:     0x7f39e0f89b60 - astconv::ast_ty_to_ty::hd6bbaa747e382891tGv
  13:     0x7f39e0fe4f50 - vec::Vec<T>.FromIterator<T>::from_iter::h12524501346137637334
  14:     0x7f39e0fe2ca0 - astconv::convert_angle_bracketed_parameters::ha7ddd9e4ce685f360Nu
  15:     0x7f39e0f89610 - astconv::ast_path_substs_for_ty::h5130e30fa04adbe9SCu
  16:     0x7f39e0feb990 - astconv::ast_path_to_ty::hc64524103586bf24pfv
  17:     0x7f39e0fecf40 - astconv::ast_ty_to_ty::closure.34235
  18:     0x7f39e0f89b60 - astconv::ast_ty_to_ty::hd6bbaa747e382891tGv
  19:     0x7f39e0fe5d90 - astconv::convert_ty_with_lifetime_elision::he66f2874a05161cbrSu
  20:     0x7f39e0ff0fb0 - astconv::ty_of_method_or_bare_fn::h9de3b09ae5e6e9abj0v
  21:     0x7f39e0ff0e50 - astconv::ty_of_method::h3c68098a52896fbeFYv
  22:     0x7f39e101a420 - collect::convert_methods::ty_of_method::h5fbe8b18816ba236Y4w
  23:     0x7f39e10021c0 - collect::convert_item::hb0c36e790686e76e18w
  24:     0x7f39e10455c0 - check_crate::closure.35307
  25:     0x7f39e1043550 - check_crate::he30443d64908a736MxB
  26:     0x7f39e1e4cd30 - driver::phase_3_run_analysis_passes::h80f0f912fb405b32SGa
  27:     0x7f39e1e32e20 - driver::compile_input::hc45b6ab8a8a517daEba
  28:     0x7f39e1f06ed0 - run_compiler::hfa1a2ebef5d2110b5bc
  29:     0x7f39e1f05530 - thunk::F.Invoke<A, R>::invoke::h630436085354906681
  30:     0x7f39e1f04420 - rt::unwind::try::try_fn::h7425008353638106352
  31:     0x7f39e18a5f30 - rust_try_inner
  32:     0x7f39e18a5f20 - rust_try
  33:     0x7f39e1f046d0 - thunk::F.Invoke<A, R>::invoke::h15407183835454361425
  34:     0x7f39e181afa0 - sys::thread::thread_start::hdf1d4d327e894353YVF
  35:     0x7f39db8f6140 - <unknown>
  36:     0x7f39e1405bb9 - __clone
  37:                0x0 - <unknown>
