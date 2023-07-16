
stack backtrace:
   1:     0x7f4b0f9d2c59 - sys::backtrace::write::h433d43302839b3d0VvD
   2:     0x7f4b0f9fd56c - panicking::on_panic::h85bb7c48a7bf6d691LJ
   3:     0x7f4b0f923803 - rt::unwind::begin_unwind_inner::h30ebc8aea74a1411IrJ
   4:     0x7f4b0cd1959d - rt::unwind::begin_unwind::h6513247106139423927
   5:     0x7f4b0cd19543 - diagnostic::SpanHandler::span_bug::h4646ed0c65aeed34GcB
   6:     0x7f4b0d74e778 - middle::traits::error_reporting::report_fulfillment_errors::hc2dd1decb6d68dbfksM
   7:     0x7f4b0e619d0c - check::vtable::select_all_fcx_obligations_or_error::h8069d3ac5867ba9b81b
   8:     0x7f4b0e6d0122 - check::check_bare_fn::hd874778758fc14aenln
   9:     0x7f4b0e6d5849 - check::check_method_body::h4be5d0afb72be9efdTn
  10:     0x7f4b0e6cb8b6 - check::check_item::h80165565e9f9866b8Dn
  11:     0x7f4b0e7a9126 - check_crate::closure.36042
  12:     0x7f4b0e7a34f3 - check_crate::hc8ab794f01a8befa5iC
  13:     0x7f4b1006136d - driver::phase_3_run_analysis_passes::hc9d84862133d3ad1oGa
  14:     0x7f4b10045295 - driver::compile_input::hc4e76294d74f8411Qba
  15:     0x7f4b100fcda5 - run_compiler::h5900aba33b437c68p2b
  16:     0x7f4b100fa85a - thunk::F.Invoke<A, R>::invoke::h9689564212340425420
  17:     0x7f4b100f9aa9 - rt::unwind::try::try_fn::h16505487216291884813
  18:     0x7f4b0fa77a78 - rust_try_inner
  19:     0x7f4b0fa77a65 - rust_try
  20:     0x7f4b100f9e07 - thunk::F.Invoke<A, R>::invoke::h14412175433843103066
  21:     0x7f4b0f9e8551 - sys::thread::create::thread_start::hf9ba913050908365ulI
  22:     0x7f4b097ff0a4 - start_thread
  23:     0x7f4b0f58acfc - __clone
  24:                0x0 - <unknown>
