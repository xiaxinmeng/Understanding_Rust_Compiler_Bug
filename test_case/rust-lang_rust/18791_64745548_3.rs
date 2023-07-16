
$ RUST_BACKTRACE=1 rustc -g non-ascii.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: bpos.to_uint() >= mbc.pos.to_uint() + mbc.bytes', /home/nodakai/src/rust-HEAD/src/libsyntax/codemap.rs:475

stack backtrace:
   1:     0x7ffe0c6fd270 - rt::backtrace::imp::write::ha585e5f8e2609f1519x
   2:     0x7ffe0c700340 - failure::on_fail::h6dd1e7919800eb47kCy
   3:     0x7ffe0c355c90 - unwind::begin_unwind_inner::hed0bde6d9fb3da6e0Qc
   4:     0x7ffe0a1ad980 - unwind::begin_unwind::h14872294802461399175
   5:     0x7ffe0a220390 - codemap::CodeMap::bytepos_to_file_charpos::he92e47167fafc5d7NkF
   6:     0x7ffe0a21ea00 - codemap::CodeMap::lookup_char_pos::h8f67dae5c50d7de6jdF
   7:     0x7ffe0cbc8820 - trans::debuginfo::set_source_location::hc856b9f5576c6f8cQmF
   8:     0x7ffe0ccea490 - trans::cleanup::LifetimeEnd.Cleanup<'tcx>::trans::hf7be10cbd942b3ffWnN
   9:     0x7ffe0cbc2350 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::h98c60c7ce929177aLaM
  10:     0x7ffe0cc64740 - trans::base::trans_closure::h2d576bfe5ee927a2tDu
  11:     0x7ffe0cc0a8a0 - trans::closure::trans_expr_fn::h960d345b7bd90b29CZy
  12:     0x7ffe0cbf48c0 - trans::expr::trans_rvalue_dps_unadjusted::hd6454136eddfd073wnj
  13:     0x7ffe0cbb1030 - trans::expr::trans_into::h21ce62673084950ehQh
  14:     0x7ffe0cbf48c0 - trans::expr::trans_rvalue_dps_unadjusted::hd6454136eddfd073wnj
  15:     0x7ffe0cbf3100 - trans::expr::trans_unadjusted::he13d84d9b3d75b34iJi
  16:     0x7ffe0cbb2960 - trans::expr::trans::h144c957bbc3b0c38dUh
  17:     0x7ffe0cbd8720 - trans::callee::trans::datum_callee::h6ec536d9aa4c6b7ePmg
  18:     0x7ffe0cbe62c0 - trans::callee::trans_call::closure.40864
  19:     0x7ffe0cbb9df0 - trans::callee::trans_call_inner::h1d40ce239697ef3dY5g
  20:     0x7ffe0cbe5ee0 - trans::callee::trans_call::h062189439456ee3ea0g
  21:     0x7ffe0cbf48c0 - trans::expr::trans_rvalue_dps_unadjusted::hd6454136eddfd073wnj
  22:     0x7ffe0cbb1030 - trans::expr::trans_into::h21ce62673084950ehQh
  23:     0x7ffe0cbb0430 - trans::controlflow::trans_stmt_semi::h4397bf973f5f306br1d
  24:     0x7ffe0cbaf9e0 - trans::controlflow::trans_stmt::h54ea715314dd2a99cXd
  25:     0x7ffe0cbb1600 - trans::controlflow::trans_block::he5cc88c0ba3d6142k2d
  26:     0x7ffe0cc64740 - trans::base::trans_closure::h2d576bfe5ee927a2tDu
  27:     0x7ffe0cba3600 - trans::base::trans_fn::h45c22622c0be15d1jPu
  28:     0x7ffe0cba05f0 - trans::base::trans_item::hfd6bcd0a16e07f623av
  29:     0x7ffe0cc690c0 - trans::base::trans_mod::h696cd29b0c68c44fygv
  30:     0x7ffe0cc6fee0 - trans::base::trans_crate::h71e6143e68c64059n9v
  31:     0x7ffe0cd1ca50 - driver::driver::phase_4_translate_to_llvm::h102b402fb473e39bPvS
  32:     0x7ffe0cd0a270 - driver::driver::compile_input::h5a8de67c93dd173512R
  33:     0x7ffe0cd8e2f0 - driver::run_compiler::h87d0535582c0ee1690T
  34:     0x7ffe0cd8e1e0 - driver::run::closure.59902
  35:     0x7ffe0cb95c70 - task::TaskBuilder::try_future::closure.39018
  36:     0x7ffe0c6cded0 - task::TaskBuilder::spawn_internal::closure.30320
  37:     0x7ffe0c353a70 - task::Task::spawn::closure.5727
  38:     0x7ffe0c3adde0 - rust_try_inner
  39:     0x7ffe0c3addd0 - rust_try
  40:     0x7ffe0c353b50 - unwind::try::hbf037d32604be4a0lFc
  41:     0x7ffe0c353910 - task::Task::run::h72992392502d45b4eOb
  42:     0x7ffe0c353500 - task::Task::spawn::closure.5703
  43:     0x7ffe0c354f50 - thread::thread_start::h26578c9615c56832h6b
  44:     0x7ffe0721cdc0 - start_thread
  45:                0x0 - <unknown>
