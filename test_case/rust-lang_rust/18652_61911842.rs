
$ rustc issue_18652.rs 
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: freevars.len() <= 1', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/librustc/middle/trans/closure.rs:325

stack backtrace:
   1:     0x7f9ccb3cd8e0 - rt::backtrace::imp::write::h90851ece7c054d3bTVp
   2:     0x7f9ccb3d0960 - failure::on_fail::hc015761685342dd1vhq
   3:     0x7f9ccbb7fa20 - unwind::begin_unwind_inner::hf5743de3e545b8bbZSd
   4:     0x7f9ccbf59820 - unwind::begin_unwind::h12836636821598676233
   5:     0x7f9ccc3bec60 - middle::trans::closure::trans_unboxed_closure::closure.127438
   6:     0x7f9ccc3907f0 - middle::trans::base::trans_closure::h0ee0e830d5f57196BNh
   7:     0x7f9ccc33e380 - middle::trans::closure::trans_unboxed_closure::h10dd08b60eae5c99h9l
   8:     0x7f9ccc329860 - middle::trans::expr::trans_rvalue_dps_unadjusted::hbba4faa267b036ffeM6
   9:     0x7f9ccc2ef050 - middle::trans::expr::trans_into::haccb966391cea9aedn5
  10:     0x7f9ccc2ee4b0 - middle::trans::controlflow::trans_stmt_semi::h4d0d2d7f5be74d97ZC1
  11:     0x7f9ccc2eda60 - middle::trans::controlflow::trans_stmt::h6a80ddbefaced6ffMy1
  12:     0x7f9ccc2ef470 - middle::trans::controlflow::trans_block::h76660195897ae193SD1
  13:     0x7f9ccc3907f0 - middle::trans::base::trans_closure::h0ee0e830d5f57196BNh
  14:     0x7f9ccc2e2530 - middle::trans::base::trans_fn::h9de5c2dcb3f96df5wZh
  15:     0x7f9ccc2dfc20 - middle::trans::base::trans_item::h5f88c04c36ef66aaKii
  16:     0x7f9ccc39b160 - middle::trans::base::trans_crate::heef2f92cab82a2c9Lgj
  17:     0x7f9ccc818b30 - driver::driver::phase_4_translate_to_llvm::h424d161d8e05b6883UB
  18:     0x7f9ccc80f320 - driver::driver::compile_input::h0cb1ebcd11f34cd6XrB
  19:     0x7f9ccc893a60 - driver::run_compiler::h92ab17fc93f0a323giF
  20:     0x7f9ccc893950 - driver::run::closure.146866
  21:     0x7f9ccbf915f0 - task::TaskBuilder<S>::try_future::closure.104323
  22:     0x7f9ccbf913e0 - task::TaskBuilder<S>::spawn_internal::closure.104294
  23:     0x7f9ccd121b90 - task::NativeSpawner.Spawner::spawn::closure.8440
  24:     0x7f9ccbbd5510 - rust_try_inner
  25:     0x7f9ccbbd5500 - rust_try
  26:     0x7f9ccbb7d370 - unwind::try::h9b7bcf92a04d9d42jHd
  27:     0x7f9ccbb7d200 - task::Task::run::h74a092677bc826a59Mc
  28:     0x7f9ccd1218d0 - task::NativeSpawner.Spawner::spawn::closure.8378
  29:     0x7f9ccbb7ea10 - thread::thread_start::h7ebb3af704f6e202o8c
  30:     0x7f9ccae9e0c0 - start_thread
  31:     0x7f9ccb848f89 - __clone
  32:                0x0 - <unknown>
