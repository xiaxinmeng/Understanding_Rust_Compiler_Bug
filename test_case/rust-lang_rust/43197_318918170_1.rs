rust
    Running2 `rustc --crate-name bug43197 src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=fd80fe04a98d7b0a -C extra-filename=-fd80fe04a98d7b0a --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/target/debug/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/target/debug/deps`
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/target/debug/deps:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! executing 'rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '--crate-name bug43197 src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=fd80fe04a98d7b0a -C extra-filename=-fd80fe04a98d7b0a --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/target/debug/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/target/debug/deps'
warning: constant evaluation error: attempt to subtract with overflow. This will become a HARD ERROR in the future
 --> src/main.rs:3:20
  |
3 |     const X: u32 = 33-44;//using src: https://github.com/rust-lang/rust/issues/43197#issuecomment-315016896
  |                    ^^^^^
  |
  = note: #[warn(const_err)] on by default

error[E0080]: constant evaluation error
 --> src/main.rs:3:20
  |
3 |     const X: u32 = 33-44;//using src: https://github.com/rust-lang/rust/issues/43197#issuecomment-315016896
  |                    ^^^^^ attempt to subtract with overflow

error: internal compiler error: src/librustc_trans/mir/constant.rs:396: _1 not initialized
 --> src/main.rs:4:20
  |
4 |     println!("{}", X);//bug
  |                    ^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-dev (53bf7903f 2017-07-29) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

!!!!! lvalue=_1 bk=Shared span=src/main.rs:4:20: 4:21
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:438:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:71
             at src/libstd/sys_common/backtrace.rs:60
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:380
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:390
   4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:610
   5: std::panicking::begin_panic
             at rust/src/libstd/panicking.rs:571
   6: rustc_errors::Handler::span_bug
             at rust/<panic macros>:3
   7: <std::thread::local::LocalKey<T>>::with
             at rust/src/librustc/session/mod.rs:858
             at rust/src/librustc/ty/context.rs:968
             at rust/src/librustc/ty/context.rs:957
             at rust/src/libstd/thread/local.rs:362
             at rust/src/libstd/thread/local.rs:276
   8: rustc::ty::context::tls::with_opt
             at rust/src/librustc/ty/context.rs:953
             at rust/src/librustc/ty/context.rs:968
   9: rustc::session::span_bug_fmt
             at rust/src/librustc/session/mod.rs:855
             at rust/src/librustc/session/mod.rs:848
  10: rustc_trans::mir::constant::MirConstContext::const_lvalue
             at src/librustc_trans/mir/constant.rs:396
             at rust/src/libcore/option.rs:370
             at src/librustc_trans/mir/constant.rs:395
  11: rustc_trans::mir::constant::MirConstContext::const_rvalue
             at src/librustc_trans/mir/constant.rs:729
  12: rustc_trans::mir::constant::MirConstContext::trans
             at src/librustc_trans/mir/constant.rs:281
  13: rustc_trans::mir::constant::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_constant
             at src/librustc_trans/mir/constant.rs:954
  14: rustc_trans::mir::operand::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_operand
             at src/librustc_trans/mir/operand.rs:298
  15: rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_rvalue_operand
             at src/librustc_trans/mir/rvalue.rs:472
  16: rustc_trans::mir::trans_mir
             at src/librustc_trans/mir/statement.rs:38
             at src/librustc_trans/mir/block.rs:49
             at src/librustc_trans/mir/mod.rs:324
  17: rustc_trans::base::trans_instance
             at src/librustc_trans/base.rs:608
  18: rustc_trans::trans_item::TransItem::define
             at src/librustc_trans/trans_item.rs:90
  19: rustc_trans::base::trans_crate::module_translation
             at src/librustc_trans/base.rs:1064
  20: rustc::dep_graph::graph::DepGraph::with_task
             at rust/src/librustc/dep_graph/graph.rs:125
  21: rustc_trans::base::trans_crate
             at src/librustc_trans/base.rs:994
             at rust/src/libcore/ops/function.rs:191
             at rust/src/libcore/option.rs:398
             at rust/src/libcore/iter/mod.rs:1073
             at rust/src/liballoc/vec.rs:1825
             at rust/src/liballoc/vec.rs:1808
             at rust/src/liballoc/vec.rs:1695
             at rust/src/libcore/iter/iterator.rs:1302
             at src/librustc_trans/base.rs:989
  22: rustc_driver::driver::phase_4_translate_to_llvm
             at src/librustc_driver/driver.rs:1070
             at rust/src/librustc/util/common.rs:48
             at src/librustc_driver/driver.rs:1068
  23: rustc_driver::driver::compile_input::{{closure}}
             at src/librustc_driver/driver.rs:211
  24: <std::thread::local::LocalKey<T>>::with
             at src/librustc_driver/driver.rs:1050
             at rust/src/librustc/ty/context.rs:941
             at rust/src/libstd/thread/local.rs:362
             at rust/src/libstd/thread/local.rs:276
  25: rustc::ty::context::tls::enter
             at rust/src/librustc/ty/context.rs:938
  26: <std::thread::local::LocalKey<T>>::with
             at rust/src/librustc/ty/context.rs:925
             at rust/src/libstd/thread/local.rs:362
             at rust/src/libstd/thread/local.rs:276
  27: rustc::ty::context::TyCtxt::create_and_enter
             at rust/src/librustc/ty/context.rs:922
             at rust/src/librustc/ty/context.rs:703
  28: rustc_driver::driver::compile_input
             at src/librustc_driver/driver.rs:963
             at src/librustc_driver/driver.rs:177
  29: rustc_driver::run_compiler
             at src/librustc_driver/lib.rs:218

error: Could not compile `bug43197`.
