
stack backtrace:
   1:         0x712b8995 - sys::backtrace::write::hfd38368ec8028bb1KRC
   2:         0x712f1489 - rt::unwind::register::h08a584336097b7c4suI
   3:         0x711870e7 - rt::unwind::begin_unwind_inner::ha607f27fd721be6aCrI
   4:           0x7e3e5c - diagnostic::SpanHandler::span_bug::hbcfacf13792ec7b7RmB
   5:           0x7e49de - diagnostic::Handler::bug::h5e7022718f8d292atsB
   6:          0x181f960 - session::Session::bug::h00ee353635d6a6cbwRp
   7:          0x344f1b1
   8:          0x3494520 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_expr::h176b043f5e5d64f1u8b
   9:          0x349b6fe - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_local::h47e175418774f1e02dc
  10:          0x34949c6 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_expr::h176b043f5e5d64f1u8b
  11:          0x3496583 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_stmt::h26fb82aef4a14a88W7b
  12:          0x34964d2 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_stmt::h26fb82aef4a14a88W7b
  13:          0x349e9df - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_ty::h693089c487ea030aTec
  14:          0x3495ccf - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_block::hbe55732ed3d9c51flbc
  15:          0x3495a14 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_expr::h176b043f5e5d64f1u8b
  16:          0x361e212 - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_ty::h2305b4702c4e64e77Pn
  17:          0x361683f - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::h9881b7ab20a4696eLPn
  18:          0x361600c - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::h9881b7ab20a4696eLPn
  19:          0x361bf8e - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_ty::h2305b4702c4e64e77Pn
  20:          0x361beef - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_ty::h2305b4702c4e64e77Pn
  21:          0x361d83b - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_ty::h2305b4702c4e64e77Pn
  22:          0x361d46c - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_ty::h2305b4702c4e64e77Pn
  23:          0x382727d - check_crate::hbe12ce8ac4dc7ab7YmC
  24:          0x3826e5a - check_crate::hbe12ce8ac4dc7ab7YmC
  25:          0x3823c04 - check_crate::hbe12ce8ac4dc7ab7YmC
  26:         0x652eed9d - driver::phase_3_run_analysis_passes::hbe0b356835407069nGa
  27:         0x652c2634 - driver::compile_input::hfc619fda1a2632e0Qba
  28:         0x654de669 - run_compiler::h653034a0f1a715a3F4b
  29:         0x654db99b - monitor::Sink.Write::flush::h8d33e737534a5fe0U7c
  30:         0x654dafe4 - run::ha9cc11ce37570e6cl4b
  31:         0x654daedd - run::ha9cc11ce37570e6cl4b
  32:         0x654dac63 - run::ha9cc11ce37570e6cl4b
  33:         0x654da5ed - run::ha9cc11ce37570e6cl4b
  34:         0x654da578 - run::ha9cc11ce37570e6cl4b
  35:         0x654da521 - run::ha9cc11ce37570e6cl4b
  36:         0x713ac1dc - rust_try
  37:         0x713ac1b9 - rust_try
  38:         0x654d9b24 - run::ha9cc11ce37570e6cl4b
  39:         0x654d979d - run::ha9cc11ce37570e6cl4b
  40:         0x654dada9 - run::ha9cc11ce37570e6cl4b
  41:         0x71239693 - sys::thread::guard::current::he68668c1a0c584c2X4G
  42:         0x712b19ed - sys_common::stack::record_sp_limit::target_record_sp_limit::he14532463c6507db1HB
  43:         0x712d5ec1 - sys::tcp::TcpListener::bind::hcd7eb5130b8064cdhUG
  44:         0x712d5ea1 - sys::tcp::TcpListener::bind::hcd7eb5130b8064cdhUG
  45:     0x7ffd2f6b16ad - BaseThreadInitThunk
