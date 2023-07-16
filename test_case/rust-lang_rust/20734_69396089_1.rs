
thread 'rustc' panicked at 'assertion failed: f.abi == Rust || f.abi == RustCall', C:\bot\slave\nightly-dist-rustc-win-64\build\src\librustc_trans\trans\base.rs:2438

stack backtrace:
   1:         0x69bec997 - sys::backtrace::write::h62d87c63e18a3ea5JRt
   2:         0x69c00672 - rt::unwind::register::haa3a38fa07bfa9beVGz
   3:         0x69b834a7 - rt::unwind::begin_unwind_inner::hbebf9f8c0125ffc9tEz
   4:          0x1473bac - trans::context::CrateContext<'b, 'tcx>::link_meta::h9c4774348aced35ahym
   5:          0x158fe67 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h2dd10ee030f9c94e6DK
   6:          0x1591bce - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h2dd10ee030f9c94e6DK
   7:          0x14ac310 - trans::context::CrateContext<'b, 'tcx>::sess::h6fdd1e7dea5490cdItm
   8:          0x14aa9a2 - trans::context::CrateContext<'b, 'tcx>::stats::hd7a1dc7a2465d605EGm
   9:          0x1593768 - trans::base::trans_crate::h4bc502f506e6c118Wsv
  10:         0x70b2af6a - driver::phase_4_translate_to_llvm::haa9c5e826483e242PFa
  11:         0x70b03623 - driver::compile_input::hf5b2f58693da03b5xba
  12:         0x70bd1eec - run::ha65e56249318cc0dV3b
  13:         0x70bd04cc - run::ha65e56249318cc0dV3b
  14:         0x70bcf16a - run::ha65e56249318cc0dV3b
  15:         0x69c2881c - rust_try
  16:         0x69c287f9 - rust_try
  17:         0x70bcf854 - run::ha65e56249318cc0dV3b
  18:         0x69bf14a7 - sys::tcp::TcpListener::bind::hc4a8da4adad0f541bsw
  19:     0x7ffd5c43a6e2 - BaseThreadInitThunk
