
ERROR:rbml::reader: failed to find block with tag 7u
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /build/rust/src/rustc-1.0.0-alpha/src/librbml/lib.rs:253

stack backtrace:
   1:     0x7fe6bf424e60 - sys::backtrace::write::h78d6cca6cec5e106Czt
   2:     0x7fe6bf446aa0 - <unknown>
   3:     0x7fe6bf3b6cf0 - rt::unwind::begin_unwind_inner::hbe118f1b5f67c2242tz
   4:     0x7fe6b91bdf90 - <unknown>
   5:     0x7fe6b91bcef0 - reader::get_doc::h8a8c9232b9c7ad1bZKa
   6:     0x7fe6bda52c30 - metadata::decoder::get_type::h1caecc7ac49c6828msj
   7:     0x7fe6bda04b90 - metadata::csearch::get_type::hde6d946068b8b918Ijm
   8:     0x7fe6bec82dc0 - collect::CollectCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::hd156e70f6ebc4e40RCt
   9:     0x7fe6bec6de70 - <unknown>
  10:     0x7fe6bec6f790 - <unknown>
  11:     0x7fe6bec25480 - astconv::ast_ty_to_ty::ha17967805811915beIs
  12:     0x7fe6bec99d20 - <unknown>
  13:     0x7fe6bec95960 - <unknown>
  14:     0x7fe6bec8cce0 - <unknown>
  15:     0x7fe6bec86cf0 - <unknown>
  16:     0x7fe6becc76c0 - <unknown>
  17:     0x7fe6becc5570 - check_crate::h6ee7b1d7b661b45c23x
  18:     0x7fe6bf968480 - driver::phase_3_run_analysis_passes::h00df603eef1b723bgwa
  19:     0x7fe6bf9515d0 - driver::compile_input::hc42a62ce2a43792axba
  20:     0x7fe6bfa1f4a0 - <unknown>
  21:     0x7fe6bfa1f370 - <unknown>
  22:     0x7fe6bfa1e0f0 - <unknown>
  23:     0x7fe6bf4a30f0 - <unknown>
  24:     0x7fe6bf4a30e0 - rust_try
  25:     0x7fe6bfa1e3e0 - <unknown>
  26:     0x7fe6bf434e30 - <unknown>
  27:     0x7fe6b9d1d250 - start_thread
  28:     0x7fe6bf06c219 - clone
  29:                0x0 - <unknown>
