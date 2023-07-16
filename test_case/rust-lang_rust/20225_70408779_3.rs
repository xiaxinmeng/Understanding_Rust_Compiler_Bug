
thread 'rustc' panicked at 'assertion failed: !generic_bounds.has_escaping_regions()', C:\bot\slave\nightly-dist-rustc-win-32\build\src\librustc_typeck\check\mod.rs:2231

stack backtrace:
   1: 0x62f3ce2f - rt::unwind::register::he1c697e5991dcbf88Dz
   2: 0x62ec350c - rt::unwind::begin_unwind_inner::ha377d8b3de16ce78GBz
   3:  0x1175635
   4:  0x11c9b4c - check::LvaluePreference...std..fmt..Show::fmt::h05e7aff324b7d3e5wqm
   5:  0x11f4685 - check::FnCtxt<'a, 'tcx>.mc..Typer<'tcx>::is_method_call::h9e9bd83f4778ce0f1Uj
   6:  0x12d9abf - check_crate::h18c07136f917886b23x
   7:  0x12d4580 - check_crate::h18c07136f917886b23x
   8: 0x6b7ddc8b - driver::phase_3_run_analysis_passes::ha45f499ff7826752gwa
   9: 0x6b7c2941 - driver::compile_input::hdb1b2f5e3352478cxba
  10: 0x6b892829 - run::h9a36f6fac224e9f5V3b
  11: 0x6b890caf - run::h9a36f6fac224e9f5V3b
  12: 0x6b88f9af - run::h9a36f6fac224e9f5V3b
  13: 0x62f6879c - rust_try
  14: 0x62f2e597 - sys::tcp::TcpListener::bind::h84f1a79868ba93248qw
  15: 0x7634919f - BaseThreadInitThunk
  16: 0x775f0bbb - RtlInitializeExceptionChain

