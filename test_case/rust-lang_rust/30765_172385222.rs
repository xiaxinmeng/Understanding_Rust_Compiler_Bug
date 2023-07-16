
no-location error: `let` is not an expression, so it cannot be used in this way
thread '<main>' panicked at 'position 0 does not resolve to a source location', src/libsyntax/codemap.rs:1049
error: internal compiler error: Error constructed but not emitted
thread '<main>' panicked at 'explicit panic', src/libsyntax/errors/mod.rs:261
stack backtrace:
   1:     0x7f2cdd6439d0 - sys::backtrace::tracing::imp::write::h01407790575153d2Afu
   2:     0x7f2cdd645d95 - panicking::default_handler::_<closure>::closure.42892
   3:     0x7f2cdd64583f - panicking::default_handler::h55b20e8b59558085Eqy
   4:     0x7f2cdd639c96 - sys_common::unwind::begin_unwind_inner::h3c652ec223587f3d97s
   5:     0x7f2cdd41b67f - sys_common::unwind::begin_unwind::h16145324357146396763
   6:     0x7f2cdd41d953 - errors..DiagnosticBuilder::drop.56135::hbe7c46c12efc554c
   7:     0x7f2cdd60eed3 - ext::quote::parse_expr_panic::h40b20d48d67152e8jWb
   8:     0x7f2cdd3fc9d7 - main::h01aa5d7192b94c41laa
   9:     0x7f2cdd6454c4 - sys_common::unwind::try::try_fn::h1519701836809394149
  10:     0x7f2cdd642e78 - __rust_try
  11:     0x7f2cdd645166 - rt::lang_start::h9cf98e5c72fa4d24Siy
  12:     0x7f2cdd41af19 - main
  13:     0x7f2cdc805b44 - __libc_start_main
  14:     0x7f2cdd3fb9e0 - <unknown>
  15:                0x0 - <unknown>
thread panicked while panicking. aborting.Illegal instruction
