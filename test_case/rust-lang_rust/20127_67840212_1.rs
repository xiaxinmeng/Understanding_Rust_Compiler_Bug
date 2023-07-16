 rust
$ rustc -g for.rs
for.rs:3:9: 3:10 error: internal compiler error: debuginfo::create_for_loop_var_metadata() - Referenced variable location is not an alloca!
for.rs:3     for i in range(0, 10i) {}
                 ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:     0x7f15831987f0 - sys::backtrace::write::hc1f09f1eae19d77aQxt
   2:     0x7f15831b9710 - failure::on_fail::h0fb87b850ab6cd900Xz
   3:     0x7f15831254a0 - rt::unwind::begin_unwind_inner::habde574b4f76ade20Bz
   4:     0x7f157e1c6f00 - rt::unwind::begin_unwind::h16328291673998487577
   5:     0x7f157e1c6e80 - diagnostic::SpanHandler::span_bug::hbb7934a78e265d5dSXF
   6:     0x7f15810a0350 - session::Session::span_bug::h8e056d7b667a2fcb1on
   7:     0x7f15820272a0 - middle::pat_util::pat_bindings::closure.49058
   8:     0x7f157e191d50 - ast_util::walk_pat::hab0b8805a667994dlxC
   9:     0x7f1581f54f90 - trans::expr::trans_rvalue_stmt_unadjusted::hb3ffc0f604cb73e4l4i
  10:     0x7f1581f0f2c0 - trans::expr::trans_into::h71668aa506e1a690uGh
  11:     0x7f1581f0fa20 - trans::controlflow::trans_block::hc167a4817215f70d9Zd
  12:     0x7f1581fbb310 - trans::base::trans_closure::h0f21c829c37744a86au
  13:     0x7f1581f03fd0 - trans::base::trans_fn::h5a95a6b70c2d9335nmu
  14:     0x7f1581eff520 - trans::base::trans_item::hf517e1dbb2128044rHu
  15:     0x7f1581fc3230 - trans::base::trans_crate::he5a814094c8a4da1HDv
  16:     0x7f15836f8f70 - driver::phase_4_translate_to_llvm::hc65b794b160978481Ca
  17:     0x7f15836cfe40 - driver::compile_input::hec1a59cac622cf55vba
  18:     0x7f1583877220 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h10851168864564136615
  19:     0x7f15838758d0 - rt::unwind::try::try_fn::h10692001439554181982
  20:     0x7f158321d3f0 - rust_try_inner
  21:     0x7f158321d3e0 - rust_try
  22:     0x7f1583875c00 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h18028200319240022992
  23:     0x7f15831a8f60 - sys::thread::thread_start::hdeb95f039f29863doww
  24:     0x7f157d9c9250 - start_thread
  25:     0x7f1582ddb219 - clone
  26:                0x0 - <unknown>
